//! # Discovery Mechanism Abstraction
//!
//! **Vendor-agnostic service discovery for the infant discovery pattern.**
//!
//! This module provides a pluggable discovery mechanism that works across
//! different infrastructure: bare metal (mDNS), cloud (consul/etcd), or
//! orchestrated (Kubernetes).
//!
//! ## Philosophy
//!
//! Each primal starts with **zero knowledge** and discovers its ecosystem
//! at runtime using whatever discovery mechanism is available:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │ Primal Startup (Infant Discovery)                   │
//! ├─────────────────────────────────────────────────────┤
//! │                                                      │
//! │  1. Self-Awareness: "I am NestGate, I provide      │
//! │     Storage and ZFS capabilities"                   │
//! │                                                      │
//! │  2. Detect Discovery: Auto-detect available         │
//! │     mechanism (k8s? consul? mdns?)                  │
//! │                                                      │
//! │  3. Announce: Register my capabilities               │
//! │                                                      │
//! │  4. Discover: Find primals by capability             │
//! │     (not by name!)                                   │
//! │                                                      │
//! └─────────────────────────────────────────────────────┘
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! use nestgate_core::discovery_mechanism::{DiscoveryMechanism, ServiceInfo};
//! use nestgate_core::self_knowledge::SelfKnowledge;
//! use nestgate_core::capabilities::Capability;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // 1. Auto-detect available discovery mechanism
//! let discovery = DiscoveryMechanism::detect().await?
//!     .unwrap_or(DiscoveryMechanism::mdns_default());
//!
//! // 2. Build self-knowledge (only know ourselves)
//! let self_knowledge = SelfKnowledge::builder()
//!     .with_name("nestgate")
//!     .with_capability(Capability::Storage)
//!     .with_capability(Capability::ZfsManagement)
//!     .build()?;
//!
//! // 3. Announce ourselves
//! discovery.announce(&self_knowledge).await?;
//!
//! // 4. Discover others by capability (not by name!)
//! let orchestrators = discovery
//!     .find_by_capability(Capability::Orchestration)
//!     .await?;
//!
//! for orch in orchestrators {
//!     println!("Found orchestrator: {} at {}", orch.id, orch.endpoint);
//! }
//! # Ok(())
//! # }
//! ```

use crate::http_client_stub as reqwest;
use crate::self_knowledge::SelfKnowledge;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Simplified capability type for service discovery
///
/// Represents a capability that a service can provide or request.
/// This is a simplified string-based representation that will eventually
/// use the proper capability taxonomy from `crate::capabilities::taxonomy`.
///
/// # Examples
/// - "storage"
/// - "compute"
/// - "orchestration"
/// - "security"
pub type Capability = String;

/// Service discovery information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Unique service ID
    pub id: String,
    /// Service name (for logging/debugging only, not for discovery!)
    pub name: String,
    /// Capabilities this service provides
    pub capabilities: Vec<Capability>,
    /// Primary endpoint
    pub endpoint: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Health check endpoint
    pub health_endpoint: Option<String>,
}

/// Discovery mechanism trait
///
/// Implemented by different discovery backends (mDNS, Consul, Kubernetes, etc.)
#[async_trait::async_trait]
pub trait DiscoveryMechanism: Send + Sync {
    /// Announce this primal's presence
    async fn announce(&self, self_knowledge: &SelfKnowledge) -> Result<()>;

    /// Find services by capability
    async fn find_by_capability(&self, capability: Capability) -> Result<Vec<ServiceInfo>>;

    /// Find a specific service by ID (for re-connection)
    async fn find_by_id(&self, id: &str) -> Result<Option<ServiceInfo>>;

    /// Health check: is this service still available?
    async fn health_check(&self, service_id: &str) -> Result<bool>;

    /// Deregister this primal (graceful shutdown)
    async fn deregister(&self, service_id: &str) -> Result<()>;

    /// Get mechanism name (for logging)
    fn mechanism_name(&self) -> &'static str;
}

