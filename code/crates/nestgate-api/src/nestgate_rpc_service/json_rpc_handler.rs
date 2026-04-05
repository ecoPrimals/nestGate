// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JSON-RPC 2.0 handler that wraps the tarpc service for HTTP access.
//!
//! Orchestration and other capability peers can use this for initial discovery before escalating to tarpc.

use tarpc::context::Context;

use super::NestGateRpc;
use super::tarpc_server::NestGateRpcServer;

/// JSON-RPC handler that delegates to the tarpc server implementation
pub struct NestGateJsonRpcHandler {
    server: NestGateRpcServer,
}

impl NestGateJsonRpcHandler {
    /// Create new JSON-RPC handler
    #[must_use]
    pub fn new() -> Self {
        Self {
            server: NestGateRpcServer::default(),
        }
    }

    /// Route a JSON-RPC method call to the appropriate tarpc handler.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the method is unknown or serialisation fails.
    pub async fn handle(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        match method {
            "list_pools" => {
                let ctx = Context::current();
                let result = self.server.clone().list_pools(ctx).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize list_pools result: {e}"))
            }
            "list_datasets" => {
                let pool: String = serde_json::from_value(params).map_err(|e| e.to_string())?;
                let ctx = Context::current();
                let result = self.server.clone().list_datasets(ctx, pool).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize list_datasets result: {e}"))
            }
            "get_metrics" => {
                let ctx = Context::current();
                let result = self.server.clone().get_metrics(ctx).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize get_metrics result: {e}"))
            }
            "health" | "health.liveness" | "health.check" => {
                let ctx = Context::current();
                let result = self.server.clone().health(ctx).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize health result: {e}"))
            }
            "health.readiness" => {
                let ctx = Context::current();
                let h = self.server.clone().health(ctx).await;
                let ready = h.status != "unhealthy";
                serde_json::to_value(serde_json::json!({
                    "ready": ready,
                    "status": h.status,
                    "version": h.version,
                    "uptime_seconds": h.uptime_seconds,
                    "pools_healthy": h.pools_healthy,
                    "pools_total": h.pools_total,
                }))
                .map_err(|e| format!("Failed to serialize health.readiness result: {e}"))
            }
            "version" => {
                let ctx = Context::current();
                let result = self.server.clone().version(ctx).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize version result: {e}"))
            }
            "capabilities" | "capabilities.list" => {
                let ctx = Context::current();
                let result = self.server.clone().capabilities(ctx).await;
                serde_json::to_value(result)
                    .map_err(|e| format!("Failed to serialize capabilities result: {e}"))
            }
            _ => Err(format!("Unknown method: {method}")),
        }
    }
}

impl Default for NestGateJsonRpcHandler {
    fn default() -> Self {
        Self::new()
    }
}
