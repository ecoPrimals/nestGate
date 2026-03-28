//! Consul-based discovery (cloud/datacenter)
//!
//! Provides Consul service discovery integration using the pure-Rust
//! bootstrap HTTP client — zero external HTTP dependencies.
//! Requires the `consul` feature flag.

use super::http::DiscoveryHttpClient;
use super::{Capability, DiscoveryBuilder, DiscoveryMechanism, ServiceInfo};
use crate::self_knowledge::SelfKnowledge;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Consul service registration payload
#[derive(Debug, Serialize)]
struct ConsulServiceRegistration {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Tags")]
    tags: Vec<String>,
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "Port")]
    port: u16,
    #[serde(rename = "Check", skip_serializing_if = "Option::is_none")]
    check: Option<ConsulHealthCheck>,
    #[serde(rename = "Meta")]
    meta: HashMap<String, String>,
}

/// Consul health check configuration
#[derive(Debug, Serialize)]
struct ConsulHealthCheck {
    #[serde(rename = "HTTP")]
    http: String,
    #[serde(rename = "Interval")]
    interval: String,
    #[serde(rename = "Timeout")]
    timeout: String,
}

/// Consul service query response
#[derive(Debug, Deserialize)]
struct ConsulService {
    #[serde(rename = "ServiceID")]
    service_id: String,
    #[serde(rename = "ServiceName")]
    service_name: String,
    #[serde(rename = "ServiceTags")]
    service_tags: Vec<String>,
    #[serde(rename = "ServiceAddress")]
    service_address: String,
    #[serde(rename = "ServicePort")]
    service_port: u16,
    #[serde(rename = "ServiceMeta")]
    service_meta: HashMap<String, String>,
}

/// Consul discovery mechanism (pure-Rust HTTP, no external deps)
pub struct ConsulDiscovery {
    timeout: Duration,
    _cache_duration: Duration,
    consul_addr: String,
    client: DiscoveryHttpClient,
}

impl ConsulDiscovery {
    /// Create a new Consul discovery instance.
    ///
    /// Reads `CONSUL_HTTP_ADDR` from environment (default: `http://localhost:8500`).
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be constructed.
    pub async fn new(builder: DiscoveryBuilder) -> Result<Self> {
        let consul_addr = std::env::var("CONSUL_HTTP_ADDR")
            .unwrap_or_else(|_| "http://localhost:8500".to_string());

        let client = DiscoveryHttpClient::new(builder.timeout);

        Ok(Self {
            timeout: builder.timeout,
            _cache_duration: builder.cache_duration,
            consul_addr,
            client,
        })
    }

    fn parse_endpoint(address: &str, port: u16) -> String {
        if address.is_empty() {
            format!("http://localhost:{port}")
        } else {
            format!("http://{address}:{port}")
        }
    }

    fn extract_address_port(endpoint: &str) -> (String, u16) {
        let without_scheme = endpoint
            .trim_start_matches("http://")
            .trim_start_matches("https://");
        if let Some((addr, port_str)) = without_scheme.rsplit_once(':') {
            let port = port_str.parse().unwrap_or(8080);
            (addr.to_string(), port)
        } else {
            (without_scheme.to_string(), 8080)
        }
    }
}