/// Discovery mechanism builder
pub struct DiscoveryBuilder {
    /// Timeout for discovery operations
    timeout: Duration,
    /// Cache duration for discovered services
    cache_duration: Duration,
    /// Preferred mechanism (if multiple available)
    preferred_mechanism: Option<String>,
}

impl Default for DiscoveryBuilder {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            cache_duration: Duration::from_secs(60),
            preferred_mechanism: None,
        }
    }
}

impl DiscoveryBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set operation timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set cache duration
    pub fn with_cache_duration(mut self, duration: Duration) -> Self {
        self.cache_duration = duration;
        self
    }

    /// Prefer a specific mechanism
    pub fn prefer_mechanism(mut self, mechanism: impl Into<String>) -> Self {
        self.preferred_mechanism = Some(mechanism.into());
        self
    }

    /// Auto-detect best available discovery mechanism
    ///
    /// Detection order (by preference):
    /// 1. Kubernetes (if KUBERNETES_SERVICE_HOST set)
    /// 2. Consul (if CONSUL_HTTP_ADDR set)
    /// 3. mDNS (default fallback)
    pub async fn detect(self) -> Result<Box<dyn DiscoveryMechanism>> {
        // Check for kubernetes
        if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
            #[cfg(feature = "kubernetes")]
            return Ok(Box::new(k8s::KubernetesDiscovery::new(self).await?));

            #[cfg(not(feature = "kubernetes"))]
            tracing::info!("Kubernetes detected but feature not enabled, falling back");
        }

        // Check for consul
        if std::env::var("CONSUL_HTTP_ADDR").is_ok() {
            #[cfg(feature = "consul")]
            return Ok(Box::new(consul::ConsulDiscovery::new(self).await?));

            #[cfg(not(feature = "consul"))]
            tracing::info!("Consul detected but feature not enabled, falling back");
        }

        // Default to mDNS
        Ok(Box::new(mdns::MdnsDiscovery::new(self).await?))
    }

    /// Build mDNS discovery (default)
    pub async fn build_mdns(self) -> Result<Box<dyn DiscoveryMechanism>> {
        Ok(Box::new(mdns::MdnsDiscovery::new(self).await?))
    }

    /// Build Consul discovery
    #[cfg(feature = "consul")]
    pub async fn build_consul(self) -> Result<Box<dyn DiscoveryMechanism>> {
        Ok(Box::new(consul::ConsulDiscovery::new(self).await?))
    }

    /// Build Kubernetes discovery
    #[cfg(feature = "kubernetes")]
    pub async fn build_kubernetes(self) -> Result<Box<dyn DiscoveryMechanism>> {
        Ok(Box::new(k8s::KubernetesDiscovery::new(self).await?))
    }
}

/// mDNS-based discovery (default, works everywhere)
pub mod mdns {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    /// In-memory service registry for mDNS
    ///
    /// This is a simple implementation that stores services in memory.
    /// In production, this would use actual mDNS protocol (avahi-daemon, dns-sd, etc.)
    type ServiceRegistry = Arc<RwLock<HashMap<String, ServiceInfo>>>;

    /// mDNS discovery mechanism
    pub struct MdnsDiscovery {
        timeout: Duration,
        cache_duration: Duration,
        registry: ServiceRegistry,
        announced_service_id: Arc<RwLock<Option<String>>>,
    }

    impl MdnsDiscovery {
        /// Create new mDNS discovery
        pub async fn new(builder: DiscoveryBuilder) -> Result<Self> {
            Ok(Self {
                timeout: builder.timeout,
                cache_duration: builder.cache_duration,
                registry: Arc::new(RwLock::new(HashMap::new())),
                announced_service_id: Arc::new(RwLock::new(None)),
            })
        }

