use crate::universal_adapter::{PrimalAgnosticAdapter, CapabilityCategory, CapabilityRequest};
/// **UNIVERSAL SECURITY CLIENT IMPLEMENTATION**
/// Universal Security Error types
#[derive(Debug, Clone)]
pub enum UniversalSecurityError {
    Network(String),
    Configuration(String),
    Timeout(String),
    Authentication(String),
}
impl std::fmt::Display for UniversalSecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UniversalSecurityError::Network(msg) => write!(f, "Network error: {msg}"),
            UniversalSecurityError::Configuration(msg) => write!(f, "Configuration error: {msg}"),
            UniversalSecurityError::Timeout(msg) => write!(f, "Timeout error: {msg}"),
            UniversalSecurityError::Authentication(msg) => {
                write!(f, "Authentication error: {msg}")
            }
        }
    }
}

impl std::error::Error for UniversalSecurityError {}
use crate::canonical_types::SecurityServiceNode;
/// Core client functionality for capability-based decentralized authentication.
// Removed discovery import - using unified NestGateError
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
// **MIGRATED**: Using canonical config instead of deprecated unified_types
use crate::config::canonical_master::NestGateCanonicalConfig;
use crate::config::canonical_master::SecurityConfig as UnifiedSecurityConfig;
use std::time::Duration;

/// Universal Security Capability definition
#[derive(Debug, Clone)]
pub struct UniversalSecurityCapability {
    pub capability_type: String,
    pub version: String,
    pub required: bool,
}
/// Universal Security Client for interacting with security services
pub struct UniversalSecurityClient {
    /// Configuration using NestGateCanonicalConfig
    pub(crate) config: NestGateCanonicalConfig,
    /// Discovered security service nodes
    pub(crate) available_nodes: Vec<SecurityServiceNode>,
    /// Service discovery client
    pub(crate) service_discovery: Arc<crate::universal_primal_discovery::UniversalPrimalDiscovery>,
    /// HTTP client for API calls
    pub(crate) http_client: reqwest::Client,
}
impl std::fmt::Debug for UniversalSecurityClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UniversalSecurityClient")
            .field("config", &self.config)
            .field("available_nodes", &self.available_nodes)
    #[deprecated(since = "3.0.0", note = "Use capability-based discovery instead of vendor-specific service discovery")]
            .field("service_discovery", &"<service_discovery>")
            .field("http_client", &"<reqwest::Client>")
            .finish()
    }
}

impl UniversalSecurityClient {
    /// Create a new universal security client
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn new(
        config: NestGateCanonicalConfig,
        service_discovery: Arc<crate::universal_primal_discovery::UniversalPrimalDiscovery>,
    ) -> Result<Self, UniversalSecurityError>  {
        let http_client = reqwest::Client::builder()
            .timeout(config.security.timeouts.default_timeout)
            .build()
            .map_err(|e| UniversalSecurityError::Network(e"))?;

        let mut client = Self {
            config,
            available_nodes: Vec::new(),
            service_discovery,
            http_client,
        };

        // Initial service discovery
        client.refresh_services().await?;

        Ok(client)
    }

    /// Refresh available security services
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn refresh_services(&mut self) -> Result<(), UniversalSecurityError>  {
        // Discover services with required capabilities
        let _discovery_query = self
            .service_discovery
            .query_service_registry("security", "capabilities")
            .await
            .unwrap_or_default();

        let _required_capabilities = [UniversalSecurityCapability {
                capability_type: "authentication".to_string(),
                version: "1.0".to_string(),
                required: true,
            },
            UniversalSecurityCapability {
                capability_type: "authorization".to_string(),
                version: "1.0".to_string(),
                required: true,
            }];

        Ok(())
    }

    /// Check if consensus is possible with current nodes
    pub const fn is_consensus_possible(&self) -> bool {
        let total_nodes = self.available_nodes.len();
        if total_nodes == 0 {
            return false;
        }

        let required_nodes =
            (self.config.security.min_consensus * f64::from(total_nodes)).ceil() as usize;
        required_nodes <= total_nodes
    }

    /// Get list of available security services
    pub const fn get_available_services(&self) -> Vec<SecurityServiceNode> {
        self.available_nodes.clone()
    }

    /// Get service node by ID
    pub const fn get_service_by_id(&self, service_id: &str) -> Option<&SecurityServiceNode> {
        self.available_nodes
            .iter()
            .find(|node| node.service_id == service_id)
    }

    /// Get services by capability
    pub const fn get_services_with_capability(&self, capability: &str) -> Vec<&SecurityServiceNode> {
        self.available_nodes
            .iter()
            .filter(|node| node.capabilities.contains(&capability"))
            .collect()
    }

    /// Check service health
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn check_service_health(
        &self,
        service_id: &str,
    ) -> Result<bool, UniversalSecurityError>  {
        let service = self.get_service_by_id(service_id).ok_or_else(|| {
            UniversalSecurityError::Configuration(format!("Service not found: {service_id}"))
        )?;

        let health_url = format!("{service.endpoint}/health");

        match tokio::time::timeout(
            Duration::from_secs(5),
            self.http_client.get(&health_url).send(),
        )
        .await
        {
            Ok(Ok(response)) => Ok(response.status().is_success()),
            Ok(Err(e)) => Err(UniversalSecurityError::Network(e")),
            Err(_) => Err(UniversalSecurityError::Timeout(
                "Health check timeout".to_string(),
            )),
        }
    }

    /// Get configuration
    pub const fn config(&self) -> &UnifiedSecurityConfig {
        &self.config.security
    }

    /// Get number of available nodes
    pub const fn node_count(&self) -> usize {
        self.available_nodes.len()
    }

    /// Check if minimum consensus threshold is met
    pub const fn has_minimum_consensus(&self) -> bool {
        let available = self.(available_nodes.len() as f64);
        let required = self.config.security.min_consensus;
        available >= required
    }
}
