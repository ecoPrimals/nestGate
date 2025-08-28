pub mod data_types;
pub mod message_event_types;
pub mod network_types;
/// Unified Enum System for NestGate
/// This module provides canonical enum definitions that replace duplicate
/// enums scattered across the codebase, ensuring type consistency and
/// eliminating maintenance overhead.
/// **PROBLEM SOLVED**: Eliminates 25+ duplicate enum definitions including
/// ServiceType, AlertType, DataType, MessageType, EventType, and more.
// Module declarations
pub mod service_types;
pub mod storage_access_types;
pub mod system_health_types;
// Note: health_status_migrations and service_status_migrations modules
// are not needed as the unified enum system provides all necessary types

// Re-export all unified enum types for backward compatibility
pub use data_types::*;
pub use message_event_types::*;
pub use network_types::*;
pub use service_types::*;
pub use storage_access_types::*;
pub use system_health_types::*;

// **UNIFIED ENUMS AVAILABILITY**
// UnifiedHealthStatus is properly defined in service_types.rs and widely used
// Re-enabling export for consistent access patterns
pub use service_types::UnifiedHealthStatus;

// **COMPATIBILITY ALIASES** for external crates
pub use storage_access_types::UnifiedTierType as StorageTier;

// Remove duplicate import - use service_types::UnifiedServiceState directly
// pub use service_types::UnifiedServiceState;

// Key unified enums for easy access
pub use message_event_types::{UnifiedAlertSeverity, UnifiedMessageType};
pub use service_types::UnifiedServiceState;
pub use service_types::UnifiedServiceState as ServiceState;

// Additional missing enum types referenced in lib.rs
pub use message_event_types::UnifiedAlertSeverity as UnifiedLogLevel;
pub use message_event_types::UnifiedEventType as UnifiedErrorCategory;
pub use crate::unified_enums::service_types::UnifiedServiceType as UnifiedCapabilityType;

/// Common enum patterns for use across the codebase
pub trait UnifiedEnum: Clone + PartialEq + Eq + std::fmt::Debug {
    /// Get the string representation of the enum variant
    fn as_str(&self) -> &str;

    /// Create from string representation
    fn from_str(s: &str) -> Self;

    /// Check if this is a custom variant
    fn is_custom(&self) -> bool;
}
