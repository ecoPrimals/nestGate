use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use crate::constants::{
    // Removed unused network constants
    // Removed unused storage constants
    system::DEFAULT_INSTANCE_NAME,
};

// ==================== SECTION ====================

/// **SERVICE TYPES** - Core service identification and management
pub mod service {
    use super::{Deserialize, HashMap, Serialize, SystemTime, DEFAULT_INSTANCE_NAME};

    /// Service identification
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub struct ServiceId(pub String);

    impl Default for ServiceId {
        fn default() -> Self {
            Self(uuid::Uuid::new_v4().to_string())
        }
    }

    /// Service states
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum ServiceState {
        /// Service is in the process of starting up
        Starting,
        /// Service is running and operational
        Running,
        /// Service is in the process of shutting down
        Stopping,
        /// Service has stopped and is not running
        Stopped,
        /// Service has encountered a failure and cannot operate
        Failed,
        /// Service is under maintenance and temporarily unavailable
        Maintenance,
        /// Service state is unknown or cannot be determined
        Unknown,
    }

    /// Service types
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum ServiceType {
        /// API service handling HTTP/REST requests
        Api,
        /// Storage service managing data persistence
        Storage,
        /// Network service handling connectivity and routing
        Network,
        /// Security service managing authentication and authorization
        Security,
        /// Monitoring service collecting metrics and health data
        Monitoring,
        /// Automation service handling scheduled tasks and workflows
        Automation,
        /// MCP (Model Context Protocol) service
        Mcp,
        /// ZFS storage management service
        Zfs,
        /// Custom service type with user-defined name
        Custom(String),
        /// Generic service type for unspecified services
        Generic,
        /// Compute service handling processing workloads
        Compute,
    }

    /// Service configuration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServiceConfig {
        /// Unique service identifier
        pub id: ServiceId,
        /// Human-readable service name
        pub name: String,
        /// Type of service
        pub service_type: ServiceType,
        /// Current operational state
        pub state: ServiceState,
        /// Network port if applicable
        pub port: Option<u16>,
        /// Host address if applicable
        pub host: Option<String>,
        /// Additional service metadata
        pub metadata: HashMap<String, String>,
        /// Timestamp when service was created
        pub created_at: SystemTime,
        /// Timestamp of last update
        pub updated_at: SystemTime,
    }
    /// **CANONICAL SERVICE INFO** - Consolidates all `ServiceInfo` definitions
    ///
    /// This replaces duplicate `ServiceInfo` structs from:
    /// - diagnostics/types.rs
    /// - automation/src/types/ecosystem.rs  
    /// - diagnostics/metrics.rs
    /// - And other scattered definitions
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServiceInfo {
        // Core identification
        /// Unique service identifier
        pub service_id: String,
        /// Primary service name
        pub service_name: String,
        /// Service name (backward compatibility alias)
        pub name: String,
        /// Service version string
        pub version: String,

        // Status and health
        /// Current operational status
        pub status: ServiceState,
        /// Human-readable health status
        pub health_status: String,
        /// Health status (backward compatibility)
        pub health: Option<String>,

        // Runtime information
        /// Service uptime in seconds
        pub uptime_seconds: Option<u64>,
        /// Process ID if available
        pub pid: Option<u32>,
        /// Service start timestamp
        pub start_time: Option<SystemTime>,

        // Performance metrics
        /// CPU usage percentage
        pub cpu_percent: Option<f64>,
        /// Memory usage in bytes
        pub memory_bytes: Option<u64>,

        // Configuration and metadata
        /// Service capabilities list
        pub capabilities: Vec<String>,
        /// Additional metadata key-value pairs
        pub metadata: HashMap<String, String>,
        /// Optional service description
        pub description: Option<String>,
    }

    impl Default for ServiceInfo {
        fn default() -> Self {
            Self {
                service_id: ServiceId::default().0,
                service_name: DEFAULT_INSTANCE_NAME.to_string(),
                name: DEFAULT_INSTANCE_NAME.to_string(),
                version: "1.0.0".to_string(),
                status: ServiceState::Unknown,
                health_status: "unknown".to_string(),
                health: None,
                uptime_seconds: None,
                pid: None,
                start_time: None,
                cpu_percent: None,
                memory_bytes: None,
                capabilities: Vec::new(),
                metadata: HashMap::new(),
                description: None,
            }
        }
    }

    /// Service metrics
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServiceMetrics {
        /// CPU usage as a percentage (0-100)
        pub cpu_usage_percent: f64,
        /// Memory usage in bytes
        pub memory_usage_bytes: u64,
        /// Service uptime in seconds
        pub uptime_seconds: u64,
        /// Average requests per second
        pub requests_per_second: f64,
        /// Error rate as a percentage (0-100)
        pub error_rate_percent: f64,
        /// Timestamp of last metrics update
        pub last_updated: SystemTime,
    }

    /// Service metadata
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServiceMetadata {
        /// Unique service identifier
        pub service_id: String,
        /// Type of service
        pub service_type: ServiceType,
        /// Timestamp when service was created
        pub created_at: SystemTime,
        /// Current health/operational status
        pub health_status: ServiceState,
        /// List of service capabilities
        pub capabilities: Vec<String>,
        /// Service endpoint URLs
        pub endpoints: HashMap<String, String>,
        /// Service configuration parameters
        pub configuration: HashMap<String, String>,
    }
}

