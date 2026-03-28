// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Unified Enums module
//!
//! This module provides canonical enum definitions that replace duplicate
//! enums scattered across the codebase, ensuring type consistency and
//! eliminating maintenance overhead.
//!
//! # Problem Solved
//!
//! Eliminates 25+ duplicate enum definitions including:
//! - `ServiceType`
//! - `AlertType`
//! - `DataType`
//! - `MessageType`
//! - `EventType`
//! - And more...
//!
//! # Modules
//!
//! - [`data_types`] - Unified data type enumerations
//! - [`message_event_types`] - Message and event type enumerations
//! - [`network_types`] - Network-related type enumerations
//! - [`storage_types`] - Storage type enumerations
//! - [`service_types`] - Service type enumerations
//! - [`storage_access_types`] - Storage access pattern enumerations
//! - [`system_health_types`] - System health monitoring enumerations

/// Unified data type enumerations for NestGate
pub mod data_types;

/// Unified message and event type enumerations
pub mod message_event_types;

/// Unified network type enumerations
pub mod network_types;

/// Unified storage type enumerations
pub mod storage_types;
// Unified Enum System for NestGate
// This module provides canonical enum definitions that replace duplicate
// enums scattered across the codebase, ensuring type consistency and
// eliminating maintenance overhead.
/// Unified service type enumerations
pub mod service_types;

/// Unified storage access pattern enumerations
pub mod storage_access_types;

/// Unified system health monitoring enumerations
pub mod system_health_types;
// Note: health_status_migrations and service_status_migrations modules
// are not needed as the unified enum system provides all necessary types
// Re-export all unified enum types for backward compatibility
pub use data_types::*;
pub use message_event_types::*;
pub use network_types::*;
pub use service_types::*;
pub use storage_access_types::*;
// Use specific import for storage_types to avoid ambiguous re-exports
pub use storage_types::{StorageOperation, UnifiedStorageCapability, UnifiedStorageType};
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
pub use service_types::UnifiedServiceType as UnifiedCapabilityType;
pub use message_event_types::UnifiedAlertSeverity as UnifiedLogLevel;
pub use message_event_types::UnifiedEventType as UnifiedErrorCategory;

/// Common enum patterns for use across the codebase
///
/// This trait provides a standard interface for all unified enums in NestGate,
/// enabling consistent string conversion, custom variant tracking, and debugging.
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::unified_enums::UnifiedEnum;
///
/// impl UnifiedEnum for MyUnifiedEnum {
///     fn as_str(&self) -> &str {
///         match self {
///             MyUnifiedEnum::Variant1 => "variant1",
///             MyUnifiedEnum::Custom(s) => s,
///         }
///     }
///
///     fn from_str(s: &str) -> Self {
///         match s {
///             "variant1" => MyUnifiedEnum::Variant1,
///             custom => MyUnifiedEnum::Custom(custom.to_string()),
///         }
///     }
///
///     fn is_custom(&self) -> bool {
///         matches!(self, MyUnifiedEnum::Custom(_))
///     }
/// }
/// ```
pub trait UnifiedEnum: Clone + PartialEq + Eq + std::fmt::Debug {
    /// Get the string representation of the enum variant
    fn as_str(&self) -> &str;

    /// Create from string representation
    fn from_str(s: &str) -> Self;

    /// Check if this is a custom variant
    fn is_custom(&self) -> bool;
}
