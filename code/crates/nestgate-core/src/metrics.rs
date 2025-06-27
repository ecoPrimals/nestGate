//! Metrics collection and monitoring for NestGate core
//! 
//! This module provides comprehensive metrics collection capabilities
//! for monitoring system performance and health.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};

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
#[derive(Debug)]
pub struct MetricsCollector {
    metrics: Arc<RwLock<HashMap<String, Metric>>>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Increment a counter metric
    pub fn increment_counter(&mut self, name: &str, value: f64) {
        let mut metrics = self.metrics.write().unwrap();
        let metric = Metric {
            name: name.to_string(),
            value,
            metric_type: MetricType::Counter,
            timestamp: std::time::SystemTime::now(),
        };
        metrics.insert(name.to_string(), metric);
    }
    
    /// Record a gauge metric
    pub fn record_gauge(&mut self, name: &str, value: f64) {
        let mut metrics = self.metrics.write().unwrap();
        let metric = Metric {
            name: name.to_string(),
            value,
            metric_type: MetricType::Gauge,
            timestamp: std::time::SystemTime::now(),
        };
        metrics.insert(name.to_string(), metric);
    }
    
    /// Record a histogram metric
    pub fn record_histogram(&mut self, name: &str, value: f64) {
        let mut metrics = self.metrics.write().unwrap();
        let metric = Metric {
            name: name.to_string(),
            value,
            metric_type: MetricType::Histogram,
            timestamp: std::time::SystemTime::now(),
        };
        metrics.insert(name.to_string(), metric);
    }
    
    /// Get a specific metric value
    pub fn get_metric(&self, name: &str) -> Option<f64> {
        let metrics = self.metrics.read().unwrap();
        metrics.get(name).map(|m| m.value)
    }
    
    /// Get all metrics
    pub fn get_all_metrics(&self) -> HashMap<String, Metric> {
        let metrics = self.metrics.read().unwrap();
        metrics.clone()
    }
    
    /// Clear all metrics
    pub fn clear(&mut self) {
        let mut metrics = self.metrics.write().unwrap();
        metrics.clear();
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
        collector.increment_counter("requests", 10.0);
        assert_eq!(collector.get_metric("requests"), Some(10.0));
        
        // Test gauge
        collector.record_gauge("cpu_usage", 75.5);
        assert_eq!(collector.get_metric("cpu_usage"), Some(75.5));
        
        // Test histogram
        collector.record_histogram("response_time", 250.0);
        assert_eq!(collector.get_metric("response_time"), Some(250.0));
        
        // Test all metrics
        let all_metrics = collector.get_all_metrics();
        assert_eq!(all_metrics.len(), 3);
        
        // Test clear
        collector.clear();
        assert_eq!(collector.get_all_metrics().len(), 0);
    }
    
    #[test]
    fn test_metric_types() {
        assert_eq!(MetricType::Counter, MetricType::Counter);
        assert_ne!(MetricType::Counter, MetricType::Gauge);
        
        // Test serialization
        let metric_type = MetricType::Histogram;
        let serialized = serde_json::to_string(&metric_type).unwrap();
        let deserialized: MetricType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(metric_type, deserialized);
    }
} 