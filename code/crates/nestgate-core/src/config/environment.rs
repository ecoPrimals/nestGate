use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Current environment
    pub environment: String,

    /// AI features enabled
    pub ai_features_enabled: bool,

    /// ZFS features enabled
    pub zfs_features_enabled: bool,

    /// Network features enabled
    pub network_features_enabled: bool,

    /// Web interface enabled
    pub web_interface_enabled: bool,

    /// Metrics enabled
    pub metrics_enabled: bool,

    /// Debug logging enabled
    pub debug_logging_enabled: bool,

    /// Directory paths
    pub cache_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub data_dir: PathBuf,
    pub config_dir: PathBuf,
    pub log_dir: PathBuf,

    /// Database configuration
    pub database_url: String,
    pub database_pool_size: u32,

    /// Performance configuration
    pub worker_threads: usize,
    pub request_timeout_seconds: u64,
    pub connection_timeout_seconds: u64,
    pub max_concurrent_connections: usize,

    /// Universal primal provider discovery configuration
    pub enable_primal_auto_discovery: bool,
    pub ai_provider_capabilities: Vec<String>,
    pub security_provider_capabilities: Vec<String>,
    pub orchestration_provider_capabilities: Vec<String>,

    /// Legacy API keys (deprecated - use universal discovery)
    pub beardog_api_key: Option<String>,
    pub songbird_api_key: Option<String>,
    pub squirrel_api_key: Option<String>, // Deprecated: use ai_provider_capabilities
    pub toadstool_api_key: Option<String>, // Deprecated: use ai_provider_capabilities

    /// External service API keys (non-primal services)
    pub huggingface_api_token: String,
    pub ncbi_api_key: String,

    /// Universal security capability requirements (replaces centralized JWT)
    pub security_capability_requirements: Vec<String>,
    pub decentralized_consensus_threshold: f64,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            environment: current_environment(),
            ai_features_enabled: ai_features_enabled(),
            zfs_features_enabled: zfs_features_enabled(),
            network_features_enabled: network_features_enabled(),
            web_interface_enabled: web_interface_enabled(),
            metrics_enabled: metrics_enabled(),
            debug_logging_enabled: debug_logging_enabled(),
            cache_dir: default_cache_dir(),
            temp_dir: default_temp_dir(),
            data_dir: default_data_dir(),
            config_dir: default_config_dir(),
            log_dir: default_log_dir(),
            database_url: database_url(),
            database_pool_size: database_pool_size(),
            worker_threads: worker_threads(),
            request_timeout_seconds: request_timeout_seconds(),
            connection_timeout_seconds: connection_timeout_seconds(),
            max_concurrent_connections: max_concurrent_connections(),
            enable_primal_auto_discovery: enable_primal_auto_discovery(),
            ai_provider_capabilities: ai_provider_capabilities(),
            security_provider_capabilities: security_provider_capabilities(),
            orchestration_provider_capabilities: orchestration_provider_capabilities(),
            beardog_api_key: Some("secure_beardog_key".to_string()),
            songbird_api_key: Some(songbird_api_key()).filter(|key| !key.is_empty()),
            squirrel_api_key: Some(squirrel_api_key()).filter(|key| !key.is_empty()),
            toadstool_api_key: Some(toadstool_api_key()).filter(|key| !key.is_empty()),
            huggingface_api_token: huggingface_api_token(),
            ncbi_api_key: ncbi_api_key(),
            security_capability_requirements: security_capability_requirements(),
            decentralized_consensus_threshold: decentralized_consensus_threshold(),
        }
    }
}

impl EnvironmentConfig {
    /// Check if running in production
    pub fn is_production(&self) -> bool {
        self.environment.to_lowercase() == "production"
    }

    /// Check if running in development
    pub fn is_development(&self) -> bool {
        self.environment.to_lowercase() == "development"
    }

