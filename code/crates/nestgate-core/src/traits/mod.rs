// **CANONICAL TRAITS SYSTEM**
//! Module definitions and exports.
// This module provides the unified trait system for NestGate, consolidating
//! all service interfaces into canonical patterns. All deprecated traits
//! have been migrated to the canonical system.

// ==================== CANONICAL TRAIT SYSTEM ====================

// **THE** canonical trait system - single source of truth for all service interfaces
pub mod canonical_unified_traits;
// Canonical provider unification patterns
pub mod canonical_provider_unification;
// **THE** Unified Storage trait - single source of truth
pub mod unified_storage;
// Domain-specific trait extensions
pub mod domain_extensions;
// Native async trait patterns (modern, zero-cost)
pub mod native_async;
// ==================== RE-EXPORTS ====================

// Re-export the canonical traits for easy access
pub use canonical_unified_traits::{
    CanonicalAutomation, CanonicalMcp, CanonicalNetwork, CanonicalProvider,
    CanonicalProviderFactory, CanonicalSecurity, CanonicalService, CanonicalServiceFactory,
    CanonicalStorage, ZeroCostService,
};

// Re-export provider unification patterns
pub use canonical_provider_unification::{
    CacheService, CanonicalUniversalProvider, NetworkProvider, NetworkService, SecurityProvider,
    SecurityService, StorageProvider, StorageService,
};

// Re-export THE unified storage trait - SINGLE SOURCE OF TRUTH
pub use unified_storage::{
    StorageCapability, StorageMetadata, StorageStream, StorageTransaction, UnifiedStorage,
};

// Re-export native async patterns
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
// use crate::traits::canonical_unified_traits::CanonicalStorage;
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

// **CANONICAL SERVICE TRAIT** - The primary service interface
pub type Service = dyn CanonicalService<
    Config = serde_json::Value,
    Health = serde_json::Value,
    Metrics = serde_json::Value,
    Error = crate::NestGateError,
>;
// **CANONICAL PROVIDER TRAIT** - The primary provider interface
pub type Provider<T> = dyn CanonicalProvider<
    T,
    Config = serde_json::Value,
    Error = crate::NestGateError,
    Metadata = serde_json::Value,
>;
// **CANONICAL STORAGE TRAIT** - THE unified storage interface
pub type Storage = dyn UnifiedStorage<
    Config = serde_json::Value,
    Health = serde_json::Value,
    Metrics = serde_json::Value,
    Item = Vec<u8>,
    Key = String,
>;
// ==================== VALIDATION FUNCTIONS ====================

// Validate that a service implements the canonical interface
pub fn validate_canonical_service<S>(_service: &S) -> bool
where
    S: CanonicalService,
{
    // Service validation logic would go here
    true
}
// Validate that a provider implements the canonical interface
pub fn validate_canonical_provider<T, P>(_provider: &P) -> bool
where
    P: CanonicalProvider<T>,
{
    // Provider validation logic would go here
    true
}
