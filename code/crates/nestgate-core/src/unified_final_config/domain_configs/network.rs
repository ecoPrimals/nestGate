//
// Network-specific configuration structures extracted from the monolithic domain_configs.rs
// for better maintainability and focused responsibility.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Network domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDomainConfig {
    pub bind_address: String,
    pub api_port: u16,
    pub health_port: u16,
    pub metrics_port: u16,
    pub connection_timeout: Duration,
    pub request_timeout: Duration,
    pub max_connections: u32,
    pub enable_dynamic_networks: bool, // For backward compatibility
    pub port: u16,                     // Alias for api_port for backward compatibility
}

impl Default for NetworkDomainConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            api_port: 8080,
            health_port: 8081,
            metrics_port: 8082,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            max_connections: 1000,
            enable_dynamic_networks: true,
            port: 8080,
        }
    }
}