    /// Check if running in test mode
    pub fn is_test(&self) -> bool {
        self.environment.to_lowercase() == "test"
    }

    /// Validate environment configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate universal security configuration in production
        if self.is_production() {
            if self.security_capability_requirements.is_empty() {
                return Err(
                    "Security capability requirements must be configured in production".to_string(),
                );
            }
            if self.decentralized_consensus_threshold < 0.5
                || self.decentralized_consensus_threshold > 1.0
            {
                return Err(
                    "Decentralized consensus threshold must be between 0.5 and 1.0".to_string(),
                );
            }
            // Note: API keys validated separately by capability-based discovery
        }

        // Validate performance settings
        if self.worker_threads == 0 {
            return Err("Worker threads must be greater than 0".to_string());
        }

        if self.request_timeout_seconds == 0 {
            return Err("Request timeout must be greater than 0".to_string());
        }

        if self.connection_timeout_seconds == 0 {
            return Err("Connection timeout must be greater than 0".to_string());
        }

        if self.max_concurrent_connections == 0 {
            return Err("Max concurrent connections must be greater than 0".to_string());
        }

        if self.database_pool_size == 0 {
            return Err("Database pool size must be greater than 0".to_string());
        }

        Ok(())
    }
}

// Environment detection functions
pub fn current_environment() -> String {
    std::env::var("ENVIRONMENT")
        .or_else(|_| std::env::var("NODE_ENV"))
        .unwrap_or_else(|_| "development".to_string())
}

// Feature flags
pub fn ai_features_enabled() -> bool {
    std::env::var("NESTGATE_AI_FEATURES_ENABLED")
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true)
}

pub fn zfs_features_enabled() -> bool {
    std::env::var("NESTGATE_ZFS_FEATURES_ENABLED")
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true)
}

pub fn network_features_enabled() -> bool {
    std::env::var("NESTGATE_NETWORK_FEATURES_ENABLED")
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true)
}

pub fn web_interface_enabled() -> bool {
    std::env::var("NESTGATE_WEB_INTERFACE_ENABLED")
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true)
}

pub fn metrics_enabled() -> bool {
    std::env::var("NESTGATE_METRICS_ENABLED")
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true)
}

pub fn debug_logging_enabled() -> bool {
    std::env::var("NESTGATE_DEBUG_LOGGING_ENABLED")
        .unwrap_or_else(|_| "false".to_string())
        .parse()
        .unwrap_or(false)
}

// Path configuration functions
pub fn default_cache_dir() -> PathBuf {
    std::env::var("NESTGATE_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./cache"))
}

pub fn default_temp_dir() -> PathBuf {
    std::env::var("NESTGATE_TEMP_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./tmp"))
}

pub fn default_data_dir() -> PathBuf {
    std::env::var("NESTGATE_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./data"))
}

pub fn default_config_dir() -> PathBuf {
    std::env::var("NESTGATE_CONFIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./config"))
}

