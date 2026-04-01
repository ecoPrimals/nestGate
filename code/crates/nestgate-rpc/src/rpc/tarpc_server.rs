// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # tarpc Server for `NestGate`
//!
//! Primal-to-primal RPC server implementation.
//!
//! `serve_tarpc()` is wired into `nestgate-bin` daemon startup behind the
//! `tarpc-server` feature flag (default-enabled). Protocol capabilities advertise
//! the port for discovery.
//!
//! ## Design
//! - tarpc primary for primal-to-primal communication
//! - Zero unsafe blocks, modern async/await
//! - Self-knowledge: exposes only storage capabilities
//! - Runtime discovery: registers with discovery system
//! - Concurrent access via `tokio::sync::RwLock`
//! - Read-biased `RwLock` for concurrent access
//! - Better multi-primal scalability
//!
//! ## Usage
//! ```rust,ignore
//! use nestgate_core::rpc::{NestGateRpcService, serve_tarpc};
//! use nestgate_core::constants::ports;
//! use std::net::SocketAddr;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let service = NestGateRpcService::new().await.expect("Failed to create service");
//! // Environment-driven: $NESTGATE_RPC_HOST and $NESTGATE_RPC_PORT
//! let addr: SocketAddr = ports::get_rpc_server_addr().parse()?;
//! serve_tarpc(addr, service).await?;
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;

use bytes::Bytes;
use futures_util::StreamExt;
use tarpc::context::Context;
use tarpc::server::Channel;
use tracing::{debug, info, warn};

use crate::rpc::tarpc_types::{
    CapabilityRegistration, DatasetInfo, DatasetParams, HealthStatus, NestGateRpc,
    NestGateRpcError, ObjectInfo, OperationResult, ProtocolInfo, RegistrationResult, ServiceInfo,
    StorageMetrics, VersionInfo,
};
use nestgate_config::config::capability_discovery::{self, DiscoverySource};
use nestgate_config::constants::ports::{self, default_tarpc_client_endpoint};
use nestgate_types::error::{NestGateError, Result};

fn unix_timestamp_secs() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}

#[inline]
fn byte_len_u64(len: usize) -> u64 {
    u64::try_from(len).unwrap_or(u64::MAX)
}

/// Object payload stored in the in-memory store. Uses `Bytes` for
/// refcounted zero-copy cloning on retrieval.
type StoredObjectPayload = (Bytes, HashMap<String, String>);

/// In-process dataset/object store for tarpc path (filesystem persistence via unix socket handlers).
#[derive(Default)]
pub(crate) struct InnerStore {
    datasets: HashMap<String, DatasetInfo>,
    /// (dataset, key) → raw bytes + user metadata
    objects: HashMap<(String, String), StoredObjectPayload>,
}

/// `NestGate` RPC service implementation.
///
/// Uses an in-memory store in this crate; production persistence remains in `nestgate-core`.
#[derive(Clone)]
pub struct NestGateRpcService {
    /// Start time for uptime calculation
    pub(crate) start_time: SystemTime,
    pub(crate) inner: Arc<tokio::sync::RwLock<InnerStore>>,
}

