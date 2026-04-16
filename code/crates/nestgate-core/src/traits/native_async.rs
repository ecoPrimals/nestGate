// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Native async trait definitions and implementations.
//!
//! Zero-cost native async traits — no `async_trait` dependency, no `Box::pin` overhead.
//! Uses native `impl Future` patterns, const generic configuration, and compile-time
//! optimization for zero-cost abstractions.

use crate::error::Result;
use std::collections::HashMap;
use std::future::Future;
// Removed unused imports: UnifiedServiceType, UnifiedServiceState
use crate::config::canonical_primary::NestGateCanonicalConfig;

// ==================== SECTION ====================

/// **THE** primary service trait - replaces all `async_trait` service patterns
/// This is the zero-cost foundation for all `NestGate` services
pub trait NativeAsyncService: Send + Sync + 'static {
    /// Service configuration type
    type Config: Clone + Send + Sync;

    /// Service health status type
    type Health: Send + Sync;

    /// Service metrics type
    type Metrics: Send + Sync;
    /// Initialize the service with configuration
    fn initialize(&self, config: Self::Config) -> impl Future<Output = Result<()>> + Send;

    /// Get service health status
    fn health_check(&self) -> impl Future<Output = Result<Self::Health>> + Send;

    /// Get service metrics
    fn get_metrics(&self) -> impl Future<Output = Result<Self::Metrics>> + Send;

    /// Graceful shutdown
    fn shutdown(&self) -> impl Future<Output = Result<()>> + Send;
}

// ==================== SECTION ====================

#[derive(Debug, Clone)]
/// Storagemetadata
pub struct StorageMetadata {
    /// Size
    pub size: u64,
    /// Created
    pub created: std::time::SystemTime,
    /// Modified
    pub modified: std::time::SystemTime,
    /// Content Type
    pub content_type: String,
    /// Checksum
    pub checksum: String,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

// ==================== SECTION ====================

/// Native async network service trait - replaces `async_trait` network patterns
pub trait NativeAsyncNetworkService: Send + Sync + 'static {
    /// Connection type
    type Connection: Send + Sync;

    /// Request type
    type Request: Send + Sync;

    /// Response type
    type Response: Send + Sync;
    /// Start listening for connections
    fn start_listening(&self, endpoint: &str) -> impl Future<Output = Result<()>> + Send;

    /// Accept a new connection
    fn accept_connection(&self) -> impl Future<Output = Result<Self::Connection>> + Send;

    /// Handle a request
    fn handle_request(
        &self,
        request: Self::Request,
    ) -> impl Future<Output = Result<Self::Response>> + Send;

    /// Close connection
    fn close_connection(
        &self,
        connection: Self::Connection,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Get network statistics
    fn get_statistics(&self) -> impl Future<Output = Result<NetworkStatistics>> + Send;
}

#[derive(Debug, Clone)]
/// Networkstatistics
pub struct NetworkStatistics {
    /// Active Connections
    pub active_connections: usize,
    /// Total Requests
    pub total_requests: u64,
    /// Bytes Sent
    pub bytes_sent: u64,
    /// Bytes Received
    pub bytes_received: u64,
    /// Errors
    pub errors: u64,
}

// ==================== SECTION ====================

/// Native async API handler trait - replaces `async_trait` API patterns
pub trait NativeAsyncApiHandler: Send + Sync + 'static {
    /// Request type
    type Request: Send + Sync;

    /// Response type
    type Response: Send + Sync;
    /// Handle API request
    fn handle(&self, request: Self::Request)
    -> impl Future<Output = Result<Self::Response>> + Send;

    /// Validate request
    fn validate_request(&self, request: &Self::Request) -> impl Future<Output = Result<()>> + Send;

    /// Get handler metrics
    fn get_handler_metrics(&self) -> impl Future<Output = Result<HandlerMetrics>> + Send;
}

#[derive(Debug, Clone)]
/// Handlermetrics
pub struct HandlerMetrics {
    /// Requests Handled
    pub requests_handled: u64,
    /// Average Response Time
    pub average_response_time: std::time::Duration,
    /// Error Rate
    pub error_rate: f64,
    /// Last Request Time
    pub last_request_time: Option<std::time::SystemTime>,
}

// ==================== SECTION ====================

/// Native async MCP service trait - replaces `async_trait` MCP patterns
pub trait NativeAsyncMcpService: Send + Sync + 'static {
    /// Message type
    type Message: Send + Sync;
    /// Start MCP server
    fn start_server(&self, port: u16) -> impl Future<Output = Result<()>> + Send;

    /// Handle MCP message
    fn handle_message(
        &self,
        message: Self::Message,
    ) -> impl Future<Output = Result<Self::Message>> + Send;

    /// Send MCP message
    fn send_message(&self, message: Self::Message) -> impl Future<Output = Result<()>> + Send;

    /// Close MCP connection
    fn close_connection(&self, connection_id: &str) -> impl Future<Output = Result<()>> + Send;
}

// ==================== SECTION ====================

