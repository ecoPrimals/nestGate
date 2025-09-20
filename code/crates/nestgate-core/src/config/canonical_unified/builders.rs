//
// Configuration builders and validation utilities for the canonical unified configuration system.

use std::time::Duration;

use super::*;

/// Configuration builder for creating NestGateCanonicalConfig
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    config: NestGateCanonicalConfig,
}
impl ConfigBuilder {
    /// Create a new configuration builder
    pub const fn new() -> Self {
        Self {
            config: NestGateCanonicalConfig::default(),
        }
    }

    /// Set system configuration
    #[must_use]
    pub fn with_system(mut self, system: SystemConfig) -> Self {
        self.config.system = system;
        self
    }

    /// Set network configuration
    #[must_use]
    pub fn with_network(mut self, network: NetworkConfig) -> Self {
        self.config.network = network;
        self
    }

    /// Set security configuration
    #[must_use]
    pub fn with_security(mut self, security: SecurityConfig) -> Self {
        self.config.security = security;
        self
    }

    /// Set storage configuration
    #[must_use]
    pub fn with_storage(mut self, storage: StorageConfig) -> Self {
        self.config.storage = storage;
        self
    }

    /// Set API configuration
    #[must_use]
    pub fn with_api(mut self, api: ApiConfig) -> Self {
        self.config.api = api;
        self
    }

    /// Set ZFS configuration
    #[must_use]
    pub fn with_zfs(mut self, zfs: ZfsConfig) -> Self {
        self.config.zfs = zfs;
        self
    }

    /// Set performance configuration
    #[must_use]
    pub fn with_performance(mut self, performance: PerformanceConfig) -> Self {
        self.config.performance = performance;
        self
    }

    /// Set environment configuration
    #[must_use]
    pub fn with_environment(mut self, environment: DeploymentEnvironment) -> Self {
        self.config.environment = environment;
        self
    }

    /// Set feature flags
    #[must_use]
    pub fn with_features(mut self, features: FeatureFlags) -> Self {
        self.config.features = features;
        self
    }

    /// Set services configuration
    #[must_use]
    pub fn with_services(mut self, services: ServiceConfigs) -> Self {
        self.config.services = services;
        self
    }

    /// Set testing configuration
    #[must_use]
    pub fn with_testing(mut self, testing: TestingConfigs) -> Self {
        self.config.testing = testing;
        self
    }

    /// Set monitoring configuration
    #[must_use]
    pub fn with_monitoring(mut self, monitoring: MonitoringConfig) -> Self {
        self.config.monitoring = monitoring;
        self
    }

    /// Set configuration metadata
    #[must_use]
    pub fn with_metadata(mut self, metadata: ConfigMetadata) -> Self {
        self.config.metadata = metadata;
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
        pub const fn build(self) -> Result<NestGateCanonicalConfig, Vec<String>>  {
        self.config.validate()?;
        Ok(self.config)
    }
}

/// System configuration builder
#[derive(Debug, Default)]
pub struct SystemConfigBuilder {
    config: SystemConfig,
}
impl SystemConfigBuilder {
    /// Create a new system configuration builder
    pub const fn new() -> Self {
        Self {
            config: SystemConfig::default(),
        }
    }

    /// Set service name
    #[must_use]
    pub fn with_service_name(mut self, name: impl Into<String>) -> Self {
        self.config.instance_name = name.into();
        self
    }

    /// Set service version
    #[must_use]
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.config.version = version.into();
        self
    }

    /// Set deployment environment
    #[must_use]
    pub fn with_environment(mut self, environment: DeploymentEnvironment) -> Self {
        self.config.environment = environment;
        self
    }

    /// Set log level
    #[must_use]
    pub fn with_log_level(mut self, level: impl Into<String>) -> Self {
        self.config.log_level = level.into();
        self
    }

    /// Set working directory
    #[must_use]
    pub fn with_working_directory(mut self, path: String) -> Self {
        self.config.working_directory = path;
        self
    }

    /// Set PID file
    #[must_use]
    pub fn with_pid_file(mut self, path: String) -> Self {
        self.config.pid_file = Some(path);
        self
    }

