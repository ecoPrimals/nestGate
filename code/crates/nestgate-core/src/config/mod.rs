use crate::error::ConfigSource;
pub mod api_paths;
/// Enhanced Configuration Management for NestGate v2
///
/// Advanced configuration capabilities with v2 orchestrator-centric architecture
///
/// ## Encryption Architecture Note
///
/// NestGate is **encryption-agnostic** by design:
/// - NestGate handles storage, ZFS operations, and replication
/// - External systems (like BearDog) handle encryption, keys, and security
/// - This separation allows NestGate to be a pure storage layer
/// - BearDog (or other providers) can use NestGate for storage while handling encryption
///
/// Configuration options marked as "encryption" are typically:
/// - Metadata tracking (is this data encrypted?)
/// - Hints/preferences for external encryption providers
/// - NOT actual encryption operations performed by NestGate
pub mod defaults;
pub mod environment;
pub mod federation;
pub mod monitoring;
pub mod network;
pub mod security;
pub mod storage;
pub mod storage_constants;

/// **CENTRALIZED CONSTANTS MODULE** (Removed during cleanup)
///
/// Configuration values are now handled through environment variables and defaults
use serde::{Deserialize, Serialize};

// Re-export from existing error module
use crate::error::{NestGateError, Result};

// Re-export common types from sub-modules
pub use api_paths::*;
pub use environment::*;
pub use federation::*;
pub use monitoring::*;
pub use network::*;
pub use security::*;
pub use storage::*;
pub use storage_constants::*;
// Note: defaults module provides implementations, not public APIs

/// Main configuration structure for the NestGate v2 system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// System-wide settings
    pub system: SystemConfig,

    /// Storage configuration
    pub storage: StorageConfig,

    /// Security settings
    pub security: SecurityConfig,

    /// Monitoring configuration
    pub monitoring: MonitoringConfig,

    /// MCP integration configuration (from Phase 1)
    pub mcp: Option<McpConfig>,

    /// Federation configuration
    pub federation: Option<FederationConfig>,

    /// Service endpoints configuration (replaces hardcoded URLs)
    pub endpoints: ServiceEndpoints,

    /// API paths configuration (replaces hardcoded API paths)
    pub api_paths: ApiPathsConfig,

    /// Storage constants configuration (replaces hardcoded sizes and limits)
    pub storage_constants: StorageConstants,
}

/// System-wide configuration settings with enhanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Log level for the system
    pub log_level: String,

    /// Data directory path
    pub data_dir: String,

    /// Temporary directory path
    pub temp_dir: String,

    /// Maximum number of concurrent operations
    pub max_concurrent_ops: usize,

    /// System identification
    pub node_id: String,

    /// Environment (dev, test, prod)
    pub environment: String,
}

impl Config {
    /// Create a new configuration with all defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("NESTGATE"))
            .build()
            .map_err(|e| NestGateError::Configuration {
                message: format!("Failed to load config: {e}"),
                config_source: ConfigSource::Defaults,
                field: None,
                suggested_fix: None,
            })?;

