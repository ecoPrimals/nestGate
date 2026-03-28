// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # 🚀 tarpc Types and Traits for NestGate
//!
//! **HIGH-PERFORMANCE PRIMAL-TO-PRIMAL RPC** (v0.2.0)
//!
//! Provides shared types and service traits for tarpc-based communication.
//! This module defines the interface used by both clients and servers.
//!
//! ## Performance
//! - ~10-20 μs latency (vs 50-100 μs for JSON-RPC, 500-1000 μs for HTTP)
//! - ~100K requests/sec (vs 10K for JSON-RPC, 1K for HTTP)
//! - Zero-copy binary serialization with bincode
//! - Type-safe at compile time
//!
//! ## Philosophy (Primal Sovereignty)
//! - **tarpc PRIMARY** for primal-to-primal communication
//! - **JSON-RPC SECONDARY** for universal access
//! - **HTTP FALLBACK** for network-only scenarios
//! - **Self-knowledge**: NestGate knows only storage capabilities
//! - **Runtime discovery**: Other primals discovered via capability
//! - **Zero hardcoding**: No primal names, ports, or endpoints
//! - Zero unsafe blocks
//! - Modern async/await patterns

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// tarpc service trait for NestGate storage operations
///
/// This trait defines the async RPC interface using tarpc.
/// Both client and server implementations use this trait.
///
/// # Protocol Priority (Per Ecosystem Standard)
/// 1. **tarpc** (PRIMARY) - High-performance binary RPC for primal-to-primal
/// 2. **JSON-RPC** (SECONDARY) - Universal, human-friendly
/// 3. **HTTP** (FALLBACK) - Enableable for network scenarios
///
/// # Self-Knowledge
/// NestGate exposes only storage operations. Discovery of other primals
/// (orchestration, security, AI, compute, management) happens at runtime
/// through the universal adapter.
#[tarpc::service]
pub trait NestGateRpc {
    // ==================== STORAGE OPERATIONS ====================

    /// Create a new dataset
    ///
    /// # Arguments
    /// * `name` - Dataset name
    /// * `params` - Dataset configuration parameters
    ///
    /// # Returns
    /// Dataset information if successful
    async fn create_dataset(
        name: String,
        params: DatasetParams,
    ) -> Result<DatasetInfo, NestGateRpcError>;

    /// List all datasets
    ///
    /// # Returns
    /// List of all available datasets
    async fn list_datasets() -> Result<Vec<DatasetInfo>, NestGateRpcError>;

    /// Get dataset information
    ///
    /// # Arguments
    /// * `name` - Dataset name
    ///
    /// # Returns
    /// Dataset information if exists
    async fn get_dataset(name: String) -> Result<DatasetInfo, NestGateRpcError>;

    /// Delete a dataset
    ///
    /// # Arguments
    /// * `name` - Dataset name
    ///
    /// # Returns
    /// Success result
    async fn delete_dataset(name: String) -> Result<OperationResult, NestGateRpcError>;

    /// Store an object
    ///
    /// # Arguments
    /// * `dataset` - Dataset name
    /// * `key` - Object key
    /// * `data` - Object data
    /// * `metadata` - Optional metadata
    ///
    /// # Returns
    /// Object information after storage
    async fn store_object(
        dataset: String,
        key: String,
        data: Vec<u8>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<ObjectInfo, NestGateRpcError>;

    /// Retrieve an object
    ///
    /// # Arguments
    /// * `dataset` - Dataset name
    /// * `key` - Object key
    ///
    /// # Returns
    /// Object data if exists
    async fn retrieve_object(dataset: String, key: String) -> Result<Vec<u8>, NestGateRpcError>;

    /// Get object metadata
    ///
    /// # Arguments
    /// * `dataset` - Dataset name
    /// * `key` - Object key
    ///
    /// # Returns
    /// Object information without data
    async fn get_object_metadata(
        dataset: String,
        key: String,
    ) -> Result<ObjectInfo, NestGateRpcError>;

    /// List objects in dataset
    ///
    /// # Arguments
    /// * `dataset` - Dataset name
    /// * `prefix` - Optional key prefix filter
    /// * `limit` - Maximum number of results
    ///
    /// # Returns
    /// List of object information
    async fn list_objects(
        dataset: String,
        prefix: Option<String>,
        limit: Option<usize>,
    ) -> Result<Vec<ObjectInfo>, NestGateRpcError>;

    /// Delete an object
    ///
    /// # Arguments
    /// * `dataset` - Dataset name
    /// * `key` - Object key
    ///
    /// # Returns
    /// Success result
    async fn delete_object(
        dataset: String,
        key: String,
    ) -> Result<OperationResult, NestGateRpcError>;

    // ==================== CAPABILITY OPERATIONS ====================

    /// Register NestGate's capabilities with discovery system
    ///
    /// # Arguments
    /// * `registration` - Service registration information
    ///
    /// # Returns
    /// Registration result
    async fn register_capability(
        registration: CapabilityRegistration,
    ) -> Result<RegistrationResult, NestGateRpcError>;

    /// Discover services by capability (runtime discovery)
    ///
    /// # Arguments
    /// * `capability` - Required capability (e.g., "security", "orchestration", "compute")
    ///
    /// # Returns
    /// List of services matching the capability
    async fn discover_capability(capability: String) -> Result<Vec<ServiceInfo>, NestGateRpcError>;

    // ==================== HEALTH & MONITORING ====================

    /// Get health status
    ///
    /// # Returns
    /// Current health status of NestGate
    async fn health() -> HealthStatus;

    /// Get storage metrics
    ///
    /// # Returns
    /// Current storage metrics and statistics
    async fn metrics() -> StorageMetrics;

    /// Get version information
    ///
    /// # Returns
    /// Version and protocol information
    async fn version() -> VersionInfo;

    /// Get available protocols
    ///
    /// # Returns
    /// List of supported protocols (tarpc, JSON-RPC, HTTP)
    async fn protocols() -> Vec<ProtocolInfo>;
}

// ==================== TYPE DEFINITIONS ====================

/// Dataset creation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetParams {
    /// Dataset description
    pub description: Option<String>,

