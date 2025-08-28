//! **CONFIGURATION BUILDERS AND MIGRATION UTILITIES**
//!
//! This module provides builders and migration utilities for constructing
//! and transitioning configuration objects.

use std::path::PathBuf;

use crate::Result;
use super::core::{NestGateCanonicalConfig, LogLevel};

// ==================== SECTION ====================

/// Configuration builder for constructing NestGateCanonicalConfig
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

    /// Set service name
    pub fn service_name(mut self, name: impl Into<String>) -> Self {
        self.config.system.service_name = name.into();
        self
    }

    /// Set environment
    pub fn environment(mut self, env: impl Into<String>) -> Self {
        self.config.system.environment = env.into();
        self
    }

    /// Set log level
    pub fn log_level(mut self, level: LogLevel) -> Self {
        self.config.system.log_level = level;
        self
    }

    /// Enable debug mode
    pub fn debug_mode(mut self, enabled: bool) -> Self {
        self.config.system.debug_mode = enabled;
        self
    }

    /// Set data directory
    pub fn data_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.config.system.data_dir = path.into();
        self
    }

    /// Set API bind address
    pub fn api_bind_address(mut self, address: impl Into<String>) -> Self {
        self.config.api.bind_address = address.into();
        self
    }

    /// Set API port override
    pub fn api_port(mut self, port: u16) -> Self {
        self.config.api.port_override = Some(port);
        self
    }

    /// Set server bind address
    pub fn server_bind_address(mut self, address: impl Into<String>) -> Self {
        self.config.server.bind_address = address.into();
        self
    }

    /// Enable TLS
    pub fn enable_tls(mut self, cert_path: impl Into<PathBuf>, key_path: impl Into<PathBuf>) -> Self {
        self.config.server.tls_enabled = true;
        self.config.server.cert_path = Some(cert_path.into());
        self.config.server.key_path = Some(key_path.into());
        self
    }

    /// Add feature flag
    pub fn feature_flag(mut self, key: impl Into<String>, value: bool) -> Self {
        self.config.features.insert(key.into(), value);
        self
    }

    /// Add environment override
    pub fn environment_override(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.environment_overrides.insert(key.into(), value.into());
        self
    }

    /// Build the configuration
    pub fn build(self) -> Result<NestGateCanonicalConfig> {
        let config = self.config;
        config.validate()?;
        Ok(config)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== SECTION ====================

/// Create production configuration
pub fn create_production_config() -> Result<NestGateCanonicalConfig> {
    ConfigBuilder::new()
        .environment("production")
        .log_level(LogLevel::Warn)
        .debug_mode(false)
        .build()
}

/// Create development configuration
pub fn create_development_config() -> Result<NestGateCanonicalConfig> {
    ConfigBuilder::new()
        .environment("development")
        .log_level(LogLevel::Debug)
        .debug_mode(true)
        .build()
}

/// Create testing configuration
pub fn create_testing_config() -> Result<NestGateCanonicalConfig> {
    ConfigBuilder::new()
        .environment("testing")
        .log_level(LogLevel::Info)
        .debug_mode(false)
        .data_dir("./test_data")
        .build()
}

// ==================== SECTION ====================

// ==================== SECTION ====================
// Migration utilities have been removed as migration is complete.
// All configurations now use canonical_master::NestGateCanonicalConfig directly.

        // Migrate common environment variables
        if let Ok(service_name) = std::env::var("NESTGATE_SERVICE_NAME") {
            builder = builder.service_name(service_name);
        }

        if let Ok(env) = std::env::var("NESTGATE_ENVIRONMENT") {
            builder = builder.environment(env);
        }

        if let Ok(log_level) = std::env::var("NESTGATE_LOG_LEVEL") {
            let level = match log_level.to_lowercase().as_str() {
                "error" => LogLevel::Error,
                "warn" => LogLevel::Warn,
                "info" => LogLevel::Info,
                "debug" => LogLevel::Debug,
                "trace" => LogLevel::Trace,
                _ => LogLevel::Info,
            };
            builder = builder.log_level(level);
        }

        if let Ok(debug) = std::env::var("NESTGATE_DEBUG") {
            let debug_mode = debug.to_lowercase() == "true" || debug == "1";
            builder = builder.debug_mode(debug_mode);
        }

        if let Ok(data_dir) = std::env::var("NESTGATE_DATA_DIR") {
            builder = builder.data_dir(data_dir);
        }

        if let Ok(api_host) = std::env::var("NESTGATE_API_HOST") {
            builder = builder.api_bind_address(api_host);
        }

        if let Ok(api_port) = std::env::var("NESTGATE_API_PORT") {
            if let Ok(port) = api_port.parse::<u16>() {
                builder = builder.api_port(port);
            }
        }

        self.migration_stats.configs_migrated += 1;
        builder.build()
    }

    /// Generate migration report
    pub fn generate_report(&self) -> String {
        format!(
            "Migration Report:\n\
             - Configurations migrated: {}\n\
             - Warnings generated: {}\n\
             - Deprecated fields removed: {}\n\
             - Warnings: {:?}",
            self.migration_stats.configs_migrated,
            self.migration_stats.warnings_generated,
            self.migration_stats.deprecated_fields_removed,
            self.warnings
        )
    }
}

impl Default for ConfigMigrationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== SECTION ====================

/// Configuration validation utilities
pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate configuration for production deployment
    pub fn validate_production(config: &NestGateCanonicalConfig) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Check production-specific requirements
        if config.system.debug_mode {
            warnings.push("Debug mode is enabled in production configuration".to_string());
        }

        if config.system.log_level == LogLevel::Debug || config.system.log_level == LogLevel::Trace {
            warnings.push("Verbose logging enabled in production configuration".to_string());
        }

        if !config.server.tls_enabled {
            warnings.push("TLS is disabled in production configuration".to_string());
        }

        if config.security.authentication.enabled == false {
            warnings.push("Authentication is disabled in production configuration".to_string());
        }

        Ok(warnings)
    }

    /// Validate configuration for security compliance
    pub fn validate_security(config: &NestGateCanonicalConfig) -> Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Check security-specific requirements
        if !config.security.encryption.at_rest_enabled {
            warnings.push("Encryption at rest is disabled".to_string());
        }

        if !config.security.encryption.in_transit_enabled {
            warnings.push("Encryption in transit is disabled".to_string());
        }

        if config.security.authentication.token_expiration_seconds > 3600 {
            warnings.push("Authentication token expiration is longer than 1 hour".to_string());
        }

        if !config.security.session.secure_cookies {
            warnings.push("Secure cookies are disabled".to_string());
        }

        Ok(warnings)
    }
}

// ==================== SECTION ====================

/// **BACKWARD COMPATIBILITY**: Standard configuration without const generics
/// This maintains compatibility with existing code while providing upgrade path
pub type StandardNestGateConfig = NestGateCanonicalConfig<1000, 65536, 30000, 8080>;

/// Create standard configuration for backward compatibility
pub fn create_standard_config() -> StandardNestGateConfig {
    StandardNestGateConfig::default()
}

/// Create configuration for specific environment
pub fn create_config_for_environment(env: &str) -> Result<NestGateCanonicalConfig> {
    match env.to_lowercase().as_str() {
        "production" | "prod" => create_production_config(),
        "development" | "dev" => create_development_config(),
        "testing" | "test" => create_testing_config(),
        _ => {
            ConfigBuilder::new()
                .environment(env)
                .build()
        }
    }
} 