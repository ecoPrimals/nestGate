// Removed unused error imports
/// Universal Interface Standards
///
/// This module defines the standardized interfaces that eliminate inconsistency
/// across all NestGate components. All services should implement these unified
/// traits to ensure consistent API patterns.
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

/// Universal result type for all interface operations
pub type InterfaceResult<T> = Result<T, InterfaceError>;

/// Unified error type for all interface operations
#[derive(Debug, thiserror::Error, Clone, Serialize, Deserialize)]
pub enum InterfaceError {
    #[error("Service unavailable: {message}")]
    ServiceUnavailable { message: String },
    #[error("Configuration error: {message}")]
    Configuration { message: String },
    #[error("Invalid input: {field} - {reason}")]
    InvalidInput { field: String, reason: String },
    #[error("Operation timeout: {operation}")]
    Timeout { operation: String },
    #[error("Permission denied: {operation}")]
    PermissionDenied { operation: String },
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },
    #[error("Internal error: {message}")]
    Internal { message: String },
}

/// Unified health status standard
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnifiedHealthStatus {
    /// Service health state
    pub status: HealthState,
    /// Human-readable status message
    pub message: String,
    /// Timestamp of health check
    pub timestamp: DateTime<Utc>,
    /// Additional health metrics
    pub metrics: HashMap<String, f64>,
    /// Service version
    pub version: String,
    /// Uptime in seconds
    pub uptime_seconds: u64,
}

/// Standard health states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthState {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Unified service metrics standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedServiceMetrics {
    /// Service identifier
    pub service_id: String,
    /// Request count
    pub request_count: u64,
    /// Error count
    pub error_count: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// 95th percentile response time
    pub p95_response_time_ms: f64,
    /// 99th percentile response time
    pub p99_response_time_ms: f64,
    /// CPU usage percentage
    pub _cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Active connections
    pub active_connections: u64,
    /// Queue depth
    pub queue_depth: u64,
    /// Throughput (requests per second)
    pub throughput_rps: f64,
    /// Error rate percentage
    pub error_rate: f64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

/// Unified service information standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedServiceInfo {
    /// Unique service identifier
    pub service_id: String,
    /// Human-readable service name
    pub name: String,
    /// Service version
    pub version: String,
    /// Service type/category
    pub service_type: String,
    /// Service description
    pub description: String,
    /// Supported capabilities
    pub capabilities: Vec<String>,
    /// API endpoints
    pub endpoints: HashMap<String, String>,
    /// Configuration schema
    pub configuration_schema: Option<serde_json::Value>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
}

/// Unified request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedRequest {
    /// Unique request identifier
    pub request_id: Uuid,
    /// Request operation/method
    pub operation: String,
    /// Request parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Security context
    pub security_context: Option<SecurityContext>,
    /// Request metadata
    pub metadata: HashMap<String, String>,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Request timeout
    pub timeout: Option<Duration>,
    /// Request priority (0-10, higher is more priority)
    pub priority: u8,
}

/// Unified response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedResponse {
    /// Request ID this response corresponds to
    pub request_id: Uuid,
    /// Response status
    pub status: ResponseStatus,
    /// Response data
    pub data: Option<serde_json::Value>,
    /// Error information (if status is Error)
    pub error: Option<InterfaceError>,
    /// Response metadata
    pub metadata: HashMap<String, String>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
    /// Processing duration
    pub processing_duration_ms: u64,
}

/// Unified response status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseStatus {
    Success,
    Error,
    Timeout,
    Accepted, // For async operations
}

/// Security context for requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Authentication token
    pub auth_token: Option<String>,
    /// User/service identity
    pub identity: String,
    /// Permissions
    pub permissions: Vec<String>,
    /// Security level required
    pub security_level: SecurityLevel,
}

/// Security levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityLevel {
    Public,
    Internal,
    Restricted,
    Confidential,
}

/// Universal Service Interface - The ONE interface all services should implement
#[async_trait]
pub trait UniversalServiceInterface: Send + Sync {
    /// Get service information
    fn service_info(&self) -> UnifiedServiceInfo;

    /// Perform health check
    async fn health_check(&self) -> InterfaceResult<UnifiedHealthStatus>;

    /// Get service metrics
    async fn get_metrics(&self) -> InterfaceResult<UnifiedServiceMetrics>;

    /// Handle unified request
    async fn handle_request(&self, request: UnifiedRequest) -> UnifiedResponse;

    /// Initialize service with configuration
    async fn initialize(&mut self, config: serde_json::Value) -> InterfaceResult<()>;

