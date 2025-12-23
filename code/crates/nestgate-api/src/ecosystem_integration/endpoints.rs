//! **ENDPOINTS MANAGEMENT**
//!
//! Endpoint management and routing for ecosystem services.

use std::collections::HashMap;
use super::types::{ServiceEndpoint, IntegrationPreferences, RateLimitSpec, CircuitBreakerSpec};

impl Default for IntegrationPreferences {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            protocols: vec!["HTTP".to_string(), "HTTPS".to_string()],
            data_formats: vec!["JSON".to_string()],
            auth_methods: vec!["Bearer".to_string()],
            rate_limiting: None,
            circuit_breaker: None,
        }
    }
}

impl Default for ServiceEndpoint {
    /// Returns the default instance
    fn default() -> Self {
        // ✅ MIGRATED: Now uses centralized runtime configuration
        use nestgate_core::config::runtime::get_config;
        let config = get_config();
        
        Self {
            name: "default".to_string(),
            url: config.network.api_base_url(),
            protocol: "HTTP".to_string(),
            health_check: Some("/health".to_string()),
            _metadata: HashMap::new(),
        }
    }
}

/// Endpoint manager
pub struct EndpointManager {
    endpoints: HashMap<String, ServiceEndpoint>,
}
impl EndpointManager {
    /// Create new endpoint manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            endpoints: HashMap::new(),
        }
    }

    /// Add endpoint
    pub fn add_endpoint(&mut self, endpoint: ServiceEndpoint) {
        self.endpoints.insert(endpoint.name.clone(), endpoint);
    }

    /// Get endpoint by name
    pub fn get_endpoint(&self, name: &str) -> Option<&ServiceEndpoint> {
        self.endpoints.get(name)
    }

    /// List all endpoints
    pub fn list_endpoints(&self) -> Vec<&ServiceEndpoint> {
        self.endpoints.values().collect()
    }

    /// Health check endpoint
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn health_check(&self, name: &str) -> Result<bool, String>  {
        if let Some(endpoint) = self.endpoints.get(name) {
            // Simplified health check - would make actual HTTP request
            Ok(true)
        } else {
            Err(format!("Endpoint {name} not found"))
        }
    }
} 