pub mod native_async;
pub mod native_async_final_services;
/// Services Module
/// Central module for all NestGate services, providing complete service implementations
/// to replace the mock/stub services that were identified in the service completeness analysis.
pub mod storage;
pub mod sync;

// Service trait migration utilities
// Provides tools to migrate from deprecated Service trait to UniversalService
// TEMPORARILY DISABLED: pub mod migration; // Needs trait migration to UniversalService

// Re-export service types for easier access
pub use storage::{
    CacheConfig, CacheStats, CacheType, EvictionPolicy, PoolHealthStatus, QuotaEnforcementPolicy,
    ServiceHealthStatus, StorageManagerService, StoragePool, StoragePoolType, StorageQuota,
    StorageServiceStats,
};
// Removed non-existent SyncService export
// pub use sync::SyncService;

// TEMPORARILY DISABLED: Migration utilities need trait updates
// pub use migration::{
//     ServiceMigrationAdapter, DefaultServiceConfig, DefaultServiceHealth,
//     ServiceRegistryMigration, create_service_config, validate_service_config,
//     MIGRATION_GUIDE,
// };

// REMOVED: Deprecated Service trait and ServiceRegistry eliminated
// All functionality consolidated into nestgate_core::traits::UniversalService
// Use the canonical service trait system instead of the deprecated legacy traits
