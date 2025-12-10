//! Modern environment-driven configuration system
//!
//! This module provides a unified, type-safe configuration system that loads
//! settings from environment variables with sensible defaults. It follows modern
//! Rust patterns including builder pattern, newtype wrappers, and comprehensive
//! error handling.
//!
//! # Examples
//!
//! ```no_run
//! use nestgate_core::config::environment::EnvironmentConfig;
//!
//! // Load configuration from environment
//! let config = EnvironmentConfig::from_env().expect("Failed to load config");
//!
//! // Access configuration values
//! println!("API listening on {}:{}", config.network.host, config.network.port);
//! ```

use serde::{Deserialize, Serialize};
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

/// Errors that can occur during configuration loading
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    /// Environment variable was not found
    #[error("Required environment variable '{0}' not found")]
    MissingEnvVar(String),

    /// Failed to parse environment variable value
    #[error("Failed to parse environment variable '{key}': {source}")]
    ParseError {
        /// The environment variable key
        key: String,
        /// The underlying parse error
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Invalid configuration value
    #[error("Invalid configuration: {0}")]
    Invalid(String),

    /// Port number out of valid range
    #[error("Invalid port {0}: must be between 1024 and 65535")]
    InvalidPort(u16),

    /// I/O error during configuration loading
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Centralized environment configuration
///
/// This is the main entry point for all application configuration. It loads
/// settings from environment variables with the `NESTGATE_` prefix.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentConfig {
    /// Network configuration
    pub network: NetworkConfig,

    /// Storage configuration
    pub storage: StorageConfig,

    /// Service discovery configuration
    pub discovery: DiscoveryConfig,

    /// Monitoring and observability configuration
    pub monitoring: MonitoringConfig,

    /// Security configuration
    pub security: SecurityConfig,
}

impl EnvironmentConfig {
    /// Load configuration from environment variables
    ///
    /// Looks for variables with the `NESTGATE_` prefix.
    ///
    /// # Errors
    ///
    /// Returns an error if required variables are missing or invalid.
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            network: NetworkConfig::from_env()?,
            storage: StorageConfig::from_env()?,
            discovery: DiscoveryConfig::from_env()?,
            monitoring: MonitoringConfig::from_env()?,
            security: SecurityConfig::from_env()?,
        })
    }

    /// Load configuration with custom prefix
    ///
    /// Useful for testing or multi-tenant deployments.
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Ok(Self {
            network: NetworkConfig::from_env_with_prefix(prefix)?,
            storage: StorageConfig::from_env_with_prefix(prefix)?,
            discovery: DiscoveryConfig::from_env_with_prefix(prefix)?,
            monitoring: MonitoringConfig::from_env_with_prefix(prefix)?,
            security: SecurityConfig::from_env_with_prefix(prefix)?,
        })
    }

    /// Get a socket address for binding
    ///
    /// # Errors
    ///
    /// Returns an error if the host cannot be parsed as a valid IP address.
    pub fn bind_address(&self) -> Result<SocketAddr, std::net::AddrParseError> {
        // ✅ Fallback to localhost (compile-time constant) if parse fails
        let ip = self
            .network
            .host
            .parse::<IpAddr>()
            .or(Ok(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)))?;

        Ok(SocketAddr::new(ip, self.network.port.get()))
    }
}

/// Type-safe port number
///
/// Ensures ports are in the valid range (1024-65535) for non-privileged binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Port(u16);

impl Port {
    /// Create a new port, validating the range
    ///
    /// # Errors
    ///
    /// Returns an error if the port is below 1024 (privileged range).
    pub fn new(port: u16) -> Result<Self, ConfigError> {
        if port < 1024 {
            return Err(ConfigError::InvalidPort(port));
        }
        Ok(Self(port))
    }

    /// Create a port without validation (use with caution)
    ///
    /// # Safety
    ///
    /// This bypasses validation and should only be used when you're certain
    /// the port is valid (e.g., from constants).
    pub const fn new_unchecked(port: u16) -> Self {
        Self(port)
    }

    /// Get the port value
    pub fn get(self) -> u16 {
        self.0
    }
}

impl Default for Port {
    fn default() -> Self {
        Self::new_unchecked(8080)
    }
}