/// **NETWORK TYPES** - Network communication and connectivity
pub mod network {
    use crate::canonical_modernization::canonical_constants::network::{
        DEFAULT_API_PORT, LOCALHOST,
    };
    use serde::{Deserialize, Serialize};

    /// Connection status
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum ConnectionStatus {
        /// Connection is established and active
        Connected,
        /// Connection is not established
        Disconnected,
        /// Connection is in progress
        Connecting,
        /// Connection attempt failed
        Failed,
        /// Connection attempt timed out
        Timeout,
    }

    /// Network protocol types
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum Protocol {
        /// HTTP protocol
        Http,
        /// HTTPS (secure HTTP) protocol
        Https,
        /// TCP protocol
        Tcp,
        /// UDP protocol
        Udp,
        /// WebSocket protocol
        WebSocket,
        /// gRPC protocol
        Grpc,
        /// Custom protocol with specified name
        Custom(String),
    }

    /// Network endpoint
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Endpoint {
        /// Hostname or IP address
        pub host: String,
        /// Port number
        pub port: u16,
        /// Network protocol
        pub protocol: Protocol,
    }

    impl Default for Endpoint {
        fn default() -> Self {
            Self {
                host: LOCALHOST.to_string(),
                port: DEFAULT_API_PORT,
                protocol: Protocol::Http,
            }
        }
    }
}
/// **SYSTEM TYPES** - Core system status and resource management
pub mod system {
    use serde::{Deserialize, Serialize};

    /// Allocation status for resources
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum AllocationStatus {
        /// Resource is actively allocated
        Active,
        /// Resource is not allocated
        Inactive,
        /// Resource allocation is pending
        Pending,
        /// Resource allocation has failed
        Failed,
    }

    impl Default for AllocationStatus {
        fn default() -> Self {
            Self::Inactive
        }
    }
}
/// **STORAGE TYPES** - Storage operations and management
pub mod storage {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::time::SystemTime;

    /// Storage tiers for data classification
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum StorageTier {
        /// Frequent access - NVMe/SSD tier
        Hot,
        /// Regular access - SSD tier
        Warm,
        /// Infrequent access - HDD tier
        Cold,
        /// Ultra-fast cache - RAM/NVMe cache tier
        Cache,
        /// Long-term storage - Tape/Cloud tier
        Archive,
    }

    /// Storage operation types
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum StorageOperation {
        /// Read data from storage
        Read,
        /// Write data to storage
        Write,
        /// Delete data from storage
        Delete,
        /// Copy data within storage
        Copy,
        /// Move data within storage
        Move,
        /// Backup data to another location
        Backup,
        /// Restore data from backup
        Restore,
        /// Compress data for storage efficiency
        Compress,
        /// Decompress data for access
        Decompress,
    }

