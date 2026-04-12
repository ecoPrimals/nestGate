// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # tarpc types and traits for `NestGate`
//!
//! Shared types and the [`NestGateRpc`] service trait for tarpc-based communication.
//!
//! ## Layout
//! - Storage types — dataset/object request and response structs.
//! - Metadata types — capability discovery, health, metrics, and protocol metadata.
//! - This module — the `#[tarpc::service]` trait ([`NestGateRpc`]) and [`NestGateRpcError`].
//!
//! The trait is intentionally large: it is the full RPC contract in one place. Supporting
//! types live in submodules to keep navigation simple without hiding the interface behind
//! excessive file splits.

mod metadata;
mod storage;

pub use metadata::{
    CapabilityRegistration, HealthStatus, ProtocolInfo, RegistrationResult, ServiceInfo,
    StorageMetrics, VersionInfo,
};
pub use storage::{DatasetInfo, DatasetParams, ObjectInfo, OperationResult};

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// tarpc service trait for `NestGate` storage operations
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
/// `NestGate` exposes only storage operations. Discovery of other primals
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
        name: Arc<str>,
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
    async fn get_dataset(name: Arc<str>) -> Result<DatasetInfo, NestGateRpcError>;

    /// Delete a dataset
    ///
    /// # Arguments
    /// * `name` - Dataset name
    ///
    /// # Returns
    /// Success result
    async fn delete_dataset(name: Arc<str>) -> Result<OperationResult, NestGateRpcError>;

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
        dataset: Arc<str>,
        key: Arc<str>,
        data: Bytes,
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
    async fn retrieve_object(dataset: Arc<str>, key: Arc<str>) -> Result<Bytes, NestGateRpcError>;

    /// Get object metadata
    ///
    /// # Arguments
    /// * `dataset` - Dataset name
    /// * `key` - Object key
    ///
    /// # Returns
    /// Object information without data
    async fn get_object_metadata(
        dataset: Arc<str>,
        key: Arc<str>,
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
        dataset: Arc<str>,
        prefix: Option<Arc<str>>,
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
        dataset: Arc<str>,
        key: Arc<str>,
    ) -> Result<OperationResult, NestGateRpcError>;

    // ==================== CAPABILITY OPERATIONS ====================

    /// Register `NestGate`'s capabilities with discovery system
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
    async fn discover_capability(
        capability: Arc<str>,
    ) -> Result<Vec<ServiceInfo>, NestGateRpcError>;

    // ==================== HEALTH & MONITORING ====================

    /// Get health status (full snapshot; used by semantic `health.check` and `health.readiness`).
    ///
    /// Semantic `health.liveness` and `health.readiness` are composed in
    /// [`crate::rpc::semantic_router::SemanticRouter`] (liveness uses `version()` to avoid
    /// heavy metrics; readiness uses this call to validate storage backends).
    ///
    /// # Returns
    /// Current health status of `NestGate`
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

/// RPC error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NestGateRpcError {
    /// Dataset not found
    DatasetNotFound {
        /// Name of the dataset that was not found
        dataset: Arc<str>,
    },

    /// Dataset already exists
    DatasetAlreadyExists {
        /// Name of the dataset that already exists
        dataset: Arc<str>,
    },

    /// Object not found
    ObjectNotFound {
        /// Dataset name
        dataset: Arc<str>,
        /// Object key
        key: Arc<str>,
    },

    /// Object already exists
    ObjectAlreadyExists {
        /// Dataset name
        dataset: Arc<str>,
        /// Object key
        key: Arc<str>,
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
        dataset: Arc<str>,
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
        operation: Arc<str>,
    },
}

