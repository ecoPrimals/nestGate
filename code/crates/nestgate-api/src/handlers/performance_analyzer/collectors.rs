//! **DATA COLLECTION COMPONENTS**
//!
//! Components for collecting performance data from various sources.

use super::metrics::{MetricsError, SystemMetrics, SystemMetricsCollector};
use std::sync::Arc;
use tokio::time::{interval, Duration};

/// Data collection coordinator
#[derive(Debug)]
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
        for i in 0..batch_size {
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