        /// Create service info from self-knowledge
        fn create_service_info(self_knowledge: &SelfKnowledge) -> ServiceInfo {
            let primary_endpoint = self_knowledge
                .endpoints
                .get("api")
                .map(|addr| addr.to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let health_endpoint = self_knowledge
                .endpoints
                .get("health")
                .map(|addr| addr.to_string());

            let mut metadata = HashMap::new();
            metadata.insert("version".to_string(), self_knowledge.version.clone());
            metadata.insert(
                "endpoints".to_string(),
                format!("{:?}", self_knowledge.endpoints),
            );

            ServiceInfo {
                id: self_knowledge.id.as_str().to_string(),
                name: self_knowledge.name.clone(),
                capabilities: self_knowledge.capabilities.clone(),
                endpoint: primary_endpoint,
                metadata,
                health_endpoint,
            }
        }
    }

    #[async_trait::async_trait]
    impl DiscoveryMechanism for MdnsDiscovery {
        async fn announce(&self, self_knowledge: &SelfKnowledge) -> Result<()> {
            tracing::info!(
                "mDNS announce: {} with capabilities: {:?}",
                self_knowledge.name,
                self_knowledge.capabilities
            );

            let service_info = Self::create_service_info(self_knowledge);
            let service_id = service_info.id.clone();

            // Store in registry
            let mut registry = self.registry.write().await;
            registry.insert(service_id.clone(), service_info);

            // Remember our service ID
            let mut announced = self.announced_service_id.write().await;
            *announced = Some(service_id);

            tracing::info!("Successfully announced to mDNS registry");
            Ok(())
        }

        async fn find_by_capability(&self, capability: Capability) -> Result<Vec<ServiceInfo>> {
            tracing::debug!("mDNS query for capability: {:?}", capability);

            let registry = self.registry.read().await;
            let matching_services: Vec<ServiceInfo> = registry
                .values()
                .filter(|service| service.capabilities.contains(&capability))
                .cloned()
                .collect();

            tracing::debug!(
                "Found {} services with capability '{}'",
                matching_services.len(),
                capability
            );

            Ok(matching_services)
        }

        async fn find_by_id(&self, id: &str) -> Result<Option<ServiceInfo>> {
            tracing::debug!("mDNS lookup service: {}", id);

            let registry = self.registry.read().await;
            let service = registry.get(id).cloned();

            if service.is_some() {
                tracing::debug!("Found service: {}", id);
            } else {
                tracing::debug!("Service not found: {}", id);
            }

            Ok(service)
        }

        async fn health_check(&self, service_id: &str) -> Result<bool> {
            tracing::debug!("mDNS health check: {}", service_id);

            // Check if service exists in registry
            let registry = self.registry.read().await;
            let healthy = registry.contains_key(service_id);

            tracing::debug!("Service {} health: {}", service_id, healthy);
            Ok(healthy)
        }

        async fn deregister(&self, service_id: &str) -> Result<()> {
            tracing::info!("mDNS deregister: {}", service_id);

            let mut registry = self.registry.write().await;
            registry.remove(service_id);

            // Clear announced service if it was us
            let mut announced = self.announced_service_id.write().await;
            if announced.as_ref().map(|id| id.as_str()) == Some(service_id) {
                *announced = None;
            }

            tracing::info!("Successfully deregistered from mDNS registry");
            Ok(())
        }

        fn mechanism_name(&self) -> &'static str {
            "mdns"
        }
    }
}

/// Consul-based discovery (cloud/datacenter)
#[cfg(feature = "consul")]
pub mod consul {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

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

    /// Consul discovery mechanism (uses Consul HTTP API via reqwest)
    pub struct ConsulDiscovery {
        timeout: Duration,
        cache_duration: Duration,
        consul_addr: String,
        client: reqwest::Client,
    }

    impl ConsulDiscovery {
        /// Create new Consul discovery
        pub async fn new(builder: DiscoveryBuilder) -> Result<Self> {
            let consul_addr = std::env::var("CONSUL_HTTP_ADDR")
                .unwrap_or_else(|_| "http://localhost:8500".to_string());

            let client = reqwest::Client::builder()
                .timeout(builder.timeout)
                .build()
                .map_err(|e| crate::error::NestGateError::config(&format!("Failed to create HTTP client: {}", e)))?;

            Ok(Self {
                timeout: builder.timeout,
                cache_duration: builder.cache_duration,
                consul_addr,
                client,
            })
        }

