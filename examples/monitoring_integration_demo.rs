//! Comprehensive Monitoring Integration Demo
//!
//! This example demonstrates how to integrate all of NestGate's monitoring
//! and observability features for a production deployment.

use nestgate_core::{
    canonical_types::StorageTier,
    monitoring::{
        health_checks::{HealthCheckManager, HealthStatus},
        metrics::{MetricsCollector, MetricsExporter},
    },
    performance_monitor::PerformanceMonitor as performance,
    universal_storage::{
        backends::{FilesystemBackend, ObjectStorageBackend},
        storage_types::StorageTier,
    },
    Result,
};

// Local type definitions for missing types
#[derive(Debug, Clone)]
pub struct ExportFormat;
#[derive(Debug, Clone)]
pub struct HealthCheckConfig;
#[derive(Debug, Clone)]
pub struct MetricsConfig;
use std::sync::Arc;
use std::time::Duration;
use tokio;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::init();

    info!("🚀 Starting NestGate with comprehensive monitoring");

    // 1. Setup Metrics Collection
    let metrics_config = MetricsConfig {
        collection_interval: Duration::from_secs(30),
        retention_period: Duration::from_secs(3600),
        export_enabled: true,
        export_endpoints: vec![
            "http://localhost:9090/metrics".to_string(), // Prometheus
            "http://localhost:8086/write".to_string(),   // InfluxDB
        ],
        labels: {
            let mut labels = std::collections::HashMap::new();
            labels.insert("service".to_string(), "nestgate".to_string());
            labels.insert("environment".to_string(), "production".to_string());
            labels.insert("version".to_string(), "1.0.0".to_string());
            labels
        },
    };

    let metrics_collector = Arc::new(MetricsCollector::new(metrics_config));

    // 2. Setup Health Checks
    let health_config = HealthCheckConfig {
        check_interval: Duration::from_secs(30),
        check_timeout: Duration::from_secs(10),
        history_size: 20,
        deep_checks_enabled: true,
        custom_endpoints: vec!["http://localhost:8080/health".to_string()],
    };

    let health_manager = Arc::new(HealthCheckManager::new(health_config));

    // 3. Setup Connection Pooling
    let pool_config = ConnectionPoolConfig {
        max_connections: 50,
        min_connections: 10,
        max_idle_time: Duration::from_secs(300),
        connection_timeout: Duration::from_secs(30),
        cleanup_interval: Duration::from_secs(60),
    };

    let connection_pool = Arc::new(HttpConnectionPool::new_http_pool(pool_config));
    let pool_manager = Arc::new(ConnectionPoolManager::new());
    pool_manager
        .register_pool("http".to_string(), (*connection_pool).clone())
        .await;

    // 4. Setup Data Providers with Monitoring
    // let mut data_adapter = UniversalDataAdapter::new();

    // Register NCBI provider
    if let Ok(ncbi_provider) =
        create_monitored_ncbi_provider(&metrics_collector, &health_manager).await
    {
        data_adapter.register_provider(ncbi_provider);
        info!("✅ Registered NCBI provider with monitoring");
    }

    // 5. Setup Storage Backends with Monitoring
    let storage_backends =
        setup_monitored_storage_backends(&metrics_collector, &health_manager).await?;
    info!(
        "✅ Setup {} monitored storage backends",
        storage_backends.len()
    );

    // 6. Start Background Monitoring Tasks
    let metrics_task = metrics_collector.start_collection_task();
    let health_task = health_manager.start_health_checking();
    let pool_cleanup_task = connection_pool.start_cleanup_task();

    info!("📊 Started all monitoring background tasks");

    // 7. Setup Metrics Export
    let prometheus_exporter =
        MetricsExporter::new(Arc::clone(&metrics_collector), ExportFormat::Prometheus);
    let json_exporter = MetricsExporter::new(Arc::clone(&metrics_collector), ExportFormat::Json);

    // 8. Simulate Production Workload
    info!("🔄 Starting simulated production workload...");

    let workload_task = tokio::spawn({
        let data_adapter = Arc::new(data_adapter);
        let metrics_collector = Arc::clone(&metrics_collector);

        async move {
            for i in 0..100 {
                // Simulate data requests
                let request = nestgate_core::data_sources::DataRequest {
                    capability_type: "genome_data".to_string(),
                    parameters: {
                        let mut params = std::collections::HashMap::new();
                        params.insert(
                            "query".to_string(),
                            serde_json::json!(format!("test_query_{}", i)),
                        );
                        params
                    },
                    metadata: std::collections::HashMap::new(),
                };

                let start_time = std::time::Instant::now();

                match data_adapter.execute_request(&request).await {
                    Ok(_response) => {
                        metrics_collector.record_provider_success("ncbi");
                        // .await; // Method doesn't return Future

                        if i % 10 == 0 {
                            info!("✅ Completed {} data requests", i + 1);
                        }
                    }
                    Err(e) => {
                        metrics_collector.record_provider_failure("ncbi");
                        // .await; // Method doesn't return Future

                        warn!("❌ Data request {} failed: {}", i, e);
                    }
                }

                // Brief pause between requests
                tokio::time::sleep(Duration::from_millis(100)).await;
            }

            info!("🎯 Completed simulated workload");
        }
    });

    // 9. Periodic Monitoring Reports
    let reporting_task = tokio::spawn({
        let health_manager = Arc::clone(&health_manager);
        let metrics_collector = Arc::clone(&metrics_collector);
        let prometheus_exporter = Arc::new(prometheus_exporter);
        let json_exporter = Arc::new(json_exporter);

        async move {
            let mut report_interval = tokio::time::interval(Duration::from_secs(60));

            for report_num in 1..=5 {
                report_interval.tick().await;

                info!("📋 Generating monitoring report #{}", report_num);

                // Health Report
                let health_report = health_manager.generate_health_report().await;
                info!("🏥 Health Report:\n{}", health_report);

                // System Metrics
                let system_metrics = metrics_collector.get_system_metrics().await;
                info!(
                    "💻 System - CPU: {:.1}%, Memory: {} MB, Uptime: {:?}",
                    system_metrics.cpu_usage,
                    system_metrics.memory_usage / 1024 / 1024,
                    system_metrics.uptime
                );

                // Provider Metrics
                let provider_metrics = metrics_collector.get_all_provider_metrics().await;
                for (name, metrics) in provider_metrics {
                    info!(
                        "🔗 Provider {} - Requests: {}, Success Rate: {:.1}%, Avg Latency: {:.1}ms",
                        name,
                        "N/A", // metrics.total_requests,
                        "N/A", // metrics.success_rate(),
                        "N/A"  // metrics.avg_response_time_ms
                    );
                }

                // Performance Metrics
                let perf_metrics = metrics_collector.get_performance_metrics().await;
                info!(
                    "⚡ Performance - P95: {:.1}ms, P99: {:.1}ms, Throughput: {:.1} RPS",
                    perf_metrics.latency_p95, perf_metrics.latency_p99, perf_metrics.throughput_rps
                );

                // Export Metrics
                match prometheus_exporter.export().await {
                    Ok(prometheus_data) => {
                        info!(
                            "📤 Exported Prometheus metrics ({} bytes)",
                            prometheus_data.len()
                        );
                        // In production, you'd send this to Prometheus
                    }
                    Err(e) => error!("Failed to export Prometheus metrics: {}", e),
                }

                match json_exporter.export().await {
                    Ok(json_data) => {
                        info!("📤 Exported JSON metrics ({} bytes)", json_data.len());
                        // In production, you'd send this to your metrics backend
                    }
                    Err(e) => error!("Failed to export JSON metrics: {}", e),
                }

                info!("✅ Monitoring report #{} completed\n", report_num);
            }
        }
    });

    // 10. Wait for workload and monitoring to complete
    let _ = tokio::try_join!(workload_task, reporting_task).map_err(|e| {
        NestGateError::system_error(&format!("Task join failed: {}", e), "main", None)
    })?;

    info!("🎉 Monitoring integration demo completed successfully!");
    info!("📊 Final system health check...");

    // Final health check
    // let final_health = health_manager.check_all_components().await?;
    // Re-enable when health check is working
    // match final_health.overall_status {
    //     HealthStatus::Healthy => {
    //         info!("✅ All systems healthy - demo completed successfully!");
    //     }
    //     HealthStatus::Degraded { warnings } => {
    //         warn!("⚠️ Some systems degraded: {:?}", warnings);
    //     }
    //     HealthStatus::Unhealthy { errors } => {
    //         error!("❌ System unhealthy: {:?}", errors);
    //     }
    //     HealthStatus::Unknown => {
    //         warn!("❓ System status unknown");
    //     }
    // }

    info!("✅ Monitoring integration demo completed successfully!");

    // Cleanup background tasks
    metrics_task.abort();
    health_task.abort();
    pool_cleanup_task.abort();

    Ok(())
}

