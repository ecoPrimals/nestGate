//
// This module provides canonical type aliases for complex types used throughout
// the NestGate ecosystem. This enables zero-copy operations and consistent typing.

pub mod config_registry;
pub mod core_types;

// Re-export all canonical types
// Specific re-exports to avoid ambiguous glob conflicts
pub use config_registry::{
    CanonicalStorageConfig, CanonicalNetworkConfig, CanonicalSecurityConfig,
    CanonicalMonitoringConfig
};
pub use core_types::{
    ProviderRegistry, CapabilityIndexMap, HealthMonitorRegistry,
    ServiceRegistry, AlertChannel, AlertRule, Alert, TimestampChangesMap
}; 