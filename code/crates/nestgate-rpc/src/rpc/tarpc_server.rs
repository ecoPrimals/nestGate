// SPDX-License-Identifier: AGPL-3.0-or-later
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
//! - **NG-01 resolved**: All storage operations delegate to a [`StorageBackend`]
//!   trait object, which is filesystem-backed via `nestgate-core` in production
//!   and in-memory for tests.
//! - Self-knowledge: exposes only storage capabilities
//! - Runtime discovery: registers with discovery system
//! - Concurrent access via `tokio::sync::RwLock`
//!
//! ## Usage
//! ```rust,ignore
//! use nestgate_rpc::rpc::{NestGateRpcService, serve_tarpc, InMemoryStorageBackend};
//! use nestgate_config::constants::ports;
//! use std::net::SocketAddr;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let backend = InMemoryStorageBackend::new();
//! let service = NestGateRpcService::with_backend(backend);
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

use crate::rpc::storage_backend::StorageBackend;
use crate::rpc::tarpc_types::{
    CapabilityRegistration, DatasetParams, HealthStatus, NestGateRpc, NestGateRpcError, ObjectInfo,
    OperationResult, ProtocolInfo, RegistrationResult, ServiceInfo, StorageMetrics, VersionInfo,
};
use nestgate_config::config::capability_discovery::{self, DiscoverySource};
use nestgate_config::constants::ports::{self, default_tarpc_client_endpoint};
use nestgate_types::error::{NestGateError, Result};

/// Semantic `NestGate` RPC API version (`VersionInfo::api_version`).
const NESTGATE_RPC_API_VERSION: &str = "1.0";

/// Wire-format protocol identifiers (`VersionInfo::protocol_versions`). Must stay aligned with
/// `ProtocolInfo::version` for each protocol in [`NestGateRpcService::protocols`].
const TARPC_WIRE_VERSION: &str = "0.34";
const PROTOCOL_VERSION_TARPC: &str = "tarpc-0.34";
const JSONRPC_WIRE_VERSION: &str = "2.0";
const PROTOCOL_VERSION_JSONRPC: &str = "jsonrpc-2.0";
const HTTP_WIRE_VERSION: &str = "1.1";

/// `NestGate` RPC service implementation.
///
/// Delegates all storage operations to the injected [`StorageBackend`]. In
/// production this is `CoreStorageBackend` (filesystem-backed via
/// `StorageManagerService`); in tests it is
/// [`InMemoryStorageBackend`](crate::rpc::storage_backend::InMemoryStorageBackend).
#[derive(Clone)]
pub struct NestGateRpcService {
    pub(crate) start_time: SystemTime,
    pub(crate) backend: Arc<dyn StorageBackend>,
}

impl NestGateRpcService {
    /// Create an RPC service backed by the given [`StorageBackend`].
    #[must_use]
    pub fn with_backend(backend: impl StorageBackend + 'static) -> Self {
        info!("🚀 Creating NestGate RPC service (StorageBackend-backed)");
        Self {
            start_time: SystemTime::now(),
            backend: Arc::new(backend),
        }
    }

    /// Create an RPC service from a pre-wrapped `Arc<dyn StorageBackend>`.
    #[must_use]
    pub fn with_backend_arc(backend: Arc<dyn StorageBackend>) -> Self {
        info!("🚀 Creating NestGate RPC service (StorageBackend-backed, Arc)");
        Self {
            start_time: SystemTime::now(),
            backend,
        }
    }

