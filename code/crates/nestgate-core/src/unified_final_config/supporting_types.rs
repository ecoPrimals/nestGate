//
// Provides supporting types for the unified configuration system.

// Re-export core types for backward compatibility
pub use super::core::{ConfigSource, DeploymentEnvironment, ValidationStatus};

/// Standard domain config pattern
/// This type alias maintains compatibility with existing code while using the new unified system
pub type StandardDomainConfig = super::core::NestGateFinalConfig;
