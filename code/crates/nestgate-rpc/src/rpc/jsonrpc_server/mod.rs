// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC 2.0 Server for `NestGate` Storage
//!
//! Provides universal, language-agnostic RPC access to `NestGate` storage
//! capabilities over HTTP. Works with any client supporting JSON-RPC 2.0.
//!
//! ## Philosophy
//!
//! - **Universal Access**: JSON-RPC works with ANY language
//! - **Self-Knowledge**: Exposes only storage capabilities
//! - **Runtime Discovery**: Capability-based service finding
//! - **Same Operations**: 18 operations (same as tarpc)
//!
//! ## Supported Methods (wateringHole `domain.operation` standard)
//!
//! Storage Dataset Operations (4):
//! - `storage.dataset.create` - Create a new dataset
//! - `storage.dataset.list` - List all datasets
//! - `storage.dataset.get` - Get dataset info
//! - `storage.dataset.delete` - Delete dataset
//!
//! Storage Object Operations (5):
//! - `storage.object.store` - Store object in dataset
//! - `storage.object.retrieve` - Retrieve object data
//! - `storage.object.metadata` - Get object metadata
//! - `storage.object.list` - List objects in dataset
//! - `storage.object.delete` - Delete object
//!
//! Discovery Operations (2):
//! - `discovery.capability.register` - Register service capability
//! - `discovery.capability.query` - Discover services by capability
//!
//! Health Operations (6):
//! - `health.check` - Service health status
//! - `health.liveness` - Process alive (minimal probe)
//! - `health.readiness` - Ready to serve traffic
//! - `health.metrics` - Storage metrics
//! - `health.info` - Service version and build metadata
//! - `health.protocols` - Supported protocols
//!
//! Capabilities (1):
//! - `capabilities.list` - Supported JSON-RPC method names

use std::borrow::Cow;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use bytes::Bytes;

use base64::Engine;
use jsonrpsee::{
    RpcModule,
    server::{Server, ServerHandle},
    types::ErrorObjectOwned,
};
use tracing::{debug, info, warn};

use nestgate_types::NestGateError;

use nestgate_config::config::capability_discovery::DiscoverySource;
use nestgate_config::constants::ports::default_tarpc_client_endpoint;

use super::tarpc_server::NestGateRpcService;
use super::tarpc_types::{DatasetParams, NestGateRpc};

/// Maps jsonrpsee method registration failures (`RegisterMethodError`) into [`NestGateError`].
/// jsonrpsee does not use our error type for `RpcModule::register_*`; we normalize at this boundary.
#[inline]
fn map_jsonrpc_registration<T>(
    result: std::result::Result<T, impl std::fmt::Display>,
) -> std::result::Result<T, NestGateError> {
    result.map_err(|e| NestGateError::internal(format!("JSON-RPC registration: {e}")))
}

/// Registered JSON-RPC method names for `capabilities.list` (static slice avoids per-request `Vec` allocation).
const JSON_RPC_CAPABILITIES_METHODS: &[&str] = &[
    "storage.dataset.create",
    "storage.dataset.list",
    "storage.dataset.get",
    "storage.dataset.delete",
    "storage.object.store",
    "storage.object.retrieve",
    "storage.object.metadata",
    "storage.object.list",
    "storage.object.delete",
    "discovery.capability.register",
    "discovery.capability.query",
    "health.check",
    "health.liveness",
    "health.readiness",
    "health.metrics",
    "health.info",
    "health.protocols",
    "capabilities.list",
    "identity.get",
];

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

/// JSON-RPC 2.0 server for `NestGate` storage
pub struct JsonRpcServer {
    config: JsonRpcConfig,
    state: JsonRpcState,
}

impl JsonRpcServer {
    /// Create a new JSON-RPC server
    #[must_use]
    pub fn new(service: NestGateRpcService, config: JsonRpcConfig) -> Self {
        Self {
            config,
            state: JsonRpcState {
                service,
                start_time: std::time::Instant::now(),
            },
        }
    }

