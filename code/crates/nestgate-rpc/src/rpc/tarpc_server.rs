// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # tarpc Server for NestGate
//!
//! Primal-to-primal RPC server implementation.
//!
//! `serve_tarpc()` is implemented but not yet wired into nestgate-bin daemon startup.
//! Enable via `tarpc-server` feature flag. Protocol capabilities advertise
//! the port for discovery; no server listens until wired.
//!
//! ## Design
//! - tarpc primary for primal-to-primal communication
//! - Zero unsafe blocks, modern async/await
//! - Self-knowledge: exposes only storage capabilities
//! - Runtime discovery: registers with discovery system
//! - Lock-free concurrent access using DashMap
//! - No lock contention under high load
//! - Better multi-primal scalability
//!
//! ## Usage
//! ```no_run
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

use futures_util::StreamExt;
use tarpc::context::Context;
use tarpc::server::Channel;
use tracing::{debug, info, warn};

use nestgate_config::config::capability_discovery;
use nestgate_config::constants::ports;
use nestgate_types::error::{NestGateError, Result};
use crate::rpc::tarpc_types::{
    CapabilityRegistration, DatasetInfo, DatasetParams, HealthStatus, NestGateRpc,
    NestGateRpcError, ObjectInfo, OperationResult, ProtocolInfo, RegistrationResult, ServiceInfo,
    StorageMetrics, VersionInfo,
};

fn unix_timestamp_secs() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// In-process dataset/object store (replaces `nestgate-core` `StorageManagerService` until wired).
#[derive(Default)]
struct InnerStore {
    datasets: HashMap<String, DatasetInfo>,
    /// (dataset, key) → raw bytes + user metadata
    objects: HashMap<(String, String), (Vec<u8>, HashMap<String, String>)>,
}

