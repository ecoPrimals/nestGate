// Configuration Defaults
//! Defaults functionality and utilities.
// This module provides default values and a builder pattern for configuration.
// Single responsibility: Provide sensible defaults and builder functionality.

use super::types::*;
use crate::Result;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

/// Configuration builder for easy setup
pub struct CanonicalConfigBuilder {
    config: CanonicalConfig,
}
impl Default for CanonicalConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CanonicalConfigBuilder {
    /// Create a new builder with sensible defaults
    pub fn new() -> Self {
        Self {
            config: CanonicalConfig::default(),
        }
    }

    /// Set the environment
    #[must_use]
    pub fn environment(mut self, env: Environment) -> Self {
        self.config.system.environment = env.clone();
        self.config.environment.name = format!("{env:?}").to_lowercase();
        self
    }

    /// Set the instance name
    #[must_use]
    pub fn instance_name(mut self, name: impl Into<String>) -> Self {
        self.config.system.instance_name = name.into();
        self
    }

    /// Set the API server configuration
    #[must_use]
    pub fn api_server(mut self, host: IpAddr, port: u16) -> Self {
        self.config.network.api.host = host;
        self.config.network.api.port = port;
        self
    }

    /// Enable development mode
    #[must_use]
    pub fn dev_mode(mut self, enabled: bool) -> Self {
        self.config.system.dev_mode = enabled;
        self
    }

    /// Build the configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn build(self) -> Result<CanonicalConfig>  {
        super::validation::ConfigValidator::validate(&self.config)?;
        Ok(self.config)
    }
}

// Default implementation derived in types.rs

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            instance_id: None,
            instance_name: "nestgate-default".to_string(),
            log_level: "info".to_string(),
            data_dir: PathBuf::from("./data"),
            config_dir: PathBuf::from("./config"),
            dev_mode: true,
        }
    }
}

// Default implementation derived in types.rs

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".parse().unwrap(), // Safe: hardcoded valid IP
            port: 8080,
            max_connections: 1000,
            timeout: Duration::from_secs(30),
            http2: true,
            tls: None,
        }
    }
}

impl Default for InternalNetworkConfig {
    fn default() -> Self {
        Self {
            port_range: (9000, 9100),
            cluster: None,
            health_check: HealthCheckConfig::default(),
        }
    }
}

impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            method: "dns".to_string(),
            endpoint: None,
            refresh_interval: Duration::from_secs(30),
        }
    }
}

// StorageConfig now uses #[derive(Default)] in types.rs

impl Default for ZfsConfig {
    fn default() -> Self {
        Self {
            pools: vec!["default".to_string()],
            compression: "lz4".to_string(),
            deduplication: false,
        }
    }
}

impl Default for StoragePerformanceConfig {
    fn default() -> Self {
        Self {
            cache_size: 1024 * 1024 * 1024, // 1GB
            read_ahead_size: 64 * 1024,     // 64KB
            write_buffer_size: 64 * 1024,   // 64KB
        }
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            schedule: Some("0 2 * * *".to_string()), // Daily at 2 AM
            retention: RetentionPolicy::default(),
        }
    }
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            daily: 7,
            weekly: 4,
            monthly: 12,
        }
    }
}

// SecurityConfig now uses #[derive(Default)] in types.rs

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            method: "bearer".to_string(),
            providers: vec![],
        }
    }
}

// AuthzConfig now uses #[derive(Default)] in types.rs

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: "aes256".to_string(),
            key_file: None,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            requests_per_minute: 60,
            burst_size: 10,
        }
    }
}

// PerformanceConfig now uses #[derive(Default)] in types.rs

impl Default for ThreadConfig {
    fn default() -> Self {
        Self {
            worker_threads: Some(num_cpus::get()),
            stack_size: None,
        }
    }
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_memory: None,
            pool_size: 1024 * 1024 * 100, // 100MB
        }
    }
}

impl Default for IoConfig {
    fn default() -> Self {
        Self {
            buffer_size: 8192,
            max_concurrent_ops: 100,
        }
    }
}

// MonitoringConfig now uses #[derive(Default)] in types.rs

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
            interval: Duration::from_secs(15),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "text".to_string(),
            output: "stdout".to_string(),
        }
    }
}

// AlertConfig now uses #[derive(Default)] in types.rs

// IntegrationsConfig now uses #[derive(Default)] in types.rs

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            name: "development".to_string(),
            variables: HashMap::new(),
            features: HashMap::new(),
        }
    }
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            retries: 3,
        }
    }
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            timeout: Duration::from_secs(30),
        }
    }
}