    /// Compression settings
    pub compression: Option<String>,

    /// Encryption enabled
    pub encrypted: bool,

    /// Deduplication enabled
    pub deduplicated: bool,

    /// Quota in bytes (None = unlimited)
    pub quota: Option<u64>,

    /// Additional custom properties
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

impl Default for DatasetParams {
    fn default() -> Self {
        Self {
            description: None,
            compression: Some("lz4".to_string()),
            encrypted: true,
            deduplicated: true,
            quota: None,
            properties: HashMap::new(),
        }
    }
}

/// Dataset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    /// Dataset name
    pub name: String,

    /// Dataset description
    pub description: Option<String>,

    /// Creation timestamp
    pub created_at: i64,

    /// Last modified timestamp
    pub modified_at: i64,

    /// Total size in bytes
    pub size_bytes: u64,

    /// Number of objects
    pub object_count: u64,

    /// Compression ratio (1.0 = no compression)
    pub compression_ratio: f64,

    /// Configuration parameters
    pub params: DatasetParams,

    /// Dataset status
    pub status: String,
}

/// Object information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInfo {
    /// Object key
    pub key: String,

    /// Dataset name
    pub dataset: String,

    /// Content size in bytes
    pub size_bytes: u64,

    /// Creation timestamp
    pub created_at: i64,

    /// Last modified timestamp
    pub modified_at: i64,

    /// Content type (MIME)
    pub content_type: Option<String>,

    /// Checksum (SHA-256)
    pub checksum: Option<String>,

    /// Encrypted
    pub encrypted: bool,

    /// Compressed
    pub compressed: bool,

    /// Custom metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    /// Success flag
    pub success: bool,

    /// Result message
    pub message: String,

    /// Operation metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Capability registration for discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRegistration {
    /// Service ID (UUID)
    pub service_id: String,

    /// Service name ("nestgate")
    pub service_name: String,

    /// Primary capability ("storage")
    pub capability: String,

    /// All capabilities provided
    pub capabilities: Vec<String>,

    /// tarpc endpoint
    pub tarpc_endpoint: String,

    /// JSON-RPC endpoint
    pub jsonrpc_endpoint: Option<String>,

    /// HTTP endpoint (if enabled)
    pub http_endpoint: Option<String>,

    /// Service metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Discovered service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service ID
    pub id: String,

    /// Service capability
    pub capability: String,

    /// Available endpoints by protocol
    pub endpoints: HashMap<String, String>,

    /// Service status
    pub status: String,

    /// Service metadata
    pub metadata: Option<serde_json::Value>,
}

/// Registration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationResult {
    /// Success flag
    pub success: bool,

    /// Result message
    pub message: String,
}

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Status string ("healthy", "degraded", "unhealthy")
    pub status: String,

    /// Service version
    pub version: String,

    /// Uptime in seconds
    pub uptime_seconds: u64,

    /// Total datasets
    pub total_datasets: usize,

    /// Total objects
    pub total_objects: u64,

    /// Storage used in bytes
    pub storage_used_bytes: u64,

    /// Additional health metrics
    #[serde(default)]
    pub metrics: HashMap<String, serde_json::Value>,
}

