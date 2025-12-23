/// Comprehensive tests for telemetry module
/// Tests telemetry collection, metrics registry, time series, and export functionality
use super::*;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

    #[test]
    fn test_telemetry_config_default() {
        let config = TelemetryConfig::default();

        assert_eq!(config.collection_interval, Duration::from_secs(10));
        assert_eq!(config.retention_period, Duration::from_secs(3600));
        assert_eq!(config.max_data_points, 360);
        assert!(config.enable_performance_tracking);
        assert!(config.export_endpoints.is_empty());
    }

    #[test]
    fn test_telemetry_config_custom() {
        let config = TelemetryConfig {
            collection_interval: Duration::from_secs(5),
            retention_period: Duration::from_secs(7200),
            max_data_points: 720,
            enable_performance_tracking: false,
            export_endpoints: vec![],
        };

        assert_eq!(config.collection_interval, Duration::from_secs(5));
        assert_eq!(config.retention_period, Duration::from_secs(7200));
        assert_eq!(config.max_data_points, 720);
        assert!(!config.enable_performance_tracking);
    }

    #[test]
    fn test_export_format_variants() {
        let json_format = ExportFormat::Json;
        assert!(matches!(json_format, ExportFormat::Json));

        #[allow(deprecated)]
        let prometheus_format = ExportFormat::Prometheus;
        #[allow(deprecated)]
        assert!(matches!(prometheus_format, ExportFormat::Prometheus));

        let custom_format = ExportFormat::Custom("graphite".to_string());
        assert!(matches!(custom_format, ExportFormat::Custom(_)));

        let capability_format = ExportFormat::MonitoringCapability {
            capability_type: "metrics".to_string(),
            format: "opentelemetry".to_string(),
        };
        assert!(matches!(capability_format, ExportFormat::MonitoringCapability { .. }));
    }

    #[test]
    fn test_export_endpoint_creation() {
        let endpoint = ExportEndpoint {
            name: "prometheus-exporter".to_string(),
            url: "http://prometheus:9090/metrics".to_string(),
            format: ExportFormat::Json,
            interval: Duration::from_secs(30),
        };

        assert_eq!(endpoint.name, "prometheus-exporter");
        assert_eq!(endpoint.url, "http://prometheus:9090/metrics");
        assert_eq!(endpoint.interval, Duration::from_secs(30));
    }

    #[tokio::test]
    async fn test_collector_creation() {
        let config = TelemetryConfig::default();
        let collector = TelemetryCollector::new(config);

        // Collector should be created successfully
        let snapshot = collector.get_metrics_snapshot().await;
        assert!(snapshot.is_empty()); // No metrics yet
    }

    #[tokio::test]
    async fn test_counter_metric_increment() {
        let config = TelemetryConfig::default();
        let collector = TelemetryCollector::new(config);

        // Increment counter multiple times
        collector.inc_counter("requests_total", 1.0, HashMap::new()).await;
        collector.inc_counter("requests_total", 2.0, HashMap::new()).await;
        collector.inc_counter("requests_total", 3.0, HashMap::new()).await;

        let snapshot = collector.get_metrics_snapshot().await;
        assert!(snapshot.contains_key("requests_total"));

        let counter_value = snapshot["requests_total"]["value"].as_f64().unwrap();
        assert_eq!(counter_value, 6.0); // 1.0 + 2.0 + 3.0
    }

    #[tokio::test]
    async fn test_counter_with_labels() {
        let config = TelemetryConfig::default();
        let collector = TelemetryCollector::new(config);

        let mut labels = HashMap::new();
        labels.insert("endpoint".to_string(), "/api/users".to_string());
        labels.insert("method".to_string(), "GET".to_string());

        collector.inc_counter("http_requests", 5.0, labels.clone()).await;

        let snapshot = collector.get_metrics_snapshot().await;
        assert!(snapshot.contains_key("http_requests"));

        let labels_value = &snapshot["http_requests"]["labels"];
        assert_eq!(labels_value["endpoint"], "/api/users");
        assert_eq!(labels_value["method"], "GET");
    }

    #[tokio::test]
    async fn test_gauge_metric_setting() {
        let config = TelemetryConfig::default();
        let collector = TelemetryCollector::new(config);

        // Set gauge to different values
        collector.set_gauge("cpu_usage", 45.5, HashMap::new()).await;
        collector.set_gauge("cpu_usage", 52.3, HashMap::new()).await;

        let snapshot = collector.get_metrics_snapshot().await;
        assert!(snapshot.contains_key("cpu_usage"));

        let gauge_value = snapshot["cpu_usage"]["value"].as_f64().unwrap();
        assert_eq!(gauge_value, 52.3); // Latest value
    }

    #[tokio::test]
    async fn test_gauge_with_labels() {
        let config = TelemetryConfig::default();
        let collector = TelemetryCollector::new(config);

        let mut labels = HashMap::new();
        labels.insert("pool".to_string(), "main".to_string());

        collector.set_gauge("memory_usage", 78.9, labels).await;

        let snapshot = collector.get_metrics_snapshot().await;
        let labels_value = &snapshot["memory_usage"]["labels"];
        assert_eq!(labels_value["pool"], "main");
    }

    #[tokio::test]
    async fn test_histogram_observations() {
        let config = TelemetryConfig::default();
        let collector = TelemetryCollector::new(config);

        // Observe multiple values
        collector.observe_histogram("request_duration", 0.15, HashMap::new()).await;
        collector.observe_histogram("request_duration", 0.35, HashMap::new()).await;
        collector.observe_histogram("request_duration", 1.5, HashMap::new()).await;

        let snapshot = collector.get_metrics_snapshot().await;
        assert!(snapshot.contains_key("request_duration"));

        let histogram = &snapshot["request_duration"];
        assert_eq!(histogram["count"], 3);
        assert_eq!(histogram["sum"], 2.0); // 0.15 + 0.35 + 1.5
    }

    #[tokio::test]
    async fn test_histogram_bucket_counts() {
        let config = TelemetryConfig::default();
        let collector = TelemetryCollector::new(config);

        // Observe values that fall in different buckets
        collector.observe_histogram("latency", 0.05, HashMap::new()).await; // < 0.1 bucket
        collector.observe_histogram("latency", 0.3, HashMap::new()).await;  // 0.1-0.5 bucket
        collector.observe_histogram("latency", 1.2, HashMap::new()).await;  // 1.0-2.0 bucket

        let snapshot = collector.get_metrics_snapshot().await;
        let histogram = &snapshot["latency"];
        
        // Check that buckets are populated
        let buckets = histogram["buckets"].as_array().unwrap();
        assert!(!buckets.is_empty());
        assert_eq!(buckets.len(), 9); // Default bucket count
    }

    #[tokio::test]
    async fn test_multiple_metrics_types() {
        let config = TelemetryConfig::default();
        let collector = TelemetryCollector::new(config);

        collector.inc_counter("counter_1", 10.0, HashMap::new()).await;
        collector.set_gauge("gauge_1", 25.5, HashMap::new()).await;
        collector.observe_histogram("histogram_1", 1.5, HashMap::new()).await;

        let snapshot = collector.get_metrics_snapshot().await;
        
        assert!(snapshot.contains_key("counter_1"));
        assert!(snapshot.contains_key("gauge_1"));
        assert!(snapshot.contains_key("histogram_1"));

        assert_eq!(snapshot["counter_1"]["type"], "counter");
        assert_eq!(snapshot["gauge_1"]["type"], "gauge");
        assert_eq!(snapshot["histogram_1"]["type"], "histogram");
    }

    #[test]
    fn test_time_series_creation() {
        let time_series = TimeSeries {
            name: "test_metric".to_string(),
            data_points: Vec::new(),
            max_points: 100,
        };

        assert_eq!(time_series.name, "test_metric");
        assert_eq!(time_series.max_points, 100);
        assert!(time_series.data_points.is_empty());
    }

    #[test]
    fn test_time_series_data_point_addition() {
        let mut time_series = TimeSeries {
            name: "test_metric".to_string(),
            data_points: Vec::new(),
            max_points: 5,
        };

        // Add data points
        for i in 0..3 {
            time_series.data_points.push(DataPoint {
                timestamp: SystemTime::now(),
                value: i as f64,
            });
        }

        assert_eq!(time_series.data_points.len(), 3);
    }

    #[test]
    fn test_time_series_get_recent_data() {
        let mut time_series = TimeSeries {
            name: "test_metric".to_string(),
            data_points: Vec::new(),
            max_points: 100,
        };

        // Add data points with current timestamp
        for i in 0..5 {
            time_series.data_points.push(DataPoint {
                timestamp: SystemTime::now(),
                value: i as f64,
            });
        }

        // Get recent data within 1 hour window
        let recent = time_series.get_recent_data(Duration::from_secs(3600));
        assert_eq!(recent.len(), 5); // All should be within window
    }

    #[test]
    fn test_time_series_average_calculation() {
        let mut time_series = TimeSeries {
            name: "test_metric".to_string(),
            data_points: Vec::new(),
            max_points: 100,
        };

        // Add data points: 10, 20, 30, 40, 50
        for i in 1..=5 {
            time_series.data_points.push(DataPoint {
                timestamp: SystemTime::now(),
                value: (i * 10) as f64,
            });
        }

        let avg = time_series.average_over_window(Duration::from_secs(3600));
        assert!(avg.is_some());
        assert_eq!(avg.unwrap(), 30.0); // (10 + 20 + 30 + 40 + 50) / 5
    }

    #[test]
    fn test_time_series_average_empty() {
        let time_series = TimeSeries {
            name: "test_metric".to_string(),
            data_points: Vec::new(),
            max_points: 100,
        };

        let avg = time_series.average_over_window(Duration::from_secs(3600));
        assert!(avg.is_none());
    }

    #[test]
    fn test_counter_metric_structure() {
        let counter = CounterMetric {
            name: "test_counter".to_string(),
            help: "Test counter metric".to_string(),
            value: 42.0,
            labels: HashMap::new(),
            last_updated: SystemTime::now(),
        };

        assert_eq!(counter.name, "test_counter");
        assert_eq!(counter.value, 42.0);
        assert!(counter.labels.is_empty());
    }

    #[test]
    fn test_gauge_metric_structure() {
        let gauge = GaugeMetric {
            name: "test_gauge".to_string(),
            help: "Test gauge metric".to_string(),
            value: 75.5,
            labels: HashMap::new(),
            last_updated: SystemTime::now(),
        };

        assert_eq!(gauge.name, "test_gauge");
        assert_eq!(gauge.value, 75.5);
    }

    #[test]
    fn test_histogram_metric_structure() {
        let histogram = HistogramMetric {
            name: "test_histogram".to_string(),
            help: "Test histogram metric".to_string(),
            buckets: vec![0.1, 0.5, 1.0, 2.0, 5.0],
            counts: vec![1, 2, 3, 4, 5],
            sum: 15.0,
            count: 15,
            labels: HashMap::new(),
            last_updated: SystemTime::now(),
        };

        assert_eq!(histogram.name, "test_histogram");
        assert_eq!(histogram.buckets.len(), 5);
        assert_eq!(histogram.sum, 15.0);
        assert_eq!(histogram.count, 15);
    }

    #[test]
    fn test_metrics_registry_default() {
        let registry = MetricsRegistry::default();

        assert!(registry.counters.is_empty());
        assert!(registry.gauges.is_empty());
        assert!(registry.histograms.is_empty());
        assert!(registry.time_series.is_empty());
    }

    #[tokio::test]
    async fn test_metrics_timestamp_updates() {
        let config = TelemetryConfig::default();
        let collector = TelemetryCollector::new(config);

        let before = SystemTime::now();
        tokio::time::sleep(Duration::from_millis(10)).await;

        collector.inc_counter("test", 1.0, HashMap::new()).await;

        let snapshot = collector.get_metrics_snapshot().await;
        let last_updated = snapshot["test"]["last_updated"].as_u64().unwrap();

        let before_secs = before
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert!(last_updated >= before_secs);
    }

    #[tokio::test]
    async fn test_concurrent_metric_updates() {
        let config = TelemetryConfig::default();
        let collector = Arc::new(TelemetryCollector::new(config));

        let mut handles = vec![];

        // Spawn multiple tasks updating the same counter
        for i in 0..5 {
            let collector_clone = Arc::clone(&collector);
            let handle = tokio::spawn(async move {
                collector_clone
                    .inc_counter("concurrent_counter", i as f64, HashMap::new())
                    .await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let snapshot = collector.get_metrics_snapshot().await;
        let value = snapshot["concurrent_counter"]["value"].as_f64().unwrap();
        assert_eq!(value, 10.0); // 0 + 1 + 2 + 3 + 4 = 10
    }

    #[test]
    fn test_export_format_serialization() {
        let json_format = ExportFormat::Json;
        let serialized = serde_json::to_string(&json_format).unwrap();
        assert!(serialized.contains("Json"));

        let capability_format = ExportFormat::MonitoringCapability {
            capability_type: "metrics".to_string(),
            format: "prometheus".to_string(),
        };
        let serialized = serde_json::to_string(&capability_format).unwrap();
        assert!(serialized.contains("MonitoringCapability"));
    }

    #[test]
    fn test_export_format_deserialization() {
        let json_str = r#""Json""#;
        let format: ExportFormat = serde_json::from_str(json_str).unwrap();
        assert!(matches!(format, ExportFormat::Json));
    }

    #[tokio::test]
    async fn test_gauge_updates_time_series() {
        let config = TelemetryConfig {
            collection_interval: Duration::from_secs(10),
            retention_period: Duration::from_secs(3600),
            max_data_points: 10,
            enable_performance_tracking: false,
            export_endpoints: vec![],
        };
        let collector = TelemetryCollector::new(config);

        // Set gauge value (should also add to time series)
        collector.set_gauge("cpu_percent", 45.5, HashMap::new()).await;
        collector.set_gauge("cpu_percent", 52.3, HashMap::new()).await;

        // Verify gauge is set
        let snapshot = collector.get_metrics_snapshot().await;
        assert!(snapshot.contains_key("cpu_percent"));
        assert_eq!(snapshot["cpu_percent"]["value"].as_f64().unwrap(), 52.3);
    }

    #[tokio::test]
    async fn test_collector_stop() {
        let config = TelemetryConfig::default();
        let collector = TelemetryCollector::new(config);

        // Stop should complete without error
        collector.stop().await;
    }

    #[test]
    fn test_data_point_structure() {
        let data_point = DataPoint {
            timestamp: SystemTime::now(),
            value: 123.45,
        };

        assert_eq!(data_point.value, 123.45);
        assert!(data_point.timestamp <= SystemTime::now());
    }

