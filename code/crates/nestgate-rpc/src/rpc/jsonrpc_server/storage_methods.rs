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

#[cfg(test)]
mod tests {
    use jsonrpsee::core::params::ArrayParams;

    use crate::rpc::jsonrpc_server::{JsonRpcServer, JsonRpcState};
    use crate::rpc::tarpc_server::NestGateRpcService;

    fn build_module_via_storage_stack()
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
    async fn storage_registration_exposes_dataset_routes() {
        let module = build_module_via_storage_stack();
        let names: Vec<_> = module.method_names().collect();
        for expected in [
            "storage.dataset.create",
            "storage.dataset.list",
            "storage.dataset.get",
            "storage.dataset.delete",
        ] {
            assert!(
                names.iter().any(|n| *n == expected),
                "missing dataset route {expected}"
            );
        }
    }

    #[tokio::test]
    async fn storage_registration_exposes_object_crud_and_list() {
        let module = build_module_via_storage_stack();
        let names: Vec<_> = module.method_names().collect();
        for expected in [
            "storage.object.store",
            "storage.object.retrieve",
            "storage.object.metadata",
            "storage.object.list",
            "storage.object.delete",
        ] {
            assert!(
                names.iter().any(|n| *n == expected),
                "missing object route {expected}"
            );
        }
    }

    #[tokio::test]
    async fn storage_dataset_list_dispatch_returns_array() {
        let module = build_module_via_storage_stack();
        let params = ArrayParams::new();
        let list: Vec<serde_json::Value> = match module.call("storage.dataset.list", params).await {
            Ok(x) => x,
            Err(e) => panic!("storage.dataset.list: {e}"),
        };
        assert!(list.iter().all(|v| v.is_object()));
    }
}
