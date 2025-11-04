//! Modern Performance Monitoring Module
//! 
//! Provides comprehensive performance monitoring, metrics collection, and real-time
//! analysis with zero-cost abstractions and native async patterns.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::error::{NestGateError, Result};

// ==================== PERFORMANCE METRICS ====================

/// High-performance metrics collector with atomic operations
#[derive(Debug)]
pub struct MetricsCollector {
    /// Request counters
    total_requests: AtomicU64,
    successful_requests: AtomicU64,
    failed_requests: AtomicU64,
    
    /// Timing metrics
    total_response_time_ns: AtomicU64,
    min_response_time_ns: AtomicU64,
    max_response_time_ns: AtomicU64,
    
    /// Resource usage
    active_connections: AtomicUsize,
    peak_connections: AtomicUsize,
    memory_usage_bytes: AtomicU64,
    
    /// Error tracking
    error_counts: Arc<RwLock<HashMap<String, AtomicU64>>>,
    
    /// Performance buckets for percentile calculation
    response_time_buckets: Arc<RwLock<Vec<u64>>>,
    
    /// Start time for uptime calculation
    start_time: Instant,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            successful_requests: AtomicU64::new(0),
            failed_requests: AtomicU64::new(0),
            total_response_time_ns: AtomicU64::new(0),
            min_response_time_ns: AtomicU64::new(u64::MAX),
            max_response_time_ns: AtomicU64::new(0),
            active_connections: AtomicUsize::new(0),
            peak_connections: AtomicUsize::new(0),
            memory_usage_bytes: AtomicU64::new(0),
            error_counts: Arc::new(RwLock::new(HashMap::new())),
            response_time_buckets: Arc::new(RwLock::new(Vec::new())),
            start_time: Instant::now(),
        }
    }

    /// Record a successful request with timing
    pub async fn record_request_success(&self, response_time: Duration) {
        let response_time_ns = response_time.as_nanos() as u64;
        
        // Update counters atomically
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.successful_requests.fetch_add(1, Ordering::Relaxed);
        self.total_response_time_ns.fetch_add(response_time_ns, Ordering::Relaxed);
        
        // Update min/max response times
        self.update_min_response_time(response_time_ns);
        self.update_max_response_time(response_time_ns);
        
        // Add to buckets for percentile calculation
        let mut buckets = self.response_time_buckets.write().await;
        buckets.push(response_time_ns);
        
        // Keep only last 10000 samples for memory efficiency
        if buckets.len() > 10000 {
            buckets.drain(0..5000);
        }
    }

    /// Record a failed request
    pub async fn record_request_failure(&self, error_type: &str) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
        
        // Update error counts
        let mut error_counts = self.error_counts.write().await;
        let counter = error_counts.entry(error_type.to_string())
            .or_insert_with(|| AtomicU64::new(0));
        counter.fetch_add(1, Ordering::Relaxed);
    }

    /// Record connection opened
    pub fn record_connection_opened(&self) {
        let current = self.active_connections.fetch_add(1, Ordering::Relaxed) + 1;
        
        // Update peak connections
        let mut peak = self.peak_connections.load(Ordering::Relaxed);
        while current > peak {
            match self.peak_connections.compare_exchange_weak(
                peak, current, Ordering::Relaxed, Ordering::Relaxed
            ) {
                Ok(_) => break,
                Err(x) => peak = x,
            }
        }
    }

    /// Record connection closed
    pub fn record_connection_closed(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Update memory usage
    pub fn update_memory_usage(&self, bytes: u64) {
        self.memory_usage_bytes.store(bytes, Ordering::Relaxed);
    }

    /// Get current performance snapshot
    pub async fn get_snapshot(&self) -> PerformanceSnapshot {
        let total_requests = self.total_requests.load(Ordering::Relaxed);
        let successful_requests = self.successful_requests.load(Ordering::Relaxed);
        let failed_requests = self.failed_requests.load(Ordering::Relaxed);
        let total_response_time_ns = self.total_response_time_ns.load(Ordering::Relaxed);
        
        let average_response_time = if total_requests > 0 {
            Duration::from_nanos(total_response_time_ns / total_requests)
        } else {
            Duration::ZERO
        };

        let min_response_time = {
            let min_ns = self.min_response_time_ns.load(Ordering::Relaxed);
            if min_ns == u64::MAX {
                Duration::ZERO
            } else {
                Duration::from_nanos(min_ns)
            }
        };

        let max_response_time = Duration::from_nanos(
            self.max_response_time_ns.load(Ordering::Relaxed)
        );

        let success_rate = if total_requests > 0 {
            (successful_requests as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        // Calculate percentiles
        let percentiles = self.calculate_percentiles().await;

        // Get error breakdown
        let error_breakdown = self.get_error_breakdown().await;

        PerformanceSnapshot {
            timestamp: SystemTime::now(),
            uptime: self.start_time.elapsed(),
            total_requests,
            successful_requests,
            failed_requests,
            success_rate,
            average_response_time,
            min_response_time,
            max_response_time,
            percentiles,
            active_connections: self.active_connections.load(Ordering::Relaxed),
            peak_connections: self.peak_connections.load(Ordering::Relaxed),
            memory_usage_bytes: self.memory_usage_bytes.load(Ordering::Relaxed),
            error_breakdown,
        }
    }

    /// Update minimum response time atomically
    fn update_min_response_time(&self, response_time_ns: u64) {
        let mut current_min = self.min_response_time_ns.load(Ordering::Relaxed);
        while response_time_ns < current_min {
            match self.min_response_time_ns.compare_exchange_weak(
                current_min, response_time_ns, Ordering::Relaxed, Ordering::Relaxed
            ) {
                Ok(_) => break,
                Err(x) => current_min = x,
            }
        }
    }

    /// Update maximum response time atomically
    fn update_max_response_time(&self, response_time_ns: u64) {
        let mut current_max = self.max_response_time_ns.load(Ordering::Relaxed);
        while response_time_ns > current_max {
            match self.max_response_time_ns.compare_exchange_weak(
                current_max, response_time_ns, Ordering::Relaxed, Ordering::Relaxed
            ) {
                Ok(_) => break,
                Err(x) => current_max = x,
            }
        }
    }

    /// Calculate response time percentiles
    async fn calculate_percentiles(&self) -> ResponseTimePercentiles {
        let buckets = self.response_time_buckets.read().await;
        if buckets.is_empty() {
            return ResponseTimePercentiles::default();
        }

        let mut sorted_times: Vec<u64> = buckets.clone();
        sorted_times.sort_unstable();

        let len = sorted_times.len();
        ResponseTimePercentiles {
            p50: Duration::from_nanos(sorted_times[len * 50 / 100]),
            p90: Duration::from_nanos(sorted_times[len * 90 / 100]),
            p95: Duration::from_nanos(sorted_times[len * 95 / 100]),
            p99: Duration::from_nanos(sorted_times[len * 99 / 100]),
        }
    }

    /// Get error breakdown
    async fn get_error_breakdown(&self) -> HashMap<String, u64> {
        let error_counts = self.error_counts.read().await;
        error_counts.iter()
            .map(|(k, v)| (k.clone(), v.load(Ordering::Relaxed)))
            .collect()
    }

    /// Reset all metrics
    pub async fn reset(&self) {
        self.total_requests.store(0, Ordering::Relaxed);
        self.successful_requests.store(0, Ordering::Relaxed);
        self.failed_requests.store(0, Ordering::Relaxed);
        self.total_response_time_ns.store(0, Ordering::Relaxed);
        self.min_response_time_ns.store(u64::MAX, Ordering::Relaxed);
        self.max_response_time_ns.store(0, Ordering::Relaxed);
        self.peak_connections.store(0, Ordering::Relaxed);
        
        let mut error_counts = self.error_counts.write().await;
        error_counts.clear();
        
        let mut buckets = self.response_time_buckets.write().await;
        buckets.clear();
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== PERFORMANCE SNAPSHOT ====================

/// Performance metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    pub timestamp: SystemTime,
    pub uptime: Duration,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub success_rate: f64,
    pub average_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub percentiles: ResponseTimePercentiles,
    pub active_connections: usize,
    pub peak_connections: usize,
    pub memory_usage_bytes: u64,
    pub error_breakdown: HashMap<String, u64>,
}

impl PerformanceSnapshot {
    /// Check if performance is healthy
    pub fn is_healthy(&self) -> bool {
        self.success_rate >= 95.0 && 
        self.average_response_time < Duration::from_millis(1000) &&
        self.memory_usage_bytes < 1_000_000_000 // 1GB
    }

    /// Get performance grade (A-F)
    pub fn get_grade(&self) -> PerformanceGrade {
        let response_time_score = if self.average_response_time < Duration::from_millis(100) {
            100
        } else if self.average_response_time < Duration::from_millis(500) {
            80
        } else if self.average_response_time < Duration::from_millis(1000) {
            60
        } else if self.average_response_time < Duration::from_millis(2000) {
            40
        } else {
            20
        };

        let success_rate_score = (self.success_rate as u32).min(100);
        
        let overall_score = (response_time_score + success_rate_score) / 2;
        
        match overall_score {
            90..=100 => PerformanceGrade::A,
            80..=89 => PerformanceGrade::B,
            70..=79 => PerformanceGrade::C,
            60..=69 => PerformanceGrade::D,
            _ => PerformanceGrade::F,
        }
    }

    /// Generate human-readable report
    pub fn generate_report(&self) -> String {
        let uptime_secs = self.uptime.as_secs();
        let uptime_str = if uptime_secs < 60 {
            format!("{}s", uptime_secs)
        } else if uptime_secs < 3600 {
            format!("{}m {}s", uptime_secs / 60, uptime_secs % 60)
        } else {
            format!("{}h {}m", uptime_secs / 3600, (uptime_secs % 3600) / 60)
        };

        format!(
            "🚀 **NESTGATE PERFORMANCE REPORT**\n\
            =====================================\n\
            📊 **Overall Grade**: {:?}\n\
            ⏱️  **Uptime**: {}\n\
            📈 **Requests**: {} total ({} success, {} failed)\n\
            ✅ **Success Rate**: {:.1}%\n\
            ⚡ **Response Times**:\n\
            \t• Average: {:?}\n\
            \t• Min: {:?}\n\
            \t• Max: {:?}\n\
            \t• P50: {:?}\n\
            \t• P90: {:?}\n\
            \t• P95: {:?}\n\
            \t• P99: {:?}\n\
            🔗 **Connections**: {} active (peak: {})\n\
            💾 **Memory**: {:.2} MB\n\
            ❌ **Error Breakdown**:\n{}",
            self.get_grade(),
            uptime_str,
            self.total_requests,
            self.successful_requests,
            self.failed_requests,
            self.success_rate,
            self.average_response_time,
            self.min_response_time,
            self.max_response_time,
            self.percentiles.p50,
            self.percentiles.p90,
            self.percentiles.p95,
            self.percentiles.p99,
            self.active_connections,
            self.peak_connections,
            self.memory_usage_bytes as f64 / 1_000_000.0,
            self.format_error_breakdown()
        )
    }

    /// Format error breakdown for display
    fn format_error_breakdown(&self) -> String {
        if self.error_breakdown.is_empty() {
            "\t• No errors recorded ✨".to_string()
        } else {
            self.error_breakdown
                .iter()
                .map(|(error_type, count)| format!("\t• {}: {}", error_type, count))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

/// Response time percentiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimePercentiles {
    pub p50: Duration,
    pub p90: Duration,
    pub p95: Duration,
    pub p99: Duration,
}

impl Default for ResponseTimePercentiles {
    fn default() -> Self {
        Self {
            p50: Duration::ZERO,
            p90: Duration::ZERO,
            p95: Duration::ZERO,
            p99: Duration::ZERO,
        }
    }
}

/// Performance grade enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerformanceGrade {
    A, // Excellent (90-100%)
    B, // Good (80-89%)
    C, // Average (70-79%)
    D, // Poor (60-69%)
    F, // Failing (<60%)
}

// ==================== PERFORMANCE MONITOR ====================

/// High-level performance monitor with alerting
#[derive(Debug)]
pub struct PerformanceMonitor {
    collector: Arc<MetricsCollector>,
    alert_thresholds: AlertThresholds,
    alert_callbacks: Arc<RwLock<Vec<Box<dyn AlertCallback>>>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(alert_thresholds: AlertThresholds) -> Self {
        Self {
            collector: Arc::new(MetricsCollector::new()),
            alert_thresholds,
            alert_callbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get the metrics collector
    pub fn collector(&self) -> Arc<MetricsCollector> {
        Arc::clone(&self.collector)
    }

    /// Add an alert callback
    pub async fn add_alert_callback(&self, callback: Box<dyn AlertCallback>) {
        let mut callbacks = self.alert_callbacks.write().await;
        callbacks.push(callback);
    }

    /// Check for alerts and trigger callbacks
    pub fn check_alerts(&self) -> impl std::future::Future<Output = Result<Vec<Alert>> + Send> {
        let snapshot = self.collector.get_snapshot().await;
        let mut alerts = Vec::new();

        // Check success rate
        if snapshot.success_rate < self.alert_thresholds.min_success_rate {
            alerts.push(Alert {
                alert_type: AlertType::LowSuccessRate,
                severity: AlertSeverity::Critical,
                message: format!(
                    "Success rate ({:.1}%) below threshold ({:.1}%)",
                    snapshot.success_rate, self.alert_thresholds.min_success_rate
                ),
                timestamp: SystemTime::now(),
                value: snapshot.success_rate,
            });
        }

        // Check response time
        if snapshot.average_response_time > self.alert_thresholds.max_response_time {
            alerts.push(Alert {
                alert_type: AlertType::HighResponseTime,
                severity: AlertSeverity::Warning,
                message: format!(
                    "Average response time ({:?}) above threshold ({:?})",
                    snapshot.average_response_time, self.alert_thresholds.max_response_time
                ),
                timestamp: SystemTime::now(),
                value: snapshot.average_response_time.as_millis() as f64,
            });
        }

        // Check memory usage
        if snapshot.memory_usage_bytes > self.alert_thresholds.max_memory_bytes {
            alerts.push(Alert {
                alert_type: AlertType::HighMemoryUsage,
                severity: AlertSeverity::Warning,
                message: format!(
                    "Memory usage ({:.2} MB) above threshold ({:.2} MB)",
                    snapshot.memory_usage_bytes as f64 / 1_000_000.0,
                    self.alert_thresholds.max_memory_bytes as f64 / 1_000_000.0
                ),
                timestamp: SystemTime::now(),
                value: snapshot.memory_usage_bytes as f64,
            });
        }

        // Trigger alert callbacks
        if !alerts.is_empty() {
            let callbacks = self.alert_callbacks.read().await;
            for callback in callbacks.iter() {
                for alert in &alerts {
                    callback.on_alert(alert).await;
                }
            }
        }

        Ok(alerts)
    }

    /// Start continuous monitoring
    pub fn start_monitoring(&self, interval: Duration) -> impl std::future::Future<Output = Result<()>> + Send {
        let collector = Arc::clone(&self.collector);
        let alert_thresholds = self.alert_thresholds.clone();
        let alert_callbacks = Arc::clone(&self.alert_callbacks);

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                let monitor = PerformanceMonitor {
                    collector: Arc::clone(&collector),
                    alert_thresholds: alert_thresholds.clone(),
                    alert_callbacks: Arc::clone(&alert_callbacks),
                };
                
                if let Err(e) = monitor.check_alerts().await {
                    eprintln!("Error checking alerts: {}", e);
                }
            }
        });

        Ok(())
    }
}

// ==================== ALERTING SYSTEM ====================

/// Alert thresholds configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub min_success_rate: f64,
    pub max_response_time: Duration,
    pub max_memory_bytes: u64,
    pub max_error_rate: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            min_success_rate: 95.0,
            max_response_time: Duration::from_millis(1000),
            max_memory_bytes: 1_000_000_000, // 1GB
            max_error_rate: 5.0,
        }
    }
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: SystemTime,
    pub value: f64,
}