/// Create an NCBI provider with full monitoring integration
async fn create_monitored_ncbi_provider(
    metrics_collector: &Arc<MetricsCollector>,
    health_manager: &Arc<HealthCheckManager>,
) -> Result<Arc<dyn nestgate_core::data_sources::DataCapability>> {
    // Register provider for metrics tracking
    metrics_collector
        .register_provider("ncbi".to_string(), "genome_data".to_string())
        .await;

    // Create health checker for NCBI
    let ncbi_health_checker =
        GenericHealthChecker::new("ncbi".to_string(), "data_provider".to_string(), || {
            // Simulate NCBI health check
            if rand::random::<f64>() > 0.1 {
                // 90% healthy
                Ok(HealthStatus::Healthy)
            } else {
                Ok(HealthStatus::Degraded {
                    warnings: vec!["NCBI API rate limiting detected".to_string()],
                })
            }
        });

    health_manager
        .register_component(Arc::new(ncbi_health_checker))
        .await;

    // Create NCBI provider (using mock for demo)
    // In production, you'd use: NCBILiveProvider::create_from_env()?
    let mock_provider = create_mock_ncbi_provider();

    Ok(Arc::new(mock_provider))
}

/// Setup storage backends with monitoring
async fn setup_monitored_storage_backends(
    metrics_collector: &Arc<MetricsCollector>,
    health_manager: &Arc<HealthCheckManager>,
) -> Result<Vec<String>> {
    let mut backends = Vec::new();

    // 1. Filesystem Backend
    metrics_collector
        .register_storage_backend("filesystem".to_string(), "local_storage".to_string())
        .await;

    let fs_health_checker = GenericHealthChecker::new(
        "filesystem".to_string(),
        "storage_backend".to_string(),
        || {
            // Check if filesystem is accessible
            if std::path::Path::new("/tmp").exists() {
                Ok(HealthStatus::Healthy)
            } else {
                Ok(HealthStatus::Unhealthy {
                    errors: vec!["Filesystem not accessible".to_string()],
                })
            }
        },
    );

    health_manager
        .register_component(Arc::new(fs_health_checker))
        .await;
    backends.push("filesystem".to_string());

    // 2. Object Storage Backend (S3/MinIO)
    metrics_collector
        .register_storage_backend("s3".to_string(), "object_storage".to_string())
        .await;

    let s3_health_checker =
        GenericHealthChecker::new("s3".to_string(), "storage_backend".to_string(), || {
            // Simulate S3 health check
            if rand::random::<f64>() > 0.05 {
                // 95% healthy
                Ok(HealthStatus::Healthy)
            } else {
                Ok(HealthStatus::Degraded {
                    warnings: vec!["S3 high latency detected".to_string()],
                })
            }
        });

    health_manager
        .register_component(Arc::new(s3_health_checker))
        .await;
    backends.push("s3".to_string());

    Ok(backends)
}

