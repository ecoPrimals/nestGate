// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Orchestrates registration of all `storage.*` JSON-RPC methods.

use jsonrpsee::RpcModule;

use nestgate_types::NestGateError;

use crate::rpc::storage_backend::StorageBackend;

use super::JsonRpcState;
use super::storage_dataset_methods;
use super::storage_object_methods;

/// Register storage-related JSON-RPC methods (dataset + object)
pub(super) fn register_storage_methods<S: StorageBackend + 'static>(
    module: &mut RpcModule<JsonRpcState<S>>,
) -> Result<(), NestGateError> {
    storage_dataset_methods::register_dataset_methods(module)?;
    storage_object_methods::register_object_methods(module)?;
    Ok(())
}
