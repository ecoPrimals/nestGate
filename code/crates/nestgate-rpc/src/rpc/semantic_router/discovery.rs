// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Discovery domain semantic methods
//!
//! **Integration:** Full discovery responses come from `nestgate-core` `service_metadata` and
//! `nestgate-discovery` when those are callable from this router.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};

pub(super) fn discovery_announce(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: discovery.announce (nestgate-discovery / service_metadata)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-discovery / service_metadata",
    ))
}

pub(super) fn discovery_query(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: discovery.query (nestgate-discovery / service_metadata)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-discovery / service_metadata",
    ))
}

pub(super) fn discovery_list(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: discovery.list (nestgate-discovery / service_metadata)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-discovery / service_metadata",
    ))
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "JSON-RPC semantic handlers use Result for uniform router dispatch"
)]
pub(super) fn discovery_capabilities(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Ok(json!({
        "capabilities": ["storage", "discovery", "metadata", "health"]
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
        SemanticRouter::new(Arc::new(client))
    }

    #[test]
    fn discovery_announce_query_list_not_implemented() {
        let r = router();
        for (label, e) in [
            (
                "announce",
                discovery_announce(&r, json!({})).expect_err("ni"),
            ),
            ("query", discovery_query(&r, json!({})).expect_err("ni")),
            ("list", discovery_list(&r, json!({})).expect_err("ni")),
        ] {
            assert!(
                e.to_string().contains("nestgate-discovery") || e.to_string().contains("wire"),
                "{label}: {e}"
            );
        }
    }

    #[test]
    fn discovery_capabilities_returns_json() {
        let r = router();
        let v = discovery_capabilities(&r, json!({})).expect("ok");
        assert!(v["capabilities"].as_array().unwrap().len() >= 4);
    }
}