        /// Parse endpoint from address string
        fn parse_endpoint(address: &str, port: u16) -> String {
            if address.is_empty() {
                format!("http://localhost:{}", port)
            } else {
                format!("http://{}:{}", address, port)
            }
        }

        /// Extract address and port from endpoint
        fn extract_address_port(endpoint: &str) -> (String, u16) {
            let without_scheme = endpoint.trim_start_matches("http://").trim_start_matches("https://");
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
            meta.insert("capabilities".to_string(), serde_json::to_string(&self_knowledge.capabilities).unwrap_or_default());

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
                .put(&url)
                .json(&registration)
                .send()
                .await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("Consul registration failed: {}", e)))?;

            tracing::info!("Successfully registered with Consul");
            Ok(())
        }

        async fn find_by_capability(&self, capability: Capability) -> Result<Vec<ServiceInfo>> {
            tracing::debug!("Consul query for capability: {:?}", capability);

            let url = format!("{}/v1/catalog/service/{}", self.consul_addr, capability);
            let response = self.client
                .get(&url)
                .send()
                .await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("Consul query failed: {}", e)))?;

            if !response.status().is_success() {
                return Ok(vec![]);
            }

            let services: Vec<ConsulService> = response
                .json()
                .await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("Failed to parse Consul response: {}", e)))?;

            Ok(services.into_iter().map(|svc| {
                ServiceInfo {
                    id: svc.service_id,
                    name: svc.service_name,
                    capabilities: svc.service_tags,
                    endpoint: Self::parse_endpoint(&svc.service_address, svc.service_port),
                    metadata: svc.service_meta,
                    health_endpoint: None,
                }
            }).collect())
        }

        async fn find_by_id(&self, id: &str) -> Result<Option<ServiceInfo>> {
            tracing::debug!("Consul lookup service: {}", id);

            let url = format!("{}/v1/agent/service/{}", self.consul_addr, id);
            let response = self.client
                .get(&url)
                .send()
                .await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("Consul lookup failed: {}", e)))?;

            if !response.status().is_success() {
                return Ok(None);
            }

            let service: ConsulService = response
                .json()
                .await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("Failed to parse Consul response: {}", e)))?;

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
            let response = self.client
                .get(&url)
                .send()
                .await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("Consul health check failed: {}", e)))?;

            Ok(response.status().is_success())
        }

        async fn deregister(&self, service_id: &str) -> Result<()> {
            tracing::info!("Consul deregister: {}", service_id);

            let url = format!("{}/v1/agent/service/deregister/{}", self.consul_addr, service_id);
            self.client
                .put(&url)
                .send()
                .await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("Consul deregistration failed: {}", e)))?;

            tracing::info!("Successfully deregistered from Consul");
            Ok(())
        }

        fn mechanism_name(&self) -> &'static str {
            "consul"
        }
    }
}

/// Kubernetes-based discovery (orchestrated)
#[cfg(feature = "kubernetes")]
pub mod k8s {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    /// Kubernetes service metadata
    #[derive(Debug, Serialize, Deserialize)]
    struct K8sMetadata {
        name: String,
        namespace: String,
        labels: HashMap<String, String>,
        annotations: HashMap<String, String>,
    }

    /// Kubernetes service spec
    #[derive(Debug, Serialize, Deserialize)]
    struct K8sServiceSpec {
        ports: Vec<K8sServicePort>,
        #[serde(rename = "clusterIP")]
        cluster_ip: Option<String>,
    }

    /// Kubernetes service port
    #[derive(Debug, Serialize, Deserialize)]
    struct K8sServicePort {
        port: i32,
        #[serde(rename = "targetPort", skip_serializing_if = "Option::is_none")]
        target_port: Option<i32>,
        protocol: String,
    }

