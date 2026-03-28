// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Canonical type system for NestGate
//!
//! This module provides canonical type aliases for complex types used throughout
//! the NestGate ecosystem. This enables zero-copy operations and consistent typing.
//!
//! # Purpose
//!
//! - **Zero-Copy Operations**: Type aliases enable passing complex types by reference
//! - **Consistent Typing**: Single source of truth for type definitions
//! - **Ecosystem Integration**: Shared types across all NestGate components
//!
//! # Modules
//!
//! - [`config_registry`] - Configuration type registries
//! - [`core_types`] - Core canonical types (alerts, registries, capabilities)
//!
//! # Examples
//!
//! ```rust
//! use nestgate_core::canonical::types::ServiceRegistry;
//!
//! // Use canonical types for consistent typing
//! fn process_registry(registry: &ServiceRegistry) {
//!     // Zero-copy access to registry
//! }
//! ```

pub mod config_registry;

/// Core canonical types including alerts, registries, and capability maps
///
/// This module contains the fundamental type aliases used throughout NestGate
/// for service discovery, monitoring, and capability management.
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