    /// Start the service
    async fn start(&mut self) -> InterfaceResult<()>;

    /// Stop the service gracefully
    async fn stop(&mut self) -> InterfaceResult<()>;

    /// Update configuration at runtime
    async fn update_config(&mut self, config: serde_json::Value) -> InterfaceResult<()>;

    /// Check if service supports a capability
    fn supports_capability(&self, capability: &str) -> bool;

    /// Get configuration schema
    fn get_configuration_schema(&self) -> Option<serde_json::Value>;
}

/// Provider Interface - For external service integrations
#[async_trait]
pub trait UniversalProviderInterface: UniversalServiceInterface {
    /// Provider type identifier
    fn provider_type(&self) -> &str;

    /// Get provider capabilities
    fn get_capabilities(&self) -> Vec<String>;

    /// Execute provider-specific operation
    async fn execute_operation(
        &self,
        operation: &str,
        parameters: HashMap<String, serde_json::Value>,
    ) -> InterfaceResult<serde_json::Value>;

    /// Register with external ecosystem
    async fn register_with_ecosystem(&self) -> InterfaceResult<String>;

    /// Deregister from external ecosystem
    async fn deregister_from_ecosystem(&self) -> InterfaceResult<()>;
}

/// Storage Interface - For storage-related services
#[async_trait]
pub trait UniversalStorageInterface: UniversalServiceInterface {
    /// List storage resources
    async fn list_resources(&self) -> InterfaceResult<Vec<StorageResource>>;

    /// Get resource details
    async fn get_resource(&self, resource_id: &str) -> InterfaceResult<Option<StorageResource>>;

    /// Create storage resource
    async fn create_resource(
        &self,
        config: StorageResourceConfig,
    ) -> InterfaceResult<StorageResource>;

    /// Delete storage resource
    async fn delete_resource(&self, resource_id: &str) -> InterfaceResult<()>;

    /// Get storage metrics
    async fn get_storage_metrics(&self) -> InterfaceResult<StorageMetrics>;
}

/// Storage resource representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResource {
    pub id: String,
    pub name: String,
    pub resource_type: String,
    pub size_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub properties: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Storage resource configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceConfig {
    pub name: String,
    pub resource_type: String,
    pub size_bytes: Option<u64>,
    pub properties: HashMap<String, String>,
}

/// Storage-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    pub total_capacity_bytes: u64,
    pub used_capacity_bytes: u64,
    pub available_capacity_bytes: u64,
    pub utilization_percentage: f64,
    pub iops: u64,
    pub throughput_bytes_per_sec: u64,
    pub average_latency_ms: f64,
    pub error_count: u64,
    pub timestamp: DateTime<Utc>,
}

/// Configuration Management Interface
#[async_trait]
pub trait UniversalConfigInterface: Send + Sync {
    /// Get current configuration
    async fn get_config(&self) -> InterfaceResult<serde_json::Value>;

    /// Update configuration
    async fn update_config(&mut self, config: serde_json::Value) -> InterfaceResult<()>;

    /// Validate configuration
    async fn validate_config(&self, config: serde_json::Value) -> InterfaceResult<bool>;

    /// Get configuration schema
    fn get_config_schema(&self) -> serde_json::Value;

    /// Reset to default configuration
    async fn reset_config(&mut self) -> InterfaceResult<()>;
}

/// Event Interface - For event-driven services
#[async_trait]
pub trait UniversalEventInterface: Send + Sync {
    /// Publish event
    async fn publish_event(&self, event: UnifiedEvent) -> InterfaceResult<()>;

    /// Subscribe to events
    async fn subscribe(&self, event_types: Vec<String>) -> InterfaceResult<String>;

    /// Unsubscribe from events
    async fn unsubscribe(&self, subscription_id: &str) -> InterfaceResult<()>;
}

/// Unified event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub source_service: String,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

/// Trait for converting existing interfaces to unified interface
pub trait ToUnified<T> {
    fn to_unified(&self) -> T;
}

/// Trait for converting from unified interface to existing interfaces
pub trait FromUnified<T> {
    fn from_unified(unified: T) -> Self;
}

impl Default for UnifiedHealthStatus {
    fn default() -> Self {
        Self {
            status: HealthState::Unknown,
            message: "No health check performed".to_string(),
            timestamp: Utc::now(),
            metrics: HashMap::new(),
            version: "unknown".to_string(),
            uptime_seconds: 0,
        }
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            auth_token: None,
            identity: "anonymous".to_string(),
            permissions: vec![],
            security_level: SecurityLevel::Public,
        }
    }
}