    /// Kubernetes service
    #[derive(Debug, Serialize, Deserialize)]
    struct K8sService {
        metadata: K8sMetadata,
        spec: K8sServiceSpec,
    }

    /// Kubernetes service list
    #[derive(Debug, Deserialize)]
    struct K8sServiceList {
        items: Vec<K8sService>,
    }

    /// Kubernetes discovery mechanism (uses k8s REST API via reqwest)
    pub struct KubernetesDiscovery {
        timeout: Duration,
        cache_duration: Duration,
        namespace: String,
        client: reqwest::Client,
        api_server: String,
        token: Option<String>,
    }

    impl KubernetesDiscovery {
        /// Create new Kubernetes discovery
        pub async fn new(builder: DiscoveryBuilder) -> Result<Self> {
            let namespace = std::env::var("NAMESPACE")
                .or_else(|_| std::env::var("POD_NAMESPACE"))
                .unwrap_or_else(|_| "default".to_string());

            // Get k8s API server address
            let api_server = std::env::var("KUBERNETES_SERVICE_HOST")
                .map(|host| {
                    let port = std::env::var("KUBERNETES_SERVICE_PORT").unwrap_or_else(|_| "443".to_string());
                    format!("https://{}:{}", host, port)
                })
                .unwrap_or_else(|_| "https://kubernetes.default.svc".to_string());

            // Get service account token (if running in pod)
            let token = std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/token").ok();

            let client = reqwest::Client::builder()
                .timeout(builder.timeout)
                .danger_accept_invalid_certs(true) // In-cluster certs are self-signed
                .build()
                .map_err(|e| crate::error::NestGateError::config(&format!("Failed to create HTTP client: {}", e)))?;

            Ok(Self {
                timeout: builder.timeout,
                cache_duration: builder.cache_duration,
                namespace,
                client,
                api_server,
                token,
            })
        }

        /// Create authorization header
        fn auth_header(&self) -> Option<String> {
            self.token.as_ref().map(|token| format!("Bearer {}", token))
        }

        /// Convert k8s service to ServiceInfo
        fn service_to_info(&self, svc: K8sService) -> Option<ServiceInfo> {
            let port = svc.spec.ports.first()?.port;
            let ip = svc.spec.cluster_ip.as_ref()?;
            
            let capabilities: Vec<String> = svc.metadata.labels
                .get("capabilities")
                .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default();

            Some(ServiceInfo {
                id: format!("{}.{}", svc.metadata.name, svc.metadata.namespace),
                name: svc.metadata.name.clone(),
                capabilities,
                endpoint: format!("http://{}:{}", ip, port),
                metadata: svc.metadata.annotations,
                health_endpoint: None,
            })
        }
    }

    #[async_trait::async_trait]
    impl DiscoveryMechanism for KubernetesDiscovery {
        async fn announce(&self, self_knowledge: &SelfKnowledge) -> Result<()> {
            tracing::info!("k8s announce: {}", self_knowledge.name);

            // In Kubernetes, services are typically pre-created via manifests
            // This would update labels/annotations on an existing service
            // For now, we log that the service should be defined in k8s manifests

            tracing::info!(
                "Kubernetes services should be defined via manifests with labels: capabilities={}",
                self_knowledge.capabilities.join(",")
            );

            // In a full implementation, this could update the service's labels
            // via PATCH /api/v1/namespaces/{namespace}/services/{name}

            Ok(())
        }

        async fn find_by_capability(&self, capability: Capability) -> Result<Vec<ServiceInfo>> {
            tracing::debug!("k8s query for capability: {:?}", capability);

            let url = format!(
                "{}/api/v1/namespaces/{}/services?labelSelector=capabilities={}",
                self.api_server, self.namespace, capability
            );

            let mut req = self.client.get(&url);
            if let Some(auth) = self.auth_header() {
                req = req.header("Authorization", auth);
            }

            let response = req.send().await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("k8s query failed: {}", e)))?;

            if !response.status().is_success() {
                return Ok(vec![]);
            }

