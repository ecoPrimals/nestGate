//
// This module provides integration between ZFS storage management and the
// NestGate automation system. It offers intelligent dataset lifecycle management,
// automated tier optimization, and policy-driven automation.
//
// The automation system includes:
// - Intelligent tier evaluation and recommendation
// - Automated dataset lifecycle management
// - Policy-driven automation with customizable rules
// - Integration with the NestGate ecosystem
// - Performance optimization and migration coordination

pub mod actions;
pub mod engine;
pub mod integration;
pub mod lifecycle;
pub mod policies;
pub mod tests;
pub mod tier_evaluation;
pub mod types;

// Import canonical automation types from modernized package
/// **CANONICAL**: Use ZFS-specific Result type for automation
pub use crate::error::ZfsResult as Result;
// Removed unresolved automation imports - using local implementations
// Import canonical types from the types module
// Removed unresolved automation types - using local definitions

// Import core types
pub use nestgate_core::canonical_types::StorageTier;
pub use nestgate_core::traits::native_async::ServiceHealth;

// Remove references to deleted discovery module
// pub use nestgate_automation::discovery::EcosystemDiscovery;

// Re-export main engine
pub use engine::DatasetAutomation;

// Re-export integration functions with correct names
pub use integration::{
    check_zfs_ecosystem_availability, initialize_automation as initialize_zfs_automation,
    initialize_automation_with_config as initialize_zfs_automation_with_config,
};

// Re-export policy management
pub use policies::{
    AccessPatternRules, LifecycleRules, MigrationPerformanceLimits, MigrationRules,
    MigrationSchedule, PerformanceRequirement, PerformanceThresholds, TierAssignmentRules,
    TierSizeThresholds,
};

// Re-export configuration types
pub use crate::config::{AiAutomationSettings, DatasetAutomationConfig};
