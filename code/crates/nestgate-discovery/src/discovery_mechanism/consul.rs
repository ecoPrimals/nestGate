// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Consul-based discovery (cloud/datacenter)
//!
//! Provides Consul service discovery integration using the pure-Rust
//! bootstrap HTTP client — zero external HTTP dependencies.
//! Requires the `consul` feature flag.
//!
//! **Announce URL**: Registration requires self-knowledge `endpoints['api']`, `NESTGATE_API_URL`, or
//! `NESTGATE_CAPABILITY_HTTP_API_ENDPOINT`. No implicit localhost HTTP fallback for peer endpoints.
//!
//! **Consul agent URL**: Resolved from `NESTGATE_CONSUL_URL`, or `CONSUL_HTTP_ADDR`, or
//! `NESTGATE_CONSUL_HTTP_ADDR` (`Option<String>` resolution — no default bind to loopback).

use super::http::DiscoveryHttpClient;
use super::{Capability, DiscoveryBuilder, DiscoveryMechanism, ServiceInfo};
use crate::self_knowledge::SelfKnowledge;
use nestgate_config::constants::get_api_port;
use nestgate_config::constants::hardcoding::addresses::LOCALHOST_IPV4;
use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
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
#[allow(clippy::struct_field_names)]
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
    /// Resolves the Consul agent HTTP base URL from (in order):
    /// - `NESTGATE_CONSUL_URL`
    /// - `CONSUL_HTTP_ADDR` (`HashiCorp` convention)
    /// - `NESTGATE_CONSUL_HTTP_ADDR` (`NestGate` alias)
    ///
    /// If none are set, returns a configuration error (no implicit localhost default).
    ///
    /// # Errors
    ///
    /// Returns an error if the Consul URL is unset or the client cannot be constructed.
    pub fn new(builder: &DiscoveryBuilder) -> Result<Self> {
        let consul_addr: Option<String> = std::env::var("NESTGATE_CONSUL_URL")
            .ok()
            .or_else(|| std::env::var("CONSUL_HTTP_ADDR").ok())
            .or_else(|| std::env::var("NESTGATE_CONSUL_HTTP_ADDR").ok());

        let consul_addr = consul_addr.ok_or_else(|| {
            nestgate_types::error::NestGateError::configuration_error(
                "NESTGATE_CONSUL_URL",
                "Consul discovery requires NESTGATE_CONSUL_URL (or CONSUL_HTTP_ADDR / \
                 NESTGATE_CONSUL_HTTP_ADDR); no default loopback address is applied",
            )
        })?;

        let client = DiscoveryHttpClient::new(builder.timeout);

        Ok(Self {
            timeout: builder.timeout,
            _cache_duration: builder.cache_duration,
            consul_addr,
            client,
        })
    }

    /// Resolve the URL used to derive Consul registration address/port for announce.
    ///
    /// Order: self-knowledge `api` endpoint, `NESTGATE_API_URL`, then
    /// `NESTGATE_CAPABILITY_HTTP_API_ENDPOINT`. No implicit localhost — callers must
    /// provide at least one source.
    fn resolve_announce_primary_endpoint(self_knowledge: &SelfKnowledge) -> Result<String> {
        if let Some(addr) = self_knowledge.endpoints.get("api") {
            return Ok(addr.to_string());
        }
        if let Ok(url) = std::env::var("NESTGATE_API_URL") {
            return Ok(url);
        }
        if let Ok(url) = std::env::var("NESTGATE_CAPABILITY_HTTP_API_ENDPOINT") {
            return Ok(url);
        }
        Err(nestgate_types::error::NestGateError::api_error(
            "Cannot announce to Consul without an API endpoint: populate self-knowledge \
             endpoints['api'], or set NESTGATE_API_URL or NESTGATE_CAPABILITY_HTTP_API_ENDPOINT",
        ))
    }

    fn parse_endpoint(address: &str, port: u16) -> String {
        if address.is_empty() {
            let host = std::env::var("NESTGATE_CONSUL_SERVICE_ADDRESS_FALLBACK_HOST")
                .unwrap_or_else(|_| {
                    tracing::warn!(
                        "Consul returned an empty ServiceAddress; using {}. \
                         Set NESTGATE_CONSUL_SERVICE_ADDRESS_FALLBACK_HOST or fix Consul advertise \
                         configuration.",
                        LOCALHOST_IPV4
                    );
                    LOCALHOST_IPV4.to_string()
                });
            format!("http://{host}:{port}")
        } else {
            format!("http://{address}:{port}")
        }
    }

    /// Split `http(s)://host:port` from self-knowledge / capability env into host + port.
    ///
    /// When the URL omits an explicit port, uses [`get_api_port`] (`NESTGATE_API_PORT`, etc.)
    /// instead of assuming `8080`.
    fn extract_address_port(endpoint: &str) -> (String, u16) {
        let without_scheme = endpoint
            .trim_start_matches("http://")
            .trim_start_matches("https://");
        let default_port = get_api_port();
        if let Some((addr, port_str)) = without_scheme.rsplit_once(':') {
            let port = port_str.parse().unwrap_or(default_port);
            (addr.to_string(), port)
        } else {
            (without_scheme.to_string(), default_port)
        }
    }
}

