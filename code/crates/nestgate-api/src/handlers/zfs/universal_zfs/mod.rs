// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **UNIVERSAL ZFS SERVICE - CANONICAL MODERNIZED**
//!
//! This module provides a universal, agnostic, and fail-safe abstraction layer
//! for ZFS operations that can work with multiple backends (native, remote)
//! and provides comprehensive error handling, circuit breakers, and observability.
//!
//! **CANONICAL MODERNIZATION COMPLETE**:
//! - Production-ready native implementations
//! - Mock services isolated to test builds only
//! - Zero-cost abstractions with real ZFS integration

/// ZFS backend implementations (native, remote)
pub mod backends;
/// Configuration for ZFS services and backends
pub mod config;
/// Factory for creating ZFS service instances
pub mod factory;
/// Fail-safe wrappers with circuit breakers and retry logic
pub mod fail_safe;
/// Core traits for ZFS service abstraction
pub mod traits;
/// Bridge between Universal ZFS and Universal Storage
pub mod universal_storage_bridge;

// Re-export main production types
pub use backends::NativeZfsService;
// pub use backends::RemoteZfsService;  // HTTP removed
pub use config::{CircuitBreakerConfig, RetryPolicy, ZfsBackend, ZfsServiceConfig};
pub use factory::ZfsServiceFactory;
pub use fail_safe::FailSafeZfsService;
pub use traits::{DynZfsService, UniversalZfsService};
pub use universal_storage_bridge::UniversalStorageBridge;

// Re-export types from sibling universal_zfs_types module
pub use super::universal_zfs_types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};

// Mock service only available in test builds
#[cfg(test)]
mod test_mocks {
    // CANONICAL MODERNIZATION: MockZfsService removed from production exports
    // Use real implementations only - mocks are test-scoped
}
