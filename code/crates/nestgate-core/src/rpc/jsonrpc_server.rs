//! JSON-RPC 2.0 Server for NestGate Storage
//!
//! Provides universal, language-agnostic RPC access to NestGate storage
//! capabilities over HTTP. Works with any client supporting JSON-RPC 2.0.
//!
//! ## Philosophy
//!
//! - **Universal Access**: JSON-RPC works with ANY language
//! - **Self-Knowledge**: Exposes only storage capabilities
//! - **Runtime Discovery**: Capability-based service finding
//! - **Same Operations**: 14 operations (same as tarpc)
//!
//! ## Supported Methods
//!
//! Storage Operations (9):
//! - `nestgate.createDataset` - Create a new dataset
//! - `nestgate.listDatasets` - List all datasets
//! - `nestgate.getDataset` - Get dataset info
//! - `nestgate.deleteDataset` - Delete dataset
//! - `nestgate.storeObject` - Store object in dataset
//! - `nestgate.retrieveObject` - Retrieve object data
//! - `nestgate.getObjectMetadata` - Get object metadata
//! - `nestgate.listObjects` - List objects in dataset
//! - `nestgate.deleteObject` - Delete object
//!
//! Capability Operations (2):
//! - `nestgate.registerCapability` - Register service capability
//! - `nestgate.discoverCapability` - Discover services by capability
//!
//! Monitoring Operations (3):
//! - `nestgate.health` - Service health status
//! - `nestgate.metrics` - Storage metrics
//! - `nestgate.version` - Service version
//! - `nestgate.protocols` - Supported protocols

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use base64::Engine;
use jsonrpsee::{
    server::{Server, ServerHandle},
    types::ErrorObjectOwned,
    RpcModule,
};
use tracing::{debug, info, warn};

use super::tarpc_server::NestGateRpcService;
use super::tarpc_types::{DatasetParams, NestGateRpc, OperationResult};

/// JSON-RPC server configuration
#[derive(Debug, Clone)]
pub struct JsonRpcConfig {
    /// Bind address
    pub addr: SocketAddr,
    /// Enable request logging
    pub log_requests: bool,
    /// Maximum request size (bytes)
    pub max_request_size: u32,
    /// Maximum response size (bytes)
    pub max_response_size: u32,
}

impl Default for JsonRpcConfig {
    fn default() -> Self {
        use std::net::{IpAddr, Ipv6Addr};
        Self {
            addr: SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 8092),
            log_requests: true,
            max_request_size: 100 * 1024 * 1024, // 100 MB for large objects
            max_response_size: 100 * 1024 * 1024, // 100 MB
        }
    }
}

/// Shared state for JSON-RPC methods
#[derive(Clone)]
pub struct JsonRpcState {
    /// RPC service instance
    pub service: NestGateRpcService,
    /// Server start time for uptime tracking
    pub start_time: std::time::Instant,
}

/// JSON-RPC 2.0 server for NestGate storage
pub struct JsonRpcServer {
    config: JsonRpcConfig,
    state: JsonRpcState,
}

impl JsonRpcServer {
    /// Create a new JSON-RPC server
    pub fn new(service: NestGateRpcService, config: JsonRpcConfig) -> Self {
        Self {
            config,
            state: JsonRpcState {
                service,
                start_time: std::time::Instant::now(),
            },
        }
    }

    /// Build and start the JSON-RPC server
    pub async fn start(self) -> Result<(ServerHandle, SocketAddr), Box<dyn std::error::Error>> {
        info!(
            "🚀 Starting NestGate JSON-RPC 2.0 server on {}",
            self.config.addr
        );

        // Build server
        let server = Server::builder().build(self.config.addr).await?;

        let addr = server.local_addr()?;

        // Create RPC module with shared state
        let mut module = RpcModule::new(self.state.clone());

        // Register all JSON-RPC methods
        Self::register_storage_methods(&mut module)?;
        Self::register_capability_methods(&mut module)?;
        Self::register_monitoring_methods(&mut module)?;

        // Start server
        let handle = server.start(module);

        info!("✅ NestGate JSON-RPC 2.0 server listening on {}", addr);
        info!("   Endpoint: http://{}/jsonrpc", addr);
        info!("   Methods: 14 registered");
        info!("   Protocol: Primary=tarpc, Secondary=JSON-RPC");

        Ok((handle, addr))
    }

