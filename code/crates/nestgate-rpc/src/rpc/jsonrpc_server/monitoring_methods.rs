// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC registration for health and metrics endpoints.

use jsonrpsee::{RpcModule, types::ErrorObjectOwned};
use tracing::debug;

use nestgate_types::NestGateError;

use crate::rpc::storage_backend::StorageBackend;
use crate::rpc::tarpc_types::NestGateRpc;

use super::JsonRpcState;
use super::map_jsonrpc_registration;

/// Register monitoring JSON-RPC methods
pub(super) fn register_monitoring_methods<S: StorageBackend + 'static>(
    module: &mut RpcModule<JsonRpcState<S>>,
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
