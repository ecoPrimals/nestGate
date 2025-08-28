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
pub struct SystemConfig<
    const MAX_CONNECTIONS: usize = 1000,
    const BUFFER_SIZE: usize = 65536,
> {
    /// System instance identifier
    pub instance_id: String,
    /// Human-readable instance name
    pub instance_name: String,
    /// Service version
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
    /// Maximum memory usage (MB)
    pub max_memory_mb: Option<u64>,
    /// Maximum CPU cores to use
    pub max_cpu_cores: Option<usize>,
    /// Startup timeout
    pub startup_timeout: Duration,
    /// Graceful shutdown timeout
    pub shutdown_timeout: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Runtime override for MAX_CONNECTIONS
    pub max_connections_override: Option<usize>,
    /// Runtime override for BUFFER_SIZE
    pub buffer_size_override: Option<usize>,
}

impl<const MAX_CONNECTIONS: usize, const BUFFER_SIZE: usize> SystemConfig<MAX_CONNECTIONS, BUFFER_SIZE> {
    /// Get effective max connections (compile-time optimized)
    pub const fn max_connections() -> usize {
        MAX_CONNECTIONS
    }
    
    /// Get effective buffer size (compile-time optimized)
    pub const fn buffer_size() -> usize {
        BUFFER_SIZE
    }
    
    /// Get runtime max connections (with override support)
    pub fn effective_max_connections(&self) -> usize {
        self.max_connections_override.unwrap_or(MAX_CONNECTIONS)
    }
    
    /// Get runtime buffer size (with override support)
    pub fn effective_buffer_size(&self) -> usize {
        self.buffer_size_override.unwrap_or(BUFFER_SIZE)
    }
}

// ==================== SECTION ====================

/// Deployment environment types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentEnvironment {
    Development,
    Testing,
    Staging,
    Production,
    Performance,
    Security,
}

impl Default for DeploymentEnvironment {
    fn default() -> Self {
        Self::Development
    }
}

/// Log level configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

/// Environment-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory usage (bytes)
    pub max_memory_bytes: Option<u64>,
    /// Maximum CPU usage (percentage)
    pub max_cpu_percent: Option<f64>,
    /// Maximum disk usage (bytes)
    pub max_disk_bytes: Option<u64>,
    /// Maximum network bandwidth (bytes/sec)
    pub max_network_bps: Option<u64>,
    /// Maximum file descriptors
    pub max_file_descriptors: Option<u32>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: None,
            max_cpu_percent: None,
            max_disk_bytes: None,
            max_network_bps: None,
            max_file_descriptors: None,
        }
    }
}

/// Feature flags configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Enable metrics (alias for metrics_collection)
    pub enable_metrics: bool,
    /// Enable tracing (alias for distributed_tracing)
    pub enable_tracing: bool,
    /// Feature-specific flags
    pub features: std::collections::HashMap<String, bool>,
}

impl Default for FeatureFlags {
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
            source: "canonical_master".to_string(),
            checksum: None,
            schema_version: "2.0.0".to_string(),
        }
    }
}

// ==================== SECTION ====================

impl<const MAX_CONNECTIONS: usize, const BUFFER_SIZE: usize> Default for SystemConfig<MAX_CONNECTIONS, BUFFER_SIZE> {
    fn default() -> Self {
        // Generate a simple UUID-like string without external dependencies
        let instance_id = format!("nestgate-{}", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
        
        Self {
            instance_id,
            instance_name: "nestgate".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: DeploymentEnvironment::Development,
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