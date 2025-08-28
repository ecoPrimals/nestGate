//! **NESTGATE CONFIGURATION SYSTEM**
//!
//! Unified configuration management for all NestGate components.
//! This module provides the canonical configuration architecture.

// PEDANTIC: Removed unused imports
// use crate::{NestGateError, Result}; // REMOVED - unused imports

// ==================== SECTION ====================

/// **CANONICAL MASTER**: The definitive configuration system
pub mod canonical_master;

/// **MIGRATION TRAITS**: Systematic config consolidation utilities
pub mod migration_traits;

// ==================== SECTION ====================
// All deprecated configuration modules have been removed. Use canonical_master directly:
// - core → canonical_master::NestGateCanonicalConfig
// - domains → canonical_master::NestGateCanonicalConfig
// - builders → canonical_master builders
// - unified_types → canonical_master detailed_configs
// - canonical → canonical_master::NestGateCanonicalConfig
// - network → canonical_master::NestGateCanonicalConfig
// - federation → canonical_master::NestGateCanonicalConfig
// - monitoring → canonical_master::NestGateCanonicalConfig
// - canonical_config → canonical_master::NestGateCanonicalConfig
// - canonical_unified → canonical_master::NestGateCanonicalConfig

// ==================== SECTION ====================

/// **THE** canonical configuration for all NestGate systems
pub use canonical_master::{
    NestGateCanonicalConfig,
    SystemConfig,
    NetworkConfig,
    StorageConfig,
    SecurityConfig,
    ApiConfig,
    PerformanceConfig,
    DeploymentEnvironment,
    LogLevel,
    FeatureFlags,
    ConfigMetadata,
};

/// Migration utilities for config consolidation
pub use migration_traits::{
    IntoCanonicalNetworkConfig, 
    IntoCanonicalStorageConfig, 
    IntoCanonicalSecurityConfig,
    ConfigMigrationHelper,
    MigrationStats
};

// Note: Detailed configuration types are defined inline in canonical_master
// and will be accessible through the canonical_master module directly

// ==================== SECTION ====================

// ==================== SECTION ====================
// All deprecated configuration type aliases have been removed. Use canonical_master directly:
// - UnifiedConfig → canonical_master::NestGateCanonicalConfig
// - StandardConfig → canonical_master::NestGateCanonicalConfig
// - MasterConfig → canonical_master::NestGateCanonicalConfig
// - NestGateCanonicalConfig → canonical_master::NestGateCanonicalConfig

// ==================== SECTION ====================

/// Create a canonical configuration with default settings
pub fn create_default_config() -> canonical_master::NestGateCanonicalConfig {
    canonical_master::NestGateCanonicalConfig::default()
}

/// Create a production-ready canonical configuration
pub fn create_production_config() -> canonical_master::NestGateCanonicalConfig {
    let mut config = canonical_master::NestGateCanonicalConfig::default();
    config.system.environment = DeploymentEnvironment::Production;
    config.system.log_level = LogLevel::Warn;
    config.system.debug_mode = false;
    config.features.enable_auto_scaling = true;
    config.features.enable_load_balancing = true;
    config
}

/// Create a development configuration
pub fn create_development_config() -> canonical_master::NestGateCanonicalConfig {
    let mut config = canonical_master::NestGateCanonicalConfig::default();
    config.system.environment = DeploymentEnvironment::Development;
    config.system.log_level = LogLevel::Debug;
    config.system.debug_mode = true;
    config
}

/// Create a testing configuration
pub fn create_testing_config() -> canonical_master::NestGateCanonicalConfig {
    let mut config = canonical_master::NestGateCanonicalConfig::default();
    config.system.environment = DeploymentEnvironment::Testing;
    config.system.log_level = LogLevel::Debug;
    config.features.enable_metrics = false;
    config.features.enable_tracing = false;
    config
}

// ==================== SECTION ====================
// All configurations now use canonical_master::NestGateCanonicalConfig directly.
// Default implementations are in the canonical_master module.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_config_creation() {
        let config = create_default_config();
        assert_eq!(config.system.instance_name, "nestgate-default");
        assert!(matches!(config.system.environment, DeploymentEnvironment::Development));
    }

    #[test]
    fn test_production_config() {
        let config = create_production_config();
        assert!(matches!(config.system.environment, DeploymentEnvironment::Production));
        assert_eq!(config.system.log_level, LogLevel::Warn);
        assert!(!config.system.debug_mode);
    }

    #[test]
    fn test_development_config() {
        let config = create_development_config();
        assert!(matches!(config.system.environment, DeploymentEnvironment::Development));
        assert_eq!(config.system.log_level, LogLevel::Debug);
        assert!(config.system.debug_mode);
    }

    #[test]
    fn test_migration_validation() {
        let config = create_default_config();
        assert!(migration::validate_migration(&config).is_ok());
    }
}