        config
            .try_deserialize()
            .map_err(|e| NestGateError::Configuration {
                message: format!("Failed to deserialize config: {e}"),
                config_source: ConfigSource::Defaults,
                field: None,
                suggested_fix: None,
            })
    }

    /// Load configuration from a file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::from(path.as_ref()))
            .add_source(config::Environment::with_prefix("NESTGATE"))
            .build()
            .map_err(|e| NestGateError::Configuration {
                message: format!("Failed to load config: {e}"),
                config_source: ConfigSource::Defaults,
                field: None,
                suggested_fix: None,
            })?;

        config
            .try_deserialize()
            .map_err(|e| NestGateError::Configuration {
                message: format!("Failed to deserialize config: {e}"),
                config_source: ConfigSource::Defaults,
                field: None,
                suggested_fix: None,
            })
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Validate system configuration
        if self.system.node_id.is_empty() {
            return Err(NestGateError::Configuration {
                message: "Node ID cannot be empty".to_string(),
                config_source: ConfigSource::File("configuration validation".to_string()),
                field: Some("node_id".to_string()),
                suggested_fix: Some("Set a unique node identifier".to_string()),
            });
        }

        if self.system.max_concurrent_ops == 0 {
            return Err(NestGateError::Configuration {
                message: "Max concurrent operations must be greater than 0".to_string(),
                config_source: ConfigSource::File("configuration validation".to_string()),
                field: Some("max_concurrent_operations".to_string()),
                suggested_fix: Some(
                    "Set a positive integer value (recommended: 10-100)".to_string(),
                ),
            });
        }

        // Validate storage configuration
        if self.storage.cache_size == 0 {
            return Err(NestGateError::Configuration {
                message: "Cache size must be greater than 0".to_string(),
                config_source: ConfigSource::File("configuration validation".to_string()),
                field: Some("cache_size_mb".to_string()),
                suggested_fix: Some("Set cache size in MB (recommended: 128-1024)".to_string()),
            });
        }

        if self.storage.max_file_size == 0 {
            return Err(NestGateError::Configuration {
                message: "Max file size must be greater than 0".to_string(),
                config_source: ConfigSource::File("configuration validation".to_string()),
                field: Some("max_file_size_mb".to_string()),
                suggested_fix: Some(
                    "Set maximum file size in MB (recommended: 100-1000)".to_string(),
                ),
            });
        }

        // Validate security configuration
        if self.security.max_failed_attempts == 0 {
            return Err(NestGateError::Configuration {
                message: "Max failed attempts must be greater than 0".to_string(),
                config_source: ConfigSource::File("configuration validation".to_string()),
                field: Some("max_failed_attempts".to_string()),
                suggested_fix: Some("Set retry limit (recommended: 3-10)".to_string()),
            });
        }

        if self.security.key_rotation_days == 0 {
            return Err(NestGateError::Configuration {
                message: "Key rotation days must be greater than 0".to_string(),
                config_source: ConfigSource::File("configuration validation".to_string()),
                field: Some("key_rotation_days".to_string()),
                suggested_fix: Some(
                    "Set key rotation interval in days (recommended: 30-90)".to_string(),
                ),
            });
        }

        // Validate monitoring configuration
        if self.monitoring.metrics_interval == 0 {
            return Err(NestGateError::Configuration {
                message: "Metrics interval must be greater than 0".to_string(),
                config_source: ConfigSource::File("configuration validation".to_string()),
                field: Some("metrics_interval_seconds".to_string()),
                suggested_fix: Some(
                    "Set metrics collection interval in seconds (recommended: 30-300)".to_string(),
                ),
            });
        }

        Ok(())
    }

    /// Get the service endpoint for a given service
    pub fn get_endpoint(&self, service: &str) -> Option<&str> {
        self.endpoints.services.get(service).map(|s| s.as_str())
    }
}

/// Check if running in production environment
pub fn is_production() -> bool {
    std::env::var("ENVIRONMENT")
        .or_else(|_| std::env::var("NODE_ENV"))
        .unwrap_or_else(|_| "development".to_string())
        .to_lowercase()
        == "production"
}

/// Check if running in development environment
pub fn is_development() -> bool {
    !is_production()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.system.log_level, "info");
        assert_eq!(config.system.environment, "development");
        assert!(config.system.max_concurrent_ops > 0);
        assert!(!config.system.node_id.is_empty());
    }

    #[test]
    fn test_config_validation() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_empty_node_id() {
        let mut config = Config::default();
        config.system.node_id = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_zero_cache_size() {
        let mut config = Config::default();
        config.storage.cache_size = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_environment_detection() {
        // Test default development environment
        assert!(is_development());

        // Test production environment
        std::env::set_var("ENVIRONMENT", "production");
        assert!(is_production());
        std::env::remove_var("ENVIRONMENT");
    }
}