/// Storage metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    /// Total capacity in bytes
    pub total_capacity_bytes: u64,

    /// Used space in bytes
    pub used_space_bytes: u64,

    /// Available space in bytes
    pub available_space_bytes: u64,

    /// Number of datasets
    pub dataset_count: usize,

    /// Total number of objects
    pub object_count: u64,

    /// Average compression ratio
    pub avg_compression_ratio: f64,

    /// Deduplication ratio
    pub dedup_ratio: f64,

    /// Read operations per second
    pub read_ops_per_sec: f64,

    /// Write operations per second
    pub write_ops_per_sec: f64,

    /// Average read latency in milliseconds
    pub avg_read_latency_ms: f64,

    /// Average write latency in milliseconds
    pub avg_write_latency_ms: f64,
}

/// Version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Service version
    pub version: String,

    /// API version
    pub api_version: String,

    /// Supported protocol versions
    pub protocol_versions: Vec<String>,

    /// Build information
    pub build_info: Option<String>,
}

/// Protocol information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInfo {
    /// Protocol name ("tarpc", "jsonrpc", "http")
    pub protocol: String,

    /// Protocol version
    pub version: String,

    /// Connection endpoint
    pub endpoint: String,

    /// Priority (1 = highest)
    pub priority: u8,

    /// Enabled flag
    pub enabled: bool,
}

/// RPC error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NestGateRpcError {
    /// Dataset not found
    DatasetNotFound {
        /// Name of the dataset that was not found
        dataset: String,
    },

    /// Dataset already exists
    DatasetAlreadyExists {
        /// Name of the dataset that already exists
        dataset: String,
    },

    /// Object not found
    ObjectNotFound {
        /// Dataset name
        dataset: String,
        /// Object key
        key: String,
    },

    /// Object already exists
    ObjectAlreadyExists {
        /// Dataset name
        dataset: String,
        /// Object key
        key: String,
    },

    /// Invalid parameters
    InvalidParameters {
        /// Error message describing the invalid parameters
        message: String,
    },

    /// Storage full
    StorageFull {
        /// Required storage space in bytes
        required: u64,
        /// Available storage space in bytes
        available: u64,
    },

    /// Quota exceeded
    QuotaExceeded {
        /// Dataset name
        dataset: String,
        /// Quota limit in bytes
        quota: u64,
        /// Requested storage in bytes
        requested: u64,
    },

    /// Permission denied
    PermissionDenied {
        /// Error message describing the permission denial
        message: String,
    },

    /// Internal error
    InternalError {
        /// Error message describing the internal error
        message: String,
    },

    /// Service unavailable
    ServiceUnavailable {
        /// Error message describing why service is unavailable
        message: String,
    },

    /// Connection error
    ConnectionError {
        /// Error message describing the connection failure
        message: String,
    },

    /// Timeout
    Timeout {
        /// Operation that timed out
        operation: String,
    },
}

impl std::fmt::Display for NestGateRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatasetNotFound { dataset } => write!(f, "Dataset not found: {}", dataset),
            Self::DatasetAlreadyExists { dataset } => {
                write!(f, "Dataset already exists: {}", dataset)
            }
            Self::ObjectNotFound { dataset, key } => {
                write!(f, "Object not found: {}/{}", dataset, key)
            }
            Self::ObjectAlreadyExists { dataset, key } => {
                write!(f, "Object already exists: {}/{}", dataset, key)
            }
            Self::InvalidParameters { message } => write!(f, "Invalid parameters: {}", message),
            Self::StorageFull {
                required,
                available,
            } => write!(
                f,
                "Storage full: required {} bytes, available {} bytes",
                required, available
            ),
            Self::QuotaExceeded {
                dataset,
                quota,
                requested,
            } => write!(
                f,
                "Quota exceeded for dataset {}: quota {} bytes, requested {} bytes",
                dataset, quota, requested
            ),
            Self::PermissionDenied { message } => write!(f, "Permission denied: {}", message),
            Self::InternalError { message } => write!(f, "Internal error: {}", message),
            Self::ServiceUnavailable { message } => write!(f, "Service unavailable: {}", message),
            Self::ConnectionError { message } => write!(f, "Connection error: {}", message),
            Self::Timeout { operation } => write!(f, "Timeout: {}", operation),
        }
    }
}

impl std::error::Error for NestGateRpcError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_params_default() {
        let params = DatasetParams::default();
        assert_eq!(params.compression, Some("lz4".to_string()));
        assert!(params.encrypted);
        assert!(params.deduplicated);
        assert!(params.quota.is_none());
    }

    #[test]
    fn test_operation_result() {
        let result = OperationResult {
            success: true,
            message: "Success".to_string(),
            metadata: HashMap::new(),
        };
        assert!(result.success);
        assert_eq!(result.message, "Success");
    }

    #[test]
    fn test_error_display() {
        let error = NestGateRpcError::DatasetNotFound {
            dataset: "test".to_string(),
        };
        assert_eq!(error.to_string(), "Dataset not found: test");
    }
}
