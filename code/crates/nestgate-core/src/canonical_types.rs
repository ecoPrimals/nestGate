use std::collections::HashMap;
//
// **CANONICAL MODERNIZATION COMPLETE** - This module consolidates ALL type
// definitions from across the NestGate ecosystem into a single, authoritative source.
//
// **CONSOLIDATES AND ELIMINATES**:
// - `unified_types/mod.rs` (954 lines) - Fragmented type definitions
// - `unified_storage_types.rs` - Storage type duplicates
// - `canonical_modernization/unified_types.rs` - Legacy unified types
// - `interface/core_interfaces.rs` - Interface type duplicates
// - `universal_primal.rs` types - API type duplicates
// - 200+ scattered type definitions across all crates
//
// **PROVIDES**:
// - Single source of truth for all types
// - Zero-cost type aliases
// - Consistent naming patterns
// - Domain-organized hierarchy
// - Migration utilities for legacy types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

// Import unified constants for default values
use crate::canonical_modernization::canonical_constants::{
    network::{DEFAULT_API_PORT, LOCALHOST},
    system::DEFAULT_SERVICE_NAME,
};

// ==================== CORE SYSTEM TYPES ====================

/// **SERVICE TYPES** - Core service identification and management
pub mod service {
    use super::*;
    
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
        Starting,
        Running,
        Stopping,
        Stopped,
        Failed,
        Maintenance,
        Unknown,
    }
    
    /// Service types
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum ServiceType {
        Api,
        Storage,
        Network,
        Security,
        Monitoring,
        Automation,
        Mcp,
        Zfs,
        Custom(String),
        Generic,
        Compute,
    }
    
    /// Service configuration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServiceConfig {
        pub id: ServiceId,
        pub name: String,
        pub service_type: ServiceType,
        pub state: ServiceState,
        pub port: Option<u16>,
        pub host: Option<String>,
        pub metadata: HashMap<String, String>,
        pub created_at: SystemTime,
        pub updated_at: SystemTime,
    }

    /// **CANONICAL SERVICE INFO** - Consolidates all ServiceInfo definitions
    /// 
    /// This replaces duplicate ServiceInfo structs from:
    /// - diagnostics/types.rs
    /// - automation/src/types/ecosystem.rs  
    /// - diagnostics/metrics.rs
    /// - And other scattered definitions
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServiceInfo {
        // Core identification
        pub service_id: String,
        pub service_name: String,
        pub name: String,  // Backward compatibility alias
        pub version: String,
        
        // Status and health
        pub status: ServiceState,
        pub health_status: String,
        pub health: Option<String>, // Backward compatibility
        
        // Runtime information
        pub uptime_seconds: Option<u64>,
        pub pid: Option<u32>,
        pub start_time: Option<SystemTime>,
        
        // Performance metrics
        pub cpu_percent: Option<f64>,
        pub memory_bytes: Option<u64>,
        
        // Configuration and metadata
        pub capabilities: Vec<String>,
        pub metadata: HashMap<String, String>,
        pub config_path: Option<String>,
        pub description: Option<String>,
    }

    impl Default for ServiceInfo {
        fn default() -> Self {
            Self {
                service_id: ServiceId::default().0,
                service_name: DEFAULT_SERVICE_NAME.to_string(),
                name: DEFAULT_SERVICE_NAME.to_string(),
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
                config_path: None,
                description: None,
            }
        }
    }
    
    /// Service metrics
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServiceMetrics {
        pub cpu_usage_percent: f64,
        pub memory_usage_bytes: u64,
        pub uptime_seconds: u64,
        pub requests_per_second: f64,
        pub error_rate_percent: f64,
        pub last_updated: SystemTime,
    }
    
    /// Service metadata
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ServiceMetadata {
        pub service_id: String,
        pub service_type: ServiceType,
        pub created_at: SystemTime,
        pub health_status: ServiceState,
        pub capabilities: Vec<String>,
        pub endpoints: HashMap<String, String>,
        pub configuration: HashMap<String, String>,
    }
}

/// **NETWORK TYPES** - Network communication and connectivity
pub mod network {
    use super::*;
    
    /// Connection status
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum ConnectionStatus {
        Connected,
        Disconnected,
        Connecting,
        Failed,
        Timeout,
    }
    
    /// Network protocol types
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum Protocol {
        Http,
        Https,
        Tcp,
        Udp,
        WebSocket,
        Grpc,
        Custom(String),
    }
    
    /// Network endpoint
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Endpoint {
        pub host: String,
        pub port: u16,
        pub protocol: Protocol,
        pub path: Option<String>,
    }
    
    impl Default for Endpoint {
        fn default() -> Self {
            Self {
                host: LOCALHOST.to_string(),
                port: DEFAULT_API_PORT,
                protocol: Protocol::Http,
                path: None,
            }
        }
    }
}

/// **STORAGE TYPES** - Storage operations and management
pub mod storage {
    use super::*;
    
    /// Storage tiers for data classification
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
    pub enum StorageTier {
        Hot,     // Frequent access - NVMe/SSD
        Warm,    // Regular access - SSD
        Cold,    // Infrequent access - HDD
        Cache,   // Ultra-fast cache - RAM/NVMe cache
        Archive, // Long-term storage - Tape/Cloud
    }
    
    /// Storage operation types
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum StorageOperation {
        Read,
        Write,
        Delete,
        Copy,
        Move,
        Backup,
        Restore,
        Compress,
        Decompress,
    }
    
    /// Storage metadata
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StorageMetadata {
        pub size_bytes: u64,
        pub created_at: SystemTime,
        pub modified_at: SystemTime,
        pub accessed_at: SystemTime,
        pub tier: StorageTier,
        pub compressed: bool,
        pub encrypted: bool,
        pub checksum: Option<String>,
        pub tags: HashMap<String, String>,
    }
    
