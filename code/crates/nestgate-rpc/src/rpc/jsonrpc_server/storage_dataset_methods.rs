// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC registration for storage dataset methods.

use std::collections::HashMap;
use std::sync::Arc;

use jsonrpsee::{RpcModule, types::ErrorObjectOwned};
use tracing::debug;

use nestgate_types::NestGateError;

use crate::rpc::storage_backend::StorageBackend;
use crate::rpc::tarpc_types::{DatasetParams, NestGateRpc};

use super::JsonRpcState;
use super::map_jsonrpc_registration;

/// Register `storage.dataset.*` JSON-RPC methods
pub(super) fn register_dataset_methods<S: StorageBackend + 'static>(
    module: &mut RpcModule<JsonRpcState<S>>,
) -> Result<(), NestGateError> {
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

    Ok(())
}