#[async_trait::async_trait]
impl DiscoveryMechanism for ConsulDiscovery {
    async fn announce(&self, self_knowledge: &SelfKnowledge) -> Result<()> {
        tracing::info!("Consul announce: {}", self_knowledge.name);

        let primary_endpoint = self_knowledge
            .endpoints
            .get("api")
            .map(|addr| addr.to_string())
            .or_else(|| std::env::var("NESTGATE_API_URL").ok())
            .unwrap_or_else(|| "http://localhost:8080".to_string());

        let (address, port) = Self::extract_address_port(&primary_endpoint);

        let health_check = self_knowledge
            .endpoints
            .get("health")
            .map(|addr| ConsulHealthCheck {
                http: addr.to_string(),
                interval: "10s".to_string(),
                timeout: format!("{}s", self.timeout.as_secs()),
            });

        let mut meta = HashMap::new();
        meta.insert("version".to_string(), self_knowledge.version.clone());
        meta.insert(
            "capabilities".to_string(),
            serde_json::to_string(&self_knowledge.capabilities).unwrap_or_default(),
        );

        let registration = ConsulServiceRegistration {
            id: self_knowledge.id.as_str().to_string(),
            name: self_knowledge.name.clone(),
            tags: self_knowledge.capabilities.clone(),
            address,
            port,
            check: health_check,
            meta,
        };

        let url = format!("{}/v1/agent/service/register", self.consul_addr);
        self.client
            .put_json(&url, &registration)
            .await
            .map_err(|e| {
                crate::error::NestGateError::api_error(&format!("Consul registration failed: {e}"))
            })?;

        tracing::info!("Successfully registered with Consul");
        Ok(())
    }

    async fn find_by_capability(&self, capability: Capability) -> Result<Vec<ServiceInfo>> {
        tracing::debug!("Consul query for capability: {:?}", capability);

        let url = format!("{}/v1/catalog/service/{}", self.consul_addr, capability);
        let response = self.client.get(&url).await.map_err(|e| {
            crate::error::NestGateError::api_error(&format!("Consul query failed: {e}"))
        })?;

        if !response.is_success() {
            return Ok(vec![]);
        }

        let services: Vec<ConsulService> = response.json().map_err(|e| {
            crate::error::NestGateError::api_error(&format!("Failed to parse Consul response: {e}"))
        })?;

        Ok(services
            .into_iter()
            .map(|svc| ServiceInfo {
                id: svc.service_id,
                name: svc.service_name,
                capabilities: svc.service_tags,
                endpoint: Self::parse_endpoint(&svc.service_address, svc.service_port),
                metadata: svc.service_meta,
                health_endpoint: None,
            })
            .collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<ServiceInfo>> {
        tracing::debug!("Consul lookup service: {}", id);

        let url = format!("{}/v1/agent/service/{}", self.consul_addr, id);
        let response = self.client.get(&url).await.map_err(|e| {
            crate::error::NestGateError::api_error(&format!("Consul lookup failed: {e}"))
        })?;

        if !response.is_success() {
            return Ok(None);
        }

        let service: ConsulService = response.json().map_err(|e| {
            crate::error::NestGateError::api_error(&format!("Failed to parse Consul response: {e}"))
        })?;

        Ok(Some(ServiceInfo {
            id: service.service_id,
            name: service.service_name,
            capabilities: service.service_tags,
            endpoint: Self::parse_endpoint(&service.service_address, service.service_port),
            metadata: service.service_meta,
            health_endpoint: None,
        }))
    }

    async fn health_check(&self, service_id: &str) -> Result<bool> {
        tracing::debug!("Consul health check: {}", service_id);

        let url = format!("{}/v1/health/service/{}", self.consul_addr, service_id);
        let response = self.client.get(&url).await.map_err(|e| {
            crate::error::NestGateError::api_error(&format!("Consul health check failed: {e}"))
        })?;

        Ok(response.is_success())
    }

    async fn deregister(&self, service_id: &str) -> Result<()> {
        tracing::info!("Consul deregister: {}", service_id);

        let url = format!(
            "{}/v1/agent/service/deregister/{}",
            self.consul_addr, service_id
        );
        self.client
            .put_json(&url, &serde_json::Value::Null)
            .await
            .map_err(|e| {
                crate::error::NestGateError::api_error(&format!(
                    "Consul deregistration failed: {e}"
                ))
            })?;

        tracing::info!("Successfully deregistered from Consul");
        Ok(())
    }

    fn mechanism_name(&self) -> &'static str {
        "consul"
    }
}
