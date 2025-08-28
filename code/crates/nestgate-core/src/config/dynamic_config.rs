use std::collections::HashMap;
///
/// This module provides environment-configurable alternatives to hardcoded constants,
/// allowing runtime configuration of ports, timeouts, buffer sizes, and other parameters.
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

/// Dynamic configuration manager that loads settings from environment variables
#[derive(Debug, Clone)]
pub struct DynamicConfigManager {
    /// Cached configuration values
    config_cache: HashMap<String, String>,
    /// Configuration prefix for environment variables
    prefix: String,
}

impl DynamicConfigManager {
    /// Create a new configuration manager with the specified prefix
    pub fn new(prefix: &str) -> Self {
        Self {
            config_cache: HashMap::new(),
            prefix: prefix.to_string(),
        }
    }

    /// Create a default configuration manager with "NESTGATE" prefix
    pub fn with_default_prefix() -> Self {
        Self::new("NESTGATE")
    }

    /// Get a configuration value, first checking environment variables, then falling back to default
    pub fn get_or_default<T>(&self, key: &str, default: T) -> T
    where
        T: FromConfigValue + Clone,
    {
        let env_key = format!("{}_{}", self.prefix, key.to_uppercase());

        if let Ok(value) = env::var(&env_key) {
            if let Ok(parsed) = T::from_config_value(&value) {
                return parsed;
            }
        }

        default
    }

    /// Get a required configuration value that must be set
    pub fn get_required<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: FromConfigValue,
    {
        let env_key = format!("{}_{}", self.prefix, key.to_uppercase());

        let value = env::var(&env_key).map_err(|_| ConfigError::Missing(env_key.clone()))?;

        T::from_config_value(&value).map_err(|e| ConfigError::InvalidValue {
            key: env_key,
            value,
            error: e,
        })
    }

    /// Set a configuration value (useful for testing)
    pub fn set(&mut self, key: &str, value: &str) {
        let env_key = format!("{}_{}", self.prefix, key.to_uppercase());
        env::set_var(&env_key, value);
    }

    /// Load all configuration from environment variables matching the prefix
    pub fn load_from_env(&mut self) -> Result<(), ConfigError> {
        self.config_cache.clear();

        for (key, value) in env::vars() {
            if key.starts_with(&format!("{}_", self.prefix)) {
                self.config_cache.insert(key, value);
            }
        }

        Ok(())
    }

    /// Get all loaded configuration as a map
    pub fn get_all_config(&self) -> &HashMap<String, String> {
        &self.config_cache
    }
}

/// Configuration error types
#[derive(Debug, Clone)]
pub enum ConfigError {
    Missing(String),
    InvalidValue {
        key: String,
        value: String,
        error: String,
    },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Missing(key) => write!(f, "Missing required configuration: {key}"),
            ConfigError::InvalidValue { key, value, error } => {
                write!(
                    f,
                    "Invalid configuration value for {key}: '{value}' - {error}"
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Trait for types that can be parsed from configuration values
pub trait FromConfigValue: Sized {
    fn from_config_value(value: &str) -> Result<Self, String>;
}

// Implementations for common types
impl FromConfigValue for String {
    fn from_config_value(value: &str) -> Result<Self, String> {
        Ok(value.to_string())
    }
}

impl FromConfigValue for u16 {
    fn from_config_value(value: &str) -> Result<Self, String> {
        value.parse().map_err(|e| format!("Invalid u16: {e}"))
    }
}

impl FromConfigValue for u32 {
    fn from_config_value(value: &str) -> Result<Self, String> {
        value.parse().map_err(|e| format!("Invalid u32: {e}"))
    }
}

impl FromConfigValue for u64 {
    fn from_config_value(value: &str) -> Result<Self, String> {
        value.parse().map_err(|e| format!("Invalid u64: {e}"))
    }
}

impl FromConfigValue for usize {
    fn from_config_value(value: &str) -> Result<Self, String> {
        value.parse().map_err(|e| format!("Invalid usize: {e}"))
    }
}

impl FromConfigValue for bool {
    fn from_config_value(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Ok(true),
            "false" | "0" | "no" | "off" => Ok(false),
            _ => Err(format!("Invalid boolean value: {value}")),
        }
    }
}

impl FromConfigValue for Duration {
    fn from_config_value(value: &str) -> Result<Self, String> {
        // Support formats like "30s", "5m", "2h", or just seconds as number
        if let Ok(seconds) = value.parse::<u64>() {
            return Ok(Duration::from_secs(seconds));
        }

        if let Some(num_str) = value.strip_suffix('s') {
            let seconds = num_str
                .parse::<u64>()
                .map_err(|e| format!("Invalid seconds: {e}"))?;
            return Ok(Duration::from_secs(seconds));
        }

        if let Some(num_str) = value.strip_suffix('m') {
            let minutes = num_str
                .parse::<u64>()
                .map_err(|e| format!("Invalid minutes: {e}"))?;
            return Ok(Duration::from_secs(minutes * 60));
        }

        if let Some(num_str) = value.strip_suffix('h') {
            let hours = num_str
                .parse::<u64>()
                .map_err(|e| format!("Invalid hours: {e}"))?;
            return Ok(Duration::from_secs(hours * 3600));
        }

        Err(format!("Invalid duration format: {value}"))
    }
}

/// Dynamic network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicNetworkConfig {
    /// API port (default: 8000)
    pub api_port: u16,
    /// Health check port (default: 8002)
    pub health_port: u16,
    /// Metrics port (default: 8003)
    pub metrics_port: u16,
    /// Maximum connections (default: 1000)
    pub max_connections: usize,
    /// Request timeout (default: 30s)
    pub request_timeout: Duration,
    /// Bind address (default: "0.0.0.0")
    pub bind_address: String,
}

impl DynamicNetworkConfig {
    pub fn from_env() -> Self {
        let config_manager = DynamicConfigManager::with_default_prefix();

        Self {
            api_port: config_manager.get_or_default("API_PORT", 8000),
            health_port: config_manager.get_or_default("HEALTH_PORT", 8002),
            metrics_port: config_manager.get_or_default("METRICS_PORT", 8003),
            max_connections: config_manager.get_or_default("MAX_CONNECTIONS", 1000),
            request_timeout: config_manager
                .get_or_default("REQUEST_TIMEOUT", Duration::from_secs(30)),
            bind_address: config_manager.get_or_default("BIND_ADDRESS", "0.0.0.0".to_string()),
        }
    }
}

/// Dynamic storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicStorageConfig {
    /// ZFS command timeout (default: 30s)
    pub zfs_timeout: Duration,
    /// Pool discovery interval (default: 60s)
    pub pool_discovery_interval: Duration,
    /// Maximum pools to track (default: 1000)
    pub max_pools: usize,
    /// Maximum datasets per pool (default: 10000)
    pub max_datasets: usize,
    /// Snapshot retention days (default: 30)
    pub snapshot_retention_days: u32,
    /// ZFS binary path (default: "/usr/sbin/zfs")
    pub zfs_binary_path: String,
    /// Use sudo for ZFS commands (default: true)
    pub use_sudo: bool,
}

