//
// This module provides canonical type aliases for complex types used throughout
// the NestGate ecosystem. This enables zero-copy operations and consistent typing.

pub mod config_registry;
pub mod core_types;

// Re-export all canonical types
// Specific re-exports to avoid ambiguous glob conflicts
#[allow(deprecated)] // CanonicalNetworkConfig moved to canonical_primary::domains::network
pub use config_registry::CanonicalNetworkConfig;
pub use config_registry::{
    CanonicalMonitoringConfig, CanonicalSecurityConfig, CanonicalStorageConfig,
};
pub use core_types::{
    Alert, AlertChannel, AlertRule, CapabilityIndexMap, HealthMonitorRegistry, ProviderRegistry,
    ServiceRegistry, TimestampChangesMap,
};
