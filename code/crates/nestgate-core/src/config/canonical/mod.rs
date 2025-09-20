// Canonical Configuration System for NestGate
//! Module definitions and exports.
// This module provides a unified, hierarchical configuration system with proper separation of concerns:
//! - **Core types**: Configuration data structures
//! - **Validation**: Configuration validation logic
//! - **Loading**: File I/O and parsing operations
//! - **Defaults**: Default value providers
//! - **Merging**: Configuration composition logic
//! - **Domain configs**: Domain-specific canonical configurations
//! - **Builders**: Fluent configuration builders

pub mod builders;
pub mod defaults;
pub mod NestGateCanonicalConfig;
pub mod loader;
pub mod merger;
pub mod types;
pub mod validation;

// Re-export main types for backwards compatibility
pub use types::{
    CanonicalConfig, Environment, EnvironmentConfig, IntegrationsConfig, MonitoringConfig,
    NetworkConfig, PerformanceConfig, SecurityConfig, StorageConfig, SystemConfig,
};

// Re-export domain configurations
pub use NestGateCanonicalConfig::{
    CanonicalDomainConfig, CanonicalNetworkConfig, CanonicalPerformanceConfig,
    CanonicalSecurityConfig, CanonicalServiceConfig, CanonicalStorageConfig, CanonicalTestConfig,
};

// Re-export builders
pub use builders::{
    chaos_test_config, integration_test_config, performance_test_config, unit_test_config,
    CanonicalConfigBuilder, CanonicalTestConfigBuilder,
};

pub use defaults::CanonicalConfigBuilder as CanonicalConfigBuilderTrait;
pub use loader::ConfigLoader;
pub use merger::ConfigMerger;
pub use validation::ConfigValidator;