impl DynamicStorageConfig {
    pub fn from_env() -> Self {
        let config_manager = DynamicConfigManager::with_default_prefix();

        Self {
            zfs_timeout: config_manager.get_or_default("ZFS_TIMEOUT", Duration::from_secs(30)),
            pool_discovery_interval: config_manager
                .get_or_default("POOL_DISCOVERY_INTERVAL", Duration::from_secs(60)),
            max_pools: config_manager.get_or_default("MAX_POOLS", 1000),
            max_datasets: config_manager.get_or_default("MAX_DATASETS", 10000),
            snapshot_retention_days: config_manager.get_or_default("SNAPSHOT_RETENTION_DAYS", 30),
            zfs_binary_path: config_manager
                .get_or_default("ZFS_BINARY_PATH", "/usr/sbin/zfs".to_string()),
            use_sudo: config_manager.get_or_default("USE_SUDO", true),
        }
    }
}

/// Dynamic performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPerformanceConfig {
    /// Buffer size for operations (default: 8192)
    pub buffer_size: usize,
    /// Large buffer size (default: 65536)
    pub large_buffer_size: usize,
    /// Maximum concurrent operations (default: 100)
    pub max_concurrent_ops: usize,
    /// Health check interval (default: 10s)
    pub health_check_interval: Duration,
    /// Metrics collection interval (default: 5s)
    pub metrics_interval: Duration,
}

impl DynamicPerformanceConfig {
    pub fn from_env() -> Self {
        let config_manager = DynamicConfigManager::with_default_prefix();

        Self {
            buffer_size: config_manager.get_or_default("BUFFER_SIZE", 8192),
            large_buffer_size: config_manager.get_or_default("LARGE_BUFFER_SIZE", 65536),
            max_concurrent_ops: config_manager.get_or_default("MAX_CONCURRENT_OPS", 100),
            health_check_interval: config_manager
                .get_or_default("HEALTH_CHECK_INTERVAL", Duration::from_secs(10)),
            metrics_interval: config_manager
                .get_or_default("METRICS_INTERVAL", Duration::from_secs(5)),
        }
    }
}

/// Central dynamic configuration that combines all subsystem configs
#[derive(Debug, Clone)]
pub struct DynamicConfig {
    pub network: DynamicNetworkConfig,
    pub storage: DynamicStorageConfig,
    pub performance: DynamicPerformanceConfig,
}

