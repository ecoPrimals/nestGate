use std::collections::HashMap;
use std::time::SystemTime;

use super::types::*;

use crate::types::StorageTier;

// PerformanceConfig removed - using ZfsConfig.extensions.performance instead
// Default implementation moved to unified configuration system

impl Default for CurrentPerformanceMetrics {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            pool_metrics: PoolPerformanceMetrics::default(),
            tier_metrics: HashMap::new(),
            system_metrics: SystemResourceMetrics::default(),
            io_statistics: IoStatistics::default(),
            trends: PerformanceTrends::default(),
        }
    }
}

impl Default for PoolPerformanceMetrics {
    fn default() -> Self {
        Self {
            total_iops: 80000.0,
            total_throughput_mbs: 1200.0,
            avg_latency_ms: 2.5,
            utilization_percent: 70.0,
            fragmentation_percent: 15.0,
            compression_ratio: 2.1,
            dedup_ratio: 1.3,
        }
    }
}

impl Default for SystemResourceMetrics {
    fn default() -> Self {
        Self {
            cpu_utilization_percent: 25.0,
            memory_usage_bytes: 8 * 1024 * 1024 * 1024, // 8GB
            available_memory_bytes: 24 * 1024 * 1024 * 1024, // 24GB available
            network_io_mbs: 150.0,
            io_wait_percent: 5.0,
            load_average_1m: 1.2,
        }
    }
}

impl Default for IoStatistics {
    fn default() -> Self {
        Self {
            total_reads: 1_000_000,
            total_writes: 500000,
            total_bytes_read: 100 * 1024 * 1024 * 1024, // 100GB
            total_bytes_written: 50 * 1024 * 1024 * 1024, // 50GB
            avg_io_size_bytes: 64 * 1024,               // 64KB
            read_write_ratio: 2.0,
        }
    }
}

impl Default for PerformanceTrends {
    fn default() -> Self {
        Self {
            iops_trend: 0.0,
            throughput_trend: 0.0,
            latency_trend: 0.0,
            utilization_trend: 0.0,
            prediction_confidence: 0.5,
        }
    }
}

impl Default for TierPerformanceTargets {
    fn default() -> Self {
        Self {
            target_iops: 50000.0,
            target_throughput_mbs: 800.0,
            target_latency_ms: 5.0,
            target_utilization_percent: 80.0,
            target_availability_percent: 99.9,
        }
    }
}

impl Default for SlaCompliance {
    fn default() -> Self {
        Self {
            latency_compliance: 98.5,
            throughput_compliance: 99.2,
            availability_percent: 99.95,
            error_rate_compliance: 99.9,
        }
    }
}

impl Default for PoolProperties {
    fn default() -> Self {
        Self {
            fragmentation: 0.0,
            compression_ratio: 1.0,
            dedup_ratio: 1.0,
        }
    }
}

impl Default for DatasetPerformanceStats {
    fn default() -> Self {
        Self {
            read_iops: 0.0,
            write_iops: 0.0,
            read_throughput_mbs: 0.0,
            write_throughput_mbs: 0.0,
            read_latency_ms: 0.0,
            write_latency_ms: 0.0,
            utilization_percent: 0.0,

            compression_effectiveness: 0.0,
            deduplication_effectiveness: 0.0,
        }
    }
}

