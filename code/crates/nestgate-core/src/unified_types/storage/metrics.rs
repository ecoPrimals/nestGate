/// **STORAGE METRICS AND PERFORMANCE TYPES**
///
/// This module contains storage performance metrics, I/O statistics, cache statistics,
/// and monitoring types. Split from consolidated_storage_types.rs for better
/// maintainability and 2000-line compliance.
use serde::{Deserialize, Serialize};
use std::time::Duration;
// ==================== SECTION ====================

/// Comprehensive storage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    /// Performance metrics
    pub performance: StoragePerformanceMetrics,
    /// I/O statistics
    pub io_stats: StorageIOStats,

    /// Cache statistics (if applicable)
    pub cache_stats: Option<StorageCacheStats>,

    /// Replication statistics (if applicable)
    pub replication_stats: Option<StorageReplicationStats>,

    /// Snapshot statistics (if applicable)
    pub snapshot_stats: Option<StorageSnapshotStats>,

    /// Last metrics update timestamp
    pub last_updated: DateTime<Utc>,

    /// Metrics collection interval
    pub collection_interval: Duration,
}

/// Storage performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformanceMetrics {
    /// Read IOPS (Input/Output Operations Per Second)
    pub read_iops: f64,
    /// Write IOPS
    pub write_iops: f64,

    /// Read throughput in bytes per second
    pub read_throughput: u64,

    /// Write throughput in bytes per second
    pub write_throughput: u64,

    /// Average read latency in milliseconds
    pub read_latency_ms: f64,

    /// Average write latency in milliseconds
    pub write_latency_ms: f64,

    /// Queue depth
    pub queue_depth: u32,

    /// Utilization percentage (0.0 - 100.0)
    pub utilization_percent: f64,
}

/// Storage I/O statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageIOStats {
    /// Total read operations
    pub read_ops: u64,
    /// Total write operations
    pub write_ops: u64,

    /// Total bytes read
    pub bytes_read: u64,

    /// Total bytes written
    pub bytes_written: u64,

    /// Read errors
    pub read_errors: u64,

    /// Write errors
    pub write_errors: u64,

    /// Total operation time
    pub total_time: Duration,

    /// Statistics collection start time
    pub collection_start: DateTime<Utc>,
}

/// Storage cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCacheStats {
    /// Cache hits
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,

    /// Cache hit ratio (0.0 - 1.0)
    pub hit_ratio: f64,

    /// Cache size in bytes
    pub cache_size: u64,

    /// Used cache size in bytes
    pub used_cache_size: u64,

    /// Cache evictions
    pub evictions: u64,

    /// Cache writes
    pub cache_writes: u64,

    /// Cache reads
    pub cache_reads: u64,
}

/// Storage replication statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageReplicationStats {
    /// Number of replicas
    pub replica_count: u32,
    /// Healthy replicas
    pub healthy_replicas: u32,

    /// Replication lag in milliseconds
    pub replication_lag_ms: f64,

    /// Bytes replicated
    pub bytes_replicated: u64,

    /// Replication errors
    pub replication_errors: u64,

    /// Last replication time
    pub last_replication: DateTime<Utc>,
}

/// Storage snapshot statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSnapshotStats {
    /// Total snapshots
    pub total_snapshots: u32,
    /// Snapshot size in bytes
    pub snapshot_size: u64,

    /// Last snapshot time
    pub last_snapshot: DateTime<Utc>,

    /// Snapshot creation time
    pub avg_creation_time: Duration,

    /// Failed snapshots
    pub failed_snapshots: u32,

    /// Oldest snapshot age
    pub oldest_snapshot_age: Duration,
}

// ==================== SECTION ====================

/// Storage performance requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformanceRequirements {
    /// Minimum IOPS requirement
    pub min_iops: Option<u32>,
    /// Minimum throughput in MB/s
    pub min_throughput_mb: Option<u32>,

    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: Option<f64>,

    /// Consistency requirements
    pub consistency_level: ConsistencyLevel,

    /// Durability requirements
    pub durability_level: DurabilityLevel,
}

