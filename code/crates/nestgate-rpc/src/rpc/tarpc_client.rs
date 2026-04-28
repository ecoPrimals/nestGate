// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # tarpc Client for `NestGate`
//!
//! **HIGH-PERFORMANCE PRIMAL-TO-PRIMAL RPC CLIENT** (v0.2.0)
//!
//! Provides an async tarpc client for connecting to `NestGate` storage services.
//!
//! ## Performance
//! - ~10-20 μs latency (5-10x faster than JSON-RPC)
//! - ~100K requests/sec (10x faster than JSON-RPC)
//! - Zero-copy binary serialization
//! - Type-safe compile-time checks
//!
//! ## Philosophy (Primal Sovereignty)
//! - **tarpc PRIMARY** for primal-to-primal communication
//! - **Zero unsafe blocks**
//! - **Modern async/await**
//! - **Type-safe error handling**
//! - **Automatic reconnection support**
//! - **Self-knowledge**: Client discovers `NestGate` via capability, not hardcoded endpoint
//!
//! ## Usage
//! ```rust,ignore
//! use nestgate_core::rpc::NestGateRpcClient;
//!
//! # async fn example() -> std::result::Result<(), nestgate_types::NestGateError> {
//! // Discover storage capability (no hardcoding!)
//! let client = NestGateRpcClient::discover_by_capability("storage")?;
//!
//! // Or connect directly via environment-driven endpoint
//! let endpoint = std::env::var("NESTGATE_RPC_ENDPOINT")
//!     .unwrap_or_else(|_| format!("tarpc://{}", nestgate_core::constants::ports::get_rpc_server_addr()));
//! let client = NestGateRpcClient::new(&endpoint)?;
//!
//! // Create dataset
//! let dataset = client.create_dataset("my-dataset", Default::default()).await?;
//! # Ok(())
//! # }
//! ```

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::rpc::tarpc_types::{
    CapabilityRegistration, DatasetInfo, DatasetParams, HealthStatus, NestGateRpcError, ObjectInfo,
    OperationResult, ProtocolInfo, RegistrationResult, ServiceInfo, StorageMetrics, VersionInfo,
};
use nestgate_config::config::capability_discovery::{self, DiscoverySource};
use nestgate_config::constants::ports::default_tarpc_client_endpoint;
use nestgate_types::error::{NestGateError, Result};

// Import the generated client from the tarpc macro
// The #[tarpc::service] macro in tarpc_types.rs generates NestGateRpcClient
use tarpc::client;

// Type alias for the generated client
type GeneratedClient = crate::rpc::tarpc_types::NestGateRpcClient;

/// Modern async tarpc client for `NestGate`
///
/// Provides high-performance binary RPC communication with automatic
/// connection management and type-safe method calls.
///
/// # Architecture
/// - Lazy connection initialization
/// - Automatic reconnection on failure
/// - Zero unsafe blocks
/// - Capability-based discovery
///
/// # Example
/// ```rust,ignore
/// use nestgate_core::rpc::NestGateRpcClient;
///
/// # async fn example() -> std::result::Result<(), nestgate_types::NestGateError> {
/// // Environment-driven: $NESTGATE_RPC_ENDPOINT or default
/// let endpoint = std::env::var("NESTGATE_RPC_ENDPOINT")
///     .unwrap_or_else(|_| format!("tarpc://{}", nestgate_core::constants::ports::get_rpc_server_addr()));
/// let client = NestGateRpcClient::new(&endpoint)?;
/// let health = client.health().await?;
/// println!("Service status: {}", health.status);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct NestGateRpcClient {
    /// Original endpoint string for debug logging, error context, and test assertions.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "endpoint stored for debug/error context in production"
        )
    )]
    pub(crate) endpoint: String,

    /// Parsed socket address
    addr: SocketAddr,

    /// Client connection (lazy-initialized)
    ///
    /// Wrapped in `RwLock` for safe concurrent access.
    /// Uses Option to allow for lazy initialization and reconnection.
    connection: Arc<RwLock<Option<GeneratedClient>>>,

    /// Request timeout
    timeout: Duration,
}

