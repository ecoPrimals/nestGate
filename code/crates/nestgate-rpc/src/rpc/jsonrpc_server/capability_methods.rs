// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC registration for capability discovery and identity endpoints.

use std::borrow::Cow;
use std::collections::HashMap;

use jsonrpsee::{RpcModule, types::ErrorObjectOwned};
use tracing::{debug, info, warn};

use nestgate_config::config::capability_discovery::DiscoverySource;
use nestgate_config::constants::ports::default_tarpc_client_endpoint;
use nestgate_types::NestGateError;

use crate::rpc::storage_backend::StorageBackend;

use super::JsonRpcState;
use super::map_jsonrpc_registration;

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
    "storage.store_stream",
    "storage.store_stream_chunk",
    "storage.retrieve_stream",
    "storage.retrieve_stream_chunk",
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

/// Register capability-related JSON-RPC methods
pub(super) fn register_capability_methods<S: StorageBackend + 'static>(
    module: &mut RpcModule<JsonRpcState<S>>,
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
                    info!("Capability '{}' registered successfully", p.capability);
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
    map_jsonrpc_registration(module.register_async_method(
        "discovery.capability.query",
        |params, _ctx, _ext| async move {
            let capability: String = params.one()?;
            debug!("JSON-RPC: discoverCapability({})", capability);

            let env_var = format!(
                "NESTGATE_{}_ENDPOINT",
                capability.to_uppercase().replace('-', "_")
            );
            let discovery_default = default_tarpc_client_endpoint();
            let se = match nestgate_config::config::capability_discovery::discover_with_fallback(
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
    ))?;

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

#[cfg(test)]
mod tests {
    use jsonrpsee::core::params::{ArrayParams, ObjectParams};

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

    #[test]
    fn capabilities_method_table_is_non_empty() {
        assert!(!super::JSON_RPC_CAPABILITIES_METHODS.is_empty());
        assert!(super::JSON_RPC_CAPABILITIES_METHODS.contains(&"capabilities.list"));
    }

    #[tokio::test]
    async fn capabilities_list_returns_primal_version_and_method_catalog() {
        let module = build_module_for_tests();
        let params = ArrayParams::new();
        let v: serde_json::Value = match module.call("capabilities.list", params).await {
            Ok(x) => x,
            Err(e) => panic!("capabilities.list: {e}"),
        };
        assert!(v.get("primal").is_some());
        assert!(v.get("version").is_some());
        let methods = match v["methods"].as_array() {
            Some(a) => a,
            None => panic!("expected methods array"),
        };
        assert!(methods.iter().any(|m| m == "health.metrics"));
        assert!(methods.iter().any(|m| m == "discovery.capability.query"));
    }

    #[tokio::test]
    async fn identity_get_includes_license_and_domain() {
        temp_env::async_with_vars([("NESTGATE_FAMILY_ID", None::<&str>)], async {
            let module = build_module_for_tests();
            let params = ArrayParams::new();
            let v: serde_json::Value = match module.call("identity.get", params).await {
                Ok(x) => x,
                Err(e) => panic!("identity.get: {e}"),
            };
            assert_eq!(v["domain"], "storage");
            assert_eq!(v["license"], "AGPL-3.0-or-later");
            assert_eq!(v["family_id"], "default");
        })
        .await;
    }

    #[tokio::test]
    async fn identity_get_reflects_nestgate_family_id() {
        temp_env::async_with_vars([("NESTGATE_FAMILY_ID", Some("fam-test-42"))], async {
            let module = build_module_for_tests();
            let params = ArrayParams::new();
            let v: serde_json::Value = match module.call("identity.get", params).await {
                Ok(x) => x,
                Err(e) => panic!("identity.get: {e}"),
            };
            assert_eq!(v["family_id"], "fam-test-42");
        })
        .await;
    }

    #[tokio::test]
    async fn discovery_capability_register_requires_capability_and_endpoint() {
        let module = build_module_for_tests();
        let params = ObjectParams::new();
        let err = module
            .call::<_, serde_json::Value>("discovery.capability.register", params)
            .await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn discovery_capability_query_accepts_capability_name() {
        let module = build_module_for_tests();
        let mut params = ArrayParams::new();
        match params.insert("storage") {
            Ok(()) => {}
            Err(e) => panic!("insert param: {e}"),
        }
        let result: Vec<serde_json::Value> =
            match module.call("discovery.capability.query", params).await {
                Ok(x) => x,
                Err(e) => panic!("discovery.capability.query: {e}"),
            };
        assert!(
            result.is_empty() || result.iter().all(|v| v.is_object()),
            "expected empty or object entries"
        );
    }
}