    /// Build RPC module with all methods registered (used by `start()` and tests)
    pub(crate) fn build_module(
        state: JsonRpcState,
    ) -> Result<RpcModule<JsonRpcState>, NestGateError> {
        let mut module = RpcModule::new(state);
        Self::register_storage_methods(&mut module)?;
        Self::register_capability_methods(&mut module)?;
        Self::register_monitoring_methods(&mut module)?;
        Ok(module)
    }

    /// Build and start the JSON-RPC server
    ///
    /// # Errors
    ///
    /// Returns an error if the HTTP server fails to bind, the listening address cannot be read,
    /// or JSON-RPC method registration fails.
    pub async fn start(self) -> Result<(ServerHandle, SocketAddr), NestGateError> {
        info!(
            "🚀 Starting NestGate JSON-RPC 2.0 server on {}",
            self.config.addr
        );

        // Build server
        let server = Server::builder().build(self.config.addr).await?;

        let addr = server.local_addr()?;

        // Create RPC module with all methods registered
        let module = Self::build_module(self.state.clone())?;

        // Start server
        let handle = server.start(module);

        info!("✅ NestGate JSON-RPC 2.0 server listening on {}", addr);
        info!("   Endpoint: http://{}/jsonrpc", addr);
        info!("   Methods: 18 registered");
        info!("   Protocol: Primary=tarpc, Secondary=JSON-RPC");

        Ok((handle, addr))
    }