    /// Storage metadata
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StorageMetadata {
        /// Size of the stored data in bytes
        pub size_bytes: u64,
        /// Timestamp when the data was created
        pub created_at: SystemTime,
        /// Timestamp when the data was last modified
        pub modified_at: SystemTime,
        /// Timestamp when the data was last accessed
        pub accessed_at: SystemTime,
        /// Storage tier where the data resides
        pub tier: StorageTier,
        /// Whether the data is compressed
        pub compressed: bool,
        /// Whether the data is encrypted
        pub encrypted: bool,
        /// Optional checksum for data integrity verification
        pub checksum: Option<String>,
        /// User-defined tags for categorization and metadata
        pub tags: HashMap<String, String>,
    }

    /// Storage resource
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StorageResource {
        /// Unique identifier for the resource
        pub id: String,
        /// Metadata about the storage resource
        pub metadata: StorageMetadata,
        /// Optional permissions string
        pub permissions: Option<String>,
        /// Optional owner identifier
        pub owner: Option<String>,
    }
}
/// **SECURITY TYPES** - Authentication, authorization, and encryption
pub mod security {
    use serde::{Deserialize, Serialize};
    use std::time::SystemTime;

    /// Authentication methods
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AuthMethod {
        /// Token-based authentication
        Token,
        /// API key authentication
        ApiKey,
        /// Certificate-based authentication
        Certificate,
        /// OAuth2 authentication
        OAuth2,
        /// Basic authentication (username/password)
        Basic,
        /// No authentication required
        None,
    }

    /// Access levels
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessLevel {
        /// Read-only access
        Read,
        /// Read and write access
        Write,
        /// Administrative access
        Admin,
        /// Owner-level access with full control
        Owner,
    }

    /// Security context
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SecurityContext {
        /// Optional user identifier
        pub user_id: Option<String>,
        /// User roles for role-based access control
        pub roles: Vec<String>,
        /// Granted permissions
        pub permissions: Vec<String>,
        /// Access level for this context
        pub access_level: AccessLevel,
        /// Authentication method used
        pub auth_method: AuthMethod,
        /// Optional token expiration timestamp
        pub token_expires_at: Option<SystemTime>,
    }
}
/// **EVENT TYPES** - Event handling and processing
pub mod events {
    use crate::canonical_modernization::canonical_constants::system::DEFAULT_SERVICE_NAME;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::time::SystemTime;

    /// Event severity levels
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
    pub enum EventSeverity {
        /// Debug-level events for detailed troubleshooting
        Debug,
        /// Informational events for general logging
        Info,
        /// Warning events for potential issues
        Warning,
        /// Error events for failures
        Error,
        /// Critical events requiring immediate attention
        Critical,
    }

    /// Event categories
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum EventCategory {
        /// System-level events
        System,
        /// Security-related events
        Security,
        /// Network events
        Network,
        /// Storage events
        Storage,
        /// User action events
        User,
        /// Application-level events
        Application,
        /// Performance-related events
        Performance,
        /// Custom event category
        Custom(String),
    }

    /// Event structure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Event {
        /// Unique event identifier
        pub id: String,
        /// Timestamp when the event occurred
        pub timestamp: SystemTime,
        /// Category of the event
        pub category: EventCategory,
        /// Severity level of the event
        pub severity: EventSeverity,
        /// Human-readable event message
        pub message: String,
        /// Source that generated the event
        pub source: String,
        /// Additional structured data for the event
        pub data: HashMap<String, serde_json::Value>,
        /// Tags for event categorization and filtering
        pub tags: Vec<String>,
    }

    impl Default for Event {
        fn default() -> Self {
            Self {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: SystemTime::now(),
                category: EventCategory::System,
                severity: EventSeverity::Info,
                message: "Default event".to_string(),
                source: DEFAULT_SERVICE_NAME.to_string(),
                data: HashMap::new(),
                tags: Vec::new(),
            }
        }
    }
}
/// **REQUEST/RESPONSE TYPES** - API communication patterns
pub mod api {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::time::SystemTime;