/// Native async automation service trait - replaces `async_trait` automation patterns
pub trait NativeAsyncAutomationService: Send + Sync + 'static {
    /// Workflow type
    type Workflow: Send + Sync;

    /// Task type
    type Task: Send + Sync;
    /// Execute workflow
    fn execute_workflow(&self, workflow: Self::Workflow)
    -> impl Future<Output = Result<()>> + Send;

    /// Schedule task
    fn schedule_task(
        &self,
        task: Self::Task,
        schedule: &str,
    ) -> impl Future<Output = Result<String>> + Send;

    /// Cancel scheduled task
    fn cancel_task(&self, task_id: &str) -> impl Future<Output = Result<()>> + Send;

    /// Get automation status
    fn get_status(&self) -> impl Future<Output = Result<AutomationStatus>> + Send;
}

#[derive(Debug, Clone)]
/// Automationstatus
pub struct AutomationStatus {
    /// Active Workflows
    pub active_workflows: usize,
    /// Scheduled Tasks
    pub scheduled_tasks: usize,
    /// Completed Workflows
    pub completed_workflows: u64,
    /// Failed Workflows
    pub failed_workflows: u64,
}

// ==================== SECTION ====================

/// Native async monitoring service trait - replaces `async_trait` monitoring patterns
pub trait NativeAsyncMonitoringService: Send + Sync + 'static {
    /// Metric type
    type Metric: Send + Sync;
    /// Collect metrics
    fn collect_metrics(&self) -> impl Future<Output = Result<Vec<Self::Metric>>> + Send;

    /// Send alert
    fn send_alert(&self, alert: Alert) -> impl Future<Output = Result<()>> + Send;

    /// Get monitoring dashboard data
    fn get_dashboard_data(&self) -> impl Future<Output = Result<DashboardData>> + Send;

    /// Update monitoring configuration
    fn update_config(&self, config: MonitoringConfig) -> impl Future<Output = Result<()>> + Send;
}

#[derive(Debug, Clone)]
/// Alert
pub struct Alert {
    /// Severity
    pub severity: AlertSeverity,
    /// Message
    pub message: String,
    /// Component
    pub component: String,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
/// Alertseverity
pub enum AlertSeverity {
    /// Info
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}

#[derive(Debug, Clone)]
/// Dashboarddata
pub struct DashboardData {
    /// Metrics
    pub metrics: HashMap<String, f64>,
    /// Status
    pub status: ServiceStatus,
    /// Alerts
    pub alerts: Vec<Alert>,
    /// Last Updated
    pub last_updated: std::time::SystemTime,
}

#[derive(Debug, Clone)]
/// Status values for Service
pub enum ServiceStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}

// ==================== SECTION ====================

/// Universal provider trait for ecosystem integration - replaces `async_trait` providers
pub trait NativeAsyncUniversalProvider<T>: Send + Sync + 'static {
    /// Provide service instance
    fn provide(&self) -> impl Future<Output = Result<T>> + Send;
    /// Check if provider is available
    fn is_available(&self) -> impl Future<Output = Result<bool>> + Send;

    /// Get provider capabilities
    fn get_capabilities(&self) -> impl Future<Output = Result<Vec<String>>> + Send;

    /// Initialize provider
    fn initialize(
        &self,
        config: NestGateCanonicalConfig,
    ) -> impl Future<Output = Result<()>> + Send;
}

// ==================== SECTION ====================

/// Utilities for migrating from `async_trait` to native async patterns
pub mod migration {
    /// Check if code uses legacy `async_trait` patterns
    #[must_use]
    pub fn has_async_trait_usage(code: &str) -> bool {
        code.contains("[async_trait]") || code.contains("async_trait::")
    }
    /// Generate migration suggestions for `async_trait` code
    #[must_use]
    pub fn generate_migration_suggestions(trait_name: &str) -> Vec<String> {
        vec![
            format!(
                "Replace #[async_trait] with native async for {}",
                trait_name
            ),
            "Change async fn to fn returning impl Future".to_string(),
            "Remove Arc<dyn> boxing for direct composition".to_string(),
            "Add const generics for compile-time configuration".to_string(),
        ]
    }
}

// ==================== SECTION ====================

/// Example native async service implementation
pub struct ExampleNativeService {
    /// Configuration for
    pub config: NestGateCanonicalConfig,
    /// Initialized
    pub initialized: bool,
}
impl NativeAsyncService for ExampleNativeService {
    /// Type alias for Config
    type Config = NestGateCanonicalConfig;
    /// Type alias for Health
    type Health = ServiceHealth;
    /// Type alias for Metrics
    type Metrics = ServiceMetrics;

    /// Initialize
    async fn initialize(&self, _config: Self::Config) -> Result<()> {
        // Initialize with zero-cost configuration access
        Ok(())
    }

    /// Health Check
    async fn health_check(&self) -> Result<Self::Health> {
        Ok(ServiceHealth {
            status: ServiceStatus::Healthy,
            uptime: std::time::Duration::from_secs(3600),
            last_check: std::time::SystemTime::now(),
        })
    }

