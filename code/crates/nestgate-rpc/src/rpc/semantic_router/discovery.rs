// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Discovery domain semantic methods
//!
//! Each primal starts with self-knowledge only and discovers peers at runtime.
//! `NestGate`'s discovery surface provides:
//!
//! - `discovery.capabilities` — `NestGate`'s own capabilities (always available)
//! - `discovery.announce` — register service metadata (requires discovery backend)
//! - `discovery.query` — find services by capability (requires discovery backend)
//! - `discovery.list` — list known services (requires discovery backend)
//!
//! Production deployments delegate full peer discovery to the orchestration
//! capability provider (discovered at runtime, not by name).

use super::{MetadataBackend, SemanticRouter};
use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};

/// `NestGate`'s self-knowledge: capabilities this primal provides.
const SELF_CAPABILITIES: &[&str] = &[
    "storage",
    "session",
    "discovery",
    "metadata",
    "health",
    "zfs",
    "automation",
];

/// Route `discovery.announce` — register service metadata with discovery backend.
///
/// Requires a discovery backend (mDNS, Consul, Kubernetes, or orchestration
/// capability provider). Returns a structured response when no backend is configured.
#[expect(
    clippy::unnecessary_wraps,
    reason = "JSON-RPC semantic handlers use Result<Value> for uniform dispatch"
)]
pub(super) fn discovery_announce(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: &Value,
) -> Result<Value> {
    let name = params["name"].as_str().unwrap_or("unknown");
    let capabilities = params["capabilities"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(serde_json::Value::as_str)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    tracing::info!(
        service = name,
        capabilities = ?capabilities,
        "discovery.announce received (local registration only)"
    );

    Ok(json!({
        "status": "registered_locally",
        "service": name,
        "capabilities": capabilities,
        "note": "Full peer discovery requires an orchestration capability provider. \
                 Configure via NESTGATE_CAPABILITY_ORCHESTRATION."
    }))
}

/// Route `discovery.query` — find services by capability.
///
/// Returns self-knowledge matches when queried for capabilities this primal provides.
pub(super) fn discovery_query(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: &Value,
) -> Result<Value> {
    let capability = params["capability"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("capability", "string required"))?;

    let self_provides = SELF_CAPABILITIES.contains(&capability);

    Ok(json!({
        "capability": capability,
        "providers": if self_provides {
            json!([{
                "name": DEFAULT_SERVICE_NAME,
                "capabilities": SELF_CAPABILITIES,
                "source": "self_knowledge"
            }])
        } else {
            json!([])
        },
        "source": "local",
        "note": "Only self-knowledge available. Configure orchestration capability \
                 for full ecosystem discovery."
    }))
}

/// Route `discovery.list` — list all known services.
///
/// Returns self-knowledge only; full ecosystem listing requires an orchestration
/// capability provider.
#[expect(
    clippy::unnecessary_wraps,
    reason = "JSON-RPC semantic handlers use Result<Value> for uniform dispatch"
)]
pub(super) fn discovery_list(
    _router: &SemanticRouter<impl MetadataBackend>,
    _params: &Value,
) -> Result<Value> {
    Ok(json!({
        "services": [{
            "name": DEFAULT_SERVICE_NAME,
            "capabilities": SELF_CAPABILITIES,
            "source": "self_knowledge",
            "status": "active"
        }],
        "source": "local",
        "note": "Only self-knowledge available. Configure orchestration capability \
                 for full ecosystem discovery."
    }))
}

/// Route `discovery.capabilities` — return this primal's own capabilities.
#[expect(
    clippy::unnecessary_wraps,
    reason = "JSON-RPC semantic handlers use Result<Value> for uniform dispatch"
)]
pub(super) fn discovery_capabilities(
    _router: &SemanticRouter<impl MetadataBackend>,
    _params: &Value,
) -> Result<Value> {
    Ok(json!({
        "primal": DEFAULT_SERVICE_NAME,
        "capabilities": SELF_CAPABILITIES,
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::NestGateRpcClient;
    use crate::rpc::semantic_router::SemanticRouter;
    use serde_json::json;
    use std::sync::Arc;

    fn router() -> SemanticRouter {
        let client = NestGateRpcClient::new("tarpc://127.0.0.1:65534").expect("client");
        SemanticRouter::new(Arc::new(client)).expect("router")
    }

    #[test]
    fn discovery_capabilities_returns_self_knowledge() {
        let r = router();
        let v = discovery_capabilities(&r, &json!({})).expect("ok");
        let caps = v["capabilities"].as_array().expect("array");
        assert!(caps.len() >= 4);
        assert!(caps.iter().any(|c| c == "storage"));
        assert!(caps.iter().any(|c| c == "health"));
    }

    #[test]
    fn discovery_query_returns_self_for_known_capability() {
        let r = router();
        let v = discovery_query(&r, &json!({"capability": "storage"})).expect("ok");
        let providers = v["providers"].as_array().expect("providers");
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0]["name"], DEFAULT_SERVICE_NAME);
    }

    #[test]
    fn discovery_query_returns_empty_for_unknown_capability() {
        let r = router();
        let v = discovery_query(&r, &json!({"capability": "quantum_computing"})).expect("ok");
        let providers = v["providers"].as_array().expect("providers");
        assert!(providers.is_empty());
    }

    #[test]
    fn discovery_list_returns_self() {
        let r = router();
        let v = discovery_list(&r, &json!({})).expect("ok");
        let services = v["services"].as_array().expect("services");
        assert_eq!(services.len(), 1);
        assert_eq!(services[0]["name"], DEFAULT_SERVICE_NAME);
    }

    #[test]
    fn discovery_announce_registers_locally() {
        let r = router();
        let v = discovery_announce(
            &r,
            &json!({"name": "test-service", "capabilities": ["compute"]}),
        )
        .expect("ok");
        assert_eq!(v["status"], "registered_locally");
    }

    #[test]
    fn discovery_query_requires_capability_param() {
        let r = router();
        let e = discovery_query(&r, &json!({})).expect_err("missing capability");
        assert!(!e.to_string().is_empty());
    }
}
