//! Storage performance metrics and I/O statistics
//!
//! Provides comprehensive metrics for monitoring storage performance,
//! health, and utilization.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Storage performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformanceMetrics {
    /// Read operations per second
    pub read_ops_per_sec: f64,
    /// Write operations per second
    pub write_ops_per_sec: f64,
    /// Read throughput in bytes per second
    pub read_bytes_per_sec: u64,
    /// Write throughput in bytes per second
    pub write_bytes_per_sec: u64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Queue depth
    pub queue_depth: u32,
    /// Utilization percentage (0-100)
    pub utilization_percent: f64,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Storage I/O metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageIoMetrics {
    /// Total read operations
    pub total_reads: u64,
    /// Total write operations
    pub total_writes: u64,
    /// Total bytes read
    pub total_bytes_read: u64,
    /// Total bytes written
    pub total_bytes_written: u64,
    /// Total errors
    pub total_errors: u64,
    /// Average response time
    pub avg_response_time: Duration,
}
