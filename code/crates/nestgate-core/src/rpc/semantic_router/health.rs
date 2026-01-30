//! Health domain semantic methods
//!
//! Handles health.* semantic method routing for health checks and metrics.

use super::SemanticRouter;
use crate::error::{NestGateError, Result};
use serde_json::{json, Value};

/// Route health.check → health_check
pub(super) async fn health_check(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let health = router.client.health().await?;

    Ok(json!({
        "status": health.status,
        "uptime_seconds": health.uptime_seconds,
        "version": health.version
    }))
}

/// Route health.metrics → get_metrics
pub(super) async fn health_metrics(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let metrics = router.client.get_storage_metrics().await?;

    Ok(serde_json::to_value(metrics)
        .map_err(|e| NestGateError::serialization(&format!("Failed to serialize metrics: {}", e)))?)
}

/// Route health.info → get_info
pub(super) async fn health_info(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let info = router.client.get_info().await?;

    Ok(serde_json::to_value(info)
        .map_err(|e| NestGateError::serialization(&format!("Failed to serialize info: {}", e)))?)
}

/// Route health.ready → readiness check
pub(super) async fn health_ready(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let health = router.client.health().await?;

    Ok(json!({
        "ready": health.status == "healthy",
        "status": health.status
    }))
}