    /// Gets Metrics
    async fn get_metrics(&self) -> Result<Self::Metrics> {
        Ok(ServiceMetrics {
            requests_handled: 1000,
            average_response_time: std::time::Duration::from_millis(50),
            error_count: 0,
            uptime: std::time::Duration::from_secs(3600),
        })
    }

    /// Shutdown
    async fn shutdown(&self) -> Result<()> {
        // Graceful shutdown logic
        Ok(())
    }
}

#[derive(Debug, Clone)]
/// Servicehealth
pub struct ServiceHealth {
    /// Status
    pub status: ServiceStatus,
    /// Uptime
    pub uptime: std::time::Duration,
    /// Last Check
    pub last_check: std::time::SystemTime,
}

#[derive(Debug, Clone)]
/// Servicemetrics
pub struct ServiceMetrics {
    /// Requests Handled
    pub requests_handled: u64,
    /// Average Response Time
    pub average_response_time: std::time::Duration,
    /// Count of error
    pub error_count: u64,
    /// Uptime
    pub uptime: std::time::Duration,
}

// Include monitoring config type - using canonical from supporting_types
use crate::config::canonical_primary::MonitoringConfig;

#[cfg(test)]
mod tests {
    use super::migration::{generate_migration_suggestions, has_async_trait_usage};
    use super::*;

    #[test]
    fn migration_detects_async_trait_attribute() {
        assert!(has_async_trait_usage(
            r"#[async_trait] impl Foo for Bar { async fn x() {} }"
        ));
        assert!(has_async_trait_usage("use async_trait::async_trait;"));
        assert!(!has_async_trait_usage("fn plain() {}"));
    }

    #[test]
    fn migration_suggestions_include_trait_name() {
        let s = generate_migration_suggestions("MyTrait");
        assert!(s.iter().any(|l| l.contains("MyTrait")));
        assert!(s.iter().any(|l| l.contains("native async")));
    }

    #[test]
    fn storage_metadata_clone_and_fields() {
        let mut m = HashMap::new();
        m.insert("a".to_string(), "b".to_string());
        let sm = StorageMetadata {
            size: 42,
            created: std::time::SystemTime::UNIX_EPOCH,
            modified: std::time::SystemTime::UNIX_EPOCH,
            content_type: "application/octet-stream".to_string(),
            checksum: "abc".to_string(),
            metadata: m,
        };
        let sm2 = sm;
        assert_eq!(sm2.size, 42);
        assert_eq!(sm2.metadata.get("a").map(String::as_str), Some("b"));
    }

    #[test]
    fn network_statistics_defaults() {
        let n = NetworkStatistics {
            active_connections: 0,
            total_requests: 0,
            bytes_sent: 0,
            bytes_received: 0,
            errors: 0,
        };
        assert_eq!(n.errors, 0);
    }

    #[test]
    fn handler_metrics_and_automation_status() {
        let hm = HandlerMetrics {
            requests_handled: 1,
            average_response_time: std::time::Duration::from_millis(10),
            error_rate: 0.01,
            last_request_time: None,
        };
        assert!((hm.error_rate - 0.01).abs() < f64::EPSILON);

        let st = AutomationStatus {
            active_workflows: 0,
            scheduled_tasks: 0,
            completed_workflows: 1,
            failed_workflows: 0,
        };
        assert_eq!(st.completed_workflows, 1);
    }

    #[test]
    fn alert_and_dashboard_data_shapes() {
        let a = Alert {
            severity: AlertSeverity::Critical,
            message: "m".to_string(),
            component: "c".to_string(),
            timestamp: std::time::SystemTime::UNIX_EPOCH,
            metadata: HashMap::new(),
        };
        assert!(matches!(a.severity, AlertSeverity::Critical));

        let mut metrics = HashMap::new();
        metrics.insert("cpu".to_string(), 0.5);
        let d = DashboardData {
            metrics,
            status: ServiceStatus::Healthy,
            alerts: vec![],
            last_updated: std::time::SystemTime::UNIX_EPOCH,
        };
        assert_eq!(d.metrics.len(), 1);
    }

    #[test]
    fn service_health_and_metrics_clone() {
        let h = ServiceHealth {
            status: ServiceStatus::Degraded,
            uptime: std::time::Duration::from_secs(1),
            last_check: std::time::SystemTime::UNIX_EPOCH,
        };
        let h2 = h;
        assert!(matches!(h2.status, ServiceStatus::Degraded));

        let m = ServiceMetrics {
            requests_handled: 9,
            average_response_time: std::time::Duration::from_nanos(1),
            error_count: 0,
            uptime: std::time::Duration::ZERO,
        };
        assert_eq!(m.requests_handled, 9);
    }

    #[tokio::test]
    async fn example_native_service_trait_methods() {
        let svc = ExampleNativeService {
            config: NestGateCanonicalConfig::default(),
            initialized: false,
        };
        svc.initialize(NestGateCanonicalConfig::default())
            .await
            .expect("init");
        let health = svc.health_check().await.expect("health");
        assert!(matches!(health.status, ServiceStatus::Healthy));
        let metrics = svc.get_metrics().await.expect("metrics");
        assert_eq!(metrics.requests_handled, 1000);
        svc.shutdown().await.expect("shutdown");
    }
}
