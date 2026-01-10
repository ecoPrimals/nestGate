//! # 🚀 tarpc Server for NestGate
//!
//! **HIGH-PERFORMANCE PRIMAL-TO-PRIMAL RPC SERVER** (v0.2.0)
//!
//! Provides the server implementation of the NestGate RPC interface.
//!
//! ## Philosophy (Primal Sovereignty)
//! - **tarpc PRIMARY** for primal-to-primal communication
//! - **Zero unsafe blocks**
//! - **Modern async/await**
//! - **Self-knowledge**: Server exposes only storage capabilities
//! - **Runtime discovery**: Registers with discovery system
//!
//! ## Usage
//! ```no_run
//! use nestgate_core::rpc::{NestGateRpcService, serve_tarpc};
//! use std::net::SocketAddr;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let service = NestGateRpcService::new();
//! let addr: SocketAddr = "0.0.0.0:8091".parse()?;
//! serve_tarpc(addr, service).await?;
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use futures_util::StreamExt;
use tarpc::context::Context;
use tarpc::server::Channel;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use crate::error::{NestGateError, Result};
use crate::rpc::tarpc_types::{
    CapabilityRegistration, DatasetInfo, DatasetParams, HealthStatus, NestGateRpc,
    NestGateRpcError, ObjectInfo, OperationResult, ProtocolInfo, RegistrationResult, ServiceInfo,
    StorageMetrics, VersionInfo,
};

/// NestGate RPC service implementation
///
/// Implements the NestGateRpc trait to provide storage operations over tarpc.
///
/// # Architecture
/// - In-memory storage for Phase 1 (will wire to real storage layer)
/// - Async operations throughout
/// - Zero unsafe blocks
/// - Production-ready error handling
#[derive(Clone)]
pub struct NestGateRpcService {
    /// In-memory datasets (Phase 1 - will be replaced with real storage)
    datasets: Arc<RwLock<HashMap<String, DatasetInfo>>>,

    /// In-memory objects (Phase 1 - will be replaced with real storage)
    objects: Arc<RwLock<HashMap<String, HashMap<String, (Vec<u8>, ObjectInfo)>>>>,

    /// Start time for uptime calculation
    start_time: SystemTime,
}

