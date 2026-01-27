//! **API DOMAIN CONFIGURATION**
//!
//! Consolidates all API-related configurations:
//! - Server configuration (`ApiServerConfig`)
//! - Handler-specific configurations
//! - Performance and security settings
//!
//! ## Usage
//!
//! ```rust
//! use nestgate_core::config::canonical_primary::domains::consolidated_domains::api::*;
//!
//! let api_config = ApiDomainConfig::default();
//! assert_eq!(api_config.server.port, 8000);
//! ```

use super::validation::DomainConfigValidation;
use crate::constants::{DEFAULT_API_PORT, LOCALHOST};
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== API DOMAIN CONFIGURATION ====================

/// **API DOMAIN CONFIGURATION**
///
/// Consolidates all API-related configurations including server settings,
/// handlers, security, and performance tuning.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiDomainConfig {
    /// HTTP server configuration
    pub server: ApiServerConfig,

    /// Handler configurations
    pub handlers: ConsolidatedApiHandlersConfig,

    /// Security configuration
    pub security: ApiSecurityConfig,

    /// Performance configuration
    pub performance: ApiPerformanceConfig,

    /// Monitoring configuration
    pub monitoring: ApiMonitoringConfig,

    /// Rate limiting configuration
    pub rate_limiting: ApiRateLimitingConfig,

    /// CORS configuration
    pub cors: ApiCorsConfig,
}

// ==================== SERVER CONFIGURATION ====================

/// API server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServerConfig {
    /// Server bind address
    pub bind_address: String,

    /// Server port
    pub port: u16,

    /// Worker thread count
    pub workers: Option<usize>,

    /// Request timeout
    pub request_timeout: Duration,

    /// Maximum request body size
    pub max_request_size: usize,

    /// Keep-alive timeout
    pub keep_alive: Duration,
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self {
            bind_address: LOCALHOST.to_string(),
            port: DEFAULT_API_PORT,
            workers: None, // Auto-detect
            request_timeout: Duration::from_secs(30),
            max_request_size: 10 * 1024 * 1024, // 10 MB
            keep_alive: Duration::from_secs(75),
        }
    }
}

// ==================== HANDLER CONFIGURATION ====================

/// Consolidated API handlers configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsolidatedApiHandlersConfig {
    /// ZFS handler configuration
    pub zfs: ZfsHandlerConfig,

    /// Performance handler configuration
    pub performance: PerformanceHandlerConfig,

    /// Dashboard handler configuration
    pub dashboard: DashboardHandlerConfig,

    /// Load testing handler configuration
    pub load_testing: LoadTestingHandlerConfig,

    /// Authentication handler configuration
    pub auth: AuthHandlerConfig,

    /// Workspace handler configuration
    pub workspace: WorkspaceHandlerConfig,
}

// Handler placeholder types - Reserved for future configuration options

/// ZFS handler configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsHandlerConfig {}

/// Performance handler configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceHandlerConfig {}

/// Dashboard handler configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardHandlerConfig {}

/// Load testing handler configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadTestingHandlerConfig {}

/// Authentication handler configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthHandlerConfig {}

/// Workspace handler configuration (reserved for future use)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceHandlerConfig {}

// ==================== ADDITIONAL API CONFIGURATION ====================

/// Configuration for API security settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiSecurityConfig {}

/// Configuration for API performance tuning
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiPerformanceConfig {}

/// Configuration for API monitoring and observability
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiMonitoringConfig {}

/// Configuration for API rate limiting
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiRateLimitingConfig {}

/// Configuration for API CORS settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiCorsConfig {}

// ==================== VALIDATION ====================

impl DomainConfigValidation for ApiDomainConfig {
    fn validate(&self) -> crate::error::Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Validate bind address
        if self.server.bind_address.is_empty() {
            warnings.push("Bind address is empty".to_string());
        }

        // Validate port
        if self.server.port == 0 {
            warnings.push("Port is set to 0 (ephemeral port)".to_string());
        }

        Ok(warnings)
    }

    fn validate_for_environment(&self, _env: &str) -> crate::error::Result<()> {
        Ok(())
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["server.bind_address", "server.port"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["server.workers", "security", "monitoring"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_domain_config_default() {
        let config = ApiDomainConfig::default();
        assert_eq!(config.server.bind_address, LOCALHOST);
        assert_eq!(config.server.port, DEFAULT_API_PORT);
    }

    #[test]
    fn test_api_server_config() {
        let server = ApiServerConfig::default();
        assert!(server.workers.is_none());
        assert_eq!(server.max_request_size, 10 * 1024 * 1024);
    }

    #[test]
    fn test_validation() {
        let config = ApiDomainConfig::default();
        let warnings = config.validate().expect("Should validate");
        assert!(warnings.is_empty());
    }
}
