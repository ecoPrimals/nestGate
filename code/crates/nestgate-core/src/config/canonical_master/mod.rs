// **CANONICAL MASTER CONFIGURATION SYSTEM**
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
// ==================== SECTION ====================

// System-level configuration types
pub mod system_config;
// Network and connectivity configuration types
pub mod network_config;
// Storage and ZFS configuration types
pub mod storage_config;
// Security and authentication configuration types
pub mod security_config;
// API and handler configuration types
pub mod api_config;
// Monitoring and metrics configuration types
pub mod monitoring;
// Performance and optimization configuration types
pub mod performance_config;
// Handler-specific configuration types (NEW - consolidates scattered handler configs)
pub mod handler_config;
// Test and validation configuration types (NEW - consolidates scattered test configs)
pub mod test_config;
// Supporting types and enums
pub mod supporting_types;
// Configuration builders and factories
pub mod builders;
// **NEW**: Domain-specific canonical configurations
pub mod domains;
// **PHASE 2C**: Configuration migration framework
pub mod migration_framework;
// **PHASE 2C**: Supporting types for configuration enhancements
pub mod phase2c_types;
// ==================== SECTION ====================

pub use api_config::*;
pub use builders::*;
pub use handler_config::*;
pub use network_config::*;
pub use performance_config::*;
pub use security_config::*;
pub use storage_config::*;
pub use supporting_types::*;
pub use system_config::*;
pub use test_config::*;

// **NEW**: Re-export consolidated domain configurations
pub use domains::{
    CanonicalHandlerConfigs, CanonicalNetworkConfig, CanonicalPerformanceConfig,
    CanonicalSecurityConfig, CanonicalStorageConfig, CanonicalTestConfigs,
    ConsolidatedDomainConfigs, ConsolidatedIntegrationConfigs, DomainConfigValidation,
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
// This replaces ALL other configuration structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateCanonicalConfig<
    const MAX_CONNECTIONS: usize = 1000,
    const BUFFER_SIZE: usize = 65536,
    const TIMEOUT_MS: u64 = 30000,
    const API_PORT: u16 = 8080,
> {
    /// System-level configuration
    pub system: SystemConfig<MAX_CONNECTIONS, BUFFER_SIZE>,

    /// Network and connectivity configuration
    pub network: NetworkConfig<API_PORT, TIMEOUT_MS>,

    /// Storage and ZFS configuration
    pub storage: StorageConfig,

    /// Security and authentication configuration
    pub security: SecurityConfig,

    /// API and handler configuration
    pub api: ApiConfig,

    /// Handler-specific configurations (NEW - consolidates 50+ scattered handler configs)
    pub handlers: CanonicalHandlerConfigs,

    /// Test and validation configurations (NEW - consolidates 40+ scattered test configs)
    pub testing: CanonicalTestConfigs,

    /// Monitoring and observability configuration
    pub monitoring: MonitoringConfig,

    /// Performance and optimization configuration
    pub performance: PerformanceConfig<MAX_CONNECTIONS, BUFFER_SIZE>,

    /// MCP (Model Context Protocol) configuration
    pub mcp: McpConfig,

    /// Automation configuration
    pub automation: AutomationConfig,

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
        const API_PORT: u16,
    > NestGateCanonicalConfig<MAX_CONNECTIONS, BUFFER_SIZE, TIMEOUT_MS, API_PORT>
{
    /// **PHASE 2C**: Create configuration from environment variables
    pub fn from_environment() -> crate::error::Result<Self> {
        let mut config = Self::default();

        // Load environment-specific overrides
        if let Ok(env_str) = std::env::var("NESTGATE_ENVIRONMENT") {
            config.environment = match env_str.as_str() {
                "production" => Environment::Production,
                "staging" => Environment::Staging,
                "development" => Environment::Development,
                _ => Environment::Development,
            };
        }

        // Load API port from environment
        if let Ok(port_str) = std::env::var("NESTGATE_API_PORT") {
            if let Ok(_port) = port_str.parse::<u16>() {
                // Note: In a real implementation, we'd update the network config
                // For now, this is a placeholder showing the pattern
            }
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

        // Apply domain-specific overrides
        if let Some(domain_overrides) = overrides.domain_overrides {
            // Apply domain configuration overrides
            for (domain, configvalue) in domain_overrides {
                // In a real implementation, we'd apply these overrides
                // For now, this is a placeholder
                let _ = (domain, configvalue);
            }
        }

        self
    }

    /// **PHASE 2C**: Validate configuration
    pub fn validate(&self) -> crate::error::Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Validate environment-specific requirements
        match self.environment {
            Environment::Production => {
                // Stricter validation for production
                if self.network.port == 8080 {
                    warnings.push("Port 8080 not recommended for production".to_string());
                }

                if !self.network.tls_enabled {
                    warnings.push("TLS should be enabled in production".to_string());
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
    pub const fn validate_for_environment(&self, env: Environment) -> crate::error::Result<()> {
        if env == Environment::Production && self.network.port == 8080 {
            return Err(crate::error::NestGateError::configuration_error_detailed(
                "network.port".to_string(),
                "Port 8080 not allowed in production".to_string(),
                Some("8080".to_string()),
                Some("443 or custom secure port".to_string()),
                true,
            ));
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
        const API_PORT: u16,
    > Default for NestGateCanonicalConfig<MAX_CONNECTIONS, BUFFER_SIZE, TIMEOUT_MS, API_PORT>
{
    fn default() -> Self {
        Self {
            system: SystemConfig::default(),
            network: NetworkConfig::default(),
            storage: StorageConfig::default(),
            security: SecurityConfig::default(),
            api: ApiConfig::default(),
            handlers: CanonicalHandlerConfigs::default(),
            testing: CanonicalTestConfigs::default(),
            monitoring: MonitoringConfig::default(),
            performance: PerformanceConfig::default(),
            mcp: McpConfig::default(),
            automation: AutomationConfig::default(),
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

// Standard configuration with default const generics
pub type StandardConfig = NestGateCanonicalConfig;
// High-performance configuration with optimized const generics
pub type HighPerformanceConfig = NestGateCanonicalConfig<2000, 131_072, 15_000, 8080>;
// Development configuration with relaxed limits
pub type DevelopmentConfig = NestGateCanonicalConfig<100, 8192, 60_000, 3000>;
// Production configuration with production-optimized settings
pub type ProductionConfig = NestGateCanonicalConfig<5000, 262_144, 10000, 443>;
