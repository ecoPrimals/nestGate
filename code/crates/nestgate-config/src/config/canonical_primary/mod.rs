// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **CANONICAL PRIMARY CONFIGURATION SYSTEM**
//! Module definitions and exports.
//! This is THE single source of truth for ALL NestGate configuration,
//! replacing and consolidating 200+ scattered configuration structures
//! across all 11 crates.
//! Module definitions and exports.
//! **MODULAR ARCHITECTURE**:
//! - `system_config`: System-level configuration
//! - `network_config`: Network and connectivity configuration  
//! - `storage_config`: Storage and ZFS configuration
//! - `security_config`: Security and authentication configuration
//! - `api_config`: API and handler configuration
//! - `performance_config`: Performance and optimization configuration
//! - `handler_config`: Handler-specific configurations (NEW)
//! - `test_config`: Test and validation configurations (NEW)
//! - `supporting_types`: Common types and enums
//! - `builders`: Configuration builders and factories
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::time::Duration;
// ==================== SECTION ====================

/// System-level configuration types (environment, logging, debug settings)
pub mod system_config;
// REMOVED: network_config → Use domains/network instead
// Service configuration types (NEW - consolidates 5 duplicate UnifiedServiceConfig structs)
pub mod service;
// Memory configuration types (NEW - consolidates 2 duplicate UnifiedMemoryConfig structs)
pub mod memory;
// Retry configuration types (NEW - consolidates retry patterns)
pub mod retry;
// Timeout configuration types (NEW - consolidates 150+ duplicate timeout fields)
pub mod timeout;
// Connection Pool configuration types (NEW - consolidates connection pooling settings)
pub mod connection_pool;

/// Storage and ZFS configuration types
pub mod storage_config;

/// Security and authentication configuration types
pub mod security_config;

/// Performance and optimization configuration types
pub mod performance_config;
// Handler-specific configuration types (NEW - consolidates scattered handler configs)
pub mod handler_config;
// Test and validation configuration types (NEW - consolidates scattered test configs) (dev-stubs only)
#[cfg(feature = "dev-stubs")]
pub mod test_config;

/// Supporting types and enums (common configuration types)
pub mod supporting_types;

/// Configuration builders and factories for constructing configs
pub mod builders;
// **NEW**: Domain-specific canonical configurations
pub mod domains;
// **PHASE 2C**: Configuration migration framework
pub mod migration_framework;
// **PHASE 2C**: Supporting types for configuration enhancements
pub mod phase2c_types;
// ==================== SECTION ====================

pub use builders::*;
pub use handler_config::*;
// REMOVED: network_config::* → Use domains::CanonicalNetworkConfig instead
pub use performance_config::*;
pub use security_config::*;
pub use storage_config::*;
pub use supporting_types::*;
pub use system_config::*;
#[cfg(feature = "dev-stubs")]
pub use test_config::*;

// Service exports (NEW - consolidates 5 duplicate UnifiedServiceConfig structs)
pub use service::{ServiceConfig, UnifiedServiceConfig};
// Memory exports (NEW - consolidates 2 duplicate UnifiedMemoryConfig structs)
pub use memory::{MemoryConfig, UnifiedMemoryConfig};
// Retry exports (NEW - consolidates retry patterns)
pub use retry::{RetryConfig, UnifiedRetryConfig};
// Timeout exports (NEW - consolidates 150+ duplicate timeout fields)
pub use timeout::{TimeoutConfig, UnifiedTimeoutConfig};
// Connection Pool exports (NEW - consolidates connection pooling settings)
pub use connection_pool::{ConnectionPoolConfig, UnifiedConnectionPoolConfig};

