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

pub mod backends;
pub mod config;
pub mod factory;
pub mod fail_safe;
pub mod traits;
pub mod types;
pub mod universal_storage_bridge;

// Re-export main production types
pub use backends::{NativeZfsService, RemoteZfsService};
pub use config::{CircuitBreakerConfig, RetryPolicy, ZfsBackend, ZfsServiceConfig};
pub use factory::ZfsServiceFactory;
pub use fail_safe::FailSafeZfsService;
pub use traits::{DynZfsService, UniversalZfsService};
pub use types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};
pub use universal_storage_bridge::UniversalStorageBridge;

// Mock service only available in test builds
#[cfg(test)]
mod test_mocks {
    // CANONICAL MODERNIZATION: MockZfsService removed from production exports
    // Use real implementations only - mocks are test-scoped
}