    /// Register storage-related JSON-RPC methods
    #[expect(clippy::too_many_lines)] // Method table mirrors JSON-RPC surface; split would obscure routing.
    fn register_storage_methods(module: &mut RpcModule<JsonRpcState>) -> Result<(), NestGateError> {
        // nestgate.createDataset
        map_jsonrpc_registration(module.register_async_method(
            "storage.dataset.create",
            |params, ctx, _ext| async move {
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
                    .create_dataset(tarpc::context::current(), Arc::from(p.name), dataset_params)
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
            },
        ))?;

        // nestgate.listDatasets
        map_jsonrpc_registration(module.register_async_method(
            "storage.dataset.list",
            |_params, ctx, _ext| async move {
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
            },
        ))?;

        // nestgate.getDataset
        map_jsonrpc_registration(module.register_async_method(
            "storage.dataset.get",
            |params, ctx, _ext| async move {
                let name: String = params.one()?;
                debug!("JSON-RPC: getDataset({})", name);

                let state = ctx.as_ref();
                let service_clone = state.service.clone();
                let dataset = service_clone
                    .get_dataset(tarpc::context::current(), Arc::from(name))
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
            },
        ))?;

        // nestgate.deleteDataset
        map_jsonrpc_registration(module.register_async_method(
            "storage.dataset.delete",
            |params, ctx, _ext| async move {
                let name: String = params.one()?;
                debug!("JSON-RPC: deleteDataset({})", name);

                let state = ctx.as_ref();
                let service_clone = state.service.clone();
                let result = service_clone
                    .delete_dataset(tarpc::context::current(), Arc::from(name))
                    .await
                    .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "success": result.success,
                    "message": result.message,
                }))
            },
        ))?;

        // nestgate.storeObject
        map_jsonrpc_registration(module.register_async_method(
            "storage.object.store",
            |params, ctx, _ext| async move {
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
                let data =
                    base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &p.data)
                        .map_err(|e| {
                            ErrorObjectOwned::owned(
                                -32602,
                                format!("Invalid base64 data: {e}"),
                                None::<()>,
                            )
                        })?;

                let state = ctx.as_ref();
                let service_clone = state.service.clone();
                let result = service_clone
                    .store_object(
                        tarpc::context::current(),
                        Arc::from(p.dataset),
                        Arc::from(p.key),
                        Bytes::from(data),
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
            },
        ))?;

        // nestgate.retrieveObject
        map_jsonrpc_registration(module.register_async_method(
            "storage.object.retrieve",
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
                    .retrieve_object(
                        tarpc::context::current(),
                        Arc::from(p.dataset),
                        Arc::from(p.key),
                    )
                    .await
                    .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

                // Encode to base64
                let encoded = base64::engine::general_purpose::STANDARD.encode(data.as_ref());

                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "data": encoded,
                    "size_bytes": data.len(),
                }))
            },
        ))?;

        // nestgate.getObjectMetadata
        map_jsonrpc_registration(module.register_async_method(
            "storage.object.metadata",
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
                    .get_object_metadata(
                        tarpc::context::current(),
                        Arc::from(p.dataset),
                        Arc::from(p.key),
                    )
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
        ))?;

        // nestgate.listObjects
        map_jsonrpc_registration(module.register_async_method(
            "storage.object.list",
            |params, ctx, _ext| async move {
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
                    .list_objects(
                        tarpc::context::current(),
                        Arc::from(p.dataset),
                        p.prefix.map(Arc::<str>::from),
                        p.limit,
                    )
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
            },
        ))?;

        // nestgate.deleteObject
        map_jsonrpc_registration(module.register_async_method(
            "storage.object.delete",
            |params, ctx, _ext| async move {
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
                    .delete_object(tarpc::context::current(), p.dataset.into(), p.key.into())
                    .await
                    .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "success": result.success,
                    "message": result.message,
                }))
            },
        ))?;

        Ok(())
    }

    /// Register capability-related JSON-RPC methods
    #[expect(
        clippy::too_many_lines,
        reason = "single cohesive registration block for all capability methods"
    )]
    fn register_capability_methods(
        module: &mut RpcModule<JsonRpcState>,
    ) -> Result<(), NestGateError> {
        // nestgate.registerCapability
        map_jsonrpc_registration(module.register_async_method(
            "discovery.capability.register",
            |params, _ctx, _ext| async move {
                #[derive(serde::Deserialize)]
                struct Params {
                    capability: String,
                    endpoint: String,
                    #[serde(default)]
                    _metadata: Option<HashMap<String, String>>,
                }

                let p: Params = params.parse()?;
                debug!("JSON-RPC: registerCapability({})", p.capability);

                // Announce capability via discovery mechanism
                match nestgate_config::config::capability_discovery::announce_capability(
                    &p.capability,
                    &p.endpoint,
                    std::time::Duration::from_secs(60),
                ) {
                    Ok(()) => {
                        info!("✅ Capability '{}' registered successfully", p.capability);
                        Ok::<_, ErrorObjectOwned>(serde_json::json!({
                            "success": true,
                            "message": format!("Capability {} registered and announced", p.capability),
                        }))
                    }
                    Err(e) => {
                        warn!("Failed to register capability '{}': {}", p.capability, e);
                        Ok::<_, ErrorObjectOwned>(serde_json::json!({
                            "success": false,
                            "message": format!("Registration failed: {}", e),
                        }))
                    }
                }
            },
        ))?;

        // nestgate.discoverCapability
        map_jsonrpc_registration(
            module.register_async_method(
                "discovery.capability.query",
                |params, _ctx, _ext| async move {
                    let capability: String = params.one()?;
                    debug!("JSON-RPC: discoverCapability({})", capability);

                    let env_var = format!(
                        "NESTGATE_{}_ENDPOINT",
                        capability.to_uppercase().replace('-', "_")
                    );
                    let discovery_default = default_tarpc_client_endpoint();
                    let se =
                        match nestgate_config::config::capability_discovery::discover_with_fallback(
                            &capability,
                            &env_var,
                            &discovery_default,
                        ) {
                            Ok(se) => se,
                            Err(e) => {
                                warn!("discovery.capability.query: {}", e);
                                return Ok::<_, ErrorObjectOwned>(serde_json::json!([]));
                            }
                        };
                    if se.source == DiscoverySource::Default {
                        warn!(
                            capability = %capability,
                            endpoint = %se.endpoint,
                            env_var = %env_var,
                            "discovery.capability.query using env-derived default tarpc endpoint"
                        );
                    }
                    let raw = se.endpoint.trim();
                    let tarpc_ep: Cow<'_, str> = if raw.starts_with("tarpc://") {
                        Cow::Borrowed(raw)
                    } else if let Some(r) = raw.strip_prefix("http://") {
                        Cow::Owned(format!("tarpc://{r}"))
                    } else if let Some(r) = raw.strip_prefix("https://") {
                        Cow::Owned(format!("tarpc://{r}"))
                    } else {
                        Cow::Owned(format!("tarpc://{raw}"))
                    };
                    Ok::<_, ErrorObjectOwned>(serde_json::json!([{
                        "id": format!("discovered-{}", capability),
                        "capability": capability,
                        "endpoints": { "tarpc": tarpc_ep },
                        "status": "discovered",
                        "metadata": null
                    }]))
                },
            ),
        )?;

        // capabilities.list — Wire Standard L2 envelope
        map_jsonrpc_registration(module.register_async_method(
            "capabilities.list",
            |_params, _ctx, _ext| async move {
                debug!("JSON-RPC: capabilities.list()");
                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "primal": nestgate_config::constants::system::DEFAULT_SERVICE_NAME,
                    "version": env!("CARGO_PKG_VERSION"),
                    "methods": JSON_RPC_CAPABILITIES_METHODS
                }))
            },
        ))?;

        // identity.get — Wire Standard L2 identity endpoint
        map_jsonrpc_registration(module.register_async_method(
            "identity.get",
            |_params, _ctx, _ext| async move {
                debug!("JSON-RPC: identity.get()");
                let family_id =
                    std::env::var("NESTGATE_FAMILY_ID").unwrap_or_else(|_| "default".to_string());
                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "primal": nestgate_config::constants::system::DEFAULT_SERVICE_NAME,
                    "version": env!("CARGO_PKG_VERSION"),
                    "domain": "storage",
                    "license": "AGPL-3.0-or-later",
                    "family_id": family_id
                }))
            },
        ))?;

        Ok(())
    }

    /// Register monitoring JSON-RPC methods
    fn register_monitoring_methods(
        module: &mut RpcModule<JsonRpcState>,
    ) -> Result<(), NestGateError> {
        // nestgate.health
        map_jsonrpc_registration(module.register_async_method(
            "health.check",
            |_params, ctx, _ext| async move {
                debug!("JSON-RPC: health.check()");

                let state = ctx.as_ref();
                let service_clone = state.service.clone();
                let health = service_clone.health(tarpc::context::current()).await;

                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "status": health.status,
                    "uptime_seconds": health.uptime_seconds,
                    "version": health.version,
                }))
            },
        ))?;

        map_jsonrpc_registration(module.register_async_method(
            "health.liveness",
            |_params, ctx, _ext| async move {
                debug!("JSON-RPC: health.liveness()");

                let state = ctx.as_ref();
                let service_clone = state.service.clone();
                let health = service_clone.health(tarpc::context::current()).await;

                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "alive": true,
                    "status": health.status,
                }))
            },
        ))?;

        map_jsonrpc_registration(module.register_async_method(
            "health.readiness",
            |_params, ctx, _ext| async move {
                debug!("JSON-RPC: health.readiness()");

                let state = ctx.as_ref();
                let service_clone = state.service.clone();
                let health = service_clone.health(tarpc::context::current()).await;

                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "ready": health.status == "healthy",
                    "status": health.status,
                }))
            },
        ))?;

        // nestgate.metrics
        map_jsonrpc_registration(module.register_async_method(
            "health.metrics",
            |_params, ctx, _ext| async move {
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
            },
        ))?;

        // nestgate.version (semantic: health.info)
        map_jsonrpc_registration(module.register_async_method(
            "health.info",
            |_params, ctx, _ext| async move {
                debug!("JSON-RPC: health.info()");

                let state = ctx.as_ref();
                let service_clone = state.service.clone();
                let version = service_clone.version(tarpc::context::current()).await;

                Ok::<_, ErrorObjectOwned>(serde_json::json!({
                    "version": version.version,
                    "api_version": version.api_version,
                    "protocol_versions": version.protocol_versions,
                    "build_info": version.build_info,
                }))
            },
        ))?;

        // nestgate.protocols
        map_jsonrpc_registration(module.register_async_method(
            "health.protocols",
            |_params, ctx, _ext| async move {
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
            },
        ))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests;