    /// Register storage-related JSON-RPC methods
    fn register_storage_methods(
        module: &mut RpcModule<JsonRpcState>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // nestgate.createDataset
        module.register_async_method("nestgate.createDataset", |params, ctx, _ext| async move {
            #[derive(serde::Deserialize)]
            struct Params {
                name: String,
                #[serde(default)]
                description: Option<String>,
                #[serde(default)]
                compression: Option<String>,
            }

            let p: Params = params.parse()?;
            debug!("JSON-RPC: createDataset({})", p.name);

            let dataset_params = DatasetParams {
                description: p.description,
                compression: p.compression,
                encrypted: false,
                deduplicated: false,
                properties: HashMap::new(),
                quota: None,
            };

            let state = ctx.as_ref();
            // Clone service to satisfy tarpc trait's self-consuming methods
            let service_clone = state.service.clone();
            let result = service_clone
                .create_dataset(tarpc::context::current(), p.name, dataset_params)
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

            Ok::<_, ErrorObjectOwned>(serde_json::json!({
                "name": result.name,
                "description": result.description,
                "created_at": result.created_at,
                "modified_at": result.modified_at,
                "size_bytes": result.size_bytes,
                "object_count": result.object_count,
                "status": result.status,
            }))
        })?;

        // nestgate.listDatasets
        module.register_async_method("nestgate.listDatasets", |_params, ctx, _ext| async move {
            debug!("JSON-RPC: listDatasets()");

            let state = ctx.as_ref();
            let service_clone = state.service.clone();
            let datasets = service_clone
                .list_datasets(tarpc::context::current())
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

            let results: Vec<serde_json::Value> = datasets
                .into_iter()
                .map(|ds| {
                    serde_json::json!({
                        "name": ds.name,
                        "description": ds.description,
                        "created_at": ds.created_at,
                        "modified_at": ds.modified_at,
                        "size_bytes": ds.size_bytes,
                        "object_count": ds.object_count,
                        "status": ds.status,
                    })
                })
                .collect();

            Ok::<_, ErrorObjectOwned>(results)
        })?;

        // nestgate.getDataset
        module.register_async_method("nestgate.getDataset", |params, ctx, _ext| async move {
            let name: String = params.one()?;
            debug!("JSON-RPC: getDataset({})", name);

            let state = ctx.as_ref();
            let service_clone = state.service.clone();
            let dataset = service_clone
                .get_dataset(tarpc::context::current(), name)
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

            Ok::<_, ErrorObjectOwned>(serde_json::json!({
                "name": dataset.name,
                "description": dataset.description,
                "created_at": dataset.created_at,
                "modified_at": dataset.modified_at,
                "size_bytes": dataset.size_bytes,
                "object_count": dataset.object_count,
                "status": dataset.status,
            }))
        })?;

        // nestgate.deleteDataset
        module.register_async_method("nestgate.deleteDataset", |params, ctx, _ext| async move {
            let name: String = params.one()?;
            debug!("JSON-RPC: deleteDataset({})", name);

            let state = ctx.as_ref();
            let service_clone = state.service.clone();
            let result = service_clone
                .delete_dataset(tarpc::context::current(), name)
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

            Ok::<_, ErrorObjectOwned>(serde_json::json!({
                "success": result.success,
                "message": result.message,
            }))
        })?;

        // nestgate.storeObject
        module.register_async_method("nestgate.storeObject", |params, ctx, _ext| async move {
            #[derive(serde::Deserialize)]
            struct Params {
                dataset: String,
                key: String,
                data: String, // base64 encoded
                #[serde(default)]
                metadata: Option<HashMap<String, String>>,
            }

            let p: Params = params.parse()?;
            debug!("JSON-RPC: storeObject({}/{})", p.dataset, p.key);

            // Decode base64 data
            let data = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &p.data)
                .map_err(|e| {
                    ErrorObjectOwned::owned(
                        -32602,
                        format!("Invalid base64 data: {}", e),
                        None::<()>,
                    )
                })?;

            let state = ctx.as_ref();
            let service_clone = state.service.clone();
            let result = service_clone
                .store_object(
                    tarpc::context::current(),
                    p.dataset,
                    p.key,
                    data,
                    p.metadata,
                )
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

            Ok::<_, ErrorObjectOwned>(serde_json::json!({
                "key": result.key,
                "dataset": result.dataset,
                "size_bytes": result.size_bytes,
                "created_at": result.created_at,
                "modified_at": result.modified_at,
            }))
        })?;

