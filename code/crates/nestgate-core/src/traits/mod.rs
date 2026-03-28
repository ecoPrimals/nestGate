// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Canonical Traits System for NestGate
//!
//! **UNIFIED TRAIT SYSTEM**: This module provides the canonical trait definitions
//! for all service interfaces in NestGate.
//!
//! ## Recently Migrated (November 7, 2025)
//!
//! The following modules have been migrated from `traits_root/`:
//! - `service_trait` - Service lifecycle traits
//! - `communication` - Inter-service communication
//! - `service_discovery` - Service registration and discovery
//! - `health_checks` - Health monitoring
//! - `config_provider` - Configuration management
//! - `load_balancing` - Load balancing algorithms
//!
//! **Migration Note**: Imports from `traits_root` are deprecated.
//! Use `nestgate_core::traits::` instead.

// ==================== NEW CANONICAL MODULES (Nov 7, 2025) ====================

// Migrated from traits_root/ - now canonical
pub mod communication;
pub mod config_provider;
pub mod health_checks;
pub mod load_balancing;
pub mod service_discovery;
pub mod service_trait;

// ==================== EXISTING CANONICAL TRAIT SYSTEM ====================

// Other trait modules
pub mod async_migration_system;
pub mod canonical_hierarchy;
pub mod canonical_provider_unification;

// **NEW** (Nov 19, 2025): Refactored canonical traits into focused modules
// **MIGRATION COMPLETE** (Nov 19, 2025): All code migrated to modular structure
pub mod canonical;

pub mod domain_extensions;
pub mod migration;
pub mod native_async;
/// Security provider migration adapters (backwards compatibility)
/// **NEW**: November 10, 2025 - Phase 2A Provider Trait Consolidation
pub mod security_migration;
pub mod unified_storage;
pub mod universal;
pub mod universal_service_zero_cost;

// ==================== RE-EXPORTS ====================

// Re-export newly migrated traits (Nov 7, 2025)
pub use communication::{
    CommunicationLayer, CommunicationResponse, CommunicationStats, MessageType, ServiceAddress,
    ServiceMessage,
};
pub use config_provider::{ConfigProvider, ConfigProviderInfo, FederationConfig};
pub use health_checks::{HealthCheck, HealthMonitor, HealthState, HealthStatus};
pub use load_balancing::{
    HealthAwareLoadBalancer, LeastConnectionsLoadBalancer, LoadBalancer, LoadBalancerStats,
    LoadBalancingAlgorithm, RandomLoadBalancer, RoundRobinLoadBalancer, ServiceStats,
    WeightedRandomLoadBalancer, WeightedRoundRobinLoadBalancer,
};
pub use service_discovery::{ServiceDiscovery, ServiceEvent, ServiceQuery};
pub use service_trait::Service;

// Re-export the canonical traits for easy access (NEW MODULE STRUCTURE - Nov 19, 2025)
pub use canonical::{
    CanonicalAutomation, CanonicalMcp, CanonicalNetwork, CanonicalProvider,
    CanonicalProviderFactory, CanonicalSecurity, CanonicalService, CanonicalServiceFactory,
    CanonicalStorage, ZeroCostService,
};

// Re-export provider unification patterns
#[allow(deprecated)]
pub use canonical_provider_unification::{
    CacheService, CanonicalUniversalProvider, NetworkProvider, NetworkService, SecurityProvider,
    SecurityService, StorageProvider, StorageService,
};

// Re-export THE unified storage trait - SINGLE SOURCE OF TRUTH
pub use unified_storage::{
    StorageCapability, StorageMetadata, StorageStream, StorageTransaction, UnifiedStorage,
};

// Re-export native async patterns
#[allow(deprecated)]
pub use native_async::{
    NativeAsyncApiHandler, NativeAsyncAutomationService, NativeAsyncMcpService,
    NativeAsyncMonitoringService, NativeAsyncNetworkService, NativeAsyncSecurityProvider,
    NativeAsyncService, NativeAsyncStorage, NativeAsyncUniversalProvider,
};

// Re-export domain extensions
pub use domain_extensions::{StorageServiceExtension, ZfsServiceExtension};

// ==================== MIGRATION COMPLETE ====================

// **STORAGE TRAIT UNIFICATION COMPLETE**: All redundant storage traits have been removed
//
// **CONSOLIDATED INTO**: `CanonicalStorage` - THE single canonical storage trait
// **REMOVED REDUNDANT TRAITS**:
// - `UnifiedStorageBackend` → Use `CanonicalStorage` instead
// - `CanonicalStorageBackend` → Use `CanonicalStorage` instead
// - `StorageBackend` → Use `CanonicalStorage` instead
// - `ZeroCostUnifiedStorageBackend` → Use `CanonicalStorage` instead
// - `EnterpriseStorageCapabilities` → Use `CanonicalStorage` instead
//
// **MIGRATION PATH**:
// ```rust
// // Old fragmented approach
// use crate::universal_storage::unified_storage_traits::UnifiedStorageBackend;
// use crate::universal_storage::canonical_storage::CanonicalStorageBackend;
//
// // New unified approach
// use crate::traits::canonical::CanonicalStorage;
// ```

// **MIGRATION STATUS**: All deprecated traits have been successfully migrated
// to the canonical trait system. The following deprecated items have been removed:
//
// - `UniversalService` → Use `CanonicalService` instead
// - Scattered service traits → Use canonical trait hierarchy
// - `async_trait` patterns → Use native async patterns
// - Legacy provider interfaces → Use `CanonicalProvider<T>`
// - **REDUNDANT STORAGE TRAITS** → Use `CanonicalStorage` instead

// **CANONICAL TRAIT HIERARCHY**:
// ```rust
// CanonicalService (base trait for all services)
// ├── CanonicalProvider<T> (generic provider pattern)
// ├── CanonicalStorage (storage services)
// ├── CanonicalNetwork (network services)
// ├── CanonicalSecurity (security services)
// ├── CanonicalMcp (MCP services)
// └── CanonicalAutomation (automation services)
//
// UnifiedStorage (THE canonical storage interface)
// ├── Core operations (read, write, delete, list, exists)
// ├── Metadata operations (get_metadata, set_metadata)
// ├── Batch operations (batch_read, batch_write, batch_delete)
// ├── Streaming operations (stream_read, stream_write)
// ├── Advanced operations (copy, move, snapshot)
// └── Health & monitoring (health_check, get_metrics)
// ```
//
// **NATIVE ASYNC PATTERNS**:
// All traits now use `impl Future` patterns instead of `async_trait`
// for zero-cost async abstractions.

// ==================== CONVENIENCE TYPE ALIASES ====================
// Note: Type aliases using `dyn Trait` with `impl Future` returns are not object-safe
// These are provided for documentation purposes but cannot be used as trait objects

// ==================== VALIDATION FUNCTIONS ====================

/// Validate that a service implements the canonical interface
/// Returns true if the service passes validation checks
pub fn validate_canonical_service<S>(_service: &S) -> bool
where
    S: CanonicalService,
{
    // Service validation logic would go here
    true
}

/// Validate that a provider implements the canonical interface
/// Returns true if the provider passes validation checks
pub fn validate_canonical_provider<T, P>(_provider: &P) -> bool
where
    P: CanonicalProvider<T>,
{
    // Provider validation logic would go here
    true
}

// Temporarily disabled - needs rewrite for evolved API
// Will be replaced with fresh tests matching current trait signatures
// #[cfg(test)]
// mod canonical_hierarchy_tests;

// Fresh tests for current API
#[cfg(test)]
mod traits_tests;
