// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for observability metrics module
//! Added: November 14, 2025 - Coverage Sprint

#[cfg(test)]
mod observability_metrics_tests {
    use crate::observability::metrics::*;
    use std::sync::Arc;

    #[test]
    fn test_metrics_registry_creation() {
        let registry = MetricsRegistry::new();
        assert!(Arc::strong_count(&registry.metrics) >= 1);
    }

    #[test]
    fn test_increment_counter() {
        let registry = MetricsRegistry::new();
        registry.increment_counter("test_counter");
        registry.increment_counter("test_counter");
        
        let value = registry.get_counter("test_counter");
        assert_eq!(value, 2);
    }

    #[test]
    fn test_record_gauge() {
        let registry = MetricsRegistry::new();
        registry.record_gauge("test_gauge", 42.5);
        registry.record_gauge("test_gauge", 100.0);
        
        let value = registry.get_gauge("test_gauge");
        assert_eq!(value, 100.0);
    }

    #[test]
    fn test_record_histogram() {
        let registry = MetricsRegistry::new();
        registry.record_histogram("test_histogram", 10);
        registry.record_histogram("test_histogram", 20);
        registry.record_histogram("test_histogram", 30);
        
        let values = registry.get_histogram("test_histogram");
        assert_eq!(values.len(), 3);
        assert!(values.contains(&10));
        assert!(values.contains(&20));
        assert!(values.contains(&30));
    }

    #[test]
    fn test_multiple_metrics() {
        let registry = MetricsRegistry::new();
        
        registry.increment_counter("requests");
        registry.increment_counter("requests");
        registry.record_gauge("cpu_usage", 75.5);
        registry.record_histogram("latency", 100);
        
        assert_eq!(registry.get_counter("requests"), 2);
        assert_eq!(registry.get_gauge("cpu_usage"), 75.5);
        assert_eq!(registry.get_histogram("latency").len(), 1);
    }

    #[test]
    fn test_metric_not_found_returns_default() {
        let registry = MetricsRegistry::new();
        
        // Non-existent metrics should return defaults
        assert_eq!(registry.get_counter("nonexistent"), 0);
        assert_eq!(registry.get_gauge("nonexistent"), 0.0);
        assert!(registry.get_histogram("nonexistent").is_empty());
    }

    #[test]
    fn test_metric_names_are_case_sensitive() {
        let registry = MetricsRegistry::new();
        
        registry.increment_counter("TestCounter");
        registry.increment_counter("testcounter");
        
        assert_eq!(registry.get_counter("TestCounter"), 1);
        assert_eq!(registry.get_counter("testcounter"), 1);
    }

    #[test]
    fn test_concurrent_metric_updates() {
        use std::thread;
        
        let registry = Arc::new(MetricsRegistry::new());
        let mut handles = vec![];
        
        for _ in 0..10 {
            let reg = Arc::clone(&registry);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    reg.increment_counter("concurrent_counter");
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Should have incremented 10 * 100 = 1000 times
        assert_eq!(registry.get_counter("concurrent_counter"), 1000);
    }

    #[test]
    fn test_gauge_overwrite_behavior() {
        let registry = MetricsRegistry::new();
        
        registry.record_gauge("temperature", 20.0);
        registry.record_gauge("temperature", 25.0);
        registry.record_gauge("temperature", 22.5);
        
        // Should only have the last value
        assert_eq!(registry.get_gauge("temperature"), 22.5);
    }

    #[test]
    fn test_histogram_accumulation() {
        let registry = MetricsRegistry::new();
        
        for i in 1..=10 {
            registry.record_histogram("response_times", i * 10);
        }
        
        let values = registry.get_histogram("response_times");
        assert_eq!(values.len(), 10);
        
        // Verify all values are present
        for i in 1..=10 {
            assert!(values.contains(&(i * 10)));
        }
    }

    #[test]
    fn test_metrics_reset() {
        let registry = MetricsRegistry::new();
        
        registry.increment_counter("test");
        registry.record_gauge("test", 50.0);
        registry.record_histogram("test", 100);
        
        registry.reset();
        
        assert_eq!(registry.get_counter("test"), 0);
        assert_eq!(registry.get_gauge("test"), 0.0);
        assert!(registry.get_histogram("test").is_empty());
    }

    #[test]
    fn test_metrics_snapshot() {
        let registry = MetricsRegistry::new();
        
        registry.increment_counter("requests");
        registry.record_gauge("memory", 1024.0);
        
        let snapshot = registry.snapshot();
        
        assert!(snapshot.contains_key("requests"));
        assert!(snapshot.contains_key("memory"));
    }
}