/// Storage consistency levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsistencyLevel {
    /// Eventual consistency
    Eventual,
    /// Strong consistency
    Strong,
    /// Session consistency
    Session,
    /// Bounded staleness
    BoundedStaleness,
}
/// Storage durability levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DurabilityLevel {
    /// No durability guarantees
    None,
    /// Single node durability
    SingleNode,
    /// Multi-node durability
    MultiNode,
    /// Cross-region durability
    CrossRegion,
}
// ==================== SECTION ====================

impl Default for StorageMetrics {
    fn default() -> Self {
        Self {
            performance: StoragePerformanceMetrics::default(),
            io_stats: StorageIOStats::default(),
            cache_stats: None,
            replication_stats: None,
            snapshot_stats: None,
            last_updated: Utc::now(),
            collection_interval: Duration::from_secs(60), // 1 minute
        }
    }
}

impl Default for StoragePerformanceMetrics {
    fn default() -> Self {
        Self {
            read_iops: 0.0,
            write_iops: 0.0,
            read_throughput: 0,
            write_throughput: 0,
            read_latency_ms: 0.0,
            write_latency_ms: 0.0,
            queue_depth: 0,
            utilization_percent: 0.0,
        }
    }
}

impl Default for StorageIOStats {
    fn default() -> Self {
        Self {
            read_ops: 0,
            write_ops: 0,
            bytes_read: 0,
            bytes_written: 0,
            read_errors: 0,
            write_errors: 0,
            total_time: Duration::from_secs(0),
            collection_start: Utc::now(),
        }
    }
}

impl Default for StoragePerformanceRequirements {
    fn default() -> Self {
        Self {
            min_iops: None,
            min_throughput_mb: None,
            max_latency_ms: None,
            consistency_level: ConsistencyLevel::Eventual,
            durability_level: DurabilityLevel::SingleNode,
        }
    }
}

// ==================== SECTION ====================

impl StorageMetrics {
    /// Update all metrics with current timestamp
    pub fn update_timestamp(&mut self) {
        self.last_updated = Utc::now();
    }

    /// Calculate total IOPS
    pub const fn total_iops(&self) -> f64 {
        self.performance.read_iops + self.performance.write_iops
    }

    /// Calculate total throughput
    pub const fn total_throughput(&self) -> u64 {
        self.performance.read_throughput + self.performance.write_throughput
    }

    /// Calculate average latency
    pub const fn average_latency(&self) -> f64 {
        (self.performance.read_latency_ms + self.performance.write_latency_ms) / 2.0
    }
}

impl StorageCacheStats {
    /// Calculate cache hit ratio
    pub fn calculate_hit_ratio(&mut self) {
        let total_requests = self.cache_hits + self.cache_misses;
        if total_requests > 0 {
            self.hit_ratio = self.f64::from(cache_hits) / f64::from(total_requests);
        } else {
            self.hit_ratio = 0.0;
        }
    }

    /// Get cache utilization percentage
    pub const fn cache_utilization(&self) -> f64 {
        if self.cache_size > 0 {
            (self.f64::from(used_cache_size) / self.f64::from(cache_size)) * 100.0
        } else {
            0.0
        }
    }
}

impl StorageIOStats {
    /// Calculate total operations
    pub const fn total_operations(&self) -> u64 {
        self.read_ops + self.write_ops
    }

    /// Calculate total bytes transferred
    pub const fn total_bytes(&self) -> u64 {
        self.bytes_read + self.bytes_written
    }

    /// Calculate error rate
    pub const fn error_rate(&self) -> f64 {
        let total_ops = self.total_operations();
        if total_ops > 0 {
            ((self.read_errors + self.write_errors) as f64 / f64::from(total_ops)) * 100.0
        } else {
            0.0
        }
    }
}
