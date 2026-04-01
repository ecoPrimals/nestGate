// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::HashMap;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

/// ZFS performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceMetrics {
    /// When these metrics were collected
    pub timestamp: SystemTime,
    /// Per-pool performance metrics
    pub pool_metrics: HashMap<String, ZfsPoolMetrics>,
    /// Per-dataset performance metrics
    pub dataset_metrics: HashMap<String, ZfsDatasetMetrics>,
    /// System memory usage statistics
    pub system_memory: SystemMemoryUsage,
    /// ARC (Adaptive Replacement Cache) statistics
    pub arc_stats: ArcStatistics,
}
/// ZFS pool metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolMetrics {
    /// Name of the ZFS pool
    pub pool_name: String,
    /// Read operations per second
    pub read_ops: f64,
    /// Write operations per second
    pub write_ops: f64,
    /// Read bandwidth in bytes/second
    pub read_bandwidth: f64,
    /// Write bandwidth in bytes/second
    pub write_bandwidth: f64,
    /// Average latency in milliseconds
    pub latency: f64,
    /// Cache hit ratio (0.0 to 1.0)
    pub cache_hit_ratio: f64,
    /// Pool fragmentation percentage (0.0 to 100.0)
    pub fragmentation: f64,
}
/// ZFS dataset metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDatasetMetrics {
    /// Name of the ZFS dataset
    pub dataset_name: String,
    /// Detected I/O access pattern
    pub access_pattern: AccessPattern,
    /// Deduplication ratio (1.0 = no dedup, >1.0 = space saved)
    pub dedup_ratio: f64,
    /// Record size in bytes
    pub record_size: u64,
}
/// Access pattern classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessPattern {
    /// Sequential I/O pattern
    Sequential,
    /// Random I/O pattern
    Random,
    /// Mixed sequential and random I/O
    Mixed,
}
/// System memory usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMemoryUsage {
    /// Total system memory in bytes
    pub total: u64,
    /// Available memory in bytes
    pub available: u64,
    /// Used memory in bytes
    pub used: u64,
}
/// ARC statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcStatistics {
    /// Current ARC size in bytes
    pub size: u64,
    /// Target ARC size in bytes
    pub target_size: u64,
    /// Cache hit ratio (0.0 to 1.0)
    pub hit_ratio: f64,
    /// Cache miss ratio (0.0 to 1.0)
    pub miss_ratio: f64,
}