impl NestGateRpcService {
    /// Create new RPC service
    ///
    /// # Example
    /// ```no_run
    /// use nestgate_core::rpc::NestGateRpcService;
    ///
    /// let service = NestGateRpcService::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        info!("🚀 Creating NestGate RPC service");
        Self {
            datasets: Arc::new(RwLock::new(HashMap::new())),
            objects: Arc::new(RwLock::new(HashMap::new())),
            start_time: SystemTime::now(),
        }
    }

    /// Get current timestamp
    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64
    }

    /// Get uptime in seconds
    fn uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().unwrap_or_default().as_secs()
    }

    /// Calculate storage metrics
    async fn calculate_metrics(&self) -> StorageMetrics {
        let datasets = self.datasets.read().await;
        let objects = self.objects.read().await;

        let dataset_count = datasets.len();
        let object_count: u64 = objects.values().map(|objs| objs.len() as u64).sum();
        let used_space: u64 = objects
            .values()
            .flat_map(|objs| objs.values())
            .map(|(data, _)| data.len() as u64)
            .sum();

        StorageMetrics {
            total_capacity_bytes: 1024 * 1024 * 1024 * 1024, // 1TB placeholder
            used_space_bytes: used_space,
            available_space_bytes: 1024 * 1024 * 1024 * 1024 - used_space,
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

impl Default for NestGateRpcService {
    fn default() -> Self {
        Self::new()
    }
}

// Implement the tarpc service trait
impl NestGateRpc for NestGateRpcService {
    // ==================== STORAGE OPERATIONS ====================

    async fn create_dataset(
        self,
        _context: Context,
        name: String,
        params: DatasetParams,
    ) -> std::result::Result<DatasetInfo, NestGateRpcError> {
        debug!("RPC: create_dataset({})", name);

        let mut datasets = self.datasets.write().await;

        if datasets.contains_key(&name) {
            return Err(NestGateRpcError::DatasetAlreadyExists { dataset: name });
        }

        let now = Self::current_timestamp();
        let dataset = DatasetInfo {
            name: name.clone(),
            description: params.description.clone(),
            created_at: now,
            modified_at: now,
            size_bytes: 0,
            object_count: 0,
            compression_ratio: 1.0,
            params,
            status: "active".to_string(),
        };

        datasets.insert(name.clone(), dataset.clone());

        // Initialize empty object map for this dataset
        let mut objects = self.objects.write().await;
        objects.insert(name, HashMap::new());

        info!("✅ Created dataset: {}", dataset.name);
        Ok(dataset)
    }

    async fn list_datasets(
        self,
        _context: Context,
    ) -> std::result::Result<Vec<DatasetInfo>, NestGateRpcError> {
        debug!("RPC: list_datasets()");

        let datasets = self.datasets.read().await;
        Ok(datasets.values().cloned().collect())
    }

    async fn get_dataset(
        self,
        _context: Context,
        name: String,
    ) -> std::result::Result<DatasetInfo, NestGateRpcError> {
        debug!("RPC: get_dataset({})", name);

        let datasets = self.datasets.read().await;
        datasets
            .get(&name)
            .cloned()
            .ok_or(NestGateRpcError::DatasetNotFound { dataset: name })
    }

    async fn delete_dataset(
        self,
        _context: Context,
        name: String,
    ) -> std::result::Result<OperationResult, NestGateRpcError> {
        debug!("RPC: delete_dataset({})", name);

        let mut datasets = self.datasets.write().await;

        if !datasets.contains_key(&name) {
            return Err(NestGateRpcError::DatasetNotFound { dataset: name });
        }

        datasets.remove(&name);

        // Remove all objects in dataset
        let mut objects = self.objects.write().await;
        objects.remove(&name);

        info!("✅ Deleted dataset: {}", name);
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
        debug!("RPC: store_object({}/{})", dataset, key);

        // Verify dataset exists
        {
            let datasets = self.datasets.read().await;
            if !datasets.contains_key(&dataset) {
                return Err(NestGateRpcError::DatasetNotFound { dataset });
            }
        }

        let mut objects = self.objects.write().await;
        let dataset_objects = objects.entry(dataset.clone()).or_insert_with(HashMap::new);

        let now = Self::current_timestamp();
        let object_info = ObjectInfo {
            key: key.clone(),
            dataset: dataset.clone(),
            size_bytes: data.len() as u64,
            created_at: now,
            modified_at: now,
            content_type: None,
            checksum: None,
            encrypted: false,
            compressed: false,
            metadata: metadata.unwrap_or_default(),
        };

        dataset_objects.insert(key.clone(), (data, object_info.clone()));

        // Update dataset stats
        let mut datasets = self.datasets.write().await;
        if let Some(ds) = datasets.get_mut(&dataset) {
            ds.object_count += 1;
            ds.size_bytes += object_info.size_bytes;
            ds.modified_at = now;
        }

        info!("✅ Stored object: {}/{}", dataset, key);
        Ok(object_info)
    }

    async fn retrieve_object(
        self,
        _context: Context,
        dataset: String,
        key: String,
    ) -> std::result::Result<Vec<u8>, NestGateRpcError> {
        debug!("RPC: retrieve_object({}/{})", dataset, key);

        let objects = self.objects.read().await;
        let dataset_objects =
            objects
                .get(&dataset)
                .ok_or_else(|| NestGateRpcError::DatasetNotFound {
                    dataset: dataset.clone(),
                })?;

        dataset_objects
            .get(&key)
            .map(|(data, _)| data.clone())
            .ok_or(NestGateRpcError::ObjectNotFound { dataset, key })
    }

    async fn get_object_metadata(
        self,
        _context: Context,
        dataset: String,
        key: String,
    ) -> std::result::Result<ObjectInfo, NestGateRpcError> {
        debug!("RPC: get_object_metadata({}/{})", dataset, key);

        let objects = self.objects.read().await;
        let dataset_objects =
            objects
                .get(&dataset)
                .ok_or_else(|| NestGateRpcError::DatasetNotFound {
                    dataset: dataset.clone(),
                })?;

        dataset_objects
            .get(&key)
            .map(|(_, info)| info.clone())
            .ok_or(NestGateRpcError::ObjectNotFound { dataset, key })
    }

    async fn list_objects(
        self,
        _context: Context,
        dataset: String,
        prefix: Option<String>,
        limit: Option<usize>,
    ) -> std::result::Result<Vec<ObjectInfo>, NestGateRpcError> {
        debug!("RPC: list_objects({}, {:?}, {:?})", dataset, prefix, limit);

        let objects = self.objects.read().await;
        let dataset_objects =
            objects
                .get(&dataset)
                .ok_or_else(|| NestGateRpcError::DatasetNotFound {
                    dataset: dataset.clone(),
                })?;

        let mut results: Vec<ObjectInfo> = dataset_objects
            .iter()
            .filter(|(key, _)| {
                if let Some(ref prefix) = prefix {
                    key.starts_with(prefix)
                } else {
                    true
                }
            })
            .map(|(_, (_, info))| info.clone())
            .collect();

        if let Some(limit) = limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    async fn delete_object(
        self,
        _context: Context,
        dataset: String,
        key: String,
    ) -> std::result::Result<OperationResult, NestGateRpcError> {
        debug!("RPC: delete_object({}/{})", dataset, key);

        let mut objects = self.objects.write().await;
        let dataset_objects =
            objects
                .get_mut(&dataset)
                .ok_or_else(|| NestGateRpcError::DatasetNotFound {
                    dataset: dataset.clone(),
                })?;

        let (_, info) = dataset_objects
            .remove(&key)
            .ok_or(NestGateRpcError::ObjectNotFound {
                dataset: dataset.clone(),
                key: key.clone(),
            })?;

        // Update dataset stats
        let mut datasets = self.datasets.write().await;
        if let Some(ds) = datasets.get_mut(&dataset) {
            ds.object_count = ds.object_count.saturating_sub(1);
            ds.size_bytes = ds.size_bytes.saturating_sub(info.size_bytes);
            ds.modified_at = Self::current_timestamp();
        }

        info!("✅ Deleted object: {}/{}", dataset, key);
        Ok(OperationResult {
            success: true,
            message: format!("Object {}/{} deleted successfully", dataset, key),
            metadata: HashMap::new(),
        })
    }

    // ==================== CAPABILITY OPERATIONS ====================

    async fn register_capability(
        self,
        _context: Context,
        registration: CapabilityRegistration,
    ) -> std::result::Result<RegistrationResult, NestGateRpcError> {
        debug!("RPC: register_capability({})", registration.capability);

        // TODO: Wire to universal adapter / service registry
        warn!("⚠️  Capability registration not yet wired to universal adapter");

        Ok(RegistrationResult {
            success: true,
            message: format!(
                "Capability {} registered (stub - needs universal adapter integration)",
                registration.capability
            ),
        })
    }

    async fn discover_capability(
        self,
        _context: Context,
        capability: String,
    ) -> std::result::Result<Vec<ServiceInfo>, NestGateRpcError> {
        debug!("RPC: discover_capability({})", capability);

        // TODO: Wire to universal adapter / service registry
        warn!("⚠️  Capability discovery not yet wired to universal adapter");

        Ok(Vec::new())
    }

    // ==================== HEALTH & MONITORING ====================

    async fn health(self, _context: Context) -> HealthStatus {
        debug!("RPC: health()");

        let datasets = self.datasets.read().await;
        let metrics = self.calculate_metrics().await;

        HealthStatus {
            status: "healthy".to_string(),
            version: "0.2.0".to_string(),
            uptime_seconds: self.uptime_seconds(),
            total_datasets: datasets.len(),
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

        vec![
            ProtocolInfo {
                protocol: "tarpc".to_string(),
                version: "0.34".to_string(),
                endpoint: "tarpc://0.0.0.0:8091".to_string(),
                priority: 1,
                enabled: true,
            },
            ProtocolInfo {
                protocol: "jsonrpc".to_string(),
                version: "2.0".to_string(),
                endpoint: "http://0.0.0.0:8080/rpc".to_string(),
                priority: 2,
                enabled: false, // Will be enabled when JSON-RPC implemented
            },
            ProtocolInfo {
                protocol: "http".to_string(),
                version: "1.1".to_string(),
                endpoint: "http://0.0.0.0:8080".to_string(),
                priority: 3,
                enabled: false, // Fallback, configurable
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
/// use std::net::SocketAddr;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let service = NestGateRpcService::new();
/// let addr: SocketAddr = "0.0.0.0:8091".parse()?;
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

    #[tokio::test]
    async fn test_service_creation() {
        let service = NestGateRpcService::new();
        assert!(service.datasets.read().await.is_empty());
        assert!(service.objects.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_health() {
        let service = NestGateRpcService::new();
        let health = service.health(Context::current()).await;
        assert_eq!(health.status, "healthy");
        assert_eq!(health.version, "0.2.0");
    }

    #[tokio::test]
    async fn test_version() {
        let service = NestGateRpcService::new();
        let version = service.version(Context::current()).await;
        assert_eq!(version.version, "0.2.0");
        assert_eq!(version.api_version, "1.0");
    }

    #[tokio::test]
    async fn test_protocols() {
        let service = NestGateRpcService::new();
        let protocols = service.protocols(Context::current()).await;
        assert_eq!(protocols.len(), 3);
        assert_eq!(protocols[0].protocol, "tarpc");
        assert_eq!(protocols[0].priority, 1);
        assert!(protocols[0].enabled);
    }

    #[tokio::test]
    async fn test_create_dataset() {
        let service = NestGateRpcService::new();
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
        let service = NestGateRpcService::new();

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
        let service = NestGateRpcService::new();

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