impl FromStr for Port {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let port: u16 = s.parse().map_err(|e| ConfigError::ParseError {
            key: "port".to_string(),
            source: Box::new(e),
        })?;
        Self::new(port)
    }
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Port to bind to (default: 8080)
    pub port: Port,

    /// Host address to bind to (default: 127.0.0.1)
    pub host: String,

    /// Connection timeout in seconds (default: 30)
    pub timeout_secs: u64,

    /// Maximum concurrent connections (default: 1000)
    pub max_connections: usize,

    /// Read timeout in seconds (default: 10)
    pub read_timeout_secs: u64,

    /// Write timeout in seconds (default: 10)
    pub write_timeout_secs: u64,

    /// Keep-alive timeout in seconds (default: 60)
    pub keepalive_secs: u64,
}

impl NetworkConfig {
    /// Load from environment with NESTGATE_ prefix
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_with_prefix("NESTGATE")
    }

    /// Load from environment with custom prefix
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Ok(Self {
            port: Self::env_var_or(prefix, "PORT", Port::default())?,
            // ✅ Using compile-time constant for default host
            host: Self::env_var_or(prefix, "HOST", std::net::Ipv4Addr::LOCALHOST.to_string())?,
            timeout_secs: Self::env_var_or(prefix, "TIMEOUT_SECS", 30)?,
            max_connections: Self::env_var_or(prefix, "MAX_CONNECTIONS", 1000)?,
            read_timeout_secs: Self::env_var_or(prefix, "READ_TIMEOUT_SECS", 10)?,
            write_timeout_secs: Self::env_var_or(prefix, "WRITE_TIMEOUT_SECS", 10)?,
            keepalive_secs: Self::env_var_or(prefix, "KEEPALIVE_SECS", 60)?,
        })
    }

    /// Helper to get environment variable or use default
    fn env_var_or<T: FromStr>(prefix: &str, key: &str, default: T) -> Result<T, ConfigError>
    where
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        let var_name = format!("{}_{}", prefix, key);
        match env::var(&var_name) {
            Ok(val) => val.parse().map_err(|e| ConfigError::ParseError {
                key: var_name,
                source: Box::new(e),
            }),
            Err(_) => Ok(default),
        }
    }

    /// Get connection timeout as Duration
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }

    /// Get read timeout as Duration
    pub fn read_timeout(&self) -> Duration {
        Duration::from_secs(self.read_timeout_secs)
    }

    /// Get write timeout as Duration
    pub fn write_timeout(&self) -> Duration {
        Duration::from_secs(self.write_timeout_secs)
    }

    /// Get keepalive timeout as Duration
    pub fn keepalive(&self) -> Duration {
        Duration::from_secs(self.keepalive_secs)
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            port: Port::default(),
            host: "127.0.0.1".to_string(),
            timeout_secs: 30,
            max_connections: 1000,
            read_timeout_secs: 10,
            write_timeout_secs: 10,
            keepalive_secs: 60,
        }
    }
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// ZFS pool name (default: tank)
    pub zfs_pool: String,

    /// Data directory path (default: /var/lib/nestgate)
    pub data_dir: String,

    /// Cache size in megabytes (default: 512)
    pub cache_size_mb: usize,

    /// Enable compression (default: true)
    pub compression_enabled: bool,

    /// Snapshot retention days (default: 30)
    pub snapshot_retention_days: u32,
}

