// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Request dispatch — routes JSON-RPC method names to handler functions.

use std::borrow::Cow;
use std::sync::Arc;

use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use nestgate_types::error::NestGateError;
use serde_json::{Value, json};

use crate::rpc::method_gate;
use crate::rpc::model_cache_handlers;
use crate::rpc::protocol::normalize_method;

use super::{
    JsonRpcError, JsonRpcRequest, JsonRpcResponse, StorageState, audit_handlers, blob_handlers,
    bonding_handlers, content_federation_handlers, content_handlers, external_handlers,
    nat_handlers, session_handlers, storage_handlers, template_handlers, zfs_handlers,
};

/// Extract owned params from a request, defaulting to `{}`.
fn take_params(request: &JsonRpcRequest) -> Value {
    request.params.clone().unwrap_or_else(|| json!({}))
}

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

    // JH-0: auth.* introspection methods (handled before dispatch table
    // because they need gate + caller context).
    if let Some(result) =
        method_gate::auth_introspection(&method, &state.method_gate, &state.caller_context)
    {
        return JsonRpcResponse {
            jsonrpc: Arc::from("2.0"),
            result: Some(result),
            error: None,
            id: request.id,
        };
    }

    // JH-0: pre-dispatch authorization gate.
    if let Err(rejection) = state.method_gate.check(&method, &state.caller_context) {
        return JsonRpcResponse {
            jsonrpc: Arc::from("2.0"),
            result: None,
            error: Some(JsonRpcError {
                code: rejection.code,
                message: Cow::Owned(format!(
                    "permission denied: method '{}' requires a capability token",
                    rejection.method,
                )),
                data: Some(json!({"method": rejection.method})),
            }),
            id: request.id,
        };
    }

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
        // Mesh routing — cross-gate capability registration (Wave 73)
        "route.register" => route_register(request.params.as_ref(), state),
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
        "storage.list_blobs" => {
            blob_handlers::storage_list_blobs(request.params.as_ref(), state).await
        }
        "storage.blob_exists" => {
            blob_handlers::storage_blob_exists(request.params.as_ref(), state).await
        }
        "storage.store_stream" => {
            let params = take_params(&request);
            crate::rpc::storage_stream::storage_store_stream_begin(
                params,
                state.family_id.as_deref(),
            )
            .await
        }
        "storage.store_stream_chunk" => {
            let params = take_params(&request);
            crate::rpc::storage_stream::storage_store_stream_chunk(params).await
        }
        "storage.retrieve_stream" => {
            let params = take_params(&request);
            crate::rpc::storage_stream::storage_retrieve_stream_begin(
                params,
                state.family_id.as_deref(),
            )
            .await
        }
        "storage.retrieve_stream_chunk" | "content.retrieve_stream_chunk" => {
            let params = take_params(&request);
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
        // Content-addressed storage (BLAKE3 hash-as-key, NG-1)
        "content.put" => content_handlers::content_put(request.params.as_ref(), state).await,
        "content.get" => content_handlers::content_get(request.params.as_ref(), state).await,
        "content.exists" => content_handlers::content_exists(request.params.as_ref(), state).await,
        "content.list" => content_handlers::content_list(request.params.as_ref(), state).await,
        // Content manifests — versioned collections (NG-2)
        "content.publish" => {
            content_handlers::content_publish(request.params.as_ref(), state).await
        }
        "content.resolve" => {
            content_handlers::content_resolve(request.params.as_ref(), state).await
        }
        "content.promote" => {
            content_handlers::content_promote(request.params.as_ref(), state).await
        }
        "content.collections" => {
            content_handlers::content_collections(request.params.as_ref(), state).await
        }
        // Content streaming (Wave 74 — chunked CAS for large blobs)
        "content.store_stream" => {
            let params = take_params(&request);
            crate::rpc::content_stream::content_store_stream_begin(
                params,
                state.family_id.as_deref(),
            )
            .await
        }
        "content.store_stream_chunk" => {
            let params = take_params(&request);
            crate::rpc::content_stream::content_store_stream_chunk(params).await
        }
        "content.retrieve_stream" => {
            let params = take_params(&request);
            crate::rpc::content_stream::content_retrieve_stream_begin(
                params,
                state.family_id.as_deref(),
            )
            .await
        }
        // Content federation (Wave 60 — waterFall / rootPulse signal graphs)
        "content.fetch_heads" => {
            content_federation_handlers::content_fetch_heads(request.params.as_ref(), state).await
        }
        "content.push" => {
            content_federation_handlers::content_push(request.params.as_ref(), state).await
        }
        "content.replicate" => {
            content_federation_handlers::content_replicate(request.params.as_ref(), state).await
        }
        "content.replicate.pull" => {
            content_federation_handlers::content_replicate_pull(request.params.as_ref(), state)
                .await
        }
        "content.sync" => {
            content_federation_handlers::content_sync(request.params.as_ref(), state).await
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
        "lifecycle.status" => Ok(json!({
            "status": "running",
            "primal": nestgate_config::constants::system::DEFAULT_SERVICE_NAME,
            "version": env!("CARGO_PKG_VERSION"),
            "storage_initialized": state.storage_initialized,
        })),
        "zfs.pool.list" => zfs_handlers::zfs_pool_list(request.params.as_ref()).await,
        "zfs.pool.get" => zfs_handlers::zfs_pool_get(request.params.as_ref()).await,
        "zfs.pool.health" => zfs_handlers::zfs_pool_health(request.params.as_ref()).await,
        "zfs.dataset.list" => zfs_handlers::zfs_dataset_list(request.params.as_ref()).await,
        "zfs.dataset.get" => zfs_handlers::zfs_dataset_get(request.params.as_ref()).await,
        "zfs.snapshot.list" => zfs_handlers::zfs_snapshot_list(request.params.as_ref()).await,
        "zfs.snapshot.create" => zfs_handlers::zfs_snapshot_create(request.params.as_ref()).await,
        "zfs.snapshot.destroy" => {
            zfs_handlers::zfs_snapshot_destroy(request.params.as_ref()).await
        }
        "zfs.health" => zfs_handlers::zfs_health(request.params.as_ref()).await,
        "btsp.capabilities" => Ok(json!({
            "protocol": "btsp-v1",
            "cipher": "chacha20-poly1305",
            "kdf": "hkdf-sha256",
            "handshake": "x25519-ephemeral",
            "required": std::env::var("NESTGATE_FAMILY_ID")
                .ok()
                .is_some_and(|fid| !matches!(fid.as_str(), "" | "default" | "standalone")),
        })),
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

/// Handle `route.register` — register `NestGate`'s storage capabilities with
/// the ecosystem mesh for cross-gate routing.
///
/// This builds the full announce payload (including gate identity, federation
/// endpoints, and storage backend info) and writes it to the local route
/// manifest. An external mesh coordinator or biomeOS can consume this manifest
/// to route cross-gate `content.*` and `storage.*` requests.
///
/// Optional params:
/// - `gate_id`: Override gate identity (default: `NESTGATE_GATE_ID` env)
/// - `ttl_seconds`: Manifest TTL (default: 300)
pub(super) fn route_register(
    params: Option<&Value>,
    state: &StorageState,
) -> std::result::Result<Value, NestGateError> {
    let socket_path = state
        .socket_path
        .as_deref()
        .map_or_else(|| std::path::Path::new("/dev/null"), std::path::Path::new);

    let payload = super::super::primal_announce::build_announce_payload(socket_path);

    let gate_id = params
        .and_then(|p| p.get("gate_id"))
        .and_then(Value::as_str)
        .map_or_else(
            || payload["gate_id"].as_str().unwrap_or("standalone").to_owned(),
            str::to_owned,
        );

    let ttl = params
        .and_then(|p| p.get("ttl_seconds"))
        .and_then(Value::as_u64)
        .unwrap_or(300);

    let endpoint = payload["socket"].as_str().unwrap_or_default();
    nestgate_config::config::capability_discovery::announce_capability(
        "storage",
        endpoint,
        std::time::Duration::from_secs(ttl),
    )?;
    nestgate_config::config::capability_discovery::announce_capability(
        "content",
        endpoint,
        std::time::Duration::from_secs(ttl),
    )?;

    Ok(json!({
        "registered": true,
        "gate_id": gate_id,
        "capabilities": ["storage", "content"],
        "federation_methods": payload["federation_methods"],
        "endpoints": payload["endpoints"],
        "storage_backend": payload["storage_backend"],
        "ttl_seconds": ttl,
    }))
}
