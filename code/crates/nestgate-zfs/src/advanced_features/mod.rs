//
// This module contains advanced ZFS analytics and monitoring capabilities
// split into focused, maintainable sub-modules for better code organization.

//! Advanced Features module

pub mod cache;
pub mod capacity;
pub mod compression;
pub mod replication;
pub mod snapshots;

// Re-export commonly used types for easier access
pub use cache::{ArcStats, CacheAnalytics, CacheEfficiency, L2arcStats};
pub use capacity::{
    detect_performance_bottlenecks, generate_maintenance_schedule, monitor_capacity_usage,
};
pub use compression::CompressionAnalytics;
pub use replication::ReplicationAnalytics;
pub use snapshots::SnapshotAnalytics;
