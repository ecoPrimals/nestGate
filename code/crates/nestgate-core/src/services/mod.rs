// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Native async service implementations
pub mod native_async;
/// Final production-ready native async services
pub mod native_async_final_services;
// Services Module
// Central module for all NestGate services, providing complete service implementations
/// to replace the mock/stub services that were identified in the service completeness analysis.
pub mod storage;
/// Synchronous service implementations
pub mod sync;
// ==================== SECTION ====================
// All functionality consolidated into crate::traits::canonical::CanonicalService

// Re-export service types for easier access
pub use storage::{
    CacheConfig, CacheType, EvictionPolicy, StorageManagerService, StoragePool, StoragePoolType,
    StorageQuota, StorageServiceStats,
};
