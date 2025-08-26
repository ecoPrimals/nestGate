//
// Security-specific configuration structures extracted from the monolithic domain_configs.rs
// for better maintainability and focused responsibility.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Security domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityDomainConfig {
    pub authentication_enabled: bool,
    pub authorization_enabled: bool,
    pub encryption_algorithm: String,
    pub key_rotation_interval: Duration,
    pub audit_logging: bool,
    pub rate_limiting: RateLimitConfig,
    pub enable_dynamic_security: bool, // For backward compatibility
    pub enable_encryption_by_default: bool, // For ZFS crate compatibility
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub burst_size: u32,
    pub enabled: bool,
}

impl Default for SecurityDomainConfig {
    fn default() -> Self {
        Self {
            authentication_enabled: true,
            authorization_enabled: true,
            encryption_algorithm: "AES-256-GCM".to_string(),
            key_rotation_interval: Duration::from_secs(86400), // 24 hours
            audit_logging: true,
            rate_limiting: RateLimitConfig::default(),
            enable_dynamic_security: true,
            enable_encryption_by_default: true,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 100,
            burst_size: 20,
            enabled: true,
        }
    }
}
