// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// **SYSTEM CONFIGURATION**
///
/// System-level configuration with const generics for performance optimization.
/// This module contains all system-level settings including deployment environment,
/// logging, resource limits, and runtime configuration.
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
// ==================== SECTION ====================

/// System-level configuration with const generics for performance
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for System
pub struct SystemConfig<const MAX_CONNECTIONS: usize = 1000, const BUFFER_SIZE: usize = 65536> {
    /// System instance identifier
    pub instance_id: String,
    /// Human-readable instance name
    pub instance_name: String,
    /// System version identifier
    pub version: String,
    /// Deployment environment
    pub environment: DeploymentEnvironment,
    /// Log level configuration
    pub log_level: LogLevel,
    /// Enable debug mode
    pub debug_mode: bool,
    /// Data directory for persistent storage
    pub data_dir: PathBuf,
    /// Configuration directory
    pub config_dir: PathBuf,
    /// Process ID file location
    pub pid_file: Option<PathBuf>,
    /// Maximum memory limit in megabytes
    pub max_memory_mb: Option<u64>,
    /// Maximum CPU cores to utilize
    pub max_cpu_cores: Option<usize>,
    /// Startup timeout
    pub startup_timeout: Duration,
    /// Graceful shutdown timeout
    pub shutdown_timeout: Duration,
    /// Interval between health checks
    pub health_check_interval: Duration,
    /// Runtime override for `MAX_CONNECTIONS`
    pub max_connections_override: Option<usize>,
    /// Runtime override for `BUFFER_SIZE`
    pub buffer_size_override: Option<usize>,
}
impl<const MAX_CONNECTIONS: usize, const BUFFER_SIZE: usize>
    SystemConfig<MAX_CONNECTIONS, BUFFER_SIZE>
{
    /// Get effective max connections (compile-time optimized)
    #[must_use]
    pub const fn max_connections() -> usize {
        MAX_CONNECTIONS
    }

    /// Get effective buffer size (compile-time optimized)
    #[must_use]
    pub const fn buffer_size() -> usize {
        BUFFER_SIZE
    }

    /// Get runtime max connections (with override support)
    #[must_use]
    pub fn effective_max_connections(&self) -> usize {
        self.max_connections_override.unwrap_or(MAX_CONNECTIONS)
    }

    /// Get runtime buffer size (with override support)
    #[must_use]
    pub fn effective_buffer_size(&self) -> usize {
        self.buffer_size_override.unwrap_or(BUFFER_SIZE)
    }
}

// ==================== SECTION ====================

/// Deployment environment types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Deploymentenvironment
pub enum DeploymentEnvironment {
    /// Development environment for local development
    Development,
    /// Testing environment for automated tests
    Testing,
    /// Staging environment for pre-production testing
    Staging,
    /// Production environment for live deployment
    Production,
    /// Performance testing environment
    Performance,
    /// Security testing environment
    Security,
}
impl Default for DeploymentEnvironment {
    /// Returns the default instance
    fn default() -> Self {
        Self::Development
    }
}

