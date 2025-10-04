use std::collections::HashMap;
//
// Provides comprehensive system metrics collection and analysis.

use crate::error::Result;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

/// Performance metrics structure
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Timestamp when metrics were collected
    pub timestamp: SystemTime,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Available memory in bytes
    pub memory_available: u64,
    /// Disk I/O operations per second
    pub disk_iops: f64,
    /// Network bytes per second
    pub network_bytes_per_sec: f64,
    /// Custom application metrics
    pub custom_metrics: HashMap<String, f64>,
}
impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            cpu_usage: 0.0,
            memory_usage: 0,
            memory_available: 0,
            disk_iops: 0.0,
            network_bytes_per_sec: 0.0,
            custom_metrics: HashMap::new(),
        }
    }
}

/// Metrics registry for collecting and storing performance data
#[derive(Debug)]
pub struct MetricsRegistry {
    metrics_history: Arc<RwLock<Vec<PerformanceMetrics>>>,
    max_history: usize,
    custom_metrics: Arc<RwLock<crate::canonical_modernization::unified_types::CustomMetricsMap>>,
}
impl Default for MetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsRegistry {
    /// Create a new metrics registry
    #[must_use]
    pub fn new() -> Self {
        Self {
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            max_history: 100, // Default max history
            custom_metrics: Arc::new(RwLock::new(HashMap::new())),
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
    pub async fn collect_system_metrics(&self) -> Result<()> {
        let metrics = self.gather_system_metrics().await?;

        let mut history = self.metrics_history.write().await;
        history.push(metrics);

        // Keep only the last max_history entries
        if history.len() > self.max_history {
            history.remove(0);
        }

        Ok(())
    }

    /// Get current metrics snapshot
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_current_metrics(&self) -> Result<PerformanceMetrics> {
        self.gather_system_metrics().await
    }

    /// Record a custom metric
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn record_custom_metric(&self, name: &str, value: f64) -> Result<()> {
        let mut custom = self.custom_metrics.write().await;
        custom.insert(
            name.to_string(),
            crate::canonical_modernization::unified_types::MetricValue::Gauge(value),
        );

        tracing::debug!("Recorded custom metric: {} = {}", name, value);
        Ok(())
    }

    /// Get metrics history for a specific duration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_metrics_history(&self, duration: Duration) -> Result<Vec<PerformanceMetrics>> {
        let history = self.metrics_history.read().await;
        let cutoff_time = SystemTime::now() - duration;

        let filtered: Vec<PerformanceMetrics> = history
            .iter()
            .filter(|metrics| metrics.timestamp >= cutoff_time)
            .cloned()
            .collect();

        Ok(filtered)
    }

    /// Gather system metrics from the OS
    async fn gather_system_metrics(&self) -> Result<PerformanceMetrics> {
        let custom = self.custom_metrics.read().await;

        // In a real implementation, this would collect actual system metrics
        // For now, return mock data
        Ok(PerformanceMetrics {
            timestamp: SystemTime::now(),
            cpu_usage: 25.0,                          // Mock CPU usage
            memory_usage: 1024 * 1024 * 512,          // Mock 512MB used
            memory_available: 1024 * 1024 * 1024 * 2, // Mock 2GB available
            disk_iops: 100.0,
            network_bytes_per_sec: 1024.0 * 1024.0, // Mock 1MB/s
            custom_metrics: custom.iter().map(|(k, v)| {
                (k.clone(), match v {
                    crate::canonical_modernization::unified_types::MetricValue::Gauge(val) => *val,
                    crate::canonical_modernization::unified_types::MetricValue::Counter(val) => *val as f64,
                    crate::canonical_modernization::unified_types::MetricValue::Histogram(val) => val.iter().sum::<f64>() / (val.len() as f64),
                    crate::canonical_modernization::unified_types::MetricValue::Summary { sum, count: _ } => *sum,
                    crate::canonical_modernization::unified_types::MetricValue::String(_) => 0.0,
                })
            }).collect(),
        })
    }
}

/// Metrics collector interface
pub trait MetricsCollector {
    /// Collect metrics from this component
    fn collect_metrics(&self) -> HashMap<String, f64>;
    /// Get component name for metrics labeling
    fn component_name(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_registry() -> crate::Result<()> {
        let registry = MetricsRegistry::new();

        // Should be able to collect metrics
        assert!(registry.collect_system_metrics().await.is_ok());

        // Should be able to get current metrics
        let metrics = registry.get_current_metrics().await?;
        assert!(metrics.cpu_usage >= 0.0);
        assert!(metrics.memory_usage > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_custom_metrics() -> crate::Result<()> {
        let registry = MetricsRegistry::new();

        // Record custom metric
        let _tags: std::collections::HashMap<String, String> = HashMap::new();
        assert!(registry
            .record_custom_metric("test_metric", 42.0)
            .await
            .is_ok());

        // Should appear in current metrics
        let metrics = registry.get_current_metrics().await?;
        assert_eq!(metrics.custom_metrics.get("test_metric"), Some(&42.0));
        Ok(())
    }

    #[tokio::test]
    async fn test_metrics_history() -> crate::Result<()> {
        let registry = MetricsRegistry::new();

        // Collect a few metrics
        registry.collect_system_metrics().await?;
        registry.collect_system_metrics().await?;

        // Should have history
        let history = registry
            .get_metrics_history(Duration::from_secs(60))
            .await?;
        assert!(!history.is_empty());
        Ok(())
    }
}
