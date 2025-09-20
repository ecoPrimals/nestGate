/// Universal Adapter Configuration
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Universal adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalAdapterConfig {
    /// Enable auto-discovery of primal providers
    pub auto_discovery: bool,
    /// Discovery interval in seconds
    pub discovery_interval: u64,
    /// Timeout for primal requests in seconds
    pub request_timeout: u64,
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Fallback behavior when no primal is available
    pub fallback_behavior: FallbackBehavior,
    /// Discovery methods to use
    pub discovery_methods: Vec<DiscoveryMethod>,
}
impl Default for UniversalAdapterConfig {
    fn default() -> Self {
        Self {
            auto_discovery: true,
            discovery_interval: 30,
            request_timeout: 30,
            max_retries: 3,
            fallback_behavior: FallbackBehavior::NoOp,
            discovery_methods: vec![
                DiscoveryMethod::Environment,
                DiscoveryMethod::ServiceRegistry,
                DiscoveryMethod::NetworkScan,
            ],
        }
    }
}

/// Fallback behavior when no primal is available
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FallbackBehavior {
    /// Return an error
    Error,
    /// Return a no-op result
    NoOp,
    /// Use a local implementation
    Local,
}
/// Discovery methods for finding primal providers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Environment variables
    Environment,
    /// Service registry lookup
    ServiceRegistry,
    /// Network scanning
    NetworkScan,
    /// Configuration file
    Configuration,
}

/// Configuration for the universal adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    pub discovery_timeout: Duration,
    pub retry_attempts: u32,
    pub cache_ttl: Duration,
    pub endpoints: Vec<String>,
    pub fallback_enabled: bool,
}

impl AdapterConfig {
    /// Create a new adapter configuration
    pub const fn new() -> Self {
        Self {
            discovery_timeout: Duration::from_secs(30),
            retry_attempts: 3,
            cache_ttl: Duration::from_secs(300), // 5 minutes
            endpoints: vec![
                "http://localhost:8083/discovery".to_string(),
                "http://localhost:8084/discovery".to_string(),
            ],
            fallback_enabled: true,
        }
    }
    
    /// Set discovery timeout
    #[must_use]
    pub fn with_discovery_timeout(mut self, timeout: Duration) -> Self {
        self.discovery_timeout = timeout;
        self
    }
    
    /// Set retry attempts
    #[must_use]
    pub fn with_retry_attempts(mut self, attempts: u32) -> Self {
        self.retry_attempts = attempts;
        self
    }
    
    /// Add discovery endpoint
    #[must_use]
    pub fn add_endpoint(mut self, endpoint: String) -> Self {
        self.endpoints.push(endpoint);
        self
    }
    
    /// Enable or disable fallback providers
    #[must_use]
    pub fn with_fallback(mut self, enabled: bool) -> Self {
        self.fallback_enabled = enabled;
        self
    }
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self::new()
    }
}