impl DiscoveryMechanism for ConsulDiscovery {
    fn announce(
        &self,
        self_knowledge: &SelfKnowledge,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let self_knowledge = self_knowledge.clone();
        let client = self.client.clone();
        let consul_addr = self.consul_addr.clone();
        let timeout = self.timeout;

        Box::pin(async move {
            tracing::info!("Consul announce: {}", self_knowledge.name);

            let primary_endpoint = Self::resolve_announce_primary_endpoint(&self_knowledge)?;

            let (address, port) = Self::extract_address_port(&primary_endpoint);

            let health_check =
                self_knowledge
                    .endpoints
                    .get("health")
                    .map(|addr| ConsulHealthCheck {
                        http: addr.to_string(),
                        interval: "10s".to_string(),
                        timeout: format!("{}s", timeout.as_secs()),
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

            let url = format!("{consul_addr}/v1/agent/service/register");
            client.put_json(&url, &registration).await.map_err(|e| {
                nestgate_types::error::NestGateError::api_error(format!(
                    "Consul registration failed: {e}"
                ))
            })?;

            tracing::info!("Successfully registered with Consul");
            Ok(())
        })
    }

    fn find_by_capability(
        &self,
        capability: Capability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ServiceInfo>>> + Send + '_>> {
        let client = self.client.clone();
        let consul_addr = self.consul_addr.clone();
        Box::pin(async move {
            tracing::debug!("Consul query for capability: {:?}", capability);

            let url = format!("{consul_addr}/v1/catalog/service/{capability}");
            let response = client.get(&url).await.map_err(|e| {
                nestgate_types::error::NestGateError::api_error(format!("Consul query failed: {e}"))
            })?;

            if !response.is_success() {
                return Ok(vec![]);
            }

            let services: Vec<ConsulService> = response.json().map_err(|e| {
                nestgate_types::error::NestGateError::api_error(format!(
                    "Failed to parse Consul response: {e}"
                ))
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
        })
    }

    fn find_by_id(
        &self,
        id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ServiceInfo>>> + Send + '_>> {
        let client = self.client.clone();
        let consul_addr = self.consul_addr.clone();
        let id = id.to_string();
        Box::pin(async move {
            tracing::debug!("Consul lookup service: {}", id);

            let url = format!("{consul_addr}/v1/agent/service/{id}");
            let response = client.get(&url).await.map_err(|e| {
                nestgate_types::error::NestGateError::api_error(format!(
                    "Consul lookup failed: {e}"
                ))
            })?;

            if !response.is_success() {
                return Ok(None);
            }

            let service: ConsulService = response.json().map_err(|e| {
                nestgate_types::error::NestGateError::api_error(format!(
                    "Failed to parse Consul response: {e}"
                ))
            })?;

            Ok(Some(ServiceInfo {
                id: service.service_id,
                name: service.service_name,
                capabilities: service.service_tags,
                endpoint: Self::parse_endpoint(&service.service_address, service.service_port),
                metadata: service.service_meta,
                health_endpoint: None,
            }))
        })
    }

    fn health_check(
        &self,
        service_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>> {
        let client = self.client.clone();
        let consul_addr = self.consul_addr.clone();
        let service_id = service_id.to_string();
        Box::pin(async move {
            tracing::debug!("Consul health check: {}", service_id);

            let url = format!("{consul_addr}/v1/health/service/{service_id}");
            let response = client.get(&url).await.map_err(|e| {
                nestgate_types::error::NestGateError::api_error(format!(
                    "Consul health check failed: {e}"
                ))
            })?;

            Ok(response.is_success())
        })
    }

    fn deregister(
        &self,
        service_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let client = self.client.clone();
        let consul_addr = self.consul_addr.clone();
        let service_id = service_id.to_string();
        Box::pin(async move {
            tracing::info!("Consul deregister: {}", service_id);

            let url = format!("{consul_addr}/v1/agent/service/deregister/{service_id}");
            client
                .put_json(&url, &serde_json::Value::Null)
                .await
                .map_err(|e| {
                    nestgate_types::error::NestGateError::api_error(format!(
                        "Consul deregistration failed: {e}"
                    ))
                })?;

            tracing::info!("Successfully deregistered from Consul");
            Ok(())
        })
    }

    fn mechanism_name(&self) -> &'static str {
        "consul"
    }
}
