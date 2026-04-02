// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Health domain semantic methods
//!
//! Handles `health.*` semantic method routing per `wateringHole/PRIMAL_IPC_PROTOCOL` and
//! `SEMANTIC_METHOD_NAMING_STANDARD`: triad `health.check` / `health.liveness` /
//! `health.readiness`, plus optional `health.metrics` and `health.info`.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};

/// Route `health.check` → full `HealthStatus` JSON from tarpc `health()` (all fields).
pub(super) async fn health_check(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let health = router.client.health().await?;

    serde_json::to_value(&health).map_err(|e| {
        NestGateError::internal_error(
            format!("Failed to serialize health: {e}"),
            "semantic_router",
        )
    })
}

/// Route `health.liveness` → minimal alive probe (cheap RPC; does **not** call full `health()`).
pub(super) async fn health_liveness(router: &SemanticRouter, _params: Value) -> Result<Value> {
    // `version()` proves the process responds without `calculate_metrics()` (used by `health()`).
    router.client.version().await?;

    Ok(json!({
        "alive": true,
        "status": "ok"
    }))
}

/// Route health.metrics → `get_metrics`
pub(super) async fn health_metrics(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let metrics = router.client.metrics().await?;

    serde_json::to_value(metrics).map_err(|e| {
        NestGateError::internal_error(
            format!("Failed to serialize metrics: {e}"),
            "semantic_router",
        )
    })
}

/// Route health.info → `get_info`
pub(super) async fn health_info(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let info = router.client.version().await?;

    serde_json::to_value(info).map_err(|e| {
        NestGateError::internal_error(
            format!("Failed to serialize version info: {e}"),
            "semantic_router",
        )
    })
}

/// Route `health.readiness` → whether backends are reachable and ready (uses full `health()` so
/// storage/metrics path is exercised, not just process liveness).
pub(super) async fn health_readiness(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let health = router.client.health().await?;
    let ready = health.status == "healthy";

    Ok(json!({
        "ready": ready,
        "status": health.status,
        "backends": {
            "storage": if ready { "ready" } else { "not_ready" }
        }
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

    #[tokio::test]
    async fn health_domain_routes_error_when_tarpc_unreachable() {
        let r = router();
        assert!(health_check(&r, json!({})).await.is_err());
        assert!(health_liveness(&r, json!({})).await.is_err());
        assert!(health_metrics(&r, json!({})).await.is_err());
        assert!(health_info(&r, json!({})).await.is_err());
        assert!(health_readiness(&r, json!({})).await.is_err());
    }

    #[tokio::test]
    async fn health_handlers_succeed_with_live_tarpc_server() {
        use super::super::tests::spawn_local_tarpc_server;
        use crate::rpc::NestGateRpcClient;

        let (addr, server_handle) = spawn_local_tarpc_server().await;
        let client = Arc::new(NestGateRpcClient::new(&format!("tarpc://{addr}")).expect("client"));
        let r = SemanticRouter::new(client);

        let check = health_check(&r, json!({})).await.expect("health_check");
        assert_eq!(check["status"], "healthy");

        let live = health_liveness(&r, json!({})).await.expect("liveness");
        assert_eq!(live["alive"], true);

        let ready = health_readiness(&r, json!({})).await.expect("readiness");
        assert_eq!(ready["ready"], true);
        assert_eq!(ready["backends"]["storage"], "ready");

        let metrics = health_metrics(&r, json!({})).await.expect("metrics");
        assert!(metrics.is_object());

        let info = health_info(&r, json!({})).await.expect("info");
        assert!(info.get("version").is_some());

        server_handle.abort();
    }
}