            let service_list: K8sServiceList = response.json().await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("Failed to parse k8s response: {}", e)))?;

            Ok(service_list.items.into_iter()
                .filter_map(|svc| self.service_to_info(svc))
                .collect())
        }

        async fn find_by_id(&self, id: &str) -> Result<Option<ServiceInfo>> {
            tracing::debug!("k8s lookup service: {}", id);

            // ID format: service-name.namespace
            let (name, ns) = if let Some((n, ns)) = id.split_once('.') {
                (n, ns.to_string())
            } else {
                (id, self.namespace.clone())
            };

            let url = format!(
                "{}/api/v1/namespaces/{}/services/{}",
                self.api_server, ns, name
            );

            let mut req = self.client.get(&url);
            if let Some(auth) = self.auth_header() {
                req = req.header("Authorization", auth);
            }

            let response = req.send().await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("k8s lookup failed: {}", e)))?;

            if !response.status().is_success() {
                return Ok(None);
            }

            let service: K8sService = response.json().await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("Failed to parse k8s response: {}", e)))?;

            Ok(self.service_to_info(service))
        }

        async fn health_check(&self, service_id: &str) -> Result<bool> {
            tracing::debug!("k8s health check: {}", service_id);

            // Check if service exists and has endpoints
            let (name, ns) = if let Some((n, ns)) = service_id.split_once('.') {
                (n, ns.to_string())
            } else {
                (service_id, self.namespace.clone())
            };

            let url = format!(
                "{}/api/v1/namespaces/{}/endpoints/{}",
                self.api_server, ns, name
            );

            let mut req = self.client.get(&url);
            if let Some(auth) = self.auth_header() {
                req = req.header("Authorization", auth);
            }

            let response = req.send().await
                .map_err(|e| crate::error::NestGateError::api_error(&format!("k8s health check failed: {}", e)))?;

            Ok(response.status().is_success())
        }

        async fn deregister(&self, service_id: &str) -> Result<()> {
            tracing::info!("k8s deregister: {}", service_id);

            // In Kubernetes, services persist and are managed by k8s
            // Deregistration typically means the pod terminates and k8s removes it from endpoints
            // We just log this action

            tracing::info!("Kubernetes services are managed by k8s control plane");
            Ok(())
        }

        fn mechanism_name(&self) -> &'static str {
            "kubernetes"
        }
    }
}

/// Testing utilities
pub mod testing;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mdns_discovery_creation() {
        let discovery = DiscoveryBuilder::new()
            .with_timeout(Duration::from_secs(10))
            .build_mdns()
            .await;

        assert!(discovery.is_ok());
        let discovery = discovery.unwrap();
        assert_eq!(discovery.mechanism_name(), "mdns");
    }

    #[tokio::test]
    async fn test_auto_detect_defaults_to_mdns() {
        // When no discovery env vars set, should default to mDNS
        let discovery = DiscoveryBuilder::new().detect().await;

        assert!(discovery.is_ok());
        let discovery = discovery.unwrap();
        assert_eq!(discovery.mechanism_name(), "mdns");
    }

    #[tokio::test]
    async fn test_mdns_announce_and_find() {
        let discovery = DiscoveryBuilder::new().build_mdns().await.unwrap();

        // Create and announce a service
        let self_knowledge = SelfKnowledge::builder()
            .with_id("test-storage")
            .with_name("Test Storage")
            .with_version("1.0.0")
            .with_capability("storage")
            .with_capability("zfs")
            .with_endpoint("api", "0.0.0.0:8080".parse().unwrap())
            .build()
            .unwrap();

        discovery.announce(&self_knowledge).await.unwrap();

        // Should be able to find by capability
        let storage_services = discovery
            .find_by_capability("storage".to_string())
            .await
            .unwrap();

        assert_eq!(storage_services.len(), 1);
        assert_eq!(storage_services[0].id, "test-storage");
        assert_eq!(storage_services[0].name, "Test Storage");
        assert!(storage_services[0]
            .capabilities
            .contains(&"storage".to_string()));
    }
}
