// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module contains advanced ZFS analytics and monitoring capabilities
// split into focused, maintainable sub-modules for better code organization.

//! Advanced Features module

/// Cache analytics and ARC/L2ARC statistics
pub mod cache;
/// Capacity monitoring and bottleneck detection
pub mod capacity;
/// Compression analytics and efficiency monitoring
pub mod compression;
/// Replication analytics and performance tracking
pub mod replication;
/// Snapshot analytics and policy management
pub mod snapshots;

// Re-export commonly used types for easier access
pub use cache::{ArcStats, CacheAnalytics, CacheEfficiency, L2arcStats};
pub use capacity::{
    detect_performance_bottlenecks, generate_maintenance_schedule, monitor_capacity_usage,
};
pub use compression::CompressionAnalytics;
pub use replication::ReplicationAnalytics;
pub use snapshots::SnapshotAnalytics;
