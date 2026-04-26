// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC registration for `storage.*_stream*` chunked transfer methods.
//!
//! These expose the same chunked-upload / chunked-download protocol available
//! on UDS (`storage_stream.rs`) over the HTTP jsonrpsee transport.

use jsonrpsee::{RpcModule, types::ErrorObjectOwned};
use serde_json::Value;
use tracing::debug;

use nestgate_types::NestGateError;

use crate::rpc::storage_backend::StorageBackend;
use crate::rpc::storage_stream;

use super::JsonRpcState;
use super::map_jsonrpc_registration;

/// Register `storage.store_stream`, `storage.store_stream_chunk`,
/// `storage.retrieve_stream`, and `storage.retrieve_stream_chunk`.
pub(super) fn register_stream_methods<S: StorageBackend + 'static>(
    module: &mut RpcModule<JsonRpcState<S>>,
) -> Result<(), NestGateError> {
    map_jsonrpc_registration(module.register_async_method(
        "storage.store_stream",
        |params, _ctx, _ext| async move {
            let p: Value = params.parse()?;
            debug!("JSON-RPC: storage.store_stream");
            storage_stream::storage_store_stream_begin(p, None)
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))
        },
    ))?;

    map_jsonrpc_registration(module.register_async_method(
        "storage.store_stream_chunk",
        |params, _ctx, _ext| async move {
            let p: Value = params.parse()?;
            debug!("JSON-RPC: storage.store_stream_chunk");
            storage_stream::storage_store_stream_chunk(p)
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))
        },
    ))?;

    map_jsonrpc_registration(module.register_async_method(
        "storage.retrieve_stream",
        |params, _ctx, _ext| async move {
            let p: Value = params.parse()?;
            debug!("JSON-RPC: storage.retrieve_stream");
            storage_stream::storage_retrieve_stream_begin(p, None)
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))
        },
    ))?;

    map_jsonrpc_registration(module.register_async_method(
        "storage.retrieve_stream_chunk",
        |params, _ctx, _ext| async move {
            let p: Value = params.parse()?;
            debug!("JSON-RPC: storage.retrieve_stream_chunk");
            storage_stream::storage_retrieve_stream_chunk(p)
                .await
                .map_err(|e| ErrorObjectOwned::owned(-32603, e.to_string(), None::<()>))
        },
    ))?;

    Ok(())
}
