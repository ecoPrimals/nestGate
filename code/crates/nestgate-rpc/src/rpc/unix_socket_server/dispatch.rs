// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Request dispatch — routes JSON-RPC method names to handler functions.

use std::borrow::Cow;
use std::sync::Arc;

use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use nestgate_types::error::NestGateError;
use serde_json::{Value, json};

use crate::rpc::model_cache_handlers;
use crate::rpc::protocol::normalize_method;

use super::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, StorageState, audit_handlers, blob_handlers,
    bonding_handlers, external_handlers, nat_handlers, session_handlers, storage_handlers,
    template_handlers, zfs_handlers,
};

/// Handle JSON-RPC request
#[expect(
    clippy::too_many_lines,
    reason = "Large method dispatch table mirrors supported JSON-RPC surface."
)]
pub(super) async fn handle_request(
    request: JsonRpcRequest,
    state: &StorageState,
) -> JsonRpcResponse {
    if request.jsonrpc.as_ref() != "2.0" {
        return JsonRpcResponse {
            jsonrpc: Arc::from("2.0"),
            result: None,
            error: Some(JsonRpcError {
                code: -32600,
                message: Cow::Borrowed("Invalid Request"),
                data: Some(json!({"error": "Only JSON-RPC 2.0 is supported"})),
            }),
            id: request.id,
        };
    }

    let method = normalize_method(&request.method);
    let result = match method.as_ref() {
        // Health — wateringHole semantic names + legacy aliases
        "health.liveness" => Ok(json!({
            "status": "alive",
            "primal": DEFAULT_SERVICE_NAME,
        })),
        "health.readiness" => Ok(if state.storage_initialized {
            json!({
                "status": "ready",
                "primal": DEFAULT_SERVICE_NAME,
                "storage": "initialized",
            })
        } else {
            json!({
                "status": "not_ready",
                "primal": DEFAULT_SERVICE_NAME,
                "storage": "not_initialized",
            })
        }),
        "health" | "health.check" => Ok(
            json!({"status": "healthy", "version": env!("CARGO_PKG_VERSION"), "primal": DEFAULT_SERVICE_NAME}),
        ),
        "identity.get" => Ok(json!({
            "primal": DEFAULT_SERVICE_NAME,
            "version": env!("CARGO_PKG_VERSION"),
            "domain": "storage",
            "license": "AGPL-3.0-or-later",
            "family_id": std::env::var("NESTGATE_FAMILY_ID").unwrap_or_else(|_| "default".into()),
        })),
        "capabilities.list" | "capability.list" => model_cache_handlers::capabilities_list(),
        "discover_capabilities" | "discover.capabilities" => {
            model_cache_handlers::discover_capabilities()
        }
        "discovery.capability.register" => discovery_capability_register(request.params.as_ref()),
        // Storage operations (filesystem-backed, durable)
        "storage.store" | "storage.put" => {
            storage_handlers::storage_store(request.params.as_ref(), state).await
        }
        "storage.retrieve" | "storage.get" => {
            storage_handlers::storage_retrieve(request.params.as_ref(), state).await
        }
        "storage.exists" => storage_handlers::storage_exists(request.params.as_ref(), state),
        "storage.delete" => storage_handlers::storage_delete(request.params.as_ref(), state).await,
        "storage.list" => storage_handlers::storage_list(request.params.as_ref(), state).await,
        "storage.stats" => storage_handlers::storage_stats(request.params.as_ref(), state).await,
        "storage.store_blob" => {
            blob_handlers::storage_store_blob(request.params.as_ref(), state).await
        }
        "storage.retrieve_blob" => {
            blob_handlers::storage_retrieve_blob(request.params.as_ref(), state).await
        }
        "storage.retrieve_range" => {
            blob_handlers::storage_retrieve_range(request.params.as_ref(), state).await
        }
        "storage.store_stream" => {
            let params = request.params.clone().unwrap_or_else(|| json!({}));
            crate::rpc::storage_stream::storage_store_stream_begin(
                params,
                state.family_id.as_deref(),
            )
            .await
        }
        "storage.store_stream_chunk" => {
            let params = request.params.clone().unwrap_or_else(|| json!({}));
            crate::rpc::storage_stream::storage_store_stream_chunk(params).await
        }
        "storage.retrieve_stream" => {
            let params = request.params.clone().unwrap_or_else(|| json!({}));
            crate::rpc::storage_stream::storage_retrieve_stream_begin(
                params,
                state.family_id.as_deref(),
            )
            .await
        }
        "storage.retrieve_stream_chunk" => {
            let params = request.params.clone().unwrap_or_else(|| json!({}));
            crate::rpc::storage_stream::storage_retrieve_stream_chunk(params).await
        }
        "storage.object.size" => {
            external_handlers::storage_object_size(request.params.as_ref(), state).await
        }
        "storage.namespaces.list" => {
            storage_handlers::storage_namespaces_list(request.params.as_ref(), state).await
        }
        "storage.fetch_external" => {
            external_handlers::storage_fetch_external(request.params.as_ref(), state).await
        }
        // Ionic bond ledger persistence (on behalf of security capability provider)
        "bonding.ledger.store" => {
            bonding_handlers::bonding_ledger_store(request.params.as_ref(), state).await
        }
        "bonding.ledger.retrieve" => {
            bonding_handlers::bonding_ledger_retrieve(request.params.as_ref(), state).await
        }
        "bonding.ledger.list" => {
            bonding_handlers::bonding_ledger_list(request.params.as_ref(), state).await
        }
        // Game session persistence (convenience over storage.*)
        "session.save" => session_handlers::session_save(request.params.as_ref(), state).await,
        "session.load" => session_handlers::session_load(request.params.as_ref(), state).await,
        // Model cache operations (filesystem-backed via model_cache_handlers.rs)
        "model.register" => model_cache_handlers::model_register(request.params.as_ref()).await,
        "model.exists" => model_cache_handlers::model_exists(request.params.as_ref()),
        "model.locate" => model_cache_handlers::model_locate(request.params.as_ref()),
        "model.metadata" => model_cache_handlers::model_metadata(request.params.as_ref()).await,
        // Template operations
        "templates.store" => {
            template_handlers::templates_store(request.params.as_ref(), state).await
        }
        "templates.retrieve" => {
            template_handlers::templates_retrieve(request.params.as_ref(), state).await
        }
        "templates.list" => template_handlers::templates_list(request.params.as_ref(), state).await,
        "templates.community_top" => {
            template_handlers::templates_community_top(request.params.as_ref(), state).await
        }
        // Audit operations
        "audit.store_execution" => {
            audit_handlers::audit_store_execution(request.params.as_ref(), state).await
        }
        // NAT traversal persistence (filesystem-backed)
        "nat.store_traversal_info" => {
            nat_handlers::nat_store_traversal_info(request.params.as_ref(), state).await
        }
        "nat.retrieve_traversal_info" => {
            nat_handlers::nat_retrieve_traversal_info(request.params.as_ref(), state).await
        }
        // Known beacon persistence (filesystem-backed)
        "beacon.store" => nat_handlers::beacon_store(request.params.as_ref(), state).await,
        "beacon.retrieve" => nat_handlers::beacon_retrieve(request.params.as_ref(), state).await,
        "beacon.list" | "nat.beacon" => {
            nat_handlers::beacon_list(request.params.as_ref(), state).await
        }
        "beacon.delete" => nat_handlers::beacon_delete(request.params.as_ref(), state).await,
        // ZFS operations (subprocess-backed, resolves GAP-MATRIX-04)
        "zfs.pool.list" => zfs_handlers::zfs_pool_list(request.params.as_ref()).await,
        "zfs.pool.get" => zfs_handlers::zfs_pool_get(request.params.as_ref()).await,
        "zfs.pool.health" => zfs_handlers::zfs_pool_health(request.params.as_ref()).await,
        "zfs.dataset.list" => zfs_handlers::zfs_dataset_list(request.params.as_ref()).await,
        "zfs.dataset.get" => zfs_handlers::zfs_dataset_get(request.params.as_ref()).await,
        "zfs.snapshot.list" => zfs_handlers::zfs_snapshot_list(request.params.as_ref()).await,
        "zfs.health" => zfs_handlers::zfs_health(request.params.as_ref()).await,
        _ => {
            return JsonRpcResponse {
                jsonrpc: Arc::from("2.0"),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: Cow::Borrowed("Method not found"),
                    data: Some(json!({"method": request.method})),
                }),
                id: request.id,
            };
        }
    };

    match result {
        Ok(value) => JsonRpcResponse {
            jsonrpc: Arc::from("2.0"),
            result: Some(value),
            error: None,
            id: request.id,
        },
        Err(e) => JsonRpcResponse {
            jsonrpc: Arc::from("2.0"),
            result: None,
            error: Some(JsonRpcError {
                code: -32603,
                message: Cow::Borrowed("Internal error"),
                data: Some(json!({"error": e.to_string()})),
            }),
            id: request.id,
        },
    }
}

/// Handle `discovery.capability.register` on the legacy Unix/TCP dispatch path.
pub(super) fn discovery_capability_register(
    params: Option<&Value>,
) -> std::result::Result<Value, NestGateError> {
    let params = params.ok_or_else(|| {
        NestGateError::configuration_error(
            "params",
            "discovery.capability.register requires params",
        )
    })?;
    let capability = params
        .get("capability")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            NestGateError::configuration_error("capability", "missing 'capability' param")
        })?;
    let endpoint = params
        .get("endpoint")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            NestGateError::configuration_error("endpoint", "missing 'endpoint' param")
        })?;
    nestgate_config::config::capability_discovery::announce_capability(
        capability,
        endpoint,
        std::time::Duration::from_secs(60),
    )?;
    Ok(json!({
        "success": true,
        "message": format!("Capability {capability} registered and announced"),
    }))
}