        // nestgate.retrieveObject
        module.register_async_method(
            "nestgate.retrieveObject",
            |params, ctx, _ext| async move {
                #[derive(serde::Deserialize)]
                struct Params {
                    dataset: String,
                    key: String,
                }

                let p: Params = params.parse()?;
                debug!("JSON-RPC: retrieveObject({}/{})", p.dataset, p.key);

                let state = ctx.as_ref();
                let service_clone = state.service.clone();
                let data = service_clone
                    .retrieve_object(tarpc::context::current(), p.dataset, p.key)
                    .await
                    .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

                // Encode to base64
                let encoded = base64::engine::general_purpose::STANDARD.encode(&data);

                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "data": encoded,
                    "size_bytes": data.len(),
                }))
            },
        )?;

        // nestgate.getObjectMetadata
        module.register_async_method(
            "nestgate.getObjectMetadata",
            |params, ctx, _ext| async move {
                #[derive(serde::Deserialize)]
                struct Params {
                    dataset: String,
                    key: String,
                }

                let p: Params = params.parse()?;
                debug!("JSON-RPC: getObjectMetadata({}/{})", p.dataset, p.key);

                let state = ctx.as_ref();
                let service_clone = state.service.clone();
                let info = service_clone
                    .get_object_metadata(tarpc::context::current(), p.dataset, p.key)
                    .await
                    .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "key": info.key,
                    "dataset": info.dataset,
                    "size_bytes": info.size_bytes,
                    "created_at": info.created_at,
                    "modified_at": info.modified_at,
                    "metadata": info.metadata,
                }))
            },
        )?;

        // nestgate.listObjects
        module.register_async_method("nestgate.listObjects", |params, ctx, _ext| async move {
            #[derive(serde::Deserialize)]
            struct Params {
                dataset: String,
                #[serde(default)]
                prefix: Option<String>,
                #[serde(default)]
                limit: Option<usize>,
            }

            let p: Params = params.parse()?;
            debug!(
                "JSON-RPC: listObjects({}, {:?}, {:?})",
                p.dataset, p.prefix, p.limit
            );

            let state = ctx.as_ref();
            let service_clone = state.service.clone();
            let objects = service_clone
                .list_objects(tarpc::context::current(), p.dataset, p.prefix, p.limit)
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

            let results: Vec<serde_json::Value> = objects
                .into_iter()
                .map(|obj| {
                    serde_json::json!({
                        "key": obj.key,
                        "dataset": obj.dataset,
                        "size_bytes": obj.size_bytes,
                        "created_at": obj.created_at,
                        "modified_at": obj.modified_at,
                    })
                })
                .collect();

            Ok::<_, ErrorObjectOwned>(results)
        })?;

        // nestgate.deleteObject
        module.register_async_method("nestgate.deleteObject", |params, ctx, _ext| async move {
            #[derive(serde::Deserialize)]
            struct Params {
                dataset: String,
                key: String,
            }

            let p: Params = params.parse()?;
            debug!("JSON-RPC: deleteObject({}/{})", p.dataset, p.key);

            let state = ctx.as_ref();
            let service_clone = state.service.clone();
            let result = service_clone
                .delete_object(tarpc::context::current(), p.dataset, p.key)
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

            Ok::<_, ErrorObjectOwned>(serde_json::json!({
                "success": result.success,
                "message": result.message,
            }))
        })?;

        Ok(())
    }

    /// Register capability-related JSON-RPC methods
    fn register_capability_methods(
        module: &mut RpcModule<JsonRpcState>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // nestgate.registerCapability
        module.register_async_method(
            "nestgate.registerCapability",
            |params, _ctx, _ext| async move {
                #[derive(serde::Deserialize)]
                struct Params {
                    capability: String,
                    endpoint: String,
                    #[serde(default)]
                    metadata: Option<HashMap<String, String>>,
                }

                let p: Params = params.parse()?;
                debug!("JSON-RPC: registerCapability({})", p.capability);

                // TODO: Wire to universal adapter
                warn!("⚠️  Capability registration not yet wired to universal adapter");

                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "success": true,
                    "message": format!("Capability {} registered (stub)", p.capability),
                }))
            },
        )?;

        // nestgate.discoverCapability
        module.register_async_method(
            "nestgate.discoverCapability",
            |params, _ctx, _ext| async move {
                let capability: String = params.one()?;
                debug!("JSON-RPC: discoverCapability({})", capability);

                // TODO: Wire to universal adapter
                warn!("⚠️  Capability discovery not yet wired to universal adapter");

                Ok::<_, ErrorObjectOwned>(Vec::<serde_json::Value>::new())
            },
        )?;

        Ok(())
    }

    /// Register monitoring JSON-RPC methods
    fn register_monitoring_methods(
        module: &mut RpcModule<JsonRpcState>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // nestgate.health
        module.register_async_method("nestgate.health", |_params, ctx, _ext| async move {
            debug!("JSON-RPC: health()");

            let state = ctx.as_ref();
            let service_clone = state.service.clone();
            let health = service_clone.health(tarpc::context::current()).await;

            Ok::<_, ErrorObjectOwned>(serde_json::json!({
                "status": health.status,
                "uptime_seconds": health.uptime_seconds,
                "version": health.version,
            }))
        })?;

        // nestgate.metrics
        module.register_async_method("nestgate.metrics", |_params, ctx, _ext| async move {
            debug!("JSON-RPC: metrics()");

            let state = ctx.as_ref();
            let service_clone = state.service.clone();
            let metrics = service_clone.metrics(tarpc::context::current()).await;

            Ok::<_, ErrorObjectOwned>(serde_json::json!({
                "total_capacity_bytes": metrics.total_capacity_bytes,
                "used_space_bytes": metrics.used_space_bytes,
                "available_space_bytes": metrics.available_space_bytes,
                "dataset_count": metrics.dataset_count,
                "object_count": metrics.object_count,
            }))
        })?;

        // nestgate.version
        module.register_async_method("nestgate.version", |_params, ctx, _ext| async move {
            debug!("JSON-RPC: version()");

            let state = ctx.as_ref();
            let service_clone = state.service.clone();
            let version = service_clone.version(tarpc::context::current()).await;

            Ok::<_, ErrorObjectOwned>(serde_json::json!({
                "version": version.version,
                "api_version": version.api_version,
                "protocol_versions": version.protocol_versions,
                "build_info": version.build_info,
            }))
        })?;

        // nestgate.protocols
        module.register_async_method("nestgate.protocols", |_params, ctx, _ext| async move {
            debug!("JSON-RPC: protocols()");

            let state = ctx.as_ref();
            let service_clone = state.service.clone();
            let protocols = service_clone.protocols(tarpc::context::current()).await;

            let results: Vec<serde_json::Value> = protocols
                .into_iter()
                .map(|proto| {
                    serde_json::json!({
                        "protocol": proto.protocol,
                        "version": proto.version,
                        "enabled": proto.enabled,
                    })
                })
                .collect();

            Ok::<_, ErrorObjectOwned>(results)
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsonrpc_config_default() {
        let config = JsonRpcConfig::default();
        assert!(config.log_requests);
        assert_eq!(config.max_request_size, 100 * 1024 * 1024);
        assert_eq!(config.max_response_size, 100 * 1024 * 1024);
    }

    #[test]
    fn test_jsonrpc_server_creation() {
        let service = NestGateRpcService::new();
        let config = JsonRpcConfig::default();
        let _server = JsonRpcServer::new(service, config);
    }

    #[test]
    fn test_jsonrpc_config_custom() {
        use std::net::{IpAddr, Ipv4Addr};
        let config = JsonRpcConfig {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9999),
            log_requests: false,
            max_request_size: 1024,
            max_response_size: 2048,
        };
        assert!(!config.log_requests);
        assert_eq!(config.max_request_size, 1024);
        assert_eq!(config.max_response_size, 2048);
        assert_eq!(config.addr.port(), 9999);
    }

    #[tokio::test]
    async fn test_jsonrpc_state_creation() {
        let service = NestGateRpcService::new();
        let state = JsonRpcState {
            service: service.clone(),
            start_time: std::time::Instant::now(),
        };

        // Verify state is clonable
        let _state_clone = state.clone();
    }

    #[test]
    fn test_base64_encoding_decoding() {
        let data = b"Hello, NestGate!";
        let encoded = base64::engine::general_purpose::STANDARD.encode(data);
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&encoded)
            .unwrap();
        assert_eq!(data.to_vec(), decoded);
    }

    #[test]
    fn test_jsonrpc_endpoint_format() {
        let config = JsonRpcConfig::default();
        let addr_str = format!("http://{}/jsonrpc", config.addr);
        assert!(addr_str.contains("/jsonrpc"));
    }

    #[test]
    fn test_multiple_servers() {
        // Verify we can create multiple server instances
        let service1 = NestGateRpcService::new();
        let service2 = NestGateRpcService::new();
        let config = JsonRpcConfig::default();

        let _server1 = JsonRpcServer::new(service1, config.clone());
        let _server2 = JsonRpcServer::new(service2, config);
    }
}
