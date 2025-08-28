pub mod native_async;
pub mod native_async_final_services;
/// Services Module
/// Central module for all NestGate services, providing complete service implementations
/// to replace the mock/stub services that were identified in the service completeness analysis.
pub mod storage;
pub mod sync;

// ==================== SECTION ====================
// All functionality consolidated into crate::traits::canonical_unified_traits::CanonicalService

// Re-export service types for easier access
pub use storage::{
    CacheConfig, CacheType, EvictionPolicy, StorageManagerService, StoragePool, StoragePoolType, StorageQuota,
    StorageServiceStats,
};
