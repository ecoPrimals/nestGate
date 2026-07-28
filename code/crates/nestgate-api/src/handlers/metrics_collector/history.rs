// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

use crate::handlers::dashboard_types::DashboardTimeRange;

use super::types::{
    CacheMetricsPoint, CapacityMetricsPoint, ComprehensiveMetricsPoint, IOMetricsPoint,
    PoolMetrics, RealTimeMetrics,
};

/// Default ring-buffer capacity (~83 minutes at 5-second intervals).
pub const DEFAULT_HISTORY_CAPACITY: usize = 1000;

/// Default interval between background metric snapshots.
pub const DEFAULT_COLLECTION_INTERVAL: Duration = Duration::from_secs(5);

/// A point-in-time metrics sample stored in the ring buffer.
#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    /// Full real-time metrics captured at collection time.
    pub metrics: RealTimeMetrics,
    /// ZFS ARC size in bytes at capture time.
    pub arc_size: u64,
    /// ZFS L2ARC size in bytes at capture time.
    pub l2arc_size: u64,
}

/// In-memory circular buffer of recent metrics snapshots.
#[derive(Debug, Clone)]
pub struct MetricsHistoryBuffer {
    snapshots: Arc<RwLock<VecDeque<MetricsSnapshot>>>,
    capacity: usize,
}

impl MetricsHistoryBuffer {
    /// Create a ring buffer with the given maximum entry count.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            snapshots: Arc::new(RwLock::new(VecDeque::with_capacity(
                capacity.min(DEFAULT_HISTORY_CAPACITY),
            ))),
            capacity: capacity.max(1),
        }
    }

    /// Append a snapshot, evicting the oldest entry when at capacity.
    pub fn push(&self, snapshot: MetricsSnapshot) {
        let Ok(mut deque) = self.snapshots.write() else {
            return;
        };
        if deque.len() >= self.capacity {
            deque.pop_front();
        }
        deque.push_back(snapshot);
    }

    /// Return snapshots whose timestamps fall within `[start, end]`.
    #[must_use]
    pub fn snapshots_in_range(&self, range: &DashboardTimeRange) -> Vec<MetricsSnapshot> {
        let Ok(deque) = self.snapshots.read() else {
            return Vec::new();
        };
        deque
            .iter()
            .filter(|snapshot| timestamp_in_range(snapshot.metrics.timestamp, range))
            .cloned()
            .collect()
    }

    /// Return all snapshots (used when no time range filter applies).
    #[must_use]
    pub fn all_snapshots(&self) -> Vec<MetricsSnapshot> {
        let Ok(deque) = self.snapshots.read() else {
            return Vec::new();
        };
        deque.iter().cloned().collect()
    }
}

impl Default for MetricsHistoryBuffer {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_HISTORY_CAPACITY)
    }
}

fn timestamp_in_range(timestamp: SystemTime, range: &DashboardTimeRange) -> bool {
    timestamp >= range.start && timestamp <= range.end
}

/// Extract per-pool metrics from historical snapshots.
#[must_use]
pub fn pool_metrics_history(snapshots: &[MetricsSnapshot], pool_name: &str) -> Vec<PoolMetrics> {
    snapshots
        .iter()
        .filter_map(|snapshot| {
            snapshot
                .metrics
                .pool_metrics
                .iter()
                .find(|pool| pool.name == pool_name)
                .cloned()
        })
        .collect()
}

/// Build I/O trend points from historical snapshots.
#[must_use]
pub fn io_metrics_history(snapshots: &[MetricsSnapshot]) -> Vec<IOMetricsPoint> {
    snapshots
        .iter()
        .map(|snapshot| {
            let (read_iops, write_iops) = aggregate_pool_iops(&snapshot.metrics.pool_metrics);
            IOMetricsPoint {
                timestamp: snapshot.metrics.timestamp,
                read_iops,
                write_iops,
                read_latency: snapshot.metrics.average_read_latency,
                write_latency: snapshot.metrics.average_write_latency,
            }
        })
        .collect()
}

/// Build comprehensive trend points from historical snapshots.
#[must_use]
pub fn comprehensive_metrics_history(
    snapshots: &[MetricsSnapshot],
) -> Vec<ComprehensiveMetricsPoint> {
    snapshots
        .iter()
        .enumerate()
        .map(|(index, snapshot)| {
            let (read_iops, write_iops) = aggregate_pool_iops(&snapshot.metrics.pool_metrics);
            let (total_capacity, used_space, growth_rate) =
                capacity_totals_at_index(snapshots, index);

            ComprehensiveMetricsPoint {
                timestamp: snapshot.metrics.timestamp,
                io_metrics: IOMetricsPoint {
                    timestamp: snapshot.metrics.timestamp,
                    read_iops,
                    write_iops,
                    read_latency: snapshot.metrics.average_read_latency,
                    write_latency: snapshot.metrics.average_write_latency,
                },
                cache_metrics: CacheMetricsPoint {
                    timestamp: snapshot.metrics.timestamp,
                    arc_hit_ratio: snapshot.metrics.arc_hit_ratio,
                    l2arc_hit_ratio: snapshot.metrics.l2arc_hit_ratio,
                    arc_size: snapshot.arc_size,
                    l2arc_size: snapshot.l2arc_size,
                },
                capacity_metrics: CapacityMetricsPoint {
                    timestamp: snapshot.metrics.timestamp,
                    total_capacity,
                    used_space,
                    growth_rate,
                },
            }
        })
        .collect()
}

