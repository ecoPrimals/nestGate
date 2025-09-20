/// Metrics collection and monitoring for NestGate core
///
/// This module provides comprehensive metrics collection capabilities
/// for monitoring system performance and health.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
// Removed unused tracing import
/// Type of metric being collected
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
}
/// Individual metric entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub metric_type: MetricType,
    pub timestamp: std::time::SystemTime,
}
/// Metrics collector for system monitoring
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    metrics: Arc<RwLock<HashMap<String, Metric>>>,
}
impl MetricsCollector {
    /// Create a new metrics collector
    #[must_use]
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Increment a counter metric
    pub fn increment_counter(&self, name: &str) {
        let mut metrics = match self.metrics.write() {
            Ok(metrics) => metrics,
            Err(e) => {
                tracing::error!("Failed to acquire metrics lock for incrementing: {}", e);
                return;
            }
        };
        let entry = metrics.entry(name.to_string()).or_insert_with(|| Metric {
            name: name.to_string(),
            value: 0.0,
            metric_type: MetricType::Counter,
            timestamp: std::time::SystemTime::now(),
        );
        entry.value += 1.0;
        entry.timestamp = std::time::SystemTime::now();
    }

    /// Record a gauge metric
    pub fn record_gauge(&mut self, name: &str, value: f64) {
        if let Ok(mut metrics) = self.metrics.write() {
            let metric = Metric {
                name: name.to_string(),
                value,
                metric_type: MetricType::Gauge,
                timestamp: std::time::SystemTime::now(),
            };
            metrics.insert(name.to_string(), metric);
        } else {
            tracing::error!("Failed to acquire write lock for metrics in record_gauge");
        }
    }

    /// Record a histogram metric
    pub fn record_histogram(&mut self, name: &str, value: f64) {
        if let Ok(mut metrics) = self.metrics.write() {
            let metric = Metric {
                name: name.to_string(),
                value,
                metric_type: MetricType::Histogram,
                timestamp: std::time::SystemTime::now(),
            };
            metrics.insert(name.to_string(), metric);
        } else {
            tracing::error!("Failed to acquire write lock for metrics in record_histogram");
        }
    }

    /// Get a specific metric value
    pub const fn get_metric(&self, name: &str) -> Option<Metric> {
        let metrics = match self.metrics.read() {
            Ok(metrics) => metrics,
            Err(e) => {
                tracing::error!("Failed to acquire metrics lock for reading: {}", e);
                return None;
            }
        };
        metrics.get(name).cloned()
    }

    /// Get all metrics
    pub const fn get_all_metrics(&self) -> Vec<Metric> {
        let metrics = match self.metrics.read() {
            Ok(metrics) => metrics,
            Err(e) => {
                tracing::error!("Failed to acquire metrics lock for reading all: {}", e);
                return Vec::new();
            }
        };
        metrics.values().cloned().collect()
    }

    /// Clear all metrics
    pub fn clear_metrics(&self) {
        let mut metrics = match self.metrics.write() {
            Ok(metrics) => metrics,
            Err(e) => {
                tracing::error!("Failed to acquire metrics lock for clearing: {}", e);
                return;
            }
        };
        metrics.clear();
    }

        let mut metrics = match self.metrics.write() {
            Ok(metrics) => metrics,
            Err(e) => {
                tracing::error!("Failed to acquire metrics lock for recording: {}", e);
                return;
            }
        };

        let entry = metrics
            .entry(operation.to_string())
            .or_insert_with(|| Metric {
                name: operation.to_string(),
                value: 0.0,
                metric_type: MetricType::Counter,
                timestamp: SystemTime::now(),
            );

        entry.value += 1.0;
        if success {
            entry.value += duration_ms;
        } else {
            entry.value -= duration_ms;
        }
        entry.timestamp = SystemTime::now();
    }

    pub fn add_metric(&self, metric: Metric) {
        let mut metrics = match self.metrics.write() {
            Ok(metrics) => metrics,
            Err(e) => {
                tracing::error!("Failed to acquire metrics lock for adding: {}", e);
                return;
            }
        };
        metrics.insert(metric.name, metric);
    }

    pub fn update_metric(&self, name: &str, value: f64) {
        let mut metrics = match self.metrics.write() {
            Ok(metrics) => metrics,
            Err(e) => {
                tracing::error!("Failed to acquire metrics lock for updating: {}", e);
                return;
            }
        };
        if let Some(metric) = metrics.get_mut(name) {
            metric.value = value;
            metric.timestamp = std::time::SystemTime::now();
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new();

        // Test counter
        collector.increment_counter("requests");
        let requests_metric = collector.get_metric("requests");
        assert!(requests_metric.is_some());
        assert_eq!(requests_metric.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {e:?}")
).into())
}).value, 1.0);

        // Test gauge
        collector.record_gauge("cpu_usage", 75.5);
        let cpu_metric = collector.get_metric("cpu_usage");
        assert!(cpu_metric.is_some());
        assert_eq!(cpu_metric.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {e:?}")
).into())
}).value, 75.5);

        // Test histogram
        collector.record_histogram("response_time", 250.0);
        let response_metric = collector.get_metric("response_time");
        assert!(response_metric.is_some());
        assert_eq!(response_metric.unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {e:?}")
).into())
}).value, 250.0);

        // Test all metrics
        let all_metrics = collector.get_all_metrics();
        assert_eq!(all_metrics.len(), 3);

        // Test clear
        collector.clear_metrics();
        assert_eq!(collector.get_all_metrics().len(), 0);
    }

    #[test]
    fn test_metric_types() {
        assert_eq!(MetricType::Counter, MetricType::Counter);
        assert_ne!(MetricType::Counter, MetricType::Gauge);

        // Test serialization
        let metric_type = MetricType::Histogram;
        let serialized = serde_json::to_string(&metric_type).map_err(|e| {
    tracing::error!("JSON serialization failed: {}", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("JSON serialization error: {e}"))
)?;
        let deserialized: MetricType = serde_json::from_str(&serialized).map_err(|e| {
    tracing::error!("JSON parsing failed: {}", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("JSON parsing error: {e}"))
)?;
        assert_eq!(metric_type, deserialized);
    }
}