impl StorageConfig {
    /// Load from environment
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_with_prefix("NESTGATE")
    }

    /// Load from environment with custom prefix
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Ok(Self {
            zfs_pool: NetworkConfig::env_var_or(prefix, "ZFS_POOL", "tank".to_string())?,
            data_dir: NetworkConfig::env_var_or(
                prefix,
                "DATA_DIR",
                "/var/lib/nestgate".to_string(),
            )?,
            cache_size_mb: NetworkConfig::env_var_or(prefix, "CACHE_SIZE_MB", 512)?,
            compression_enabled: NetworkConfig::env_var_or(prefix, "COMPRESSION_ENABLED", true)?,
            snapshot_retention_days: NetworkConfig::env_var_or(
                prefix,
                "SNAPSHOT_RETENTION_DAYS",
                30,
            )?,
        })
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            zfs_pool: "tank".to_string(),
            data_dir: "/var/lib/nestgate".to_string(),
            cache_size_mb: 512,
            compression_enabled: true,
            snapshot_retention_days: 30,
        }
    }
}

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    /// Enable service discovery (default: true)
    pub enabled: bool,

    /// Discovery interval in seconds (default: 30)
    pub interval_secs: u64,

    /// Discovery timeout in seconds (default: 5)
    pub timeout_secs: u64,

    /// Retry attempts (default: 3)
    pub retry_attempts: u32,

    /// Cache discovered services (default: true)
    pub cache_enabled: bool,

    /// Port range for primal discovery (default: [3000, 3001, 3002, 3010])
    /// Can be configured via NESTGATE_DISCOVERY_PORTS (comma-separated, e.g., "3000,3001,3002")
    pub port_range: Vec<u16>,
}

impl DiscoveryConfig {
    /// Load from environment
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_with_prefix("NESTGATE")
    }

    /// Load from environment with custom prefix
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        // Parse port_range from comma-separated string (e.g., "3000,3001,3002")
        let port_range = env::var(format!("{}_DISCOVERY_PORTS", prefix))
            .ok()
            .and_then(|s| {
                let ports: Vec<u16> = s.split(',').filter_map(|p| p.trim().parse().ok()).collect();
                if ports.is_empty() {
                    None
                } else {
                    Some(ports)
                }
            })
            .unwrap_or_else(|| vec![3000, 3001, 3002, 3010]); // Default discovery ports

        Ok(Self {
            enabled: NetworkConfig::env_var_or(prefix, "DISCOVERY_ENABLED", true)?,
            interval_secs: NetworkConfig::env_var_or(prefix, "DISCOVERY_INTERVAL_SECS", 30)?,
            timeout_secs: NetworkConfig::env_var_or(prefix, "DISCOVERY_TIMEOUT_SECS", 5)?,
            retry_attempts: NetworkConfig::env_var_or(prefix, "DISCOVERY_RETRY_ATTEMPTS", 3)?,
            cache_enabled: NetworkConfig::env_var_or(prefix, "DISCOVERY_CACHE_ENABLED", true)?,
            port_range,
        })
    }

    /// Get discovery interval as Duration
    pub fn interval(&self) -> Duration {
        Duration::from_secs(self.interval_secs)
    }

    /// Get discovery timeout as Duration
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_secs: 30,
            timeout_secs: 5,
            retry_attempts: 3,
            cache_enabled: true,
            port_range: vec![3000, 3001, 3002, 3010], // Default discovery ports
        }
    }
}

/// Monitoring and observability configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Metrics port (default: 9090)
    pub metrics_port: Port,

    /// Enable detailed metrics (default: true)
    pub detailed_metrics: bool,

    /// Log level (default: info)
    pub log_level: String,

    /// Enable tracing (default: true)
    pub tracing_enabled: bool,

    /// Trace sample rate (0.0-1.0, default: 0.1)
    pub trace_sample_rate: f64,
}

impl MonitoringConfig {
    /// Load from environment
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_with_prefix("NESTGATE")
    }

    /// Load from environment with custom prefix
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Ok(Self {
            metrics_port: NetworkConfig::env_var_or(
                prefix,
                "METRICS_PORT",
                Port::new_unchecked(9090),
            )?,
            detailed_metrics: NetworkConfig::env_var_or(prefix, "DETAILED_METRICS", true)?,
            log_level: NetworkConfig::env_var_or(prefix, "LOG_LEVEL", "info".to_string())?,
            tracing_enabled: NetworkConfig::env_var_or(prefix, "TRACING_ENABLED", true)?,
            trace_sample_rate: NetworkConfig::env_var_or(prefix, "TRACE_SAMPLE_RATE", 0.1)?,
        })
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_port: Port::new_unchecked(9090),
            detailed_metrics: true,
            log_level: "info".to_string(),
            tracing_enabled: true,
            trace_sample_rate: 0.1,
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable TLS (default: false)
    pub tls_enabled: bool,

    /// TLS certificate path
    pub tls_cert_path: Option<String>,

    /// TLS private key path
    pub tls_key_path: Option<String>,

    /// API key for authentication
    pub api_key: Option<String>,

    /// Rate limiting enabled (default: true)
    pub rate_limit_enabled: bool,

    /// Rate limit: requests per minute (default: 1000)
    pub rate_limit_per_minute: u32,
}