impl NestGateRpcService {
    /// Create new RPC service (in-memory storage until cross-crate wiring).
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`] when the service cannot be constructed; the current
    /// implementation always succeeds and reserves this for future persistence initialization.
    pub fn new() -> Result<Self> {
        info!(
            "🚀 Creating NestGate RPC service (in-memory storage; nestgate-core persistence optional)"
        );
        debug!("feature pending: StorageManagerService-backed persistence from nestgate-core");

        Ok(Self {
            start_time: SystemTime::now(),
            inner: Arc::new(tokio::sync::RwLock::new(InnerStore::default())),
        })
    }

    /// Get uptime in seconds
    fn uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().unwrap_or_default().as_secs()
    }

    /// Calculate storage metrics from in-memory store.
    async fn calculate_metrics(&self) -> StorageMetrics {
        let g = self.inner.read().await;
        let dataset_count = g.datasets.len();
        let used_space: u64 = g.datasets.values().map(|d| d.size_bytes).sum();
        let object_count: u64 = g.datasets.values().map(|d| d.object_count).sum();
        let compression_sum: f64 = g.datasets.values().map(|d| d.compression_ratio).sum();
        let avg_compression_ratio = if dataset_count > 0 {
            #[allow(clippy::cast_precision_loss)]
            {
                compression_sum / dataset_count as f64
            }
        } else {
            1.0
        };

        // In-process store only: no fabricated capacity ceiling or IOPS/latency — those are
        // not tracked here. `total_capacity_bytes` matches committed dataset bytes; no separate
        // "free pool" exists in this implementation.
        StorageMetrics {
            total_capacity_bytes: used_space,
            used_space_bytes: used_space,
            available_space_bytes: 0,
            dataset_count,
            object_count,
            avg_compression_ratio,
            dedup_ratio: 1.0,
            read_ops_per_sec: 0.0,
            write_ops_per_sec: 0.0,
            avg_read_latency_ms: 0.0,
            avg_write_latency_ms: 0.0,
        }
    }
}

// Note: Default implementation removed as new() is now async
// Use NestGateRpcService::new().await instead

// Implement the tarpc service trait
impl NestGateRpc for NestGateRpcService {
    // ==================== STORAGE OPERATIONS ====================

    async fn create_dataset(
        self,
        _context: Context,
        name: String,
        params: DatasetParams,
    ) -> std::result::Result<DatasetInfo, NestGateRpcError> {
        debug!("RPC: create_dataset({}) → in-memory store", name);

        let mut g = self.inner.write().await;
        if g.datasets.contains_key(&name) {
            return Err(NestGateRpcError::DatasetAlreadyExists { dataset: name });
        }
        let ts = unix_timestamp_secs();
        let info = DatasetInfo {
            name: name.clone(),
            description: params.description.clone(),
            created_at: ts,
            modified_at: ts,
            size_bytes: 0,
            object_count: 0,
            compression_ratio: 1.0,
            params: params.clone(),
            status: String::from("active"),
        };
        g.datasets.insert(name, info.clone());
        Ok(info)
    }

    async fn list_datasets(
        self,
        _context: Context,
    ) -> std::result::Result<Vec<DatasetInfo>, NestGateRpcError> {
        debug!("RPC: list_datasets() → in-memory store");

        let g = self.inner.read().await;
        Ok(g.datasets.values().cloned().collect())
    }

    async fn get_dataset(
        self,
        _context: Context,
        name: String,
    ) -> std::result::Result<DatasetInfo, NestGateRpcError> {
        debug!("RPC: get_dataset({}) → in-memory store", name);

        let g = self.inner.read().await;
        g.datasets
            .get(&name)
            .cloned()
            .ok_or(NestGateRpcError::DatasetNotFound { dataset: name })
    }

    async fn delete_dataset(
        self,
        _context: Context,
        name: String,
    ) -> std::result::Result<OperationResult, NestGateRpcError> {
        debug!("RPC: delete_dataset({}) → in-memory store", name);

        let mut g = self.inner.write().await;
        if g.datasets.remove(&name).is_none() {
            return Err(NestGateRpcError::DatasetNotFound { dataset: name });
        }
        g.objects.retain(|k, _| k.0 != name);
        Ok(OperationResult {
            success: true,
            message: format!("Dataset {name} deleted successfully"),
            metadata: HashMap::new(),
        })
    }

    async fn store_object(
        self,
        _context: Context,
        dataset: String,
        key: String,
        data: Vec<u8>,
        metadata: Option<HashMap<String, String>>,
    ) -> std::result::Result<ObjectInfo, NestGateRpcError> {
        debug!("RPC: store_object({}/{}) → in-memory store", dataset, key);

        let mut g = self.inner.write().await;
        if !g.datasets.contains_key(&dataset) {
            return Err(NestGateRpcError::DatasetNotFound { dataset });
        }
        let ts = unix_timestamp_secs();
        let size = byte_len_u64(data.len());
        let meta = metadata.unwrap_or_default();
        let object_info = ObjectInfo {
            key: key.clone(),
            dataset: dataset.clone(),
            size_bytes: size,
            created_at: ts,
            modified_at: ts,
            content_type: None,
            checksum: None,
            encrypted: false,
            compressed: false,
            metadata: meta.clone(),
        };
        g.objects.insert((dataset, key), (Bytes::from(data), meta));

        let object_count = byte_len_u64(
            g.objects
                .keys()
                .filter(|(d, _)| d == &object_info.dataset)
                .count(),
        );
        let used_bytes: u64 = g
            .objects
            .iter()
            .filter(|((d, _), _)| d == &object_info.dataset)
            .map(|(_, (b, _))| byte_len_u64(b.len()))
            .sum();
        if let Some(ds) = g.datasets.get_mut(&object_info.dataset) {
            ds.object_count = object_count;
            ds.size_bytes = used_bytes;
            ds.modified_at = ts;
        }

        info!(
            "✅ Stored object: {}/{}",
            object_info.dataset, object_info.key
        );
        Ok(object_info)
    }

    async fn retrieve_object(
        self,
        _context: Context,
        dataset: String,
        key: String,
    ) -> std::result::Result<Vec<u8>, NestGateRpcError> {
        debug!(
            "RPC: retrieve_object({}/{}) → in-memory store",
            dataset, key
        );

        let g = self.inner.read().await;
        let lookup = (dataset.clone(), key.clone());
        g.objects
            .get(&lookup)
            .map(|(b, _)| b.to_vec())
            .ok_or(NestGateRpcError::ObjectNotFound { dataset, key })
    }

    async fn get_object_metadata(
        self,
        _context: Context,
        dataset: String,
        key: String,
    ) -> std::result::Result<ObjectInfo, NestGateRpcError> {
        debug!(
            "RPC: get_object_metadata({}/{}) → in-memory store",
            dataset, key
        );

        let g = self.inner.read().await;
        let lookup = (dataset.clone(), key.clone());
        g.objects
            .get(&lookup)
            .map(|(data, meta)| ObjectInfo {
                key: key.clone(),
                dataset: dataset.clone(),
                size_bytes: byte_len_u64(data.len()),
                created_at: unix_timestamp_secs(),
                modified_at: unix_timestamp_secs(),
                content_type: None,
                checksum: None,
                encrypted: false,
                compressed: false,
                metadata: meta.clone(),
            })
            .ok_or(NestGateRpcError::ObjectNotFound { dataset, key })
    }

    async fn list_objects(
        self,
        _context: Context,
        dataset: String,
        prefix: Option<String>,
        limit: Option<usize>,
    ) -> std::result::Result<Vec<ObjectInfo>, NestGateRpcError> {
        debug!(
            "RPC: list_objects({}, {:?}, {:?}) → in-memory store",
            dataset, prefix, limit
        );

        let g = self.inner.read().await;
        let mut results = Vec::new();
        for ((ds, key), (data, meta)) in &g.objects {
            if ds != &dataset {
                continue;
            }
            if let Some(ref pfx) = prefix
                && !key.starts_with(pfx)
            {
                continue;
            }
            results.push(ObjectInfo {
                key: key.clone(),
                dataset: dataset.clone(),
                size_bytes: byte_len_u64(data.len()),
                created_at: unix_timestamp_secs(),
                modified_at: unix_timestamp_secs(),
                content_type: None,
                checksum: None,
                encrypted: false,
                compressed: false,
                metadata: meta.clone(),
            });
            if let Some(lim) = limit
                && results.len() >= lim
            {
                break;
            }
        }
        Ok(results)
    }

    async fn delete_object(
        self,
        _context: Context,
        dataset: String,
        key: String,
    ) -> std::result::Result<OperationResult, NestGateRpcError> {
        debug!("RPC: delete_object({}/{}) → in-memory store", dataset, key);

        let mut g = self.inner.write().await;
        let lookup = (dataset.clone(), key.clone());
        let removed = g.objects.remove(&lookup).is_some();
        if !removed {
            return Err(NestGateRpcError::ObjectNotFound { dataset, key });
        }
        let object_count = byte_len_u64(g.objects.keys().filter(|(d, _)| d == &dataset).count());
        let used_bytes: u64 = g
            .objects
            .iter()
            .filter(|((d, _), _)| d == &dataset)
            .map(|(_, (b, _))| byte_len_u64(b.len()))
            .sum();
        if let Some(ds) = g.datasets.get_mut(&dataset) {
            ds.object_count = object_count;
            ds.size_bytes = used_bytes;
            ds.modified_at = unix_timestamp_secs();
        }
        Ok(OperationResult {
            success: true,
            message: format!("Object {dataset}/{key} deleted successfully"),
            metadata: std::collections::HashMap::new(),
        })
    }

    // ==================== CAPABILITY OPERATIONS ====================

    async fn register_capability(
        self,
        _context: Context,
        registration: CapabilityRegistration,
    ) -> std::result::Result<RegistrationResult, NestGateRpcError> {
        debug!("RPC: register_capability({})", registration.capability);

        let endpoint = if !registration.tarpc_endpoint.is_empty() {
            &registration.tarpc_endpoint
        } else if let Some(ref ep) = registration.jsonrpc_endpoint {
            ep
        } else {
            &registration.tarpc_endpoint
        };

        match capability_discovery::announce_capability(
            &registration.capability,
            endpoint,
            std::time::Duration::from_secs(60),
        ) {
            Ok(()) => {
                info!(
                    "✅ Capability '{}' registered successfully",
                    registration.capability
                );
                Ok(RegistrationResult {
                    success: true,
                    message: format!(
                        "Capability {} registered and announced via discovery",
                        registration.capability
                    ),
                })
            }
            Err(e) => {
                warn!(
                    "Failed to register capability '{}': {}",
                    registration.capability, e
                );
                Ok(RegistrationResult {
                    success: false,
                    message: format!("Capability registration failed: {e}"),
                })
            }
        }
    }

    async fn discover_capability(
        self,
        _context: Context,
        capability: String,
    ) -> std::result::Result<Vec<ServiceInfo>, NestGateRpcError> {
        debug!("RPC: discover_capability({})", capability);
        let env_var = format!(
            "NESTGATE_{}_ENDPOINT",
            capability.to_uppercase().replace('-', "_")
        );
        let discovery_default = default_tarpc_client_endpoint();
        let se = match capability_discovery::discover_with_fallback(
            &capability,
            &env_var,
            &discovery_default,
        ) {
            Ok(se) => se,
            Err(e) => {
                warn!("discover_capability: {}", e);
                return Ok(Vec::new());
            }
        };
        if se.source == DiscoverySource::Default {
            warn!(
                capability = %capability,
                endpoint = %se.endpoint,
                env_var = %env_var,
                "discover_capability using env-derived default tarpc endpoint"
            );
        }
        let raw = se.endpoint.trim();
        let tarpc_ep = if raw.starts_with("tarpc://") {
            raw.to_string()
        } else if let Some(r) = raw.strip_prefix("http://") {
            format!("tarpc://{r}")
        } else if let Some(r) = raw.strip_prefix("https://") {
            format!("tarpc://{r}")
        } else {
            format!("tarpc://{raw}")
        };
        let mut endpoints = HashMap::new();
        endpoints.insert(String::from("tarpc"), tarpc_ep);
        Ok(vec![ServiceInfo {
            id: Arc::from(format!("discovered-{capability}")),
            capability: Arc::from(capability),
            endpoints,
            status: Arc::from("discovered"),
            metadata: None,
        }])
    }

    // ==================== HEALTH & MONITORING ====================

    async fn health(self, _context: Context) -> HealthStatus {
        debug!("RPC: health()");

        let metrics = self.calculate_metrics().await;

        HealthStatus {
            status: String::from("healthy"),
            version: String::from("0.2.0"),
            uptime_seconds: self.uptime_seconds(),
            total_datasets: metrics.dataset_count,
            total_objects: metrics.object_count,
            storage_used_bytes: metrics.used_space_bytes,
            metrics: HashMap::new(),
        }
    }

    async fn metrics(self, _context: Context) -> StorageMetrics {
        debug!("RPC: metrics()");
        self.calculate_metrics().await
    }

    async fn version(self, _context: Context) -> VersionInfo {
        debug!("RPC: version()");

        VersionInfo {
            version: String::from("0.2.0"),
            api_version: String::from("1.0"),
            protocol_versions: vec![String::from("tarpc-0.34"), String::from("jsonrpc-2.0")],
            build_info: Some(String::from("NestGate Storage Primal")),
        }
    }

    async fn protocols(self, _context: Context) -> Vec<ProtocolInfo> {
        debug!("RPC: protocols()");

        let api_addr = ports::get_api_server_addr();
        let rpc_addr = ports::get_rpc_server_addr();

        vec![
            ProtocolInfo {
                protocol: String::from("tarpc"),
                version: String::from("0.34"),
                endpoint: format!("tarpc://{rpc_addr}"),
                priority: 1,
                enabled: true,
            },
            ProtocolInfo {
                protocol: String::from("jsonrpc"),
                version: String::from("2.0"),
                endpoint: format!("http://{api_addr}/rpc"),
                priority: 2,
                enabled: false,
            },
            ProtocolInfo {
                protocol: String::from("http"),
                version: String::from("1.1"),
                endpoint: format!("http://{api_addr}"),
                priority: 3,
                enabled: false,
            },
        ]
    }
}

/// Serve `NestGate` tarpc RPC on the specified address
///
/// # Arguments
/// * `addr` - Socket address to bind to
/// * `service` - `NestGate` RPC service implementation
///
/// # Errors
/// Returns error if server fails to start or bind to address
///
/// # Example
/// ```rust,ignore
/// use nestgate_core::rpc::{NestGateRpcService, serve_tarpc};
/// use nestgate_core::constants::ports;
/// use std::net::SocketAddr;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let service = NestGateRpcService::new().await.expect("Failed to create service");
/// // Environment-driven: $NESTGATE_RPC_HOST and $NESTGATE_RPC_PORT
/// let addr: SocketAddr = ports::get_rpc_server_addr().parse()?;
/// serve_tarpc(addr, service).await?;
/// # Ok(())
/// # }
/// ```
pub async fn serve_tarpc(addr: SocketAddr, service: NestGateRpcService) -> Result<()> {
    info!("🚀 Starting NestGate tarpc server on {}", addr);

    let listener =
        tarpc::serde_transport::tcp::listen(addr, tokio_serde::formats::Bincode::default)
            .await
            .map_err(|e| NestGateError::network_error(format!("Failed to bind to {addr}: {e}")))?;

    info!("✅ NestGate tarpc server listening on {}", addr);

    let inner = Arc::clone(&service.inner);
    let start_time = service.start_time;

    listener
        .filter_map(|conn| async move {
            match conn {
                Ok(conn) => Some(conn),
                Err(e) => {
                    warn!("Connection error: {}", e);
                    None
                }
            }
        })
        .map(|transport| {
            let server = tarpc::server::BaseChannel::with_defaults(transport);
            let service = NestGateRpcService {
                start_time,
                inner: Arc::clone(&inner),
            };
            server.execute(service.serve())
        })
        .flatten()
        .for_each(|response| async move {
            tokio::spawn(response);
        })
        .await;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    async fn create_test_service() -> Result<NestGateRpcService> {
        NestGateRpcService::new()
    }

    #[tokio::test]
    async fn test_service_creation() {
        let service = create_test_service()
            .await
            .expect("Failed to create service");
        let store = service.inner.read().await;
        assert!(
            store.datasets.is_empty(),
            "New service should start with empty storage"
        );
    }

    #[tokio::test]
    async fn test_health() {
        let service = create_test_service()
            .await
            .expect("Failed to create service");
        let health = service.health(Context::current()).await;
        assert_eq!(health.status, "healthy");
        assert_eq!(health.version, "0.2.0");
    }

    #[tokio::test]
    async fn test_version() {
        let service = create_test_service()
            .await
            .expect("Failed to create service");
        let version = service.version(Context::current()).await;
        assert_eq!(version.version, "0.2.0");
        assert_eq!(version.api_version, "1.0");
    }

    #[tokio::test]
    async fn test_protocols() {
        let service = create_test_service()
            .await
            .expect("Failed to create service");
        let protocols = service.protocols(Context::current()).await;
        assert_eq!(protocols.len(), 3);
        assert_eq!(protocols[0].protocol, "tarpc");
        assert_eq!(protocols[0].priority, 1);
        assert!(protocols[0].enabled);
    }

    #[tokio::test]
    async fn test_create_dataset() {
        let service = create_test_service()
            .await
            .expect("Failed to create service");
        let result = service
            .create_dataset(
                Context::current(),
                "test-dataset".to_string(),
                DatasetParams::default(),
            )
            .await;

        assert!(result.is_ok());
        let dataset = result.unwrap();
        assert_eq!(dataset.name, "test-dataset");
        assert_eq!(dataset.object_count, 0);
    }

    #[tokio::test]
    async fn test_list_datasets() {
        let service = create_test_service()
            .await
            .expect("Failed to create service");

        // Create a dataset
        service
            .clone()
            .create_dataset(
                Context::current(),
                "test-dataset".to_string(),
                DatasetParams::default(),
            )
            .await
            .unwrap();

        // List datasets
        let datasets = service
            .clone()
            .list_datasets(Context::current())
            .await
            .unwrap();
        assert_eq!(datasets.len(), 1);
        assert_eq!(datasets[0].name, "test-dataset");
    }

    #[tokio::test]
    async fn test_store_retrieve_object() {
        let service = create_test_service()
            .await
            .expect("Failed to create service");

        // Create dataset
        service
            .clone()
            .create_dataset(
                Context::current(),
                "test-dataset".to_string(),
                DatasetParams::default(),
            )
            .await
            .unwrap();

        // Store object
        let data = vec![1, 2, 3, 4, 5];
        service
            .clone()
            .store_object(
                Context::current(),
                "test-dataset".to_string(),
                "test-key".to_string(),
                data.clone(),
                None,
            )
            .await
            .unwrap();

        // Retrieve object
        let retrieved = service
            .clone()
            .retrieve_object(
                Context::current(),
                "test-dataset".to_string(),
                "test-key".to_string(),
            )
            .await
            .unwrap();

        assert_eq!(retrieved, data);
    }
}