impl TierMetrics {
    /// Create default tier metrics for a specific tier
    pub fn default_for_tier(tier: StorageTier) -> Self {
        match tier {
            StorageTier::Hot => Self {
                tier,
                read_iops: 100_000.0,
                write_iops: 50000.0,
                read_throughput_mbs: 1500.0,
                write_throughput_mbs: 800.0,
                avg_read_latency_ms: 0.5,
                avg_write_latency_ms: 1.0,
                queue_depth: 32.0,
                cache_hit_ratio: 95.0,
                utilization_percent: 80.0,
                targets: TierPerformanceTargets {
                    target_iops: 120000.0,
                    target_throughput_mbs: 1800.0,
                    target_latency_ms: 1.0,
                    target_utilization_percent: 85.0,
                    target_availability_percent: 99.99,
                },
                sla_compliance: SlaCompliance::default(),
            },
            StorageTier::Warm => Self {
                tier,
                read_iops: 50000.0,
                write_iops: 25000.0,
                read_throughput_mbs: 800.0,
                write_throughput_mbs: 400.0,
                avg_read_latency_ms: 2.0,
                avg_write_latency_ms: 4.0,
                queue_depth: 16.0,
                cache_hit_ratio: 85.0,
                utilization_percent: 70.0,
                targets: TierPerformanceTargets {
                    target_iops: 60000.0,
                    target_throughput_mbs: 1000.0,
                    target_latency_ms: 3.0,
                    target_utilization_percent: 75.0,
                    target_availability_percent: 99.9,
                },
                sla_compliance: SlaCompliance::default(),
            },
            StorageTier::Cold => Self {
                tier,
                read_iops: 10_000.0,
                write_iops: 5000.0,
                read_throughput_mbs: 200.0,
                write_throughput_mbs: 100.0,
                avg_read_latency_ms: 10.0,
                avg_write_latency_ms: 20.0,
                queue_depth: 8.0,
                cache_hit_ratio: 60.0,
                utilization_percent: 50.0,
                targets: TierPerformanceTargets {
                    target_iops: 15000.0,
                    target_throughput_mbs: 300.0,
                    target_latency_ms: 15.0,
                    target_utilization_percent: 60.0,
                    target_availability_percent: 99.5,
                },
                sla_compliance: SlaCompliance::default(),
            },
            StorageTier::Cache => Self {
                tier,
                read_iops: 200000.0,
                write_iops: 100_000.0,
                read_throughput_mbs: 3000.0,
                write_throughput_mbs: 1500.0,
                avg_read_latency_ms: 0.1,
                avg_write_latency_ms: 0.2,
                queue_depth: 64.0,
                cache_hit_ratio: 99.0,
                utilization_percent: 40.0,
                targets: TierPerformanceTargets {
                    target_iops: 250000.0,
                    target_throughput_mbs: 4000.0,
                    target_latency_ms: 0.5,
                    target_utilization_percent: 50.0,
                    target_availability_percent: 99.99,
                },
                sla_compliance: SlaCompliance::default(),
            },
            StorageTier::Archive => Self {
                tier,
                read_iops: 500.0,
                write_iops: 100.0,
                read_throughput_mbs: 100.0,
                write_throughput_mbs: 50.0,
                avg_read_latency_ms: 100.0,
                avg_write_latency_ms: 200.0,
                queue_depth: 1.0,
                cache_hit_ratio: 10.0,     // Low cache hit for archive
                utilization_percent: 95.0, // High utilization for cost efficiency
                targets: TierPerformanceTargets {
                    target_iops: 500.0,
                    target_throughput_mbs: 100.0,
                    target_latency_ms: 100.0,
                    target_utilization_percent: 95.0,
                    target_availability_percent: 98.0,
                },
                sla_compliance: SlaCompliance::default(),
            },
        }
    }
}

impl CurrentPerformanceMetrics {
    /// Get tier metrics for a specific tier
    pub fn get_tier_metrics(&self, tier: StorageTier) -> Option<&TierMetrics> {
        self.tier_metrics.get(&tier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_performance_metrics_defaults() {
        // Test that performance metrics can be created with defaults
        let metrics = CurrentPerformanceMetrics::default();

        // Verify metrics are properly initialized
        assert!(metrics.pool_metrics.total_iops >= 0.0);
        assert_eq!(metrics.tier_metrics.len(), 0);
    }
    #[test]
    fn test_tier_metrics_default() {
        let hot_metrics = TierMetrics::default_for_tier(StorageTier::Hot);
        let warm_metrics = TierMetrics::default_for_tier(StorageTier::Warm);
        let cold_metrics = TierMetrics::default_for_tier(StorageTier::Cold);

        // Hot tier should have highest performance
        assert!(hot_metrics.read_iops > warm_metrics.read_iops);
        assert!(warm_metrics.read_iops > cold_metrics.read_iops);

        // Latency should increase from hot to cold
        assert!(hot_metrics.avg_read_latency_ms < warm_metrics.avg_read_latency_ms);
        assert!(warm_metrics.avg_read_latency_ms < cold_metrics.avg_read_latency_ms);
    }

    #[test]
    fn test_alert_condition_creation() {
        let condition = AlertCondition {
            id: "test-alert".to_string(),
            name: "Test Alert".to_string(),
            description: "Test alert condition".to_string(),
            metric: AlertMetric::Latency,
            operator: AlertOperator::GreaterThan,
            threshold: 100.0,
            duration: Duration::from_secs(
                std::env::var("NESTGATE_ZFS_PERFORMANCE_ALERT_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300), // 5 minutes default
            ),
            severity: AlertSeverity::Warning,
            enabled: true,
        };

        assert_eq!(condition.threshold, 100.0);
        assert!(condition.enabled);
        assert!(matches!(condition.severity, AlertSeverity::Warning));
    }
}