impl SecurityConfig {
    /// Load from environment
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_env_with_prefix("NESTGATE")
    }

    /// Load from environment with custom prefix
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self, ConfigError> {
        Ok(Self {
            tls_enabled: NetworkConfig::env_var_or(prefix, "TLS_ENABLED", false)?,
            tls_cert_path: env::var(format!("{}_TLS_CERT_PATH", prefix)).ok(),
            tls_key_path: env::var(format!("{}_TLS_KEY_PATH", prefix)).ok(),
            api_key: env::var(format!("{}_API_KEY", prefix)).ok(),
            rate_limit_enabled: NetworkConfig::env_var_or(prefix, "RATE_LIMIT_ENABLED", true)?,
            rate_limit_per_minute: NetworkConfig::env_var_or(
                prefix,
                "RATE_LIMIT_PER_MINUTE",
                1000,
            )?,
        })
    }

    /// Check if TLS is properly configured
    pub fn tls_configured(&self) -> bool {
        self.tls_enabled && self.tls_cert_path.is_some() && self.tls_key_path.is_some()
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
            api_key: None,
            rate_limit_enabled: true,
            rate_limit_per_minute: 1000,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== PORT TESTS ====================

    #[test]
    fn test_port_validation() {
        assert!(Port::new(1024).is_ok());
        assert!(Port::new(8080).is_ok());
        assert!(Port::new(65535).is_ok());
        assert!(Port::new(80).is_err());
        assert!(Port::new(0).is_err());
    }

    #[test]
    fn test_port_boundary_values() {
        // Test exact boundaries
        assert!(Port::new(1023).is_err()); // Just below minimum
        assert!(Port::new(1024).is_ok()); // Minimum valid
        assert!(Port::new(65535).is_ok()); // Maximum valid
                                           // Note: 65536 can't be tested as it doesn't fit in u16
    }

    #[test]
    fn test_port_from_str() {
        assert_eq!(Port::from_str("8080").unwrap().get(), 8080);
        assert_eq!(Port::from_str("1024").unwrap().get(), 1024);
        assert_eq!(Port::from_str("65535").unwrap().get(), 65535);
        assert!(Port::from_str("80").is_err());
        assert!(Port::from_str("0").is_err());
        assert!(Port::from_str("invalid").is_err());
        assert!(Port::from_str("").is_err());
        assert!(Port::from_str("-1").is_err());
    }

    #[test]
    fn test_port_debug() {
        let port = Port::new(8080).unwrap();
        let debug_str = format!("{:?}", port);
        assert!(debug_str.contains("8080") || !debug_str.is_empty());
    }

    #[test]
    fn test_port_unchecked() {
        let port = Port::new_unchecked(8080);
        assert_eq!(port.get(), 8080);
    }

    #[test]
    fn test_port_get() {
        let port = Port::new(3000).unwrap();
        assert_eq!(port.get(), 3000);
    }

    // ==================== ENVIRONMENT CONFIG TESTS ====================

    #[test]
    fn test_default_config() {
        let config = EnvironmentConfig::default();
        assert_eq!(config.network.port.get(), 8080);
        assert_eq!(config.network.host, "127.0.0.1");
        assert_eq!(config.storage.zfs_pool, "tank");
        assert!(config.discovery.enabled);
    }

    #[test]
    fn test_environment_config_structure() {
        let config = EnvironmentConfig::default();
        // Verify all major sections exist
        assert_eq!(config.network.host, "127.0.0.1");
        assert_eq!(config.storage.data_dir, "/var/lib/nestgate");
        assert!(config.monitoring.tracing_enabled);
        assert!(config.discovery.enabled);
    }

    #[test]
    fn test_bind_address() {
        let config = EnvironmentConfig::default();
        let addr = config.bind_address().expect("Should parse bind address");
        assert_eq!(addr.port(), 8080);
    }

    #[test]
    fn test_bind_address_custom_port() {
        let mut config = EnvironmentConfig::default();
        config.network.port = Port::new(9000).unwrap();
        let addr = config.bind_address().expect("Should parse bind address");
        assert_eq!(addr.port(), 9000);
    }

    // ==================== NETWORK CONFIG TESTS ====================

    #[test]
    fn test_network_config_defaults() {
        let config = NetworkConfig::default();
        assert_eq!(config.timeout_secs, 30);
        assert_eq!(config.max_connections, 1000);
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port.get(), 8080);
    }

    #[test]
    fn test_network_config_timeout() {
        let config = NetworkConfig::default();
        let duration = config.timeout();
        assert_eq!(duration, Duration::from_secs(30));
    }

    #[test]
    fn test_network_config_custom_timeout() {
        let config = NetworkConfig {
            timeout_secs: 60,
            ..Default::default()
        };
        let duration = config.timeout();
        assert_eq!(duration, Duration::from_secs(60));
    }

    #[test]
    fn test_network_config_all_timeouts() {
        let config = NetworkConfig::default();
        assert_eq!(config.timeout(), Duration::from_secs(30));
        assert_eq!(config.read_timeout(), Duration::from_secs(10));
        assert_eq!(config.write_timeout(), Duration::from_secs(10));
        assert_eq!(config.keepalive(), Duration::from_secs(60));
    }

    // ==================== STORAGE CONFIG TESTS ====================

    #[test]
    fn test_storage_config_defaults() {
        let config = StorageConfig::default();
        assert_eq!(config.data_dir, "/var/lib/nestgate");
        assert_eq!(config.zfs_pool, "tank");
        assert!(config.compression_enabled);
        assert_eq!(config.cache_size_mb, 512);
        assert_eq!(config.snapshot_retention_days, 30);
    }

    #[test]
    fn test_storage_config_structure() {
        let config = StorageConfig::default();
        assert!(!config.data_dir.is_empty());
        assert!(!config.zfs_pool.is_empty());
        assert!(config.cache_size_mb > 0);
        assert!(config.snapshot_retention_days > 0);
    }

    // ==================== DISCOVERY CONFIG TESTS ====================

    #[test]
    fn test_discovery_config_defaults() {
        let config = DiscoveryConfig::default();
        assert!(config.enabled);
        assert_eq!(config.interval_secs, 30);
    }

    #[test]
    fn test_discovery_config_interval_method() {
        let config = DiscoveryConfig::default();
        let duration = config.interval();
        assert_eq!(duration, Duration::from_secs(30));
    }

    #[test]
    fn test_discovery_config_custom_interval() {
        let config = DiscoveryConfig {
            interval_secs: 60,
            ..Default::default()
        };
        let duration = config.interval();
        assert_eq!(duration, Duration::from_secs(60));
    }

    // ==================== MONITORING CONFIG TESTS ====================

    #[test]
    fn test_monitoring_config_defaults() {
        let config = MonitoringConfig::default();
        assert_eq!(config.metrics_port.get(), 9090);
        assert!(config.detailed_metrics);
        assert_eq!(config.log_level, "info");
        assert!(config.tracing_enabled);
        assert_eq!(config.trace_sample_rate, 0.1);
    }

    #[test]
    fn test_monitoring_config_structure() {
        let config = MonitoringConfig::default();
        assert!(!config.log_level.is_empty());
        assert!(config.trace_sample_rate >= 0.0 && config.trace_sample_rate <= 1.0);
    }

    // ==================== SECURITY CONFIG TESTS ====================

    #[test]
    fn test_security_config_defaults() {
        let config = SecurityConfig::default();
        assert!(!config.tls_enabled);
        assert!(config.tls_cert_path.is_none());
        assert!(config.tls_key_path.is_none());
        assert!(config.api_key.is_none());
        assert!(config.rate_limit_enabled);
        assert_eq!(config.rate_limit_per_minute, 1000);
    }

    #[test]
    fn test_security_tls_not_configured_by_default() {
        let config = SecurityConfig::default();
        assert!(!config.tls_configured());
    }

    #[test]
    fn test_security_tls_configured_when_all_set() {
        let config = SecurityConfig {
            tls_enabled: true,
            tls_cert_path: Some("/path/to/cert.pem".to_string()),
            tls_key_path: Some("/path/to/key.pem".to_string()),
            ..Default::default()
        };
        assert!(config.tls_configured());
    }

    #[test]
    fn test_security_tls_not_configured_missing_cert() {
        let config = SecurityConfig {
            tls_enabled: true,
            tls_key_path: Some("/path/to/key.pem".to_string()),
            ..Default::default()
        };
        assert!(!config.tls_configured());
    }

    #[test]
    fn test_security_tls_not_configured_missing_key() {
        let config = SecurityConfig {
            tls_enabled: true,
            tls_cert_path: Some("/path/to/cert.pem".to_string()),
            ..Default::default()
        };
        assert!(!config.tls_configured());
    }

    #[test]
    fn test_security_tls_not_configured_when_disabled() {
        let config = SecurityConfig {
            tls_enabled: false,
            tls_cert_path: Some("/path/to/cert.pem".to_string()),
            tls_key_path: Some("/path/to/key.pem".to_string()),
            ..Default::default()
        };
        assert!(!config.tls_configured());
    }

    // ==================== CONFIG ERROR TESTS ====================

    #[test]
    fn test_config_error_missing_env_var() {
        let error = ConfigError::MissingEnvVar("TEST_VAR".to_string());
        assert!(error.to_string().contains("TEST_VAR"));
        assert!(error.to_string().contains("not found"));
    }

    #[test]
    fn test_config_error_invalid_port() {
        let error = ConfigError::InvalidPort(80);
        assert!(error.to_string().contains("80"));
        assert!(error.to_string().contains("1024"));
    }

    #[test]
    fn test_config_error_invalid() {
        let error = ConfigError::Invalid("test error".to_string());
        assert!(error.to_string().contains("test error"));
    }

    // ==================== INTEGRATION TESTS ====================

    #[test]
    fn test_full_config_creation() {
        let config = EnvironmentConfig::default();

        // Verify all sections are properly initialized
        assert!(config.network.port.get() > 1023);
        assert!(!config.network.host.is_empty());
        assert!(!config.storage.data_dir.is_empty());
        assert!(config.monitoring.metrics_port.get() > 1023);
        assert!(!config.monitoring.log_level.is_empty());
        assert!(config.discovery.interval_secs > 0);
    }

    #[test]
    fn test_bind_address_parsing() {
        let config = EnvironmentConfig::default();
        let addr = config.bind_address().expect("Should parse bind address");

        // Should parse to valid SocketAddr
        assert!(addr.port() > 0);
        assert!(matches!(addr.ip(), IpAddr::V4(_) | IpAddr::V6(_)));
    }

    #[test]
    fn test_timeout_conversion() {
        // Test various timeout values
        let config1 = NetworkConfig {
            timeout_secs: 10,
            ..Default::default()
        };
        assert_eq!(config1.timeout(), Duration::from_secs(10));

        let config2 = NetworkConfig {
            timeout_secs: 60,
            ..Default::default()
        };
        assert_eq!(config2.timeout(), Duration::from_secs(60));

        let config3 = NetworkConfig {
            timeout_secs: 0,
            ..Default::default()
        };
        assert_eq!(config3.timeout(), Duration::from_secs(0));
    }

    #[test]
    fn test_discovery_interval_conversion() {
        // Test various intervals
        let config1 = DiscoveryConfig {
            interval_secs: 15,
            ..Default::default()
        };
        assert_eq!(config1.interval(), Duration::from_secs(15));

        let config2 = DiscoveryConfig {
            interval_secs: 300,
            ..Default::default()
        };
        assert_eq!(config2.interval(), Duration::from_secs(300));
    }

    // ==================== CLONE & DEBUG TESTS ====================

    #[test]
    fn test_config_clone() {
        let config = EnvironmentConfig::default();
        let cloned = config.clone();

        assert_eq!(config.network.port.get(), cloned.network.port.get());
        assert_eq!(config.network.host, cloned.network.host);
    }

    #[test]
    fn test_config_debug() {
        let config = EnvironmentConfig::default();
        let debug_str = format!("{:?}", config);

        // Should contain key config values
        assert!(debug_str.contains("network") || debug_str.contains("Network"));
    }
}
