/*!
 * Metrics collection and monitoring for the Port Manager
 */

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tokio::time::interval;

use crate::errors::Result;
use crate::service::{ServiceInstance, ServiceStatus};

/// System metrics
#[derive(Debug, Clone, Serialize)]
pub struct SystemMetrics {
    /// Timestamp when metrics were collected
    pub timestamp: u64,
    
    /// Total number of services
    pub total_services: usize,
    
    /// Number of running services
    pub running_services: usize,
    
    /// Number of failed services
    pub failed_services: usize,
    
    /// Total allocated ports
    pub allocated_ports: usize,
    
    /// Total active processes
    pub active_processes: usize,
    
    /// System uptime in seconds
    pub uptime_seconds: u64,
    
    /// Memory usage in bytes
    pub memory_usage: u64,
    
    /// CPU usage percentage
    pub cpu_usage: f64,
    
    /// Disk usage percentage
    pub disk_usage: f64,
    
    /// Network I/O statistics
    pub network_io: NetworkIoMetrics,
}

/// Network I/O metrics
#[derive(Debug, Clone, Serialize)]
pub struct NetworkIoMetrics {
    /// Bytes received
    pub bytes_received: u64,
    
    /// Bytes transmitted
    pub bytes_transmitted: u64,
    
    /// Packets received
    pub packets_received: u64,
    
    /// Packets transmitted
    pub packets_transmitted: u64,
}

/// Service metrics
#[derive(Debug, Clone, Serialize)]
pub struct ServiceMetrics {
    /// Service ID
    pub service_id: String,
    
    /// Service name
    pub service_name: String,
    
    /// Current status
    pub status: String,
    
    /// Uptime in seconds
    pub uptime_seconds: u64,
    
    /// Number of restarts
    pub restart_count: u32,
    
    /// Memory usage in bytes
    pub memory_usage: u64,
    
    /// CPU usage percentage
    pub cpu_usage: f64,
    
    /// Response time in milliseconds (for web services)
    pub response_time_ms: Option<f64>,
    
    /// Error count in the last hour
    pub error_count: u32,
    
    /// Health status
    pub health_status: String,
    
    /// Last health check timestamp
    pub last_health_check: Option<u64>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize)]
pub struct PerformanceMetrics {
    /// Timestamp
    pub timestamp: u64,
    
    /// Service start time in milliseconds
    pub service_start_time_ms: f64,
    
    /// Service stop time in milliseconds
    pub service_stop_time_ms: f64,
    
    /// Port allocation time in milliseconds
    pub port_allocation_time_ms: f64,
    
    /// Health check time in milliseconds
    pub health_check_time_ms: f64,
    
    /// API response time in milliseconds
    pub api_response_time_ms: f64,
}

/// Metrics point for time series data
#[derive(Debug, Clone, Serialize)]
pub struct MetricsPoint {
    /// Timestamp
    pub timestamp: u64,
    
    /// Metric name
    pub metric_name: String,
    
    /// Metric value
    pub value: f64,
    
    /// Tags/labels
    pub tags: HashMap<String, String>,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    
    /// Collection interval in seconds
    pub collection_interval: u64,
    
    /// Retention period in hours
    pub retention_hours: u64,
    
    /// Enable Prometheus metrics
    pub prometheus_enabled: bool,
    
    /// Prometheus metrics port
    pub prometheus_port: u16,
    
    /// Enable detailed service metrics
    pub detailed_service_metrics: bool,
    
    /// Enable performance timing metrics
    pub performance_metrics: bool,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: 10,
            retention_hours: 168, // 7 days
            prometheus_enabled: false,
            prometheus_port: 9001,
            detailed_service_metrics: true,
            performance_metrics: true,
        }
    }
}

/// Metrics collector
#[derive(Clone)]
pub struct MetricsCollector {
    /// Configuration
    config: MetricsConfig,
    
    /// Time series data storage
    metrics_data: Arc<RwLock<Vec<MetricsPoint>>>,
    