// **NEW**: Re-export consolidated domain configurations
#[cfg(feature = "dev-stubs")]
pub use domains::CanonicalTestConfigs;
pub use domains::{
    CanonicalHandlerConfigs, CanonicalNetworkConfig, CanonicalPerformanceConfig,
    CanonicalSecurityConfig, CanonicalStorageConfig, ConsolidatedDomainConfigs,
    ConsolidatedIntegrationConfigs, DomainConfigValidation,
};
// **PHASE 2C**: Migration framework exports
pub use migration_framework::{
    ConfigMigrator, ErrorSeverity, MigrationBackup, MigrationError, MigrationOptions,
    MigrationPhase, MigrationProgress, MigrationReport, SafeConfigMigration, ValidationRule,
};
// **PHASE 2C**: Supporting types exports
pub use phase2c_types::{
    ConfigMetadata, ConfigOverrides, ConfigSource, Environment, FeatureFlags, NetworkOverrides,
    OptimizationLevel, PerformanceOverrides, SecurityOverrides, ValidationWarning, WarningSeverity,
};
// ==================== SECTION ====================

// **THE** canonical configuration for the entire NestGate ecosystem
/// The canonical configuration structure for all `NestGate` systems
///
/// This replaces ALL other configuration structures with a single,
/// unified configuration that uses const generics for compile-time optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `NestGateCanonical`
pub struct NestGateCanonicalConfig<
    const MAX_CONNECTIONS: usize = 1000,
    const BUFFER_SIZE: usize = 65536,
    const TIMEOUT_MS: u64 = 30000,
    // Api Port (const generic parameter)
    const API_PORT: u16 = 8080,
> {
    /// System-level configuration
    pub system: SystemConfig<MAX_CONNECTIONS, BUFFER_SIZE>,

    /// Network and connectivity configuration
    pub network: CanonicalNetworkConfig,

    /// Storage and ZFS configuration
    pub storage: StorageConfig,

    /// Security and authentication configuration
    pub security: SecurityConfig,

    /// API and handler configuration (canonical from `domains::network`)
    pub api: domains::network::ApiConfig,

    /// Handler-specific configurations (NEW - consolidates 50+ scattered handler configs)
    pub handlers: CanonicalHandlerConfigs,

    /// Test and validation configurations (NEW - consolidates 40+ scattered test configs)
    /// **⚠️ DEV/TEST ONLY**: Only available with `dev-stubs` feature
    #[cfg(feature = "dev-stubs")]
    /// Testing
    pub testing: CanonicalTestConfigs,

    /// Monitoring and observability configuration
    pub monitoring: MonitoringConfig,

    /// Performance and optimization configuration
    pub performance: PerformanceConfig<MAX_CONNECTIONS, BUFFER_SIZE>,

    /// MCP (Model Context Protocol) configuration
    pub mcp: McpConfig,

    /// Automation configuration (canonical from `domains::automation`)
    pub automation: domains::automation::AutomationConfig,

    /// File system monitor configuration
    pub fsmonitor: FsMonitorConfig,

    /// NAS configuration
    pub nas: NasConfig,

    /// Middleware configuration
    pub middleware: MiddlewareConfig,

    /// **PHASE 2C ENHANCEMENT**: Consolidated domain configurations
    pub domains: domains::ConsolidatedDomainConfigs,

    /// **PHASE 2C ENHANCEMENT**: Consolidated integration configurations
    pub integrations: domains::ConsolidatedIntegrationConfigs,

    /// Environment-specific settings
    pub environment: Environment,

    /// Feature flags
    pub features: FeatureFlags,

    /// Configuration metadata
    pub metadata: ConfigMetadata,
}
// ==================== SECTION ====================

// ==================== IMPLEMENTATION ====================

impl<
    const MAX_CONNECTIONS: usize,
    const BUFFER_SIZE: usize,
    const TIMEOUT_MS: u64,
    // Api Port (const generic parameter)
    const API_PORT: u16,
