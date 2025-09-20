//! Core API configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub server: ServerConfig,
    pub endpoints: EndpointConfig,
    pub middleware: MiddlewareConfig,
    pub security: SecurityConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub timeout_seconds: u64,
}

/// Endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    pub enabled_endpoints: Vec<String>,
    pub rate_limits: HashMap<String, RateLimit>,
}

/// Middleware configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareConfig {
    pub cors_enabled: bool,
    pub logging_enabled: bool,
    pub compression_enabled: bool,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub auth_required: bool,
    pub tls_enabled: bool,
    pub api_key_required: bool,
}

/// Rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            endpoints: EndpointConfig::default(),
            middleware: MiddlewareConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            max_connections: 1000,
            timeout_seconds: 30,
        }
    }
}

impl Default for EndpointConfig {
    fn default() -> Self {
        Self {
            enabled_endpoints: vec![
                "/health".to_string(),
                "/metrics".to_string(),
                "/api/v1".to_string(),
            ],
            rate_limits: HashMap::new(),
        }
    }
}

impl Default for MiddlewareConfig {
    fn default() -> Self {
        Self {
            cors_enabled: true,
            logging_enabled: true,
            compression_enabled: true,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auth_required: true,
            tls_enabled: true,
            api_key_required: false,
        }
    }
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            burst_size: 10,
        }
    }
} 