//
// This module provides a universal, agnostic, and fail-safe abstraction layer
// for ZFS operations that can work with multiple backends (native, mock, remote)
// and provides comprehensive error handling, circuit breakers, and observability.

pub mod backends;
pub mod config;
pub mod factory;
pub mod fail_safe;
pub mod traits;
pub mod types;
pub mod universal_storage_bridge;

// Re-export main types for convenience
pub use backends::{MockZfsService, NativeZfsService, RemoteZfsService};
pub use config::{CircuitBreakerConfig, RetryPolicy, ZfsBackend, ZfsServiceConfig};
pub use factory::ZfsServiceFactory;
pub use fail_safe::FailSafeZfsService;
pub use traits::UniversalZfsService;
pub use types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};
pub use universal_storage_bridge::UniversalStorageBridge;