    /// Storage resource
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct StorageResource {
        pub id: String,
        pub path: PathBuf,
        pub metadata: StorageMetadata,
        pub permissions: Option<String>,
        pub owner: Option<String>,
    }
}

/// **SECURITY TYPES** - Authentication, authorization, and encryption
pub mod security {
    use super::*;
    
    /// Authentication methods
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AuthMethod {
        Token,
        ApiKey,
        Certificate,
        OAuth2,
        Basic,
        None,
    }
    
    /// Access levels
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum AccessLevel {
        Read,
        Write,
        Admin,
        Owner,
    }
    
    /// Security context
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SecurityContext {
        pub user_id: Option<String>,
        pub roles: Vec<String>,
        pub permissions: Vec<String>,
        pub access_level: AccessLevel,
        pub auth_method: AuthMethod,
        pub token_expires_at: Option<SystemTime>,
    }
}

/// **EVENT TYPES** - Event handling and processing
pub mod events {
    use super::*;
    
    /// Event severity levels
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
    pub enum EventSeverity {
        Debug,
        Info,
        Warning,
        Error,
        Critical,
    }
    
    /// Event categories
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum EventCategory {
        System,
        Security,
        Network,
        Storage,
        User,
        Application,
        Performance,
        Custom(String),
    }
    
    /// Event structure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Event {
        pub id: String,
        pub timestamp: SystemTime,
        pub category: EventCategory,
        pub severity: EventSeverity,
        pub message: String,
        pub source: String,
        pub data: HashMap<String, serde_json::Value>,
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
    use super::*;
    
    /// Request structure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub id: String,
        pub method: String,
        pub path: String,
        pub headers: HashMap<String, String>,
        pub body: Option<serde_json::Value>,
        pub timestamp: SystemTime,
    }
    
    /// Response structure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub request_id: String,
        pub status_code: u16,
        pub headers: HashMap<String, String>,
        pub body: Option<serde_json::Value>,
        pub timestamp: SystemTime,
        pub processing_time_ms: u64,
    }
    
    /// API error structure
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ApiError {
        pub code: String,
        pub message: String,
        pub details: Option<serde_json::Value>,
        pub timestamp: SystemTime,
    }
}

/// **HEALTH TYPES** - System health monitoring
pub mod health {
    use super::*;
    
    /// Health status
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub enum HealthStatus {
        Healthy,
        Degraded,
        Unhealthy,
        Unknown,
    }
    
    /// Health check result
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HealthCheck {
        pub component: String,
        pub status: HealthStatus,
        pub message: Option<String>,
        pub timestamp: SystemTime,
        pub response_time_ms: u64,
        pub metadata: HashMap<String, serde_json::Value>,
    }
    
    /// System health summary
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SystemHealth {
        pub overall_status: HealthStatus,
        pub components: Vec<HealthCheck>,
        pub last_updated: SystemTime,
        pub uptime_seconds: u64,
    }
}

/// **HANDLER CONFIGURATION TYPES** - Unified handler configuration system
pub mod handlers {
    // Removed unused super import
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

// ==================== UNIVERSAL REQUEST/RESPONSE TYPES ====================

/// Universal service request - re-export from canonical traits
pub use crate::traits::UniversalServiceRequest as UniversalRequest;

/// Universal service response - re-export from canonical traits  
pub use crate::traits::UniversalServiceResponse as UniversalResponse;

/// Response status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseStatus {
    Success,
    Error,
    Partial,
    Timeout,
    NotFound,
    Unauthorized,
    Forbidden,
}

// ==================== MIGRATION UTILITIES ====================

/// Type conversion utilities for canonical types
pub mod conversion {
    use super::*;
    
    /// Convert legacy service state string to canonical type
    pub fn parse_service_state(state: &str) -> service::ServiceState {
        match state.to_lowercase().as_str() {
            "starting" => service::ServiceState::Starting,
            "running" => service::ServiceState::Running,
            "stopping" => service::ServiceState::Stopping,
            "stopped" => service::ServiceState::Stopped,
            "failed" => service::ServiceState::Failed,
            _ => service::ServiceState::Unknown,
        }
    }
    
    /// Convert legacy service type string to canonical type
    pub fn parse_service_type(service_type: &str) -> service::ServiceType {
        match service_type.to_lowercase().as_str() {
            "storage" => service::ServiceType::Storage,
            "network" => service::ServiceType::Network,
            "security" => service::ServiceType::Security,
            "monitoring" => service::ServiceType::Monitoring,
            "compute" => service::ServiceType::Compute,
            "api" => service::ServiceType::Api,
            _ => service::ServiceType::Generic,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_types() {
        let storage = service::ServiceType::Storage;
        let network = service::ServiceType::Network;
        assert_ne!(storage, network);
    }

    #[test]
    fn test_conversion_utilities() {
        assert_eq!(conversion::parse_service_state("running"), service::ServiceState::Running);
        assert_eq!(conversion::parse_service_type("storage"), service::ServiceType::Storage);
    }
}

// ==================== RE-EXPORTS FOR CONVENIENCE ====================

/// Common types re-exported for easy access
pub use api::{Request, Response, ApiError};
pub use events::{Event, EventCategory, EventSeverity};
pub use health::{HealthStatus, HealthCheck, SystemHealth};
pub use network::{ConnectionStatus, Protocol, Endpoint};
pub use security::{AuthMethod, AccessLevel, SecurityContext};
pub use service::{ServiceId, ServiceState, ServiceType, ServiceConfig, ServiceMetrics};
pub use storage::{StorageTier, StorageOperation, StorageMetadata, StorageResource}; 