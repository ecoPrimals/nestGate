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