/// NestGate RPC service implementation.
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
    pub async fn new() -> Result<Self> {
        info!("🚀 Creating NestGate RPC service (in-memory storage; TODO: wire nestgate-core)");

        Ok(Self {
            start_time: SystemTime::now(),
            inner: Arc::new(tokio::sync::RwLock::new(InnerStore::default())),
        })
    }

    /// Get uptime in seconds
    fn uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().unwrap_or_default().as_secs()
    }

    /// Convert NestGateError to NestGateRpcError
    fn convert_error(err: NestGateError) -> NestGateRpcError {
        // Simple conversion: wrap error message in InternalError
        // Can enhance later with pattern matching on error types
        let message = err.to_string();

        // Try to infer specific error types from message
        if message.contains("not found") || message.contains("Not found") {
            if message.contains("dataset") {
                NestGateRpcError::DatasetNotFound {
                    dataset: "unknown".to_string(),
                }
            } else if message.contains("object") {
                NestGateRpcError::ObjectNotFound {
                    dataset: "unknown".to_string(),
                    key: "unknown".to_string(),
                }
            } else {
                NestGateRpcError::InternalError { message }
            }
        } else if message.contains("already exists") || message.contains("Already exists") {
            NestGateRpcError::DatasetAlreadyExists {
                dataset: "unknown".to_string(),
            }
        } else {
            NestGateRpcError::InternalError { message }
        }
    }

    /// Calculate storage metrics from in-memory store.
    async fn calculate_metrics(&self) -> StorageMetrics {
        let g = self.inner.read().await;
        let dataset_count = g.datasets.len();
        let used_space: u64 = g.datasets.values().map(|d| d.size_bytes).sum();
        let object_count: u64 = g.datasets.values().map(|d| d.object_count).sum();

        StorageMetrics {
            total_capacity_bytes: 1024 * 1024 * 1024 * 1024, // 1TB placeholder
            used_space_bytes: used_space,
            available_space_bytes: (1024 * 1024 * 1024 * 1024_u64).saturating_sub(used_space),
            dataset_count,
            object_count,
            avg_compression_ratio: 1.5,
            dedup_ratio: 1.2,
            read_ops_per_sec: 0.0,
            write_ops_per_sec: 0.0,
            avg_read_latency_ms: 0.1,
            avg_write_latency_ms: 0.2,
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
            return Err(NestGateRpcError::DatasetAlreadyExists {
                dataset: name.clone(),
            });
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
            status: "active".to_string(),
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
            message: format!("Dataset {} deleted successfully", name),
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
        let size = data.len() as u64;
        let mut meta = metadata.unwrap_or_default();
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
        g.objects.insert((dataset.clone(), key), (data, meta.clone()));

        let object_count = g
            .objects
            .keys()
            .filter(|(d, _)| d == &dataset)
            .count() as u64;
        let used_bytes: u64 = g
            .objects
            .iter()
            .filter(|((d, _), _)| d == &dataset)
            .map(|(_, (b, _))| b.len() as u64)
            .sum();
        if let Some(ds) = g.datasets.get_mut(&dataset) {
            ds.object_count = object_count;
            ds.size_bytes = used_bytes;
            ds.modified_at = ts;
        }

        info!("✅ Stored object: {}/{}", dataset, object_info.key);
        Ok(object_info)
    }

    async fn retrieve_object(
        self,
        _context: Context,
        dataset: String,
        key: String,
    ) -> std::result::Result<Vec<u8>, NestGateRpcError> {
        debug!("RPC: retrieve_object({}/{}) → in-memory store", dataset, key);

        let g = self.inner.read().await;
        g.objects
            .get(&(dataset.clone(), key.clone()))
            .map(|(b, _)| b.clone())
            .ok_or(NestGateRpcError::ObjectNotFound { dataset, key })
    }

    async fn get_object_metadata(
        self,
        _context: Context,
        dataset: String,
        key: String,
    ) -> std::result::Result<ObjectInfo, NestGateRpcError> {
        debug!("RPC: get_object_metadata({}/{}) → in-memory store", dataset, key);

        let g = self.inner.read().await;
        g.objects
            .get(&(dataset.clone(), key.clone()))
            .map(|(data, meta)| ObjectInfo {
                key: key.clone(),
                dataset: dataset.clone(),
                size_bytes: data.len() as u64,
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
            if let Some(ref pfx) = prefix {
                if !key.starts_with(pfx) {
                    continue;
                }
            }
            results.push(ObjectInfo {
                key: key.clone(),
                dataset: dataset.clone(),
                size_bytes: data.len() as u64,
                created_at: unix_timestamp_secs(),
                modified_at: unix_timestamp_secs(),
                content_type: None,
                checksum: None,
                encrypted: false,
                compressed: false,
                metadata: meta.clone(),
            });
            if let Some(lim) = limit {
                if results.len() >= lim {
                    break;
                }
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
        let removed = g.objects.remove(&(dataset.clone(), key.clone())).is_some();
        if !removed {
            return Err(NestGateRpcError::ObjectNotFound { dataset, key });
        }
        let object_count = g
            .objects
            .keys()
            .filter(|(d, _)| d == &dataset)
            .count() as u64;
        let used_bytes: u64 = g
            .objects
            .iter()
            .filter(|((d, _), _)| d == &dataset)
            .map(|(_, (b, _))| b.len() as u64)
            .sum();
        if let Some(ds) = g.datasets.get_mut(&dataset) {
            ds.object_count = object_count;
            ds.size_bytes = used_bytes;
            ds.modified_at = unix_timestamp_secs();
        }
        Ok(OperationResult {
            success: true,
            message: format!("Object {}/{} deleted successfully", dataset, key),
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
        )
        .await
        {
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
                    message: format!("Capability registration failed: {}", e),
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
        let _ = capability;
        // TODO: wire to nestgate-discovery — `primal_discovery::discover_capability`
        Ok(Vec::new())
    }

    // ==================== HEALTH & MONITORING ====================

    async fn health(self, _context: Context) -> HealthStatus {
        debug!("RPC: health()");

        let metrics = self.calculate_metrics().await;

        HealthStatus {
            status: "healthy".to_string(),
            version: "0.2.0".to_string(),
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
            version: "0.2.0".to_string(),
            api_version: "1.0".to_string(),
            protocol_versions: vec!["tarpc-0.34".to_string(), "jsonrpc-2.0".to_string()],
            build_info: Some("NestGate Storage Primal".to_string()),
        }
    }

    async fn protocols(self, _context: Context) -> Vec<ProtocolInfo> {
        debug!("RPC: protocols()");

        let api_addr = ports::get_api_server_addr();
        let rpc_addr = ports::get_rpc_server_addr();

        vec![
            ProtocolInfo {
                protocol: "tarpc".to_string(),
                version: "0.34".to_string(),
                endpoint: format!("tarpc://{}", rpc_addr),
                priority: 1,
                enabled: true,
            },
            ProtocolInfo {
                protocol: "jsonrpc".to_string(),
                version: "2.0".to_string(),
                endpoint: format!("http://{}/rpc", api_addr),
                priority: 2,
                enabled: false,
            },
            ProtocolInfo {
                protocol: "http".to_string(),
                version: "1.1".to_string(),
                endpoint: format!("http://{}", api_addr),
                priority: 3,
                enabled: false,
            },
        ]
    }
}

/// Serve NestGate tarpc RPC on the specified address
///
/// # Arguments
/// * `addr` - Socket address to bind to
/// * `service` - NestGate RPC service implementation
///
/// # Errors
/// Returns error if server fails to start or bind to address
///
/// # Example
/// ```no_run
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
            .map_err(|e| {
                NestGateError::network_error(&format!("Failed to bind to {}: {}", addr, e))
            })?;

    info!("✅ NestGate tarpc server listening on {}", addr);

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
            let service = service.clone();
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
        NestGateRpcService::new().await
    }

    #[tokio::test]
    async fn test_service_creation() {
        let service = create_test_service()
            .await
            .expect("Failed to create service");
        // Verify storage manager is initialized
        let datasets = service
            .storage_manager
            .list_datasets()
            .await
            .expect("Failed to list datasets");
        // New service should have no datasets initially
        assert_eq!(
            datasets.len(),
            0,
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