> NestGateCanonicalConfig<MAX_CONNECTIONS, BUFFER_SIZE, TIMEOUT_MS, API_PORT>
{
    /// **PHASE 2C**: Create configuration from environment variables
    pub fn from_environment() -> nestgate_types::error::Result<Self> {
        let mut config = Self::default();

        // Load environment-specific overrides
        if let Ok(env_str) = std::env::var("NESTGATE_ENVIRONMENT") {
            config.environment = match env_str.as_str() {
                "production" => Environment::Production,
                "staging" => Environment::Staging,
                _ => Environment::Development,
            };
        }

        if let Ok(port_str) = std::env::var("NESTGATE_API_PORT")
            && let Ok(port) = port_str.parse::<u16>()
        {
            config.network.api.port = port;
            config.api.port = port;
            config.domains.api.server.port = port;
        }

        Ok(config)
    }

    /// **PHASE 2C**: Apply configuration overrides
    #[must_use]
    pub fn with_overrides(mut self, overrides: ConfigOverrides) -> Self {
        // Apply overrides to the configuration
        if let Some(env) = overrides.environment {
            self.environment = env;
        }

        if let Some(domain_overrides) = overrides.domain_overrides {
            self.domains.merge_domain_json_overrides(domain_overrides);
        }

        if let Some(net) = overrides.network_overrides {
            if let Some(port) = net.api_port {
                self.network.api.port = port;
                self.api.port = port;
                self.domains.api.server.port = port;
            }
            if let Some(ref addr) = net.bind_address {
                if let Ok(ip) = addr.parse::<IpAddr>() {
                    self.network.api.bind_address = ip;
                }
                self.domains.api.server.bind_address.clone_from(addr);
            }
            if let Some(ms) = net.timeout_ms {
                let d = Duration::from_millis(ms);
                self.network.api.request_timeout = d;
                self.network.api.connection_timeout = d;
                self.domains.api.server.request_timeout = d;
            }
            if let Some(workers) = net.workers {
                self.domains.api.server.workers = Some(workers);
            }
        }

        if let Some(sec) = overrides.security_overrides {
            if let Some(v) = sec.tls_enabled {
                self.network
                    .api
                    .api_settings
                    .insert("tls_enabled".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sec.require_auth {
                self.security.auth.enabled = v;
                self.network.api.security.auth_enabled = v;
            }
            if let Some(v) = sec.dev_mode_bypass {
                self.system.debug_mode = v;
            }
            if let Some(ref path) = sec.cert_path {
                self.network.api.tls.cert_path.clone_from(path);
            }
        }

        if let Some(perf) = overrides.performance_overrides {
            if let Some(n) = perf.max_connections {
                self.system.max_connections_override = Some(n);
                self.performance.max_connections = n;
            }
            if let Some(n) = perf.buffer_size {
                self.system.buffer_size_override = Some(n);
                self.performance.buffer_size = n;
                self.network.api.performance.buffer_size = n;
            }
            if let Some(n) = perf.cache_size {
                self.network.api.performance.cache_size = n as u64;
            }
            if let Some(level) = perf.optimization_level
                && let Ok(v) = serde_json::to_value(level)
            {
                self.performance
                    .performance_settings
                    .insert("optimization_level".to_string(), v);
            }
        }

        self
    }

    /// **PHASE 2C**: Validate configuration
    pub fn validate(&self) -> nestgate_types::error::Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Validate environment-specific requirements
        match self.environment {
            Environment::Production => {
                // Stricter validation for production
                {
                    use crate::constants::hardcoding::runtime_fallback_ports;
                    if self.network.api.port == runtime_fallback_ports::HTTP {
                        warnings.push(format!(
                            "Port {} not recommended for production",
                            runtime_fallback_ports::HTTP
                        ));
                    }

                    // TLS validation moved to API config
                    if !self.network.security.firewall_enabled {
                        warnings.push("Firewall should be enabled in production".to_string());
                    }
                }
            }
            Environment::Development => {
                // More lenient validation for development
                if self.security.auth.enabled {
                    warnings.push("Authentication enabled in development mode".to_string());
                }
            }
            _ => {}
        }

        // Validate domain configurations
        warnings.extend(self.domains.validate()?);

        Ok(warnings)
    }

    /// **PHASE 2C**: Validate for specific environment
    ///
    /// # Errors
    ///
    /// Returns [`nestgate_types::error::NestGateError`] when the configuration is incompatible
    /// with the given [`Environment`], such as insecure defaults in production.
    pub fn validate_for_environment(&self, env: Environment) -> nestgate_types::error::Result<()> {
        if env == Environment::Production && self.network.api.port == 8080 {
            return Err(
                nestgate_types::error::NestGateError::configuration_error_detailed(
                    "network.port".to_string(),
                    "Port 8080 not allowed in production".to_string(),
                    Some("8080".into()),
                    Some("443 or custom secure port".into()),
                    true,
                ),
            );
        }

        self.domains.validate_for_environment(match env {
            Environment::Production => "production",
            Environment::Staging => "staging",
            Environment::Development => "development",
            Environment::Testing => "testing",
        })?;

        Ok(())
    }
}

impl<
    const MAX_CONNECTIONS: usize,
    const BUFFER_SIZE: usize,
    const TIMEOUT_MS: u64,
    // Api Port (const generic parameter)
    const API_PORT: u16,
> Default for NestGateCanonicalConfig<MAX_CONNECTIONS, BUFFER_SIZE, TIMEOUT_MS, API_PORT>
{
    fn default() -> Self {
        Self {
            system: SystemConfig::default(),
            network: CanonicalNetworkConfig::default(),
            storage: StorageConfig::default(),
            security: SecurityConfig::default(),
            api: domains::network::ApiConfig::default(),
            handlers: CanonicalHandlerConfigs::default(),
            #[cfg(feature = "dev-stubs")]
            testing: CanonicalTestConfigs::default(),
            monitoring: MonitoringConfig::default(),
            performance: PerformanceConfig::default(),
            mcp: McpConfig::default(),
            automation: domains::automation::AutomationConfig::default(),
            fsmonitor: FsMonitorConfig::default(),
            nas: NasConfig::default(),
            middleware: MiddlewareConfig::default(),
            domains: domains::ConsolidatedDomainConfigs::default(),
            integrations: domains::ConsolidatedIntegrationConfigs::default(),
            environment: Environment::Development,
            features: FeatureFlags::default(),
            metadata: ConfigMetadata::default(),
        }
    }
}

// ==================== SECTION ====================

/// Standard configuration with default const generics
pub type StandardConfig = NestGateCanonicalConfig;

/// High-performance configuration with optimized const generics
pub type HighPerformanceConfig = NestGateCanonicalConfig<2000, 131_072, 15_000, 8080>;

/// Development configuration with relaxed limits for easier debugging
pub type DevelopmentConfig = NestGateCanonicalConfig<100, 8192, 60_000, 3000>;

/// Production configuration with production-optimized settings
pub type ProductionConfig = NestGateCanonicalConfig<5000, 262_144, 10000, 443>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::hardcoding::runtime_fallback_ports;
    use serial_test::serial;
    use temp_env::with_vars;

    type StdCanonical = NestGateCanonicalConfig<1000, 65536, 30000, 8080>;

    #[test]
    fn nest_gate_canonical_config_default() {
        let c = StdCanonical::default();
        assert_eq!(c.environment, Environment::Development);
        assert!(c.validate().is_ok());
    }

    #[test]
    fn nest_gate_canonical_config_serde_roundtrip() {
        let original = StdCanonical::default();
        let json = serde_json::to_string(&original).expect("serialize NestGateCanonicalConfig");
        let parsed: StdCanonical =
            serde_json::from_str(&json).expect("deserialize NestGateCanonicalConfig");
        assert_eq!(original.environment, parsed.environment);
        assert_eq!(original.network.api.port, parsed.network.api.port);
        assert_eq!(original.security.auth.enabled, parsed.security.auth.enabled);
    }

    #[test]
    fn nest_gate_canonical_config_with_overrides_port() {
        let c = StdCanonical::default();
        let updated = c.with_overrides(ConfigOverrides {
            environment: None,
            domain_overrides: None,
            network_overrides: Some(NetworkOverrides {
                api_port: Some(9100),
                bind_address: None,
                timeout_ms: None,
                workers: None,
            }),
            security_overrides: None,
            performance_overrides: None,
        });
        assert_eq!(updated.network.api.port, 9100);
        assert_eq!(updated.api.port, 9100);
        assert_eq!(updated.domains.api.server.port, 9100);
    }

    #[test]
    fn with_overrides_environment_and_network_bind_timeout_workers() {
        let c = StdCanonical::default();
        let updated = c.with_overrides(ConfigOverrides {
            environment: Some(Environment::Staging),
            domain_overrides: None,
            network_overrides: Some(NetworkOverrides {
                api_port: Some(7443),
                bind_address: Some("192.0.2.1".to_string()),
                timeout_ms: Some(12_000),
                workers: Some(8),
            }),
            security_overrides: None,
            performance_overrides: None,
        });
        assert_eq!(updated.environment, Environment::Staging);
        assert_eq!(updated.network.api.port, 7443);
        assert_eq!(updated.domains.api.server.bind_address, "192.0.2.1");
        assert_eq!(
            updated.network.api.request_timeout,
            std::time::Duration::from_millis(12_000)
        );
        assert_eq!(updated.domains.api.server.workers, Some(8));
    }

    #[test]
    fn with_overrides_security_and_performance() {
        let c = StdCanonical::default();
        let updated = c.with_overrides(ConfigOverrides {
            environment: None,
            domain_overrides: None,
            network_overrides: None,
            security_overrides: Some(SecurityOverrides {
                tls_enabled: Some(true),
                require_auth: Some(true),
                dev_mode_bypass: Some(true),
                cert_path: Some("/tmp/nestgate-test.pem".to_string()),
            }),
            performance_overrides: Some(PerformanceOverrides {
                max_connections: Some(2048),
                buffer_size: Some(4096),
                cache_size: Some(128),
                optimization_level: Some(OptimizationLevel::Performance),
            }),
        });
        assert!(updated.security.auth.enabled);
        assert!(updated.network.api.security.auth_enabled);
        assert!(updated.system.debug_mode);
        assert_eq!(updated.network.api.tls.cert_path, "/tmp/nestgate-test.pem");
        assert_eq!(updated.performance.max_connections, 2048);
        assert_eq!(updated.performance.buffer_size, 4096);
        assert_eq!(updated.network.api.performance.cache_size, 128);
        assert!(
            updated
                .performance
                .performance_settings
                .contains_key("optimization_level")
        );
    }

    #[test]
    fn validate_production_warns_on_default_http_port_and_firewall() {
        let mut c = StdCanonical::default();
        c.environment = Environment::Production;
        c.network.api.port = runtime_fallback_ports::HTTP;
        c.network.security.firewall_enabled = false;
        let warnings = c.validate().expect("validate");
        let joined = warnings.join(" ");
        assert!(joined.contains("8080") || joined.contains("Port"));
        assert!(joined.contains("Firewall") || joined.contains("firewall"));
    }

    #[test]
    fn validate_development_warns_when_auth_enabled() {
        let mut c = StdCanonical::default();
        c.environment = Environment::Development;
        c.security.auth.enabled = true;
        c.network.api.security.auth_enabled = true;
        let warnings = c.validate().expect("validate");
        assert!(
            warnings
                .iter()
                .any(|w| w.contains("Authentication") && w.contains("development"))
        );
    }

    #[test]
    fn validate_for_environment_rejects_port_8080_in_production() {
        let mut c = StdCanonical::default();
        c.network.api.port = 8080;
        let err = c
            .validate_for_environment(Environment::Production)
            .expect_err("production must reject 8080");
        let msg = err.to_string();
        assert!(msg.contains("8080") || msg.contains("Port"));
    }

    #[test]
    fn validate_for_environment_accepts_non_8080_in_production() {
        let mut c = StdCanonical::default();
        c.network.api.port = 8443;
        c.validate_for_environment(Environment::Production)
            .expect("8443 allowed in production");
    }

    #[test]
    #[serial]
    fn from_environment_sets_production_and_port() {
        let c = with_vars(
            vec![
                ("NESTGATE_ENVIRONMENT", Some("production")),
                ("NESTGATE_API_PORT", Some("9090")),
            ],
            || StdCanonical::from_environment().expect("from_environment"),
        );
        assert_eq!(c.environment, Environment::Production);
        assert_eq!(c.network.api.port, 9090);
        assert_eq!(c.api.port, 9090);
    }

    #[test]
    #[serial]
    fn from_environment_unknown_env_string_defaults_to_development() {
        let c = with_vars(
            vec![
                ("NESTGATE_ENVIRONMENT", Some("custom-lab")),
                ("NESTGATE_API_PORT", None::<&str>),
            ],
            || StdCanonical::from_environment().expect("from_environment"),
        );
        assert_eq!(c.environment, Environment::Development);
    }
}