    /// Create new RPC service with the default in-memory backend (tests / standalone).
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`] when the service cannot be constructed; the current
    /// implementation always succeeds and reserves this for future initialization.
    pub fn new() -> Result<Self> {
        info!("🚀 Creating NestGate RPC service (in-memory backend for standalone/test)");
        Ok(Self::with_backend(
            crate::rpc::storage_backend::InMemoryStorageBackend::new(),
        ))
    }

    /// Get uptime in seconds
    fn uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().unwrap_or_default().as_secs()
    }

    /// Calculate storage metrics from the backend.
    async fn calculate_metrics(&self) -> StorageMetrics {
        let datasets = self.backend.list_datasets().await.unwrap_or_default();
        let dataset_count = datasets.len();
        let used_space: u64 = datasets.iter().map(|d| d.size_bytes).sum();
        let object_count: u64 = datasets.iter().map(|d| d.object_count).sum();
        let compression_sum: f64 = datasets.iter().map(|d| d.compression_ratio).sum();
        let avg_compression_ratio = if dataset_count > 0 {
            #[expect(clippy::cast_precision_loss)]
            {
                compression_sum / dataset_count as f64
            }
        } else {
            1.0
        };

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

fn rpc_err(e: &nestgate_types::error::NestGateError) -> NestGateRpcError {
    NestGateRpcError::InternalError {
        message: e.to_string(),
    }
}

impl NestGateRpc for NestGateRpcService {
    // ==================== STORAGE OPERATIONS (delegated to StorageBackend) ====

    async fn create_dataset(
        self,
        _context: Context,
        name: String,
        params: DatasetParams,
    ) -> std::result::Result<crate::rpc::tarpc_types::DatasetInfo, NestGateRpcError> {
        debug!("RPC: create_dataset({}) → backend", name);
        self.backend
            .create_dataset(&name, params)
            .await
            .map_err(|e| rpc_err(&e))
    }

    async fn list_datasets(
        self,
        _context: Context,
    ) -> std::result::Result<Vec<crate::rpc::tarpc_types::DatasetInfo>, NestGateRpcError> {
        debug!("RPC: list_datasets() → backend");
        self.backend.list_datasets().await.map_err(|e| rpc_err(&e))
    }

    async fn get_dataset(
        self,
        _context: Context,
        name: String,
    ) -> std::result::Result<crate::rpc::tarpc_types::DatasetInfo, NestGateRpcError> {
        debug!("RPC: get_dataset({}) → backend", name);
        self.backend
            .get_dataset(&name)
            .await
            .map_err(|e| rpc_err(&e))
    }

    async fn delete_dataset(
        self,
        _context: Context,
        name: String,
    ) -> std::result::Result<OperationResult, NestGateRpcError> {
        debug!("RPC: delete_dataset({}) → backend", name);
        self.backend
            .delete_dataset(&name)
            .await
            .map_err(|e| rpc_err(&e))
    }

    async fn store_object(
        self,
        _context: Context,
        dataset: String,
        key: String,
        data: Vec<u8>,
        metadata: Option<HashMap<String, String>>,
    ) -> std::result::Result<ObjectInfo, NestGateRpcError> {
        debug!("RPC: store_object({}/{}) → backend", dataset, key);
        self.backend
            .store_object(&dataset, &key, Bytes::from(data), metadata)
            .await
            .map_err(|e| rpc_err(&e))
    }

    async fn retrieve_object(
        self,
        _context: Context,
        dataset: String,
        key: String,
    ) -> std::result::Result<Vec<u8>, NestGateRpcError> {
        debug!("RPC: retrieve_object({}/{}) → backend", dataset, key);
        self.backend
            .retrieve_object(&dataset, &key)
            .await
            .map(|b| b.to_vec())
            .map_err(|e| rpc_err(&e))
    }

    async fn get_object_metadata(
        self,
        _context: Context,
        dataset: String,
        key: String,
    ) -> std::result::Result<ObjectInfo, NestGateRpcError> {
        debug!("RPC: get_object_metadata({}/{}) → backend", dataset, key);
        self.backend
            .get_object_metadata(&dataset, &key)
            .await
            .map_err(|e| rpc_err(&e))
    }

    async fn list_objects(
        self,
        _context: Context,
        dataset: String,
        prefix: Option<String>,
        limit: Option<usize>,
    ) -> std::result::Result<Vec<ObjectInfo>, NestGateRpcError> {
        debug!(
            "RPC: list_objects({}, {:?}, {:?}) → backend",
            dataset, prefix, limit
        );
        self.backend
            .list_objects(&dataset, prefix.as_deref(), limit)
            .await
            .map_err(|e| rpc_err(&e))
    }

    async fn delete_object(
        self,
        _context: Context,
        dataset: String,
        key: String,
    ) -> std::result::Result<OperationResult, NestGateRpcError> {
        debug!("RPC: delete_object({}/{}) → backend", dataset, key);
        self.backend
            .delete_object(&dataset, &key)
            .await
            .map_err(|e| rpc_err(&e))
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
            version: env!("CARGO_PKG_VERSION").to_string(),
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
            version: env!("CARGO_PKG_VERSION").to_string(),
            api_version: NESTGATE_RPC_API_VERSION.to_string(),
            protocol_versions: vec![
                PROTOCOL_VERSION_TARPC.to_string(),
                PROTOCOL_VERSION_JSONRPC.to_string(),
            ],
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
                version: TARPC_WIRE_VERSION.to_string(),
                endpoint: format!("tarpc://{rpc_addr}"),
                priority: 1,
                enabled: true,
            },
            ProtocolInfo {
                protocol: String::from("jsonrpc"),
                version: JSONRPC_WIRE_VERSION.to_string(),
                endpoint: format!("http://{api_addr}/rpc"),
                priority: 2,
                enabled: false,
            },
            ProtocolInfo {
                protocol: String::from("http"),
                version: HTTP_WIRE_VERSION.to_string(),
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
pub async fn serve_tarpc(addr: SocketAddr, service: NestGateRpcService) -> Result<()> {
    info!("🚀 Starting NestGate tarpc server on {}", addr);

    let listener =
        tarpc::serde_transport::tcp::listen(addr, tokio_serde::formats::Bincode::default)
            .await
            .map_err(|e| NestGateError::network_error(format!("Failed to bind to {addr}: {e}")))?;

    info!("✅ NestGate tarpc server listening on {}", addr);

    let backend = Arc::clone(&service.backend);
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
        .map(move |transport| {
            let server = tarpc::server::BaseChannel::with_defaults(transport);
            let service = NestGateRpcService {
                start_time,
                backend: Arc::clone(&backend),
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
        let datasets = service.backend.list_datasets().await.unwrap();
        assert!(
            datasets.is_empty(),
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
        assert_eq!(health.version, env!("CARGO_PKG_VERSION"));
    }

    #[tokio::test]
    async fn test_version() {
        let service = create_test_service()
            .await
            .expect("Failed to create service");
        let version = service.version(Context::current()).await;
        assert_eq!(version.version, env!("CARGO_PKG_VERSION"));
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

        service
            .clone()
            .create_dataset(
                Context::current(),
                "test-dataset".to_string(),
                DatasetParams::default(),
            )
            .await
            .unwrap();

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

        service
            .clone()
            .create_dataset(
                Context::current(),
                "test-dataset".to_string(),
                DatasetParams::default(),
            )
            .await
            .unwrap();

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