/// Create a mock NCBI provider for demo purposes
fn create_mock_ncbi_provider() -> MockDataProvider {
    MockDataProvider {
        name: "ncbi".to_string(),
        provider_type: "genome_data".to_string(),
    }
}

/// Mock data provider for demo
struct MockDataProvider {
    name: String,
    provider_type: String,
}

// **CANONICAL MODERNIZATION**: Native async implementation
impl nestgate_core::data_sources::DataCapability for MockDataProvider {
    fn capability_type(&self) -> &str {
        &self.provider_type
    }

    fn can_handle(
        &self,
        request: &nestgate_core::data_sources::DataRequest,
    ) -> impl std::future::Future<Output = Result<bool>> + Send {
        let provider_type = self.provider_type.clone();
        async move { Ok(request.capability_type == provider_type) }
    }

    fn execute_request(
        &self,
        request: &nestgate_core::data_sources::DataRequest,
    ) -> impl std::future::Future<Output = Result<nestgate_core::data_sources::DataResponse>> + Send
    {
        let name = self.name.clone();
        let provider_type = self.provider_type.clone();
        let request_id = request.id.clone();

        async move {
            // Simulate processing time
            tokio::time::sleep(Duration::from_millis(50 + rand::random::<u64>() % 200)).await;

            Ok(nestgate_core::data_sources::DataResponse {
                id: request_id,
                provider: name,
                data: serde_json::json!({
                    "provider_type": provider_type,
                    "timestamp": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    "data": format!("Mock data from {}", provider_type)
                }),
                metadata: std::collections::HashMap::new(),
            })
        }
    }
}