    /// Request structure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request {
        /// Unique request identifier
        pub id: String,
        /// HTTP method (GET, POST, etc.)
        pub method: String,
        /// Request headers
        pub headers: HashMap<String, String>,
        /// Optional request body
        pub body: Option<serde_json::Value>,
        /// Timestamp when the request was received
        pub timestamp: SystemTime,
    }

    /// Response structure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        /// Associated request identifier
        pub request_id: String,
        /// HTTP status code
        pub status_code: u16,
        /// Response headers
        pub headers: HashMap<String, String>,
        /// Optional response body
        pub body: Option<serde_json::Value>,
        /// Timestamp when the response was generated
        pub timestamp: SystemTime,
        /// Processing time in milliseconds
        pub processing_time_ms: u64,
    }

    /// API error structure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ApiError {
        /// Error code for categorization
        pub code: String,
        /// Human-readable error message
        pub message: String,
        /// Optional additional error details
        pub details: Option<serde_json::Value>,
        /// Timestamp when the error occurred
        pub timestamp: SystemTime,
    }
}
/// **HEALTH TYPES** - System health monitoring
pub mod health {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::time::SystemTime;

    /// Health status
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum HealthStatus {
        /// System is fully operational
        Healthy,
        /// System is operational but with reduced performance
        Degraded,
        /// System is not operational
        Unhealthy,
        /// Health status cannot be determined
        Unknown,
    }

    /// Health check result
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HealthCheck {
        /// Component being checked
        pub component: String,
        /// Health status of the component
        pub status: HealthStatus,
        /// Optional message with additional details
        pub message: Option<String>,
        /// Timestamp of the health check
        pub timestamp: SystemTime,
        /// Response time in milliseconds
        pub response_time_ms: u64,
        /// Additional metadata about the health check
        pub metadata: HashMap<String, serde_json::Value>,
    }

    /// System health summary
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SystemHealth {
        /// Overall system health status
        pub overall_status: HealthStatus,
        /// Individual component health checks
        pub components: Vec<HealthCheck>,
        /// Timestamp of last health check update
        pub last_updated: SystemTime,
        /// System uptime in seconds
        pub uptime_seconds: u64,
    }
}
/// **HANDLER CONFIGURATION TYPES** - Unified handler configuration system
pub mod handlers {
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    /// Universal handler configuration pattern
    /// Replaces all scattered handler-specific config structs
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UniversalHandlerConfig<T = ()> {
        /// Handler identification
        pub handler_id: String,
        /// Enable/disable handler
        pub enabled: bool,
        /// Request timeout
        pub timeout: Duration,
        /// Maximum concurrent requests
        pub max_concurrent_requests: usize,
        /// Rate limiting configuration
        pub rate_limit: RateLimitConfig,
        /// Retry configuration
        pub retry: RetryConfig,
        /// Monitoring configuration
        pub monitoring: MonitoringConfig,
        /// Handler-specific configuration
        pub specific: T,
    }
    /// Rate limiting configuration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RateLimitConfig {
        /// Enable rate limiting
        pub enabled: bool,
        /// Requests per minute
        pub requests_per_minute: u32,
        /// Burst size
        pub burst_size: u32,
    }

    /// Retry configuration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RetryConfig {
        /// Enable retries
        pub enabled: bool,
        /// Maximum retry attempts
        pub max_attempts: u32,
        /// Base delay between retries
        pub base_delay: Duration,
        /// Maximum delay between retries
        pub max_delay: Duration,
        /// Use exponential backoff
        pub exponential_backoff: bool,
    }

    /// Monitoring configuration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MonitoringConfig {
        /// Enable monitoring
        pub enabled: bool,
        /// Metrics collection interval
        pub metrics_interval: Duration,
        /// Health check interval
        pub health_check_interval: Duration,
        /// Enable performance monitoring
        pub performance_monitoring: bool,
    }

    // Default implementations
    impl<T: Default> Default for UniversalHandlerConfig<T> {
        fn default() -> Self {
            Self {
                handler_id: String::new(),
                enabled: true,
                timeout: Duration::from_secs(crate::canonical_modernization::canonical_constants::handlers::DEFAULT_HANDLER_TIMEOUT_SECS),
                max_concurrent_requests: crate::canonical_modernization::canonical_constants::handlers::MAX_CONCURRENT_REQUESTS,
                rate_limit: RateLimitConfig::default(),
                retry: RetryConfig::default(),
                monitoring: MonitoringConfig::default(),
                specific: T::default(),
            }
        }
    }

    impl Default for RateLimitConfig {
        fn default() -> Self {
            Self {
                enabled: false,
                requests_per_minute: crate::canonical_modernization::canonical_constants::handlers::DEFAULT_RATE_LIMIT_RPM,
                burst_size: crate::canonical_modernization::canonical_constants::handlers::DEFAULT_RATE_LIMIT_BURST,
            }
        }
    }

    impl Default for RetryConfig {
        fn default() -> Self {
            Self {
                enabled: true,
                max_attempts: crate::canonical_modernization::canonical_constants::handlers::DEFAULT_RETRY_ATTEMPTS,
                base_delay: Duration::from_millis(crate::canonical_modernization::canonical_constants::handlers::DEFAULT_RETRY_DELAY_MS),
                max_delay: Duration::from_secs(30),
                exponential_backoff: true,
            }
        }
    }

    impl Default for MonitoringConfig {
        fn default() -> Self {
            Self {
                enabled: true,
                metrics_interval: Duration::from_secs(crate::canonical_modernization::canonical_constants::handlers::METRICS_COLLECTION_INTERVAL_SECS),
                health_check_interval: Duration::from_secs(crate::canonical_modernization::canonical_constants::handlers::HEALTH_CHECK_INTERVAL_SECS),
                performance_monitoring: true,
            }
        }
    }
}