impl std::fmt::Display for NestGateRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatasetNotFound { dataset } => write!(f, "Dataset not found: {dataset}"),
            Self::DatasetAlreadyExists { dataset } => {
                write!(f, "Dataset already exists: {dataset}")
            }
            Self::ObjectNotFound { dataset, key } => {
                write!(f, "Object not found: {dataset}/{key}")
            }
            Self::ObjectAlreadyExists { dataset, key } => {
                write!(f, "Object already exists: {dataset}/{key}")
            }
            Self::InvalidParameters { message } => write!(f, "Invalid parameters: {message}"),
            Self::StorageFull {
                required,
                available,
            } => write!(
                f,
                "Storage full: required {required} bytes, available {available} bytes"
            ),
            Self::QuotaExceeded {
                dataset,
                quota,
                requested,
            } => write!(
                f,
                "Quota exceeded for dataset {dataset}: quota {quota} bytes, requested {requested} bytes"
            ),
            Self::PermissionDenied { message } => write!(f, "Permission denied: {message}"),
            Self::InternalError { message } => write!(f, "Internal error: {message}"),
            Self::ServiceUnavailable { message } => write!(f, "Service unavailable: {message}"),
            Self::ConnectionError { message } => write!(f, "Connection error: {message}"),
            Self::Timeout { operation } => write!(f, "Timeout: {operation}"),
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
            dataset: Arc::from("test"),
        };
        assert_eq!(error.to_string(), "Dataset not found: test");
    }

    #[test]
    fn round5_rpc_error_display_dataset_already_exists() {
        let e = NestGateRpcError::DatasetAlreadyExists {
            dataset: Arc::from("tank/data"),
        };
        assert!(e.to_string().contains("already exists"));
        assert!(e.to_string().contains("tank/data"));
    }

    #[test]
    fn round5_rpc_error_display_object_not_found() {
        let e = NestGateRpcError::ObjectNotFound {
            dataset: Arc::from("d"),
            key: Arc::from("k"),
        };
        assert_eq!(e.to_string(), "Object not found: d/k");
    }

    #[test]
    fn round5_rpc_error_display_object_already_exists() {
        let e = NestGateRpcError::ObjectAlreadyExists {
            dataset: Arc::from("d"),
            key: Arc::from("k"),
        };
        assert_eq!(e.to_string(), "Object already exists: d/k");
    }

    #[test]
    fn round5_rpc_error_display_invalid_parameters() {
        let e = NestGateRpcError::InvalidParameters {
            message: "bad".to_string(),
        };
        assert_eq!(e.to_string(), "Invalid parameters: bad");
    }

    #[test]
    fn round5_rpc_error_display_storage_full() {
        let e = NestGateRpcError::StorageFull {
            required: 100,
            available: 10,
        };
        assert!(e.to_string().contains("Storage full"));
        assert!(e.to_string().contains("100"));
    }

    #[test]
    fn round5_rpc_error_display_quota_exceeded() {
        let e = NestGateRpcError::QuotaExceeded {
            dataset: Arc::from("z"),
            quota: 50,
            requested: 60,
        };
        assert!(e.to_string().contains("Quota exceeded"));
        assert!(e.to_string().contains("z"));
    }

    #[test]
    fn round5_rpc_error_display_permission_denied() {
        let e = NestGateRpcError::PermissionDenied {
            message: "nope".to_string(),
        };
        assert_eq!(e.to_string(), "Permission denied: nope");
    }

    #[test]
    fn round5_rpc_error_display_internal_error() {
        let e = NestGateRpcError::InternalError {
            message: "panic".to_string(),
        };
        assert_eq!(e.to_string(), "Internal error: panic");
    }

    #[test]
    fn round5_rpc_error_display_service_unavailable() {
        let e = NestGateRpcError::ServiceUnavailable {
            message: "down".to_string(),
        };
        assert_eq!(e.to_string(), "Service unavailable: down");
    }

    #[test]
    fn round5_rpc_error_display_connection_error() {
        let e = NestGateRpcError::ConnectionError {
            message: "reset".to_string(),
        };
        assert_eq!(e.to_string(), "Connection error: reset");
    }

    #[test]
    fn round5_rpc_error_display_timeout() {
        let e = NestGateRpcError::Timeout {
            operation: Arc::from("read"),
        };
        assert_eq!(e.to_string(), "Timeout: read");
    }
}
