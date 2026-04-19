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

#[cfg(test)]
mod tests {
    use jsonrpsee::core::params::ArrayParams;

    use crate::rpc::jsonrpc_server::{JsonRpcServer, JsonRpcState};
    use crate::rpc::tarpc_server::NestGateRpcService;

    fn build_module_for_tests()
    -> jsonrpsee::RpcModule<JsonRpcState<crate::rpc::storage_backend::InMemoryStorageBackend>> {
        let service = match NestGateRpcService::new() {
            Ok(s) => s,
            Err(e) => panic!("NestGateRpcService::new: {e}"),
        };
        let state = JsonRpcState {
            service,
            start_time: std::time::Instant::now(),
        };
        match JsonRpcServer::build_module(state) {
            Ok(m) => m,
            Err(e) => panic!("build_module: {e}"),
        }
    }

    #[tokio::test]
    async fn health_check_returns_status_uptime_and_version_fields() {
        let module = build_module_for_tests();
        let params = ArrayParams::new();
        let v: serde_json::Value = match module.call("health.check", params).await {
            Ok(x) => x,
            Err(e) => panic!("health.check: {e}"),
        };
        assert!(v.get("status").is_some());
        assert!(v.get("uptime_seconds").is_some());
        assert!(v.get("version").is_some());
    }

    #[tokio::test]
    async fn health_liveness_marks_alive_and_reports_status() {
        let module = build_module_for_tests();
        let params = ArrayParams::new();
        let v: serde_json::Value = match module.call("health.liveness", params).await {
            Ok(x) => x,
            Err(e) => panic!("health.liveness: {e}"),
        };
        assert_eq!(v["alive"], true);
        assert!(v.get("status").is_some());
    }

    #[tokio::test]
    async fn health_readiness_reports_ready_flag() {
        let module = build_module_for_tests();
        let params = ArrayParams::new();
        let v: serde_json::Value = match module.call("health.readiness", params).await {
            Ok(x) => x,
            Err(e) => panic!("health.readiness: {e}"),
        };
        assert!(v.get("ready").is_some());
        assert!(v.get("status").is_some());
    }

    #[tokio::test]
    async fn health_metrics_surface_capacity_and_counts() {
        let module = build_module_for_tests();
        let params = ArrayParams::new();
        let v: serde_json::Value = match module.call("health.metrics", params).await {
            Ok(x) => x,
            Err(e) => panic!("health.metrics: {e}"),
        };
        for key in [
            "total_capacity_bytes",
            "used_space_bytes",
            "available_space_bytes",
            "dataset_count",
            "object_count",
        ] {
            assert!(v.get(key).is_some(), "missing key {key}");
        }
    }

    #[tokio::test]
    async fn health_info_returns_version_and_protocol_metadata() {
        let module = build_module_for_tests();
        let params = ArrayParams::new();
        let v: serde_json::Value = match module.call("health.info", params).await {
            Ok(x) => x,
            Err(e) => panic!("health.info: {e}"),
        };
        assert!(v.get("version").is_some());
        assert!(v.get("api_version").is_some());
        assert!(v.get("protocol_versions").is_some());
        assert!(v.get("build_info").is_some());
    }

    #[tokio::test]
    async fn health_protocols_returns_non_empty_protocol_table() {
        let module = build_module_for_tests();
        let params = ArrayParams::new();
        let rows: Vec<serde_json::Value> = match module.call("health.protocols", params).await {
            Ok(x) => x,
            Err(e) => panic!("health.protocols: {e}"),
        };
        assert!(!rows.is_empty());
        let first = match rows.first() {
            Some(r) => r,
            None => panic!("expected at least one protocol row"),
        };
        assert!(first.get("protocol").is_some());
        assert!(first.get("version").is_some());
        assert!(first.get("enabled").is_some());
    }
}