/// Alert types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertType {
    LowSuccessRate,
    HighResponseTime,
    HighMemoryUsage,
    HighErrorRate,
    ConnectionPoolExhausted,
}

/// Alert severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Alert callback trait
pub trait AlertCallback: Send + Sync {
    async fn on_alert(&self, alert: &Alert);
}

/// Console alert callback
pub struct ConsoleAlertCallback;

impl AlertCallback for ConsoleAlertCallback {
    async fn on_alert(&self, alert: &Alert) {
        let severity_icon = match alert.severity {
            AlertSeverity::Info => "ℹ️",
            AlertSeverity::Warning => "⚠️",
            AlertSeverity::Critical => "🚨",
        };
        
        println!(
            "{} [{}] {:?}: {}",
            severity_icon,
            alert.timestamp
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            alert.alert_type,
            alert.message
        );
    }
}

// ==================== PERFORMANCE TIMER ====================

/// High-precision performance timer
pub struct PerformanceTimer {
    start: Instant,
    collector: Arc<MetricsCollector>,
    operation_name: String,
}

impl PerformanceTimer {
    /// Start a new performance timer
    pub fn start(collector: Arc<MetricsCollector>, operation_name: String) -> Self {
        Self {
            start: Instant::now(),
            collector,
            operation_name,
        }
    }

