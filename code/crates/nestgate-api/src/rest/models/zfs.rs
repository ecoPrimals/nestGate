// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! ZFS-specific models and statistics

use serde::{Deserialize, Serialize};

// ZFS-specific models and metrics for storage management

/// Comprehensive ZFS system metrics and statistics
///
/// Aggregates performance, capacity, and health metrics across all ZFS pools
/// and datasets for system-wide monitoring and analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsmetrics
pub struct ZfsMetrics {
    /// Total number of ZFS pools in the system
    pub pool_count: u32,
    /// Total number of datasets across all pools
    pub total_datasets: u32,
    /// Total number of snapshots across all pools
    pub total_snapshots: u32,
    /// Total used storage space in bytes across all pools
    pub total_used_bytes: u64,
    /// Total available storage space in bytes across all pools
    pub total_available_bytes: u64,
    /// Overall compression ratio across all pools (e.g., 2.1 = 2.1:1 compression)
    pub overall_compression_ratio: f64,
    /// ARC (Adaptive Replacement Cache) hit ratio as percentage (0.0 to 1.0)
    pub cache_hit_ratio: f64,
}

/// ZFS Adaptive Replacement Cache (ARC) statistics
///
/// Provides detailed metrics about ZFS memory cache performance including
/// size, hit ratios, and memory utilization patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Arcstats
pub struct ArcStats {
    /// Current ARC size in bytes
    pub size_bytes: u64,
    /// Target ARC size in bytes (what ZFS wants to achieve)
    pub target_size_bytes: u64,
    /// Minimum allowed ARC size in bytes
    pub min_size_bytes: u64,
    /// Maximum allowed ARC size in bytes
    pub max_size_bytes: u64,
    /// Cache hit ratio (0.0 to 1.0, higher is better)
    pub hit_ratio: f64,
    /// Cache miss ratio (0.0 to 1.0, lower is better)
    pub miss_ratio: f64,
}

/// ZFS compression statistics and efficiency metrics
///
/// Tracks compression algorithm performance, space savings, and efficiency
/// across datasets and pools.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Compressionstats
pub struct CompressionStats {
    /// Compression algorithm in use (lz4, gzip, zstd, etc.)
    pub algorithm: String,
    /// Effective compression ratio achieved (e.g., 2.1 = 2.1:1 compression)
    pub compression_ratio: f64,
    /// Total bytes stored after compression
    pub compressed_bytes: u64,
    /// Original uncompressed size in bytes
    pub uncompressed_bytes: u64,
}

/// ZFS deduplication statistics and space savings
///
/// Provides metrics on deduplication effectiveness, space savings, and
/// performance impact across the storage system.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Deduplicationstats
pub struct DeduplicationStats {
    /// Deduplication ratio achieved (e.g., 1.5 = 1.5:1 dedup)
    pub dedup_ratio: f64,
    /// Total logical bytes referenced by the filesystem
    pub referenced_bytes: u64,
    /// Actual physical bytes stored after deduplication
    pub deduplicated_bytes: u64,
    /// Total bytes saved through deduplication
    pub saved_bytes: u64,
}

/// ZFS I/O performance statistics
///
/// Captures read/write operations, throughput, and latency metrics
/// for performance monitoring and optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Iostats
pub struct IoStats {
    /// Total read operations performed
    pub read_ops: u64,
    /// Total write operations performed
    pub write_ops: u64,
    /// Total bytes read from storage
    pub read_bytes: u64,
    /// Total bytes written to storage
    pub write_bytes: u64,
    /// Average read latency in milliseconds
    pub read_latency_ms: f64,
    /// Average write latency in milliseconds
    pub write_latency_ms: f64,
}

/// ZFS pool information and health status
///
/// Represents a single ZFS storage pool with its configuration,
/// capacity, health status, and performance characteristics.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolinfo
pub struct PoolInfo {
    /// Pool name identifier
    pub name: String,
    /// Current health status (ONLINE, DEGRADED, FAULTED, etc.)
    pub health: String,
    /// Total pool capacity in bytes
    pub total_bytes: u64,
    /// Used space in bytes
    pub used_bytes: u64,
    /// Available space in bytes
    pub available_bytes: u64,
    /// Fragmentation percentage (0.0 to 100.0, lower is better)
    pub fragmentation_percent: f64,
}

/// Generic data response wrapper with _metadata
///
/// Provides a consistent response format with data payload and
/// associated _metadata for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Data operation
pub struct DataResponse<T> {
    /// The actual data payload
    pub data: T,
    /// Additional _metadata as key-value pairs
    pub _metadata: std::collections::HashMap<String, String>,
    /// Response timestamp in UTC
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