// ==================== SECTION ====================

// Universal request/response types removed - use domain-specific types instead
// Migration: UniversalRequest → Domain-specific request types
// Migration: UniversalResponse → Domain-specific response types

/// Response status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseStatus {
    /// Operation completed successfully
    Success,
    /// Operation failed with an error
    Error,
    /// Operation completed partially
    Partial,
    /// Operation timed out
    Timeout,
    /// Requested resource was not found
    NotFound,
    /// Unauthorized access attempted
    Unauthorized,
    /// Access forbidden
    Forbidden,
}
// ==================== SECTION ====================

/// Type conversion utilities for canonical types
pub mod conversion {
    use super::{ServiceState, ServiceType};

    /// Convert legacy service state string to canonical type
    #[must_use]
    pub fn parse_service_state(state: &str) -> ServiceState {
        match state.to_lowercase().as_str() {
            "starting" => ServiceState::Starting,
            "running" => ServiceState::Running,
            "stopping" => ServiceState::Stopping,
            "stopped" => ServiceState::Stopped,
            "failed" => ServiceState::Failed,
            _ => ServiceState::Unknown,
        }
    }

    /// Convert legacy service type string to canonical type
    #[must_use]
    pub fn parse_service_type(service_type: &str) -> ServiceType {
        match service_type.to_lowercase().as_str() {
            "storage" => ServiceType::Storage,
            "network" => ServiceType::Network,
            "security" => ServiceType::Security,
            "monitoring" => ServiceType::Monitoring,
            "compute" => ServiceType::Compute,
            "api" => ServiceType::Api,
            _ => ServiceType::Generic,
        }
    }
}

// ==================== SECTION ====================

/// Common types re-exported for easy access
pub use api::{ApiError, Request, Response};
pub use events::{Event, EventCategory, EventSeverity};
pub use health::{HealthCheck, HealthStatus, SystemHealth};
pub use network::{ConnectionStatus, Endpoint, Protocol};
pub use security::{AccessLevel, AuthMethod, SecurityContext};
pub use service::{ServiceConfig, ServiceId, ServiceMetrics, ServiceState, ServiceType};
pub use storage::{StorageMetadata, StorageOperation, StorageResource, StorageTier};
pub use system::AllocationStatus;
#[cfg(test)]
mod tests {
    use super::*;

    fn parse_service_state(state: &str) -> ServiceState {
        match state {
            "running" => ServiceState::Running,
            "stopped" => ServiceState::Stopped,
            "starting" => ServiceState::Starting,
            "stopping" => ServiceState::Stopping,
            _ => ServiceState::Failed,
        }
    }

    fn parse_service_type(service_type: &str) -> ServiceType {
        match service_type {
            "storage" => ServiceType::Storage,
            "network" => ServiceType::Network,
            "security" => ServiceType::Security,
            _ => ServiceType::Storage, // Default fallback
        }
    }

    #[test]
    fn test_service_types() {
        let storage = ServiceType::Storage;
        let network = ServiceType::Network;
        assert_ne!(storage, network);
    }

    #[test]
    fn test_conversion_utilities() {
        assert_eq!(parse_service_state("running"), ServiceState::Running);
        assert_eq!(parse_service_type("storage"), ServiceType::Storage);
    }
}
