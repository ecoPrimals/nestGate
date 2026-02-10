//! **DATA COLLECTION COMPONENTS**
//!
//! Components for collecting performance data from various sources.

use super::metrics::{MetricsError, SystemMetrics, SystemMetricsCollector};
use std::sync::Arc;
use tokio::time::{interval, Duration};

/// Data collection coordinator
#[derive(Debug)]
/// Datacollector
pub struct DataCollector {
    /// System metrics collector
    pub system_collector: Arc<SystemMetricsCollector>,
    /// Collection interval
    pub interval: Duration,
}

impl DataCollector {
    /// Create new data collector
    #[must_use]
    pub fn new(interval_seconds: u64) -> Self {
        Self {
            system_collector: Arc::new(SystemMetricsCollector::new(interval_seconds)),
            interval: Duration::from_secs(interval_seconds),
        }
    }

    /// Start continuous data collection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn start_collection(&self) -> Result<(), MetricsError> {
        let mut interval_timer = interval(self.interval);

        loop {
            interval_timer.tick().await;

            match self.collect_all_metrics().await {
                Ok(metrics) => {
                    tracing::debug!("Collected metrics: CPU {:.1}%", metrics.cpu_usage_percent);
                    // In a real implementation, metrics would be stored or sent somewhere
                }
                Err(e) => {
                    tracing::error!("Failed to collect metrics: {}", e);
                }
            }
        }
    }

    /// Collect metrics from all sources
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn collect_all_metrics(&self) -> Result<SystemMetrics, MetricsError> {
        self.system_collector.collect_metrics().await
    }

    /// Get latest metrics snapshot
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_latest_snapshot(&self) -> Result<MetricsSnapshot, MetricsError> {
        let metrics = self.collect_all_metrics().await?;

        Ok(MetricsSnapshot {
            system_metrics: metrics,
            collection_timestamp: std::time::SystemTime::now(),
            collector_id: "default".to_string(),
        })
    }
}

/// Metrics snapshot with metadata
#[derive(Debug, Clone)]
/// Metricssnapshot
pub struct MetricsSnapshot {
    /// Collected system performance metrics
    pub system_metrics: SystemMetrics,
    /// When these metrics were collected
    pub collection_timestamp: std::time::SystemTime,
    /// Identifier of the collector that gathered these metrics
    pub collector_id: String,
}

/// Batch metrics collector for high-throughput scenarios
#[derive(Debug)]
/// Batchcollector
pub struct BatchCollector {
    /// Individual collectors
    pub collectors: Vec<Arc<SystemMetricsCollector>>,
    /// Batch size
    pub batch_size: usize,
}

impl BatchCollector {
    /// Create new batch collector
    #[must_use]
    pub fn new(batch_size: usize) -> Self {
        let mut collectors = Vec::new();
        for _i in 0..batch_size {
            collectors.push(Arc::new(SystemMetricsCollector::new(60))); // 60 second intervals
        }

        Self {
            collectors,
            batch_size,
        }
    }