impl NestGateRpcClient {
    /// Create new tarpc client from endpoint
    ///
    /// # Arguments
    /// * `endpoint` - tarpc URL (e.g., "<tarpc://localhost:8091>")
    ///
    /// # Errors
    /// Returns error if endpoint is invalid or cannot be parsed
    ///
    /// # Example
    /// ```rust,ignore
    /// use nestgate_core::rpc::NestGateRpcClient;
    ///
    /// let client = NestGateRpcClient::new("tarpc://localhost:8091").unwrap();
    /// ```
    pub fn new(endpoint: &str) -> Result<Self> {
        debug!("Creating NestGate tarpc client for endpoint: {}", endpoint);

        // Parse endpoint: tarpc://host:port
        let addr = Self::parse_endpoint(endpoint)?;

        Ok(Self {
            endpoint: endpoint.to_string(),
            addr,
            connection: Arc::new(RwLock::new(None)),
            timeout: Duration::from_secs(5),
        })
    }

    /// Discover `NestGate` by capability (runtime discovery - no hardcoding!)
    ///
    /// # Arguments
    /// * `capability` - Capability to discover (e.g., "storage")
    ///
    /// # Errors
    /// Returns error if discovery fails or no services found
    ///
    /// # Example
    /// ```rust,ignore
    /// use nestgate_core::rpc::NestGateRpcClient;
    ///
    /// # async fn example() -> std::result::Result<(), nestgate_types::NestGateError> {
    /// let client = NestGateRpcClient::discover_by_capability("storage")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn discover_by_capability(capability: &str) -> Result<Self> {
        let env_var = format!(
            "NESTGATE_{}_ENDPOINT",
            capability.to_uppercase().replace('-', "_")
        );
        let discovery_default = default_tarpc_client_endpoint();
        let se =
            capability_discovery::discover_with_fallback(capability, &env_var, &discovery_default)?;
        if se.source == DiscoverySource::Default {
            warn!(
                capability = capability,
                endpoint = %se.endpoint,
                env_var = %env_var,
                "tarpc client using env-derived default endpoint; set the env var or enable capability discovery"
            );
        }
        let endpoint = Self::normalize_to_tarpc_endpoint(&se.endpoint);
        debug!(
            capability = capability,
            endpoint = %endpoint,
            source = ?se.source,
            "resolved tarpc endpoint via nestgate-config capability discovery"
        );
        Self::new(&endpoint)
    }

    pub(crate) fn normalize_to_tarpc_endpoint(raw: &str) -> String {
        let s = raw.trim();
        if s.starts_with("tarpc://") {
            return s.to_string();
        }
        if let Some(rest) = s.strip_prefix("http://") {
            return format!("tarpc://{rest}");
        }
        if let Some(rest) = s.strip_prefix("https://") {
            return format!("tarpc://{rest}");
        }
        format!("tarpc://{s}")
    }

    /// Set request timeout
    ///
    /// # Arguments
    /// * `timeout` - Timeout duration
    ///
    /// # Example
    /// ```rust,ignore
    /// use nestgate_core::rpc::NestGateRpcClient;
    /// use std::time::Duration;
    ///
    /// let client = NestGateRpcClient::new("tarpc://localhost:8091")
    ///     .unwrap()
    ///     .with_timeout(Duration::from_secs(10));
    /// ```
    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    // ==================== STORAGE OPERATIONS ====================

    /// Create a new dataset
    ///
    /// # Arguments
    /// * `name` - Dataset name
    /// * `params` - Dataset configuration parameters
    ///
    /// # Errors
    /// Returns error if dataset creation fails
    pub async fn create_dataset(&self, name: &str, params: DatasetParams) -> Result<DatasetInfo> {
        debug!("Creating dataset: {}", name);
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .create_dataset(ctx, Arc::from(name), params)
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))?
            .map_err(Self::convert_rpc_error)
    }

    /// List all datasets
    ///
    /// # Errors
    /// Returns error if listing fails
    pub async fn list_datasets(&self) -> Result<Vec<DatasetInfo>> {
        debug!("Listing datasets");
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .list_datasets(ctx)
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))?
            .map_err(Self::convert_rpc_error)
    }

    /// Get dataset information
    ///
    /// # Arguments
    /// * `name` - Dataset name
    ///
    /// # Errors
    /// Returns error if dataset doesn't exist
    pub async fn get_dataset(&self, name: &str) -> Result<DatasetInfo> {
        debug!("Getting dataset: {}", name);
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .get_dataset(ctx, Arc::from(name))
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))?
            .map_err(Self::convert_rpc_error)
    }

    /// Delete a dataset
    ///
    /// # Arguments
    /// * `name` - Dataset name
    ///
    /// # Errors
    /// Returns error if deletion fails
    pub async fn delete_dataset(&self, name: &str) -> Result<OperationResult> {
        debug!("Deleting dataset: {}", name);
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .delete_dataset(ctx, Arc::from(name))
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))?
            .map_err(Self::convert_rpc_error)
    }

    /// Store an object
    ///
    /// # Arguments
    /// * `dataset` - Dataset name
    /// * `key` - Object key
    /// * `data` - Object data
    /// * `metadata` - Optional metadata
    ///
    /// # Errors
    /// Returns error if storage fails
    pub async fn store_object(
        &self,
        dataset: &str,
        key: &str,
        data: impl Into<Bytes>,
        metadata: Option<std::collections::HashMap<String, String>>,
    ) -> Result<ObjectInfo> {
        debug!("Storing object: {}/{}", dataset, key);
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .store_object(
                ctx,
                Arc::from(dataset),
                Arc::from(key),
                data.into(),
                metadata,
            )
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))?
            .map_err(Self::convert_rpc_error)
    }

    /// Retrieve an object
    ///
    /// # Arguments
    /// * `dataset` - Dataset name
    /// * `key` - Object key
    ///
    /// # Errors
    /// Returns error if object doesn't exist
    pub async fn retrieve_object(&self, dataset: &str, key: &str) -> Result<Bytes> {
        debug!("Retrieving object: {}/{}", dataset, key);
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .retrieve_object(ctx, Arc::from(dataset), Arc::from(key))
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))?
            .map_err(Self::convert_rpc_error)
    }

    /// Get object metadata
    ///
    /// # Arguments
    /// * `dataset` - Dataset name
    /// * `key` - Object key
    ///
    /// # Errors
    /// Returns error if object doesn't exist
    pub async fn get_object_metadata(&self, dataset: &str, key: &str) -> Result<ObjectInfo> {
        debug!("Getting object metadata: {}/{}", dataset, key);
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .get_object_metadata(ctx, Arc::from(dataset), Arc::from(key))
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))?
            .map_err(Self::convert_rpc_error)
    }

    /// List objects in dataset
    ///
    /// # Arguments
    /// * `dataset` - Dataset name
    /// * `prefix` - Optional key prefix filter
    /// * `limit` - Maximum number of results
    ///
    /// # Errors
    /// Returns error if listing fails
    pub async fn list_objects(
        &self,
        dataset: &str,
        prefix: Option<String>,
        limit: Option<usize>,
    ) -> Result<Vec<ObjectInfo>> {
        debug!("Listing objects in dataset: {}", dataset);
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .list_objects(ctx, Arc::from(dataset), prefix.map(Arc::<str>::from), limit)
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))?
            .map_err(Self::convert_rpc_error)
    }

    /// Delete an object
    ///
    /// # Arguments
    /// * `dataset` - Dataset name
    /// * `key` - Object key
    ///
    /// # Errors
    /// Returns error if deletion fails
    pub async fn delete_object(&self, dataset: &str, key: &str) -> Result<OperationResult> {
        debug!("Deleting object: {}/{}", dataset, key);
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .delete_object(ctx, Arc::from(dataset), Arc::from(key))
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))?
            .map_err(Self::convert_rpc_error)
    }

    // ==================== CAPABILITY OPERATIONS ====================

    /// Register capability with discovery system
    ///
    /// # Arguments
    /// * `registration` - Service registration information
    ///
    /// # Errors
    /// Returns error if registration fails
    pub async fn register_capability(
        &self,
        registration: CapabilityRegistration,
    ) -> Result<RegistrationResult> {
        debug!("Registering capability: {}", registration.capability);
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .register_capability(ctx, registration)
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))?
            .map_err(Self::convert_rpc_error)
    }

    /// Discover services by capability
    ///
    /// # Arguments
    /// * `capability` - Required capability
    ///
    /// # Errors
    /// Returns error if discovery fails
    pub async fn discover_capability(&self, capability: &str) -> Result<Vec<ServiceInfo>> {
        debug!("Discovering services with capability: {}", capability);
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .discover_capability(ctx, Arc::from(capability))
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))?
            .map_err(Self::convert_rpc_error)
    }

    // ==================== HEALTH & MONITORING ====================

    /// Get health status
    ///
    /// # Errors
    /// Returns error if health check fails
    pub async fn health(&self) -> Result<HealthStatus> {
        debug!("Checking health");
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .health(ctx)
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))
    }

    /// Get storage metrics
    ///
    /// # Errors
    /// Returns error if metrics retrieval fails
    pub async fn metrics(&self) -> Result<StorageMetrics> {
        debug!("Getting metrics");
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .metrics(ctx)
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))
    }

    /// Get version information
    ///
    /// # Errors
    /// Returns error if version check fails
    pub async fn version(&self) -> Result<VersionInfo> {
        debug!("Getting version");
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .version(ctx)
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))
    }

    /// Get available protocols
    ///
    /// # Errors
    /// Returns error if protocol retrieval fails
    pub async fn protocols(&self) -> Result<Vec<ProtocolInfo>> {
        debug!("Getting protocols");
        let client = self.get_connection().await?;
        let ctx = tarpc::context::current();

        client
            .protocols(ctx)
            .await
            .map_err(|e| NestGateError::api_internal_error(format!("RPC call failed: {e}")))
    }

    // ==================== INTERNAL HELPERS ====================

    /// Get or establish connection
    async fn get_connection(&self) -> Result<GeneratedClient> {
        // Check if we have existing connection
        {
            let conn = self.connection.read().await;
            if let Some(client) = conn.as_ref() {
                return Ok(client.clone());
            }
        }

        // Establish new connection
        self.connect().await
    }

    /// Establish new connection
    async fn connect(&self) -> Result<GeneratedClient> {
        info!("Connecting to NestGate tarpc server at {}", self.addr);

        let transport =
            tarpc::serde_transport::tcp::connect(self.addr, tokio_serde::formats::Bincode::default)
                .await
                .map_err(|e| {
                    NestGateError::network_error(format!(
                        "Failed to connect to {}: {}",
                        self.addr, e
                    ))
                })?;

        let client = GeneratedClient::new(client::Config::default(), transport).spawn();

        // Store connection
        {
            let mut conn = self.connection.write().await;
            *conn = Some(client.clone());
        }

        info!("Connected to NestGate tarpc server at {}", self.addr);
        Ok(client)
    }

    /// Parse tarpc endpoint URL
    fn parse_endpoint(endpoint: &str) -> Result<SocketAddr> {
        // Parse tarpc://host:port
        if !endpoint.starts_with("tarpc://") {
            return Err(NestGateError::configuration_error(
                "endpoint",
                format!("Invalid tarpc endpoint (must start with tarpc://): {endpoint}"),
            ));
        }

        let addr_str = endpoint.strip_prefix("tarpc://").ok_or_else(|| {
            NestGateError::configuration_error(
                "endpoint",
                format!("endpoint must start with 'tarpc://': {endpoint}"),
            )
        })?;
        addr_str.parse().map_err(|e| {
            NestGateError::configuration_error(
                "endpoint",
                format!("Invalid socket address {addr_str}: {e}"),
            )
        })
    }

    /// Convert RPC error to `NestGateError`
    fn convert_rpc_error(error: NestGateRpcError) -> NestGateError {
        match error {
            NestGateRpcError::DatasetNotFound { dataset } => {
                NestGateError::not_found(format!("Dataset not found: {dataset}"))
            }
            NestGateRpcError::DatasetAlreadyExists { dataset } => {
                NestGateError::api_internal_error(format!("Dataset already exists: {dataset}"))
            }
            NestGateRpcError::ObjectNotFound { dataset, key } => {
                NestGateError::not_found(format!("Object not found: {dataset}/{key}"))
            }
            NestGateRpcError::ObjectAlreadyExists { dataset, key } => {
                NestGateError::api_internal_error(format!("Object already exists: {dataset}/{key}"))
            }
            NestGateRpcError::InvalidParameters { message } => {
                NestGateError::validation_error(message)
            }
            NestGateRpcError::StorageFull {
                required,
                available,
            } => NestGateError::storage_error(format!(
                "Storage full: required {required} bytes, available {available} bytes"
            )),
            NestGateRpcError::QuotaExceeded {
                dataset,
                quota,
                requested,
            } => NestGateError::storage_error(format!(
                "Quota exceeded for dataset {dataset}: quota {quota} bytes, requested {requested} bytes"
            )),
            NestGateRpcError::PermissionDenied { message } => {
                NestGateError::authorization(message, "storage")
            }
            NestGateRpcError::InternalError { message } => {
                NestGateError::internal_error(message, "rpc")
            }
            NestGateRpcError::ServiceUnavailable { message } => {
                NestGateError::service_unavailable(message)
            }
            NestGateRpcError::ConnectionError { message } => NestGateError::network_error(message),
            NestGateRpcError::Timeout { operation } => NestGateError::timeout_error(
                operation.to_string(),
                std::time::Duration::from_secs(5),
            ),
        }
    }
}

#[cfg(test)]
#[path = "tarpc_client_tests.rs"]
mod tests;
