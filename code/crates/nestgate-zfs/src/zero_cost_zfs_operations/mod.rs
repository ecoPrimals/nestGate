//! **ZERO-COST ZFS OPERATIONS MODULE**
//! This module provides zero-cost abstractions for ZFS operations
//! Replaces `Arc<dyn>` patterns with compile-time dispatch for maximum performance

mod manager;
mod traits;
mod types;
mod utilities;

// Re-export main types and traits
pub use manager::ZeroCostZfsManager;
pub use traits::ZeroCostZfsOperations;
pub use types::{
    DatasetInfoMap, PoolInfoMap, SnapshotInfoMap, ZeroCostDatasetInfo, ZeroCostPoolInfo,
    ZeroCostSnapshotInfo,
};
pub use utilities::{ZfsBenchmark, ZfsMigrationGuide};

// Re-export type aliases for different deployment sizes
pub use manager::{
    DevelopmentZfsManager, EnterpriseZfsManager, HighPerformanceZfsManager, ProductionZfsManager,
    TestingZfsManager,
};