    /// Collect metrics from all collectors in parallel
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn collect_batch(&self) -> Result<Vec<SystemMetrics>, MetricsError> {
        let mut tasks = Vec::new();

        for collector in &self.collectors {
            let collector_clone = collector.clone();
            tasks.push(tokio::spawn(async move {
                collector_clone.collect_metrics().await
            }));
        }

        let mut results = Vec::new();
        for task in tasks {
            match task.await {
                Ok(Ok(metrics)) => results.push(metrics),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(MetricsError::SystemRead(format!("Task error: {e}"))),
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_collector_new() {
        let collector = DataCollector::new(60);
        assert_eq!(collector.interval, Duration::from_secs(60));
        assert_eq!(collector.system_collector.interval_seconds, 60);
    }

    #[tokio::test]
    async fn test_collect_all_metrics() {
        let collector = DataCollector::new(5);
        let metrics = collector
            .collect_all_metrics()
            .await
            .expect("Should collect metrics");

        assert!(metrics.cpu_usage_percent >= 0.0);
        assert!(metrics.memory_usage_bytes > 0);
    }

    #[tokio::test]
    async fn test_get_latest_snapshot() {
        let collector = DataCollector::new(5);
        let snapshot = collector
            .get_latest_snapshot()
            .await
            .expect("Should get snapshot");

        assert_eq!(snapshot.collector_id, "default");
        assert!(snapshot.system_metrics.cpu_usage_percent >= 0.0);
    }

    #[test]
    fn test_metrics_snapshot_creation() {
        let metrics = SystemMetrics {
            cpu_usage_percent: 35.5,
            memory_usage_bytes: 2 * 1024 * 1024 * 1024,
            disk_io_metrics: super::super::metrics::DiskIOMetrics {
                read_bytes_per_sec: 1024 * 1024,
                write_bytes_per_sec: 512 * 1024,
                read_ops_per_sec: 100,
                write_ops_per_sec: 50,
            },
            network_metrics: super::super::metrics::NetworkMetrics {
                rx_bytes_per_sec: 1024 * 1024,
                tx_bytes_per_sec: 512 * 1024,
                rx_packets_per_sec: 1000,
                tx_packets_per_sec: 500,
            },
            timestamp: std::time::SystemTime::now(),
        };

        let snapshot = MetricsSnapshot {
            system_metrics: metrics,
            collection_timestamp: std::time::SystemTime::now(),
            collector_id: "test-collector".to_string(),
        };

        assert_eq!(snapshot.collector_id, "test-collector");
        assert_eq!(snapshot.system_metrics.cpu_usage_percent, 35.5);
    }

    #[test]
    fn test_batch_collector_new() {
        let batch_collector = BatchCollector::new(5);
        assert_eq!(batch_collector.batch_size, 5);
        assert_eq!(batch_collector.collectors.len(), 5);
    }

    #[tokio::test]
    async fn test_batch_collector_collect() {
        let batch_collector = BatchCollector::new(3);
        let results = batch_collector
            .collect_batch()
            .await
            .expect("Should collect batch");

        assert_eq!(results.len(), 3);
        for metrics in results {
            assert!(metrics.cpu_usage_percent >= 0.0);
            assert!(metrics.memory_usage_bytes > 0);
        }
    }

    #[test]
    fn test_data_collector_different_intervals() {
        let collector1 = DataCollector::new(30);
        let collector2 = DataCollector::new(60);
        let collector3 = DataCollector::new(120);

        assert_eq!(collector1.interval, Duration::from_secs(30));
        assert_eq!(collector2.interval, Duration::from_secs(60));
        assert_eq!(collector3.interval, Duration::from_secs(120));
    }

    #[test]
    fn test_batch_collector_multiple_sizes() {
        let small = BatchCollector::new(2);
        let medium = BatchCollector::new(5);
        let large = BatchCollector::new(10);

        assert_eq!(small.collectors.len(), 2);
        assert_eq!(medium.collectors.len(), 5);
        assert_eq!(large.collectors.len(), 10);
    }

    #[test]
    fn test_metrics_snapshot_with_different_collectors() {
        let metrics = SystemMetrics {
            cpu_usage_percent: 50.0,
            memory_usage_bytes: 1024 * 1024 * 1024,
            disk_io_metrics: super::super::metrics::DiskIOMetrics {
                read_bytes_per_sec: 1024,
                write_bytes_per_sec: 512,
                read_ops_per_sec: 10,
                write_ops_per_sec: 5,
            },
            network_metrics: super::super::metrics::NetworkMetrics {
                rx_bytes_per_sec: 2048,
                tx_bytes_per_sec: 1024,
                rx_packets_per_sec: 100,
                tx_packets_per_sec: 50,
            },
            timestamp: std::time::SystemTime::now(),
        };

        let snapshots = [
            MetricsSnapshot {
                system_metrics: metrics.clone(),
                collection_timestamp: std::time::SystemTime::now(),
                collector_id: "collector-1".to_string(),
            },
            MetricsSnapshot {
                system_metrics: metrics.clone(),
                collection_timestamp: std::time::SystemTime::now(),
                collector_id: "collector-2".to_string(),
            },
            MetricsSnapshot {
                system_metrics: metrics,
                collection_timestamp: std::time::SystemTime::now(),
                collector_id: "collector-3".to_string(),
            },
        ];

        assert_eq!(snapshots.len(), 3);
        assert_eq!(snapshots[0].collector_id, "collector-1");
        assert_eq!(snapshots[1].collector_id, "collector-2");
        assert_eq!(snapshots[2].collector_id, "collector-3");
    }
}
