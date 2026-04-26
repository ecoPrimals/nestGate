// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC registration for storage object methods.

use std::collections::HashMap;
use std::sync::Arc;

use base64::Engine;
use bytes::Bytes;
use jsonrpsee::{RpcModule, types::ErrorObjectOwned};
use tracing::debug;

use nestgate_types::NestGateError;

use crate::rpc::storage_backend::StorageBackend;
use crate::rpc::tarpc_types::NestGateRpc;

use super::JsonRpcState;
use super::map_jsonrpc_registration;

/// Register `storage.object.*` JSON-RPC methods
#[expect(
    clippy::too_many_lines,
    reason = "method table mirrors JSON-RPC surface; split would obscure routing"
)]
pub(super) fn register_object_methods<S: StorageBackend + 'static>(
    module: &mut RpcModule<JsonRpcState<S>>,
) -> Result<(), NestGateError> {
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

            let data = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &p.data)
                .map_err(|e| {
                    ErrorObjectOwned::owned(-32602, format!("Invalid base64 data: {e}"), None::<()>)
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

    map_jsonrpc_registration(module.register_async_method(
        "storage.object.retrieve",
        |params, ctx, _ext| async move {
            const MAX_INLINE: usize = 64 * 1024 * 1024;

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
                    Arc::from(p.dataset.clone()),
                    Arc::from(p.key.clone()),
                )
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))?;

            if data.len() > MAX_INLINE {
                return Err(ErrorObjectOwned::owned(
                    -32603,
                    format!(
                        "Object {}/{} is {} bytes — exceeds inline limit ({MAX_INLINE}). \
                         Use storage.retrieve_stream for large payloads.",
                        p.dataset,
                        p.key,
                        data.len()
                    ),
                    None::<()>,
                ));
            }

            let encoded = base64::engine::general_purpose::STANDARD.encode(data.as_ref());

            Ok::<_, ErrorObjectOwned>(serde_json::json!({
                "data": encoded,
                "size_bytes": data.len(),
            }))
        },
    ))?;

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