/// Log level configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Loglevel
pub enum LogLevel {
    /// Error log level (only errors)
    Error,
    /// Warning log level (warnings and errors)
    Warn,
    /// Info log level (informational messages)
    Info,
    /// Debug log level (detailed debugging information)
    Debug,
    /// Trace log level (very detailed tracing)
    Trace,
}
impl Default for LogLevel {
    /// Returns the default instance
    fn default() -> Self {
        Self::Info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_config_const_generics() {
        assert_eq!(SystemConfig::<1000, 65536>::max_connections(), 1000);
        assert_eq!(SystemConfig::<1000, 65536>::buffer_size(), 65536);
        assert_eq!(SystemConfig::<500, 32768>::max_connections(), 500);
        assert_eq!(SystemConfig::<500, 32768>::buffer_size(), 32768);
    }

    #[test]
    fn test_system_config_effective_values_without_override() {
        let config = SystemConfig::<1000, 65536> {
            instance_id: "test".to_string(),
            instance_name: "Test".to_string(),
            version: "1.0.0".to_string(),
            environment: DeploymentEnvironment::Development,
            log_level: LogLevel::Info,
            debug_mode: false,
            data_dir: PathBuf::from("/data"),
            config_dir: PathBuf::from("/config"),
            pid_file: None,
            max_memory_mb: None,
            max_cpu_cores: None,
            startup_timeout: Duration::from_secs(30),
            shutdown_timeout: Duration::from_secs(10),
            health_check_interval: Duration::from_secs(5),
            max_connections_override: None,
            buffer_size_override: None,
        };

        assert_eq!(config.effective_max_connections(), 1000);
        assert_eq!(config.effective_buffer_size(), 65536);
    }

    #[test]
    fn test_system_config_effective_values_with_override() {
        let config = SystemConfig::<1000, 65536> {
            instance_id: "test".to_string(),
            instance_name: "Test".to_string(),
            version: "1.0.0".to_string(),
            environment: DeploymentEnvironment::Production,
            log_level: LogLevel::Warn,
            debug_mode: false,
            data_dir: PathBuf::from("/data"),
            config_dir: PathBuf::from("/config"),
            pid_file: None,
            max_memory_mb: None,
            max_cpu_cores: None,
            startup_timeout: Duration::from_secs(30),
            shutdown_timeout: Duration::from_secs(10),
            health_check_interval: Duration::from_secs(5),
            max_connections_override: Some(2000),
            buffer_size_override: Some(131_072),
        };

        assert_eq!(config.effective_max_connections(), 2000);
        assert_eq!(config.effective_buffer_size(), 131_072);
    }

    #[test]
    fn test_deployment_environment_default() {
        let default_env = DeploymentEnvironment::default();
        assert_eq!(default_env, DeploymentEnvironment::Development);
    }

    #[test]
    fn test_deployment_environment_variants() {
        assert_eq!(
            DeploymentEnvironment::Development,
            DeploymentEnvironment::Development
        );
        assert_ne!(
            DeploymentEnvironment::Development,
            DeploymentEnvironment::Production
        );
        assert_ne!(
            DeploymentEnvironment::Testing,
            DeploymentEnvironment::Staging
        );
    }

    #[test]
    fn test_log_level_default() {
        let default_level = LogLevel::default();
        assert_eq!(default_level, LogLevel::Info);
    }

    #[test]
    fn test_log_level_variants() {
        assert_eq!(LogLevel::Error, LogLevel::Error);
        assert_ne!(LogLevel::Error, LogLevel::Warn);
        assert_ne!(LogLevel::Info, LogLevel::Debug);
    }

    #[test]
    fn test_system_config_production_settings() {
        let config = SystemConfig::<5000, 262_144> {
            instance_id: "prod-001".to_string(),
            instance_name: "Production Instance 1".to_string(),
            version: "2.0.0".to_string(),
            environment: DeploymentEnvironment::Production,
            log_level: LogLevel::Warn,
            debug_mode: false,
            data_dir: PathBuf::from("/var/lib/nestgate"),
            config_dir: PathBuf::from("/etc/nestgate"),
            pid_file: Some(PathBuf::from("/var/run/nestgate.pid")),
            max_memory_mb: Some(16384),
            max_cpu_cores: Some(16),
            startup_timeout: Duration::from_secs(60),
            shutdown_timeout: Duration::from_secs(30),
            health_check_interval: Duration::from_secs(10),
            max_connections_override: None,
            buffer_size_override: None,
        };

        assert_eq!(config.environment, DeploymentEnvironment::Production);
        assert_eq!(config.log_level, LogLevel::Warn);
        assert!(!config.debug_mode);
    }

    #[test]
    fn test_system_config_timeouts() {
        let config = SystemConfig::<1000, 65536> {
            instance_id: "test".to_string(),
            instance_name: "Test".to_string(),
            version: "1.0.0".to_string(),
            environment: DeploymentEnvironment::Development,
            log_level: LogLevel::Info,
            debug_mode: false,
            data_dir: PathBuf::from("/data"),
            config_dir: PathBuf::from("/config"),
            pid_file: None,
            max_memory_mb: None,
            max_cpu_cores: None,
            startup_timeout: Duration::from_secs(45),
            shutdown_timeout: Duration::from_secs(15),
            health_check_interval: Duration::from_secs(3),
            max_connections_override: None,
            buffer_size_override: None,
        };

        assert_eq!(config.startup_timeout, Duration::from_secs(45));
        assert_eq!(config.shutdown_timeout, Duration::from_secs(15));
        assert_eq!(config.health_check_interval, Duration::from_secs(3));
    }

    #[test]
    fn test_const_generics_compile_time_optimization() {
        // Test that const generics provide compile-time type differentiation
        let max_conn = SystemConfig::<2000, 128_000>::max_connections();
        let buffer = SystemConfig::<2000, 128_000>::buffer_size();

        assert_eq!(max_conn, 2000);
        assert_eq!(buffer, 128_000);
    }

    #[test]
    fn test_different_const_generic_configurations() {
        // Type alias for SmallConfig
        type SmallConfig = SystemConfig<100, 4096>;
        assert_eq!(SmallConfig::max_connections(), 100);
        assert_eq!(SmallConfig::buffer_size(), 4096);

        // Type alias for MediumConfig
        type MediumConfig = SystemConfig<1000, 65536>;
        assert_eq!(MediumConfig::max_connections(), 1000);
        assert_eq!(MediumConfig::buffer_size(), 65536);

        // Type alias for LargeConfig
        type LargeConfig = SystemConfig<10_000, 1_048_576>;
        assert_eq!(LargeConfig::max_connections(), 10_000);
        assert_eq!(LargeConfig::buffer_size(), 1_048_576);
    }
}

/// Environment-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Environment
pub struct EnvironmentConfig {
    /// Environment name
    pub name: String,
    /// Environment-specific variables
    pub variables: std::collections::HashMap<String, String>,
    /// Environment-specific feature flags
    pub features: std::collections::HashMap<String, bool>,
    /// Environment-specific resource limits
    pub resource_limits: ResourceLimits,
}
impl Default for EnvironmentConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            name: "development".to_string(),
            variables: std::collections::HashMap::new(),
            features: std::collections::HashMap::new(),
            resource_limits: ResourceLimits::default(),
        }
    }
}