    /// Complete the timer and record success
    pub async fn complete_success(self) {
        let duration = self.start.elapsed();
        self.collector.record_request_success(duration).await;
    }

    /// Complete the timer and record failure
    pub async fn complete_failure(self, error_type: &str) {
        self.collector.record_request_failure(error_type).await;
    }

    /// Get elapsed time without completing
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

// ==================== UTILITY FUNCTIONS ====================

/// Create a default performance monitor
pub fn create_monitor() -> PerformanceMonitor {
    PerformanceMonitor::new(AlertThresholds::default())
}

/// Create a performance monitor with custom thresholds
pub fn create_monitor_with_thresholds(thresholds: AlertThresholds) -> PerformanceMonitor {
    PerformanceMonitor::new(thresholds)
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_metrics_collector() {
        let collector = MetricsCollector::new();
        
        // Record some metrics
        collector.record_request_success(Duration::from_millis(100)).await;
        collector.record_request_success(Duration::from_millis(200)).await;
        collector.record_request_failure("timeout").await;
        
        let snapshot = collector.get_snapshot().await;
        
        assert_eq!(snapshot.total_requests, 3);
        assert_eq!(snapshot.successful_requests, 2);
        assert_eq!(snapshot.failed_requests, 1);
        assert_eq!(snapshot.success_rate, 66.66666666666667);
        assert!(snapshot.average_response_time > Duration::ZERO);
    }

    #[tokio::test]
    async fn test_performance_timer() {
        let collector = Arc::new(MetricsCollector::new());
        
        let timer = PerformanceTimer::start(
            Arc::clone(&collector),
            "test_operation".to_string()
        );
        
        sleep(Duration::from_millis(10)).await;
        timer.complete_success().await;
        
        let snapshot = collector.get_snapshot().await;
        assert_eq!(snapshot.total_requests, 1);
        assert_eq!(snapshot.successful_requests, 1);
    }

    #[tokio::test]
    async fn test_alert_system() {
        let thresholds = AlertThresholds {
            min_success_rate: 99.0,
            max_response_time: Duration::from_millis(50),
            max_memory_bytes: 100,
            max_error_rate: 1.0,
        };
        
        let monitor = PerformanceMonitor::new(thresholds);
        let collector = monitor.collector();
        
        // Record a slow request to trigger alert
        collector.record_request_success(Duration::from_millis(100)).await;
        collector.update_memory_usage(200);
        
        let alerts = monitor.check_alerts().await.expect("Operation failed");
        assert!(!alerts.is_empty());
    }

    #[test]
    fn test_performance_grade() {
        let snapshot = PerformanceSnapshot {
            timestamp: SystemTime::now(),
            uptime: Duration::from_secs(3600),
            total_requests: 1000,
            successful_requests: 950,
            failed_requests: 50,
            success_rate: 95.0,
            average_response_time: Duration::from_millis(50),
            min_response_time: Duration::from_millis(10),
            max_response_time: Duration::from_millis(200),
            percentiles: ResponseTimePercentiles::default(),
            active_connections: 10,
            peak_connections: 50,
            memory_usage_bytes: 500_000_000,
            error_breakdown: HashMap::new(),
        };
        
        assert_eq!(snapshot.get_grade(), PerformanceGrade::A);
        assert!(snapshot.is_healthy());
    }
} 