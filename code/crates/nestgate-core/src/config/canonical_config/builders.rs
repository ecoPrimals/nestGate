//
// Builder patterns for constructing configuration objects with validation
// and environment variable support.

use super::*;

/// Configuration builder for NestGateCanonicalConfig
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    config: NestGateCanonicalConfig,
}

impl ConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self {
            config: NestGateCanonicalConfig::default(),
        }
    }

    /// Set system configuration
    pub fn with_system(mut self, system: SystemConfig) -> Self {
        self.config.system = system;
        self
    }

    /// Set network configuration
    pub fn with_network(mut self, network: NetworkConfig) -> Self {
        self.config.network = network;
        self
    }

    /// Set security configuration
    pub fn with_security(mut self, security: SecurityConfig) -> Self {
        self.config.security = security;
        self
    }

    /// Set storage configuration
    pub fn with_storage(mut self, storage: StorageConfig) -> Self {
        self.config.storage = storage;
        self
    }

    /// Set API configuration
    pub fn with_api(mut self, api: ApiConfig) -> Self {
        self.config.api = api;
        self
    }

    /// Set ZFS configuration
    pub fn with_zfs(mut self, zfs: ZfsConfig) -> Self {
        self.config.zfs = zfs;
        self
    }

    /// Set performance configuration
    pub fn with_performance(mut self, performance: PerformanceConfig) -> Self {
        self.config.performance = performance;
        self
    }

    /// Set monitoring configuration
    pub fn with_monitoring(mut self, monitoring: MonitoringConfig) -> Self {
        self.config.monitoring = monitoring;
        self
    }

    /// Load configuration from environment variables
    pub fn from_env(mut self) -> Result<Self, String> {
        // Load environment-specific overrides
        if let Ok(log_level) = std::env::var("NESTGATE_LOG_LEVEL") {
            self.config.system.log_level = log_level;
        }

        if let Ok(port) = std::env::var("NESTGATE_PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                self.config.network.http_server.port = port_num;
            }
        }

        if let Ok(bind_address) = std::env::var("NESTGATE_BIND_ADDRESS") {
            self.config.network.http_server.bind_address = bind_address;
        }

        Ok(self)
    }

    /// Load configuration from file (simplified for now - TOML support can be added later)
    pub fn from_file<P: AsRef<std::path::Path>>(self, path: P) -> Result<Self, String> {
        let _content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {e}"))?;
        
        // For now, just return self - TOML parsing can be added when needed
        // let file_config: NestGateCanonicalConfig = toml::from_str(&content)
        //     .map_err(|e| format!("Failed to parse config file: {}", e))?;
        // self.config = file_config;
        
        Ok(self)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        // Basic validation
        if self.config.system.service_name.is_empty() {
            return Err("Service name cannot be empty".to_string());
        }

        if self.config.network.http_server.port == 0 {
            return Err("HTTP server port cannot be 0".to_string());
        }

        if self.config.network.http_server.max_connections == 0 {
            return Err("Max connections cannot be 0".to_string());
        }

        Ok(())
    }

    /// Build the final configuration
    pub fn build(self) -> Result<NestGateCanonicalConfig, String> {
        self.validate()?;
        Ok(self.config)
    }
} 