pub fn default_log_dir() -> PathBuf {
    std::env::var("NESTGATE_LOG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("./logs"))
}

// Database configuration
pub fn database_url() -> String {
    std::env::var("NESTGATE_DATABASE_URL").unwrap_or_else(|_| "sqlite:./nestgate.db".to_string())
}

pub fn database_pool_size() -> u32 {
    std::env::var("NESTGATE_DATABASE_POOL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10)
}

// Performance configuration
pub fn worker_threads() -> usize {
    std::env::var("NESTGATE_WORKER_THREADS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            // Use 1 as fallback if num_cpus is not available
            num_cpus::get().max(1)
        })
}

pub fn request_timeout_seconds() -> u64 {
    std::env::var("NESTGATE_REQUEST_TIMEOUT_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60)
}

pub fn connection_timeout_seconds() -> u64 {
    std::env::var("NESTGATE_CONNECTION_TIMEOUT_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30)
}

pub fn max_concurrent_connections() -> usize {
    std::env::var("NESTGATE_MAX_CONCURRENT_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000)
}

// Security configuration
pub fn beardog_api_key() -> String {
    std::env::var("BEARDOG_API_KEY")
        .unwrap_or_else(|_| "default_beardog_key_change_in_production".to_string())
}

pub fn songbird_api_key() -> String {
    std::env::var("SONGBIRD_API_KEY")
        .unwrap_or_else(|_| "default_songbird_key_change_in_production".to_string())
}

pub fn squirrel_api_key() -> String {
    std::env::var("SQUIRREL_API_KEY")
        .unwrap_or_else(|_| "default_squirrel_key_change_in_production".to_string())
}

pub fn toadstool_api_key() -> String {
    std::env::var("TOADSTOOL_API_KEY")
        .unwrap_or_else(|_| "default_toadstool_key_change_in_production".to_string())
}

pub fn huggingface_api_token() -> String {
    std::env::var("HUGGINGFACE_API_TOKEN")
        .unwrap_or_else(|_| "default_huggingface_token_change_in_production".to_string())
}

pub fn ncbi_api_key() -> String {
    std::env::var("NCBI_API_KEY")
        .unwrap_or_else(|_| "default_ncbi_key_change_in_production".to_string())
}

/// Universal primal provider discovery configuration
pub fn enable_primal_auto_discovery() -> bool {
    std::env::var("NESTGATE_ENABLE_PRIMAL_AUTO_DISCOVERY")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(true) // Default to enabled for universal discovery
}

pub fn ai_provider_capabilities() -> Vec<String> {
    std::env::var("NESTGATE_AI_PROVIDER_CAPABILITIES")
        .unwrap_or_else(|_| "text-generation,analysis,embedding,classification".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn security_provider_capabilities() -> Vec<String> {
    std::env::var("NESTGATE_SECURITY_PROVIDER_CAPABILITIES")
        .unwrap_or_else(|_| "encryption,authentication,signing,validation".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn orchestration_provider_capabilities() -> Vec<String> {
    std::env::var("NESTGATE_ORCHESTRATION_PROVIDER_CAPABILITIES")
        .unwrap_or_else(|_| "service-discovery,load-balancing,health-checks,deployment".to_string())
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn security_capability_requirements() -> Vec<String> {
    std::env::var("SECURITY_CAPABILITY_REQUIREMENTS")
        .map(|capabilities_str| {
            capabilities_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_else(|_| {
            vec![
                "security.authentication.decentralized".to_string(),
                "security.consensus.distributed_validation".to_string(),
                "security.cryptography.proof_verification".to_string(),
            ]
        })
}

pub fn decentralized_consensus_threshold() -> f64 {
    std::env::var("DECENTRALIZED_CONSENSUS_THRESHOLD")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0.66) // Require 66% consensus by default
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_config_default() {
        let config = EnvironmentConfig::default();
        // In test environment, we can't assume specific environment detection
        assert!(!config.environment.is_empty());
        assert!(config.ai_features_enabled);
        assert!(config.zfs_features_enabled);
        assert!(config.network_features_enabled);
        assert!(config.web_interface_enabled);
        assert!(config.metrics_enabled);
    }

    #[test]
    fn test_environment_detection() {
        // Test current environment detection
        let env = current_environment();
        assert!(!env.is_empty());

        // Test production detection - set env var BEFORE creating config
        std::env::set_var("ENVIRONMENT", "production");
        let config = EnvironmentConfig {
            environment: current_environment(), // This will pick up the env var
            ..EnvironmentConfig::default()
        };
        assert!(config.is_production());
        assert!(!config.is_development());
        std::env::remove_var("ENVIRONMENT");
    }

    #[test]
    fn test_feature_flags() {
        assert!(ai_features_enabled());
        assert!(zfs_features_enabled());
        assert!(network_features_enabled());
        assert!(web_interface_enabled());
        assert!(metrics_enabled());
        assert!(!debug_logging_enabled());
    }

    #[test]
    fn test_path_configuration() {
        let cache_dir = default_cache_dir();
        let temp_dir = default_temp_dir();
        let data_dir = default_data_dir();
        let config_dir = default_config_dir();
        let log_dir = default_log_dir();

        assert_eq!(cache_dir, PathBuf::from("./cache"));
        assert_eq!(temp_dir, PathBuf::from("./tmp"));
        assert_eq!(data_dir, PathBuf::from("./data"));
        assert_eq!(config_dir, PathBuf::from("./config"));
        assert_eq!(log_dir, PathBuf::from("./logs"));
    }

    #[test]
    fn test_database_configuration() {
        let url = database_url();
        let pool_size = database_pool_size();

        assert!(!url.is_empty());
        assert!(pool_size > 0);
    }

    #[test]
    fn test_performance_configuration() {
        let worker_threads = worker_threads();
        let request_timeout = request_timeout_seconds();
        let connection_timeout = connection_timeout_seconds();
        let max_connections = max_concurrent_connections();

        assert!(worker_threads > 0);
        assert!(request_timeout > 0);
        assert!(connection_timeout > 0);
        assert!(max_connections > 0);
    }

    #[test]
    fn test_security_configuration() {
        let beardog_key = beardog_api_key();
        let songbird_key = songbird_api_key();
        let squirrel_key = squirrel_api_key();
        let toadstool_key = toadstool_api_key();
        let huggingface_token = huggingface_api_token();
        let ncbi_key = ncbi_api_key();
        let security_capabilities = security_capability_requirements();
        let consensus_threshold = decentralized_consensus_threshold();

        assert!(!beardog_key.is_empty());
        assert!(!songbird_key.is_empty());
        assert!(!squirrel_key.is_empty());
        assert!(!toadstool_key.is_empty());
        assert!(!huggingface_token.is_empty());
        assert!(!ncbi_key.is_empty());
        assert!(!security_capabilities.is_empty());
        assert!((0.5..=1.0).contains(&consensus_threshold));
    }

    #[test]
    fn test_environment_validation() {
        // Ensure we're in development mode for this test
        std::env::set_var("ENVIRONMENT", "development");
        let mut config = EnvironmentConfig::default();

        // Valid configuration should pass
        assert!(config.validate().is_ok());

        // Zero worker threads should fail
        config.worker_threads = 0;
        assert!(config.validate().is_err());

        // Zero request timeout should fail
        config.worker_threads = 1;
        config.request_timeout_seconds = 0;
        assert!(config.validate().is_err());

        // Zero connection timeout should fail
        config.request_timeout_seconds = 60;
        config.connection_timeout_seconds = 0;
        assert!(config.validate().is_err());

        // Zero max connections should fail
        config.connection_timeout_seconds = 30;
        config.max_concurrent_connections = 0;
        assert!(config.validate().is_err());

        // Zero database pool size should fail
        config.max_concurrent_connections = 1000;
        config.database_pool_size = 0;
        assert!(config.validate().is_err());

        // Clean up environment variable
        std::env::remove_var("ENVIRONMENT");
    }

    #[test]
    fn test_production_validation() {
        let config = EnvironmentConfig {
            environment: "production".to_string(),
            security_capability_requirements: Vec::new(), // Empty - should fail in production
            ..Default::default()
        };

        // Production with empty security capabilities should fail
        assert!(config.validate().is_err());

        // Create config with proper keys
        let config_with_capabilities = EnvironmentConfig {
            environment: "production".to_string(),
            security_capability_requirements: vec![
                "security.authentication.decentralized".to_string(),
                "security.consensus.distributed_validation".to_string(),
            ],
            decentralized_consensus_threshold: 0.75,
            beardog_api_key: Some("secure_beardog_key".to_string()),
            ..Default::default()
        };

        assert!(config_with_capabilities.validate().is_ok());
    }
}
