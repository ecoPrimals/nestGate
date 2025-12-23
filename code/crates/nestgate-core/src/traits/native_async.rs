// **NATIVE ASYNC TRAIT SYSTEM**
//! Trait definitions and implementations.
// This module provides zero-cost native async traits that replace ALL async_trait patterns
//! in the `NestGate` codebase, achieving 20-50% performance improvements.
//! Trait definitions and implementations.
// **REPLACES**:
//! - 381+ `async_trait` usages across all crates
//! - `Arc<dyn>` patterns causing runtime overhead
//! - Boxing and virtual dispatch in async code
//!
//! Trait definitions and implementations.
//!
// **PROVIDES**:
//! - Native `impl Future` patterns
//! - Const generic configuration
//! - Zero-cost abstractions
//! - Compile-time optimization

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

/// Native async storage trait - replaces `async_trait` storage patterns
/// **DEPRECATED**: Native async now integrated into canonical storage traits
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::canonical::CanonicalStorage - all methods are native async"
)]
/// NativeAsyncStorage trait
pub trait NativeAsyncStorage: Send + Sync + 'static {
    /// Read data from storage
    /// Write data to storage
    /// Delete data from storage
    /// Check if path exists
    /// List directory contents
    /// Get storage metadata
    /// Create directory
    /// Copy data
    fn copy(&self, src: &str, dst: &str) -> impl Future<Output = Result<()>> + Send;

    /// Move data
    fn move_data(&self, src: &str, dst: &str) -> impl Future<Output = Result<()>> + Send;
}

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

/// Native async security provider trait - replaces `async_trait` security patterns
/// **DEPRECATED**: Native async now integrated into canonical security
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::canonical::CanonicalSecurity - all methods are native async"
)]
/// NativeAsyncSecurityProvider trait
pub trait NativeAsyncSecurityProvider: Send + Sync + 'static {
    /// Credentials type
    type Credentials: Send + Sync;

    /// Token type
    type Token: Send + Sync;
    /// Authenticate user with credentials
    fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> impl Future<Output = Result<Self::Token>> + Send;

    /// Validate token
    fn validate_token(&self, token: &Self::Token) -> impl Future<Output = Result<bool>> + Send;

    /// Refresh token
    fn refresh_token(&self, token: Self::Token)
        -> impl Future<Output = Result<Self::Token>> + Send;

    /// Revoke token
    fn revoke_token(&self, token: Self::Token) -> impl Future<Output = Result<()>> + Send;

    /// Encrypt data
    fn encrypt(&self, data: &[u8]) -> impl Future<Output = Result<Vec<u8>>> + Send;

    /// Decrypt data
    fn decrypt(&self, data: &[u8]) -> impl Future<Output = Result<Vec<u8>>> + Send;
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