    /// System metrics cache
    system_metrics: Arc<Mutex<SystemMetrics>>,
    
    /// Service metrics cache
    service_metrics: Arc<RwLock<HashMap<String, ServiceMetrics>>>,
    
    /// Performance metrics cache
    performance_metrics: Arc<RwLock<PerformanceMetrics>>,
    
    /// Start time for uptime calculation
    start_time: SystemTime,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            config,
            metrics_data: Arc::new(RwLock::new(Vec::new())),
            system_metrics: Arc::new(Mutex::new(SystemMetrics {
                timestamp: 0,
                total_services: 0,
                running_services: 0,
                failed_services: 0,
                allocated_ports: 0,
                active_processes: 0,
                uptime_seconds: 0,
                memory_usage: 0,
                cpu_usage: 0.0,
                disk_usage: 0.0,
                network_io: NetworkIoMetrics {
                    bytes_received: 0,
                    bytes_transmitted: 0,
                    packets_received: 0,
                    packets_transmitted: 0,
                },
            })),
            service_metrics: Arc::new(RwLock::new(HashMap::new())),
            performance_metrics: Arc::new(RwLock::new(PerformanceMetrics {
                timestamp: 0,
                service_start_time_ms: 0.0,
                service_stop_time_ms: 0.0,
                port_allocation_time_ms: 0.0,
                health_check_time_ms: 0.0,
                api_response_time_ms: 0.0,
            })),
            start_time: SystemTime::now(),
        }
    }
    
    /// Initialize the metrics collector
    pub async fn initialize(&self) -> Result<()> {
        if !self.config.enabled {
            tracing::info!("Metrics collection disabled");
            return Ok(());
        }
        
        tracing::info!("Initializing metrics collector");
        tracing::info!("Collection interval: {}s", self.config.collection_interval);
        tracing::info!("Retention period: {}h", self.config.retention_hours);
        tracing::info!("Prometheus enabled: {}", self.config.prometheus_enabled);
        
        // Start metrics collection task
        self.start_collection_task().await;
        
        // Start cleanup task
        self.start_cleanup_task().await;
        
        Ok(())
    }
    
    /// Start the metrics collection task
    async fn start_collection_task(&self) {
        let collector = self.clone();
        let mut interval = interval(Duration::from_secs(collector.config.collection_interval));
        
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                if let Err(e) = collector.collect_system_metrics().await {
                    tracing::warn!("Failed to collect system metrics: {}", e);
                }
            }
        });
    }
    
    /// Start the cleanup task
    async fn start_cleanup_task(&self) {
        let collector = self.clone();
        let mut interval = interval(Duration::from_secs(3600)); // Run every hour
        
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                if let Err(e) = collector.cleanup_old_metrics().await {
                    tracing::warn!("Failed to cleanup old metrics: {}", e);
                }
            }
        });
    }
    
    /// Collect system metrics
    async fn collect_system_metrics(&self) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        // Collect basic system information
        let (memory_usage, cpu_usage, disk_usage) = self.collect_system_info().await;
        
        let system_metrics = SystemMetrics {
            timestamp: now,
            total_services: 0, // Will be updated by the service registry
            running_services: 0,
            failed_services: 0,
            allocated_ports: 0,
            active_processes: 0,
            uptime_seconds: self.start_time.elapsed().unwrap_or(Duration::ZERO).as_secs(),
            memory_usage,
            cpu_usage,
            disk_usage,
            network_io: self.collect_network_metrics().await,
        };
        
        // Update system metrics cache
        {
            let mut metrics = self.system_metrics.lock().unwrap();
            *metrics = system_metrics.clone();
        }
        
        // Add to time series data
        self.add_metric_point("system.memory_usage", memory_usage as f64, HashMap::new()).await;
        self.add_metric_point("system.cpu_usage", cpu_usage, HashMap::new()).await;
        self.add_metric_point("system.disk_usage", disk_usage, HashMap::new()).await;
        self.add_metric_point("system.uptime", system_metrics.uptime_seconds as f64, HashMap::new()).await;
        
        Ok(())
    }
    
    /// Collect system information (mock implementation)
    async fn collect_system_info(&self) -> (u64, f64, f64) {
        // In a real implementation, this would collect actual system metrics
        // For now, return mock data
        (
            1024 * 1024 * 512, // 512 MB memory usage
            15.5,              // 15.5% CPU usage
            45.2,              // 45.2% disk usage
        )
    }
    
    /// Collect network metrics (mock implementation)
    async fn collect_network_metrics(&self) -> NetworkIoMetrics {
        // In a real implementation, this would collect actual network metrics
        NetworkIoMetrics {
            bytes_received: 1024 * 1024,
            bytes_transmitted: 512 * 1024,
            packets_received: 1000,
            packets_transmitted: 800,
        }
    }
    
    /// Add a metric point to the time series data
    async fn add_metric_point(&self, metric_name: &str, value: f64, tags: HashMap<String, String>) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let point = MetricsPoint {
            timestamp: now,
            metric_name: metric_name.to_string(),
            value,
            tags,
        };
        
        let mut data = self.metrics_data.write().await;
        data.push(point);
    }
    
    /// Update service metrics
    pub async fn update_service_metrics(&self, services: &[ServiceInstance]) -> Result<()> {
        if !self.config.enabled || !self.config.detailed_service_metrics {
            return Ok(());
        }
        
        let mut service_metrics = self.service_metrics.write().await;
        service_metrics.clear();
        
        for service in services {
            let uptime = service.started_at
                .map(|start| chrono::Utc::now().signed_duration_since(start).num_seconds() as u64)
                .unwrap_or(0);
                
            let metrics = ServiceMetrics {
                service_id: service.definition.id.clone(),
                service_name: service.definition.name.clone(),
                status: format!("{:?}", service.status),
                uptime_seconds: uptime,
                restart_count: service.restart_count,
                memory_usage: 0, // Would be collected from actual process
                cpu_usage: 0.0,  // Would be collected from actual process
                response_time_ms: None, // Would be measured during health checks
                error_count: 0,  // Would be tracked from logs
                health_status: if service.is_healthy { "healthy".to_string() } else { "unhealthy".to_string() },
                last_health_check: service.last_health_check
                    .map(|check| check.timestamp() as u64),
            };
            
            service_metrics.insert(service.definition.id.clone(), metrics);
            
            // Add to time series data
            let mut tags = HashMap::new();
            tags.insert("service_id".to_string(), service.definition.id.clone());
            tags.insert("service_name".to_string(), service.definition.name.clone());
            
            self.add_metric_point("service.uptime", uptime as f64, tags.clone()).await;
            self.add_metric_point("service.restart_count", service.restart_count as f64, tags.clone()).await;
            
            let status_value = match service.status {
                ServiceStatus::Running => 1.0,
                ServiceStatus::Failed(_) => -1.0,
                _ => 0.0,
            };
            self.add_metric_point("service.status", status_value, tags).await;
        }
        
        // Update system metrics with service counts
        let total_services = services.len();
        let running_services = services.iter()
            .filter(|s| matches!(s.status, ServiceStatus::Running))
            .count();
        let failed_services = services.iter()
            .filter(|s| matches!(s.status, ServiceStatus::Failed(_)))
            .count();
            
        {
            let mut system_metrics = self.system_metrics.lock().unwrap();
            system_metrics.total_services = total_services;
            system_metrics.running_services = running_services;
            system_metrics.failed_services = failed_services;
        }
        
        Ok(())
    }
    
    /// Record performance timing
    pub async fn record_timing(&self, operation: &str, duration: Duration) -> Result<()> {
        if !self.config.enabled || !self.config.performance_metrics {
            return Ok(());
        }
        
        let duration_ms = duration.as_secs_f64() * 1000.0;
        
        // Update performance metrics cache
        {
            let mut perf_metrics = self.performance_metrics.write().await;
            match operation {
                "service_start" => perf_metrics.service_start_time_ms = duration_ms,
                "service_stop" => perf_metrics.service_stop_time_ms = duration_ms,
                "port_allocation" => perf_metrics.port_allocation_time_ms = duration_ms,
                "health_check" => perf_metrics.health_check_time_ms = duration_ms,
                "api_response" => perf_metrics.api_response_time_ms = duration_ms,
                _ => {}
            }
        }
        
        // Add to time series data
        let mut tags = HashMap::new();
        tags.insert("operation".to_string(), operation.to_string());
        
        self.add_metric_point("performance.timing", duration_ms, tags).await;
        
        Ok(())
    }
    
    /// Get current system metrics
    pub async fn get_system_metrics(&self) -> SystemMetrics {
        let metrics = self.system_metrics.lock().unwrap();
        metrics.clone()
    }
    
    /// Get service metrics
    pub async fn get_service_metrics(&self) -> HashMap<String, ServiceMetrics> {
        let metrics = self.service_metrics.read().await;
        metrics.clone()
    }
    
    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        let metrics = self.performance_metrics.read().await;
        metrics.clone()
    }
    
    /// Get time series data for a metric
    pub async fn get_time_series(&self, metric_name: &str, since: Option<u64>) -> Vec<MetricsPoint> {
        let data = self.metrics_data.read().await;
        
        let since_timestamp = since.unwrap_or(0);
        
        data.iter()
            .filter(|point| {
                point.metric_name == metric_name && point.timestamp >= since_timestamp
            })
            .cloned()
            .collect()
    }
    
    /// Cleanup old metrics based on retention policy
    async fn cleanup_old_metrics(&self) -> Result<()> {
        let cutoff_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() - (self.config.retention_hours as u64 * 3600);
            
        let mut data = self.metrics_data.write().await;
        data.retain(|point| point.timestamp > cutoff_time);
        
        let removed_count = data.capacity() - data.len();
        if removed_count > 0 {
            tracing::info!("Cleaned up {} old metric points", removed_count);
        }
        
        Ok(())
    }
    
    /// Export metrics in Prometheus format (basic implementation)
    pub async fn export_prometheus_metrics(&self) -> String {
        let mut output = String::new();
        
        // System metrics
        let system_metrics = self.get_system_metrics().await;
        output.push_str(&format!("# HELP nestgate_uptime_seconds System uptime in seconds\n"));
        output.push_str(&format!("# TYPE nestgate_uptime_seconds gauge\n"));
        output.push_str(&format!("nestgate_uptime_seconds {}\n", system_metrics.uptime_seconds));
        
        output.push_str(&format!("# HELP nestgate_memory_usage_bytes Memory usage in bytes\n"));
        output.push_str(&format!("# TYPE nestgate_memory_usage_bytes gauge\n"));
        output.push_str(&format!("nestgate_memory_usage_bytes {}\n", system_metrics.memory_usage));
        
        output.push_str(&format!("# HELP nestgate_cpu_usage_percent CPU usage percentage\n"));
        output.push_str(&format!("# TYPE nestgate_cpu_usage_percent gauge\n"));
        output.push_str(&format!("nestgate_cpu_usage_percent {}\n", system_metrics.cpu_usage));
        
        // Service metrics
        let service_metrics = self.get_service_metrics().await;
        output.push_str(&format!("# HELP nestgate_services_total Total number of services\n"));
        output.push_str(&format!("# TYPE nestgate_services_total gauge\n"));
        output.push_str(&format!("nestgate_services_total {}\n", service_metrics.len()));
        
        for (service_id, metrics) in &service_metrics {
            output.push_str(&format!(
                "nestgate_service_uptime_seconds{{service_id=\"{}\"}} {}\n",
                service_id, metrics.uptime_seconds
            ));
            output.push_str(&format!(
                "nestgate_service_restart_count{{service_id=\"{}\"}} {}\n",
                service_id, metrics.restart_count
            ));
        }
        
        output
    }
} 