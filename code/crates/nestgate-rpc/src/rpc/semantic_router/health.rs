// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Health domain semantic methods
//!
//! Handles health.* semantic method routing for health checks and metrics.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};

/// Route health.check → `health_check`
pub(super) async fn health_check(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let health = router.client.health().await?;

    Ok(json!({
        "status": health.status,
        "uptime_seconds": health.uptime_seconds,
        "version": health.version
    }))
}

/// Route health.liveness → minimal alive signal (orchestrator / kube probes)
pub(super) async fn health_liveness(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let health = router.client.health().await?;

    Ok(json!({
        "alive": true,
        "status": health.status
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

/// Route health.readiness → readiness check
pub(super) async fn health_ready(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let health = router.client.health().await?;

    Ok(json!({
        "ready": health.status == "healthy",
        "status": health.status
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
        assert!(health_ready(&r, json!({})).await.is_err());
    }
}