/// Resource limits configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceLimits {
    /// Maximum memory allocation in bytes
    pub max_memory_bytes: Option<u64>,
    /// Maximum CPU usage as percentage
    pub max_cpu_percent: Option<f64>,
    /// Maximum disk space in bytes
    pub max_disk_bytes: Option<u64>,
    /// Maximum network bandwidth in bits per second
    pub max_network_bps: Option<u64>,
    /// Maximum number of file descriptors
    pub max_file_descriptors: Option<u32>,
}

/// Feature flags configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Featureflags
pub struct FeatureFlags {
    /// Enable experimental features
    pub experimental_features: bool,
    /// Enable performance monitoring
    pub performance_monitoring: bool,
    /// Enable debug logging
    pub debug_logging: bool,
    /// Enable metrics collection
    pub metrics_collection: bool,
    /// Enable distributed tracing
    pub distributed_tracing: bool,
    /// Enable auto scaling
    pub enable_auto_scaling: bool,
    /// Enable load balancing
    pub enable_load_balancing: bool,
    /// Enable metrics (alias for `metrics_collection`)
    pub enable_metrics: bool,
    /// Enable tracing (alias for `distributed_tracing`)
    pub enable_tracing: bool,
    /// Feature-specific flags
    pub features: std::collections::HashMap<String, bool>,
}
impl Default for FeatureFlags {
    /// Returns the default instance
    fn default() -> Self {
        let mut features = std::collections::HashMap::new();
        features.insert("async_trait_migration".to_string(), false);
        features.insert("zero_cost_abstractions".to_string(), true);
        features.insert("canonical_config".to_string(), true);

        Self {
            experimental_features: false,
            performance_monitoring: true,
            debug_logging: false,
            metrics_collection: true,
            distributed_tracing: false,
            enable_auto_scaling: false,
            enable_load_balancing: false,
            enable_metrics: true,
            enable_tracing: false,
            features,
        }
    }
}

/// Configuration metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configmetadata
pub struct ConfigMetadata {
    /// Configuration version
    pub version: String,
    /// Creation timestamp
    pub created_at: String,
    /// Last modified timestamp
    pub modified_at: String,
    /// Configuration source
    pub source: String,
    /// Configuration checksum
    pub checksum: Option<String>,
    /// Configuration schema version
    pub schema_version: String,
}
impl Default for ConfigMetadata {
    /// Returns the default instance
    fn default() -> Self {
        // Use a simple timestamp format instead of chrono
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string();

        Self {
            version: "1.0.0".to_string(),
            created_at: now.clone(),
            modified_at: now,
            source: "canonical_primary".to_string(),
            checksum: None,
            schema_version: "2.0.0".to_string(),
        }
    }
}

// ==================== SECTION ====================

impl<const MAX_CONNECTIONS: usize, const BUFFER_SIZE: usize> Default
    for SystemConfig<MAX_CONNECTIONS, BUFFER_SIZE>
{
    /// Returns the default instance
    fn default() -> Self {
        // Generate a simple UUID-like string without external dependencies
        let instance_id = format!(
            "nestgate-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );

        Self {
            instance_id,
            instance_name: "nestgate-default".to_string(),
            version: "3.0.0".to_string(),
            environment: DeploymentEnvironment::default(),
            log_level: LogLevel::Info,
            debug_mode: false,
            data_dir: PathBuf::from("./data"),
            config_dir: PathBuf::from("./config"),
            pid_file: Some(PathBuf::from("./nestgate.pid")),
            max_memory_mb: None,
            max_cpu_cores: None,
            startup_timeout: Duration::from_secs(30),
            shutdown_timeout: Duration::from_secs(10),
            health_check_interval: Duration::from_secs(60),
            max_connections_override: None,
            buffer_size_override: None,
        }
    }
}
