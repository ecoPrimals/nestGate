// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Metrics collection for adaptive optimization.

use super::types::{CurrentMetrics, PerformanceSnapshot};
use nestgate_core::error::Result;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Metrics collector for comprehensive system performance monitoring
pub struct MetricsCollector {
    cpu_utilization: AtomicU64, // Percentage * 100 for precision
    memory_utilization: AtomicU64,
    network_throughput: AtomicU64, // Bytes per second
    disk_iops: AtomicU64,
    cache_hit_ratio: AtomicU64, // Percentage * 100
    lock_contention_ratio: AtomicU64,
    simd_utilization: AtomicU64,
    allocation_efficiency: AtomicU64,
}

impl MetricsCollector {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cpu_utilization: AtomicU64::new(0),
            memory_utilization: AtomicU64::new(0),
            network_throughput: AtomicU64::new(0),
            disk_iops: AtomicU64::new(0),
            cache_hit_ratio: AtomicU64::new(9500), // Default 95%
            lock_contention_ratio: AtomicU64::new(0),
            simd_utilization: AtomicU64::new(0),
            allocation_efficiency: AtomicU64::new(8000), // Default 80%
        }
    }

    /// Collect current system metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn collect_current_metrics(&self) -> Result<CurrentMetrics> {
        // In a real implementation, these would query actual system metrics
        // For now, we'll return the stored atomic values
        Ok(CurrentMetrics {
            cpu_usage: self.cpu_utilization.load(Ordering::Relaxed) as f64 / 100.0,
            memory_usage: self.memory_utilization.load(Ordering::Relaxed) as f64 / 100.0,
            network_throughput: self.network_throughput.load(Ordering::Relaxed),
            disk_iops: self.disk_iops.load(Ordering::Relaxed),
            cache_hit_ratio: self.cache_hit_ratio.load(Ordering::Relaxed) as f64 / 100.0,
            lock_contention: self.lock_contention_ratio.load(Ordering::Relaxed) as f64 / 100.0,
            simd_utilization: self.simd_utilization.load(Ordering::Relaxed) as f64 / 100.0,
            allocation_efficiency: self.allocation_efficiency.load(Ordering::Relaxed) as f64
                / 100.0,
        })
    }

    /// Create a performance snapshot
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn create_snapshot(&self) -> Result<PerformanceSnapshot> {
        let metrics = self.collect_current_metrics()?;

        Ok(PerformanceSnapshot {
            timestamp: Instant::now(),
            cpu_utilization: metrics.cpu_usage,
            memory_utilization: metrics.memory_usage,
            network_throughput: metrics.network_throughput,
            disk_iops: metrics.disk_iops,
            cache_hit_ratio: metrics.cache_hit_ratio,
            lock_contention_ratio: metrics.lock_contention,
            simd_utilization: metrics.simd_utilization,
            allocation_efficiency: metrics.allocation_efficiency,
        })
    }

    /// Update CPU utilization metric
    pub fn update_cpu_utilization(&self, percentage: f64) {
        self.cpu_utilization
            .store((percentage * 100.0) as u64, Ordering::Relaxed);
    }

    /// Update memory utilization metric
    pub fn update_memory_utilization(&self, percentage: f64) {
        self.memory_utilization
            .store((percentage * 100.0) as u64, Ordering::Relaxed);
    }

    /// Update network throughput metric
    pub fn update_network_throughput(&self, bytes_per_sec: u64) {
        self.network_throughput
            .store(bytes_per_sec, Ordering::Relaxed);
    }

    /// Update disk IOPS metric
    pub fn update_disk_iops(&self, iops: u64) {
        self.disk_iops.store(iops, Ordering::Relaxed);
    }

    /// Update cache hit ratio metric
    pub fn update_cache_hit_ratio(&self, ratio: f64) {
        self.cache_hit_ratio
            .store((ratio * 100.0) as u64, Ordering::Relaxed);
    }

    /// Update lock contention ratio metric
    pub fn update_lock_contention_ratio(&self, ratio: f64) {
        self.lock_contention_ratio
            .store((ratio * 100.0) as u64, Ordering::Relaxed);
    }

    /// Update SIMD utilization metric
    pub fn update_simd_utilization(&self, percentage: f64) {
        self.simd_utilization
            .store((percentage * 100.0) as u64, Ordering::Relaxed);
    }

    /// Update allocation efficiency metric
    pub fn update_allocation_efficiency(&self, percentage: f64) {
        self.allocation_efficiency
            .store((percentage * 100.0) as u64, Ordering::Relaxed);
    }
}

impl Default for MetricsCollector {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector_new_and_default() {
        let _ = MetricsCollector::new();
        let _ = MetricsCollector::default();
    }

    #[test]
    fn test_collect_current_metrics_defaults() {
        let collector = MetricsCollector::new();
        let metrics = collector.collect_current_metrics().unwrap();
        assert_eq!(metrics.cpu_usage, 0.0);
        assert_eq!(metrics.memory_usage, 0.0);
        assert_eq!(metrics.network_throughput, 0);
        assert_eq!(metrics.disk_iops, 0);
        assert!((metrics.cache_hit_ratio - 95.0).abs() < 0.01);
        assert!((metrics.allocation_efficiency - 80.0).abs() < 0.01);
    }

    #[test]
    fn test_update_and_collect_metrics() {
        let collector = MetricsCollector::new();
        collector.update_cpu_utilization(75.5);
        collector.update_memory_utilization(60.0);
        collector.update_network_throughput(1_000_000);
        collector.update_disk_iops(500);
        collector.update_cache_hit_ratio(0.92);
        collector.update_lock_contention_ratio(0.05);
        collector.update_simd_utilization(80.0);
        collector.update_allocation_efficiency(85.0);

        let metrics = collector.collect_current_metrics().unwrap();
        assert!((metrics.cpu_usage - 75.5).abs() < 0.01);
        assert!((metrics.memory_usage - 60.0).abs() < 0.01);
        assert_eq!(metrics.network_throughput, 1_000_000);
        assert_eq!(metrics.disk_iops, 500);
        assert!((metrics.cache_hit_ratio - 0.92).abs() < 0.01);
        assert!((metrics.lock_contention - 0.05).abs() < 0.01);
        assert!((metrics.simd_utilization - 80.0).abs() < 0.01);
        assert!((metrics.allocation_efficiency - 85.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_create_snapshot() {
        let collector = MetricsCollector::new();
        collector.update_cpu_utilization(50.0);
        let snapshot = collector.create_snapshot().await.unwrap();
        assert!((snapshot.cpu_utilization - 50.0).abs() < 0.01);
    }
}
