//
// Provides metrics collection and aggregation for ZFS operations

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

/// ZFS metrics collector
#[derive(Debug)]
pub struct ZfsMetrics {
    /// Total operations counter
    total_operations: AtomicU64,
    /// Total bytes processed
    total_bytes: AtomicU64,
    /// Error counter
    error_count: AtomicU64,
    /// Average latency (milliseconds) stored as u64 bits
    avg_latency_bits: AtomicU64,
    /// Start time for metrics collection
    start_time: SystemTime,
}
/// Current metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub operations_per_second: f64,
    pub throughput_bytes_per_second: u64,
    pub average_latency_ms: f64,
    pub error_rate: f64,
    pub total_operations: u64,
    pub total_bytes: u64,
    pub uptime_seconds: u64,
    pub timestamp: SystemTime,
}
impl ZfsMetrics {
    /// Create metrics collector for testing
    pub fn new_for_testing() -> Self {
        Self::new()
    }

    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            total_operations: AtomicU64::new(0),
            total_bytes: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            avg_latency_bits: AtomicU64::new(0),
            start_time: SystemTime::now(),
        }
    }

    /// Record a successful operation
    pub fn record_operation(&self, bytes_processed: u64, latency_ms: f64) {
        self.total_operations.fetch_add(1, Ordering::Relaxed);
        self.total_bytes
            .fetch_add(bytes_processed, Ordering::Relaxed);

        // Simple moving average for latency using atomic operations
        let current_bits = self.avg_latency_bits.load(Ordering::Relaxed);
        let current_avg = f64::from_bits(current_bits);
        let new_avg = (current_avg * 0.9) + (latency_ms * 0.1);
        self.avg_latency_bits
            .store(new_avg.to_bits(), Ordering::Relaxed);
    }

    /// Record an error
    pub fn record_error(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current metrics snapshot
    pub fn get_current_metrics(&self) -> MetricsSnapshot {
        let now = SystemTime::now();
        let uptime = now
            .duration_since(self.start_time)
            .unwrap_or_default()
            .as_secs();

        let total_ops = self.total_operations.load(Ordering::Relaxed);
        let total_bytes = self.total_bytes.load(Ordering::Relaxed);
        let errors = self.error_count.load(Ordering::Relaxed);

        let ops_per_second = if uptime > 0 {
            total_ops as f64 / uptime as f64
        } else {
            0.0
        };
        let throughput_bps = if uptime > 0 { total_bytes / uptime } else { 0 };
        let error_rate = if total_ops > 0 {
            errors as f64 / total_ops as f64
        } else {
            0.0
        };

        let avg_latency = f64::from_bits(self.avg_latency_bits.load(Ordering::Relaxed));

        MetricsSnapshot {
            operations_per_second: ops_per_second,
            throughput_bytes_per_second: throughput_bps,
            average_latency_ms: avg_latency,
            error_rate,
            total_operations: total_ops,
            total_bytes,
            uptime_seconds: uptime,
            timestamp: now,
        }
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.total_operations.store(0, Ordering::Relaxed);
        self.total_bytes.store(0, Ordering::Relaxed);
        self.error_count.store(0, Ordering::Relaxed);
        self.avg_latency_bits.store(0, Ordering::Relaxed);
    }
}

impl Default for ZfsMetrics {
    fn default() -> Self {
        Self::new()
    }
}