/// Build capacity trend points from historical snapshots.
#[must_use]
pub fn capacity_metrics_history(snapshots: &[MetricsSnapshot]) -> Vec<CapacityMetricsPoint> {
    snapshots
        .iter()
        .enumerate()
        .map(|(index, snapshot)| {
            let (total_capacity, used_space, growth_rate) =
                capacity_totals_at_index(snapshots, index);
            CapacityMetricsPoint {
                timestamp: snapshot.metrics.timestamp,
                total_capacity,
                used_space,
                growth_rate,
            }
        })
        .collect()
}

fn aggregate_pool_iops(pools: &[PoolMetrics]) -> (u64, u64) {
    if pools.is_empty() {
        return (0, 0);
    }
    let read_iops = pools.iter().map(|pool| pool.read_iops).sum();
    let write_iops = pools.iter().map(|pool| pool.write_iops).sum();
    (read_iops, write_iops)
}

fn capacity_totals_at_index(snapshots: &[MetricsSnapshot], index: usize) -> (u64, u64, f64) {
    let snapshot = &snapshots[index];
    let total_capacity: u64 = snapshot
        .metrics
        .pool_metrics
        .iter()
        .map(|pool| pool.total_capacity)
        .sum();
    let used_space: u64 = snapshot
        .metrics
        .pool_metrics
        .iter()
        .map(|pool| pool.used_space)
        .sum();

    let growth_rate = if index == 0 {
        0.0
    } else {
        let previous = &snapshots[index - 1];
        let previous_used: u64 = previous
            .metrics
            .pool_metrics
            .iter()
            .map(|pool| pool.used_space)
            .sum();
        let elapsed = snapshot
            .metrics
            .timestamp
            .duration_since(previous.metrics.timestamp)
            .map(|duration| duration.as_secs_f64())
            .unwrap_or(0.0);
        if elapsed > 0.0 {
            (used_space.saturating_sub(previous_used) as f64 / elapsed) * 86_400.0
        } else {
            0.0
        }
    };

    (total_capacity, used_space, growth_rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::metrics_collector::types::{
        DiskIOMetrics, NetworkIOMetrics, SystemMetrics,
    };

    fn sample_metrics(timestamp: SystemTime, used_space: u64) -> RealTimeMetrics {
        RealTimeMetrics {
            timestamp,
            pool_metrics: vec![PoolMetrics {
                name: "pool-a".into(),
                health_status: "ONLINE".into(),
                utilization_percentage: 50.0,
                total_capacity: 1_000,
                used_space,
                available_space: 1_000 - used_space,
                read_iops: 10,
                write_iops: 5,
                read_throughput: 1.0,
                write_throughput: 2.0,
                fragmentation_level: 0.1,
                error_count: 0,
            }],
            system_metrics: SystemMetrics {
                cpu_usage: 1.0,
                memory_usage: 2.0,
                memory_total: 100,
                memory_available: 50,
                network_io: NetworkIOMetrics {
                    bytes_sent: 1,
                    bytes_received: 2,
                    packets_sent: 3,
                    packets_received: 4,
                },
                disk_io: DiskIOMetrics {
                    read_bytes: 10,
                    write_bytes: 20,
                    read_operations: 30,
                    write_operations: 40,
                },
            },
            arc_hit_ratio: 0.9,
            l2arc_hit_ratio: 0.1,
            compression_ratio: 1.2,
            total_throughput: 3.0,
            average_read_latency: 1.0,
            average_write_latency: 2.0,
        }
    }

    #[test]
    fn ring_buffer_evicts_oldest_at_capacity() {
        let buffer = MetricsHistoryBuffer::with_capacity(2);
        let base = SystemTime::UNIX_EPOCH;
        buffer.push(MetricsSnapshot {
            metrics: sample_metrics(base, 100),
            arc_size: 1,
            l2arc_size: 2,
        });
        buffer.push(MetricsSnapshot {
            metrics: sample_metrics(base + Duration::from_secs(5), 200),
            arc_size: 3,
            l2arc_size: 4,
        });
        buffer.push(MetricsSnapshot {
            metrics: sample_metrics(base + Duration::from_secs(10), 300),
            arc_size: 5,
            l2arc_size: 6,
        });

        let all = buffer.all_snapshots();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].metrics.pool_metrics[0].used_space, 200);
        assert_eq!(all[1].metrics.pool_metrics[0].used_space, 300);
    }

    #[test]
    fn snapshots_in_range_filters_by_timestamp() {
        let buffer = MetricsHistoryBuffer::with_capacity(10);
        let start = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
        let end = start + Duration::from_secs(60);
        buffer.push(MetricsSnapshot {
            metrics: sample_metrics(start - Duration::from_secs(10), 100),
            arc_size: 0,
            l2arc_size: 0,
        });
        buffer.push(MetricsSnapshot {
            metrics: sample_metrics(start + Duration::from_secs(10), 200),
            arc_size: 0,
            l2arc_size: 0,
        });
        buffer.push(MetricsSnapshot {
            metrics: sample_metrics(end + Duration::from_secs(10), 300),
            arc_size: 0,
            l2arc_size: 0,
        });

        let range = DashboardTimeRange::new(start, end, Duration::from_secs(5));
        let filtered = buffer.snapshots_in_range(&range);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].metrics.pool_metrics[0].used_space, 200);
    }
}