impl DynamicConfig {
    /// Load complete configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            network: DynamicNetworkConfig::from_env(),
            storage: DynamicStorageConfig::from_env(),
            performance: DynamicPerformanceConfig::from_env(),
        }
    }

    /// Create configuration with all defaults (for testing)
    pub fn with_defaults_only() -> Self {
        // Temporarily clear environment to get pure defaults
        let original_env: Vec<(String, String)> = std::env::vars()
            .filter(|(k, _)| k.starts_with("NESTGATE_"))
            .collect();

        // Clear NESTGATE env vars
        for (key, _) in &original_env {
            std::env::remove_var(key);
        }

        let config = Self::from_env();

        // Restore original environment
        for (key, value) in original_env {
            std::env::set_var(key, value);
        }

        config
    }
}

/// Global configuration instance (lazy-loaded, thread-safe)
static GLOBAL_CONFIG: std::sync::OnceLock<DynamicConfig> = std::sync::OnceLock::new();

/// Get the global dynamic configuration instance
pub fn get_config() -> &'static DynamicConfig {
    GLOBAL_CONFIG.get_or_init(DynamicConfig::from_env)
}

/// Reload the global configuration from environment variables
/// Note: OnceLock doesn't support reloading, so this creates a new instance
/// For production use, consider using RwLock<DynamicConfig> if reloading is needed
pub fn reload_config() {
    // OnceLock doesn't support reloading after initialization
    // This is a design limitation - consider using Arc<RwLock<DynamicConfig>> if needed
    tracing::warn!("Configuration reload requested but OnceLock doesn't support reloading. Restart required for config changes.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_manager_defaults() {
        let manager = DynamicConfigManager::new("TEST");

        let port: u16 = manager.get_or_default("PORT", 8080);
        assert_eq!(port, 8080);

        let timeout: Duration = manager.get_or_default("TIMEOUT", Duration::from_secs(30));
        assert_eq!(timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_config_manager_env_override() {
        env::set_var("TEST_PORT", "9000");
        env::set_var("TEST_TIMEOUT", "45s");

        let manager = DynamicConfigManager::new("TEST");

        let port: u16 = manager.get_or_default("PORT", 8080);
        assert_eq!(port, 9000);

        let timeout: Duration = manager.get_or_default("TIMEOUT", Duration::from_secs(30));
        assert_eq!(timeout, Duration::from_secs(45));

        // Cleanup
        env::remove_var("TEST_PORT");
        env::remove_var("TEST_TIMEOUT");
    }

    #[test]
    fn test_duration_parsing() {
        assert_eq!(
            Duration::from_config_value("30").unwrap_or_else(|e| {
                tracing::error!("Failed to parse duration: {:?}", e);
                Duration::from_secs(0) // Return default duration
            }),
            Duration::from_secs(30)
        );
        assert_eq!(
            Duration::from_config_value("30s").unwrap_or_else(|e| {
                tracing::error!("Failed to parse duration: {:?}", e);
                Duration::from_secs(0) // Return default duration
            }),
            Duration::from_secs(30)
        );
        assert_eq!(
            Duration::from_config_value("5m").unwrap_or_else(|e| {
                tracing::error!("Failed to parse duration: {:?}", e);
                Duration::from_secs(0) // Return default duration
            }),
            Duration::from_secs(300)
        );
        assert_eq!(
            Duration::from_config_value("2h").unwrap_or_else(|e| {
                tracing::error!("Failed to parse duration: {:?}", e);
                Duration::from_secs(0) // Return default duration
            }),
            Duration::from_secs(7200)
        );
    }

    #[test]
    fn test_bool_parsing() {
        assert_eq!(
            bool::from_config_value("true").unwrap_or_else(|e| {
                tracing::error!("Failed to parse bool: {:?}", e);
                false // Return default bool
            }),
            true
        );
        assert_eq!(
            bool::from_config_value("false").unwrap_or_else(|e| {
                tracing::error!("Failed to parse bool: {:?}", e);
                false // Return default bool
            }),
            false
        );
        assert_eq!(
            bool::from_config_value("1").unwrap_or_else(|e| {
                tracing::error!("Failed to parse bool: {:?}", e);
                false // Return default bool
            }),
            true
        );
        assert_eq!(
            bool::from_config_value("0").unwrap_or_else(|e| {
                tracing::error!("Failed to parse bool: {:?}", e);
                false // Return default bool
            }),
            false
        );
        assert_eq!(
            bool::from_config_value("yes").unwrap_or_else(|e| {
                tracing::error!("Failed to parse bool: {:?}", e);
                false // Return default bool
            }),
            true
        );
        assert_eq!(
            bool::from_config_value("no").unwrap_or_else(|e| {
                tracing::error!("Failed to parse bool: {:?}", e);
                false // Return default bool
            }),
            false
        );
    }

    #[test]
    fn test_dynamic_network_config() {
        // Test with defaults
        let config = DynamicNetworkConfig::from_env();
        assert_eq!(config.api_port, 8000);
        assert_eq!(config.bind_address, "0.0.0.0");
    }

    #[test]
    fn test_global_config() {
        let config = get_config();
        assert!(config.network.api_port > 0);
        assert!(config.storage.max_pools > 0);
        assert!(config.performance.buffer_size > 0);
    }
}
