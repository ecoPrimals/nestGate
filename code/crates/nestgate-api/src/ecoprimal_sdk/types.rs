//
// This module contains the fundamental data types, enums, and structures
// used throughout the EcoPrimal ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Primal _metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Primalmetadata
pub struct PrimalMetadata {
    /// Primal name (e.g., "NestGate")
    pub name: String,
    /// Primal version
    pub version: String,
    /// Primal type category
    pub primal_type: PrimalType,
    /// Primal description
    pub description: String,
    /// Primal maintainer/author
    pub maintainer: String,
    /// Repository URL
    pub repository: Option<String>,
    /// Documentation URL
    pub documentation: Option<String>,
    /// License information
    pub license: String,
    /// Supported platforms
    pub supported_platforms: Vec<String>,
    /// Minimum management version required
    pub min_management_version: String,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}
/// Primal types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of Primal
pub enum PrimalType {
    /// Core system primals (authentication, routing, etc.)
    Core,
    /// Infrastructure primals (storage, networking, monitoring)
    Infrastructure,
    /// Application primals (web servers, databases, etc.)
    Application,
    /// Development primals (build tools, testing frameworks)
    Development,
    /// Community-contributed primals
    Community,
    /// Experimental/beta primals
    Experimental,
}
/// Primal capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Primalcapability
pub enum PrimalCapability {
    /// HTTP/HTTPS server capabilities
    HttpServer,
    /// Database operations
    Database,
    /// File system operations
    FileSystem,
    /// Network operations
    Network,
    /// Authentication services
    Authentication,
    /// Authorization services
    Authorization,
    /// Monitoring and metrics
    Monitoring,
    /// Logging services
    Logging,
    /// Caching services
    Cache,
    /// Message queue operations
    MessageQueue,
    /// Container orchestration
    Orchestration,
    /// Service discovery
    ServiceDiscovery,
    /// Load balancing
    LoadBalancing,
    /// SSL/TLS termination
    TlsTermination,
    /// API gateway functionality
    ApiGateway,
    /// Custom capability (with string identifier)
    Custom(String),
}
/// Primal health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Primalhealth
pub enum PrimalHealth {
    /// Primal is healthy and operational
    Healthy,
    /// Primal is degraded but still operational
    Degraded,
    /// Primal is unhealthy and may not be operational
    Unhealthy,
    /// Primal health is unknown
    Unknown,
}
/// Primal request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Primal operation
pub struct PrimalRequest {
    /// Request ID for tracking
    pub request_id: String,
    /// HTTP method (if applicable)
    pub method: Option<String>,
    /// Request path or command
    pub path: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request body/payload
    pub body: Option<Vec<u8>>,
    /// Query parameters
    pub query: HashMap<String, String>,
    /// Target capability for this request
    pub target_capability: Option<PrimalCapability>,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Client IP address
    pub client_ip: Option<String>,
}
/// Primal response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Primal operation
pub struct PrimalResponse {
    /// Response status code
    pub status_code: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: Option<Vec<u8>>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
    /// Processing duration in milliseconds
    pub duration_ms: u64,
}
/// Primal metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Primalmetrics
pub struct PrimalMetrics {
    /// Request count
    pub request_count: u64,
    /// Error count
    pub error_count: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Current memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Current CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
}
/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::LoggingConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::LoggingConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for Logging
pub struct LoggingConfig {
    /// Log level (error, warn, info, debug, trace)
    pub level: String,
    /// Log format (json, text)
    pub format: String,
    /// Log destination (stdout, file, syslog)
    pub destination: String,
    /// Log file path (if destination is file)
    pub file_path: Option<String>,
    /// Enable structured logging
    pub structured: bool,
    /// Custom logging configuration
    pub custom: HashMap<String, serde_json::Value>,
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Loggingconfigcanonical
pub type LoggingConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using LoggingConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