    /// Set maximum memory usage
    #[must_use]
    pub fn with_max_memory_mb(mut self, mb: u64) -> Self {
        self.config.max_memory_mb = Some(mb);
        self
    }

    /// Set maximum CPU cores
    #[must_use]
    pub fn with_max_cpu_cores(mut self, cores: usize) -> Self {
        self.config.max_cpu_cores = Some(cores);
        self
    }

    /// Set startup timeout
    #[must_use]
    pub fn with_startup_timeout(mut self, timeout: Duration) -> Self {
        self.config.startup_timeout = timeout;
        self
    }

    /// Set shutdown timeout
    #[must_use]
    pub fn with_shutdown_timeout(mut self, timeout: Duration) -> Self {
        self.config.shutdown_timeout = timeout;
        self
    }

    /// Set health check interval
    #[must_use]
    pub fn with_health_check_interval(mut self, interval: Duration) -> Self {
        self.config.health_check_interval = interval;
        self
    }

    /// Build the system configuration
    pub const fn build(self) -> SystemConfig {
        self.config
    }
}

/// Network configuration builder
#[derive(Debug, Default)]
pub struct NetworkConfigBuilder {
    config: NetworkConfig,
}
impl NetworkConfigBuilder {
    /// Create a new network configuration builder
    pub const fn new() -> Self {
        Self {
            config: NetworkConfig::default(),
        }
    }

    /// Set HTTP server configuration
    #[must_use]
    pub fn with_http_server(mut self, http_server: HttpServerConfig) -> Self {
        self.config.http_server = http_server;
        self
    }

    /// Set RPC configuration
    #[must_use]
    pub fn with_rpc(mut self, rpc: RpcConfig) -> Self {
        self.config.rpc = rpc;
        self
    }

    /// Set load balancing configuration
    #[must_use]
    pub fn with_load_balancing(mut self, load_balancing: LoadBalancingConfig) -> Self {
        self.config.load_balancing = load_balancing;
        self
    }

    /// Set circuit breaker configuration
    #[must_use]
    pub fn with_circuit_breaker(mut self, circuit_breaker: CircuitBreakerConfig) -> Self {
        self.config.circuit_breaker = circuit_breaker;
        self
    }

    /// Set rate limiting configuration
    #[must_use]
    pub fn with_rate_limiting(mut self, rate_limiting: RateLimitConfig) -> Self {
        self.config.rate_limiting = rate_limiting;
        self
    }

    /// Set timeout configuration
    #[must_use]
    pub fn with_timeouts(mut self, timeouts: TimeoutConfig) -> Self {
        self.config.timeouts = timeouts;
        self
    }

    /// Set connection pool configuration
    #[must_use]
    pub fn with_connection_pool(mut self, connection_pool: ConnectionPoolConfig) -> Self {
        self.config.connection_pool = connection_pool;
        self
    }

    /// Set TLS configuration
    #[must_use]
    pub fn with_tls(mut self, tls: TlsConfig) -> Self {
        self.config.tls = tls;
        self
    }

    /// Build the network configuration
    pub const fn build(self) -> NetworkConfig {
        self.config
    }
}

/// Validation utilities
pub struct ConfigValidator;
impl ConfigValidator {
    /// Validate port number
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn validate_port(port: u16) -> Result<(), String>  {
        if port == 0 {
            return Err("Port cannot be 0".to_string());
        }
        if port < 1024 && port != 80 && port != 443 {
            return Err("Port should be >= 1024 for non-privileged services".to_string());
        }
        Ok(())
    }

    /// Validate file path
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn validate_path(path: &std::path::Path) -> Result<(), String>  ", 
        if !path.exists() {
            return Err(format!("Path does not exist: {path.display()")));
        }
        Ok(())
    }

    /// Validate duration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn validate_duration(duration: Duration, min: Duration) -> Result<(), String>  {
        if duration < min {
            return Err(format!("Duration {duration:?} is less than minimum {min:?}"));
        }
        Ok(())
    }

    /// Validate memory size
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn validate_memory_size(size_mb: u64) -> Result<(), String>  {
        if size_mb < 128 {
            return Err("Memory size should be at least 128 MB".to_string());
        }
        if size_mb > 1024 * 1024 {
            return Err("Memory size should not exceed 1 TB".to_string());
        }
        Ok(())
    }
} 