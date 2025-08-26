use std::collections::HashMap;
//
// API-specific configuration structures extracted from the monolithic domain_configs.rs
// for better maintainability and focused responsibility.

use super::security::RateLimitConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDomainConfig {
    pub version: String,
    pub cors_enabled: bool,
    pub rate_limiting: RateLimitConfig,
    pub documentation_enabled: bool,
    pub endpoints: HashMap<String, String>,
}

impl Default for ApiDomainConfig {
    fn default() -> Self {
        let mut endpoints = HashMap::new();
        endpoints.insert("health".to_string(), "/health".to_string());
        endpoints.insert("metrics".to_string(), "/metrics".to_string());
        endpoints.insert("api".to_string(), "/api/v1".to_string());

        Self {
            version: "1.0.0".to_string(),
            cors_enabled: true,
            rate_limiting: RateLimitConfig::default(),
            documentation_enabled: true,
            endpoints,
        }
    }
}
