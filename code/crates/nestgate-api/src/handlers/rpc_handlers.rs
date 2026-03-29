// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! RPC Handlers for JSON-RPC and Protocol Discovery
//!
//! This module provides HTTP handlers for:
//! - JSON-RPC endpoint for inter-primal communication
//! - Protocol capabilities discovery
//! - Protocol negotiation

use axum::{Json, http::StatusCode};
use nestgate_core::constants::capability_port_discovery::{
    discover_api_port_sync, discover_tarpc_port_sync,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};

use crate::nestgate_rpc_service::NestGateJsonRpcHandler;

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (must be "2.0")
    pub jsonrpc: Arc<str>,
    /// Request identifier
    pub id: String,
    /// Method name to invoke
    pub method: Arc<str>,
    /// Method parameters
    #[serde(default)]
    pub params: serde_json::Value,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: Arc<str>,
    /// Request identifier (matches request)
    pub id: String,
    /// Result value (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error object (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: Cow<'static, str>,
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Protocol information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInfo {
    /// Protocol name
    pub name: String,
    /// Protocol version
    pub version: String,
    /// Connection endpoint
    pub endpoint: String,
    /// Port number (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// Protocol features
    pub features: Vec<String>,
    /// Typical latency in microseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_us: Option<u64>,
    /// Availability status - "available", "planned", etc. Omitted if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Protocol capabilities response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolCapabilities {
    /// Service name
    pub service: String,
    /// Service version
    pub version: String,
    /// Available protocols
    pub protocols: HashMap<String, ProtocolInfo>,
    /// Service capabilities
    pub capabilities: Vec<String>,
}

/// Handle JSON-RPC requests
///
/// This endpoint provides HTTP-based RPC access for inter-primal communication.
/// Songbird and other primals can use this for initial discovery before
/// escalating to tarpc for performance.
pub async fn handle_jsonrpc(
    Json(request): Json<JsonRpcRequest>,
) -> Result<Json<JsonRpcResponse>, (StatusCode, Json<JsonRpcResponse>)> {
    info!("📞 JSON-RPC request: method={}", request.method);
    debug!("Request details: {:?}", request);

    // Validate JSON-RPC version
    if request.jsonrpc.as_ref() != "2.0" {
        let error_response = JsonRpcResponse {
            jsonrpc: Arc::from("2.0"),
            id: request.id,
            result: None,
            error: Some(JsonRpcError {
                code: -32600,
                message: Cow::Borrowed("Invalid Request - jsonrpc must be '2.0'"),
                data: None,
            }),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    // Create handler
    let handler = NestGateJsonRpcHandler::new();

    // Execute method
    match handler.handle(&*request.method, request.params).await {
        Ok(result) => {
            let response = JsonRpcResponse {
                jsonrpc: Arc::from("2.0"),
                id: request.id,
                result: Some(result),
                error: None,
            };
            Ok(Json(response))
        }
        Err(err) => {
            let error_response = JsonRpcResponse {
                jsonrpc: Arc::from("2.0"),
                id: request.id,
                result: None,
                error: Some(JsonRpcError {
                    code: -32603,
                    message: Cow::Owned(format!("Internal error: {err}")),
                    data: None,
                }),
            };
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

/// Get protocol capabilities
///
/// This endpoint advertises available protocols for protocol escalation.
/// Songbird uses this to discover tarpc endpoint for high-performance operations.
pub async fn get_protocol_capabilities() -> Json<ProtocolCapabilities> {
    info!("🔍 Protocol capabilities requested");

    // Discover ports at runtime (no hardcoding!)
    let api_port = discover_api_port_sync();
    let tarpc_port = discover_tarpc_port_sync();

    let mut protocols = HashMap::new();

    // HTTP/REST protocol
    protocols.insert(
        "http".to_string(),
        ProtocolInfo {
            name: "HTTP/REST".to_string(),
            version: "1.1".to_string(),
            endpoint: format!("http://0.0.0.0:{api_port}"),
            port: Some(api_port),
            features: vec![
                "rest".to_string(),
                "json".to_string(),
                "streaming".to_string(),
            ],
            latency_us: Some(5000), // ~5ms
            status: None,
        },
    );

    // JSON-RPC protocol
    protocols.insert(
        "jsonrpc".to_string(),
        ProtocolInfo {
            name: "JSON-RPC".to_string(),
            version: "2.0".to_string(),
            endpoint: format!("http://0.0.0.0:{api_port}/jsonrpc"),
            port: Some(api_port),
            features: vec![
                "rpc".to_string(),
                "universal".to_string(),
                "language-agnostic".to_string(),
            ],
            latency_us: Some(2000), // ~2ms
            status: None,
        },
    );

    // tarpc protocol - planned for v0.2.0, not yet running (serve_tarpc not wired)
    protocols.insert(
        "tarpc".to_string(),
        ProtocolInfo {
            name: "tarpc".to_string(),
            version: "0.34".to_string(),
            endpoint: format!("tarpc://0.0.0.0:{tarpc_port}"),
            port: Some(tarpc_port),
            features: vec![
                "binary".to_string(),
                "high-performance".to_string(),
                "rust-native".to_string(),
                "type-safe".to_string(),
                "zero-copy".to_string(),
            ],
            latency_us: Some(50), // ~50μs (40x faster than JSON-RPC!)
            status: Some("planned_v0.2.0".to_string()),
        },
    );

    Json(ProtocolCapabilities {
        service: "nestgate".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        protocols,
        capabilities: vec![
            "storage".to_string(),
            "zfs".to_string(),
            "snapshots".to_string(),
            "replication".to_string(),
            "compression".to_string(),
            "deduplication".to_string(),
        ],
    })
}

/// Health check for RPC services
pub async fn rpc_health() -> Json<serde_json::Value> {
    debug!("RPC health check");

    Json(serde_json::json!({
        "status": "healthy",
        "rpc": {
            "jsonrpc": "available",
            "tarpc": "planned_v0.2.0"
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_jsonrpc_handler() {
        let request = JsonRpcRequest {
            jsonrpc: Arc::from("2.0"),
            id: "test-1".to_string(),
            method: Arc::from("health"),
            params: serde_json::Value::Null,
        };

        let result = handle_jsonrpc(Json(request)).await;
        assert!(result.is_ok());

        let response = result.unwrap().0;
        assert_eq!(response.jsonrpc.as_ref(), "2.0");
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_invalid_jsonrpc_version() {
        let request = JsonRpcRequest {
            jsonrpc: Arc::from("1.0"),
            id: "test-2".to_string(),
            method: Arc::from("health"),
            params: serde_json::Value::Null,
        };

        let result = handle_jsonrpc(Json(request)).await;
        assert!(result.is_err());

        let (status, response) = result.unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert!(response.0.error.is_some());
    }

    #[tokio::test]
    async fn test_protocol_capabilities() {
        let capabilities = get_protocol_capabilities().await.0;

        assert_eq!(capabilities.service, "nestgate");
        assert!(capabilities.protocols.contains_key("http"));
        assert!(capabilities.protocols.contains_key("jsonrpc"));
        assert!(capabilities.protocols.contains_key("tarpc"));

        // Verify tarpc has best performance and is marked as planned (not yet running)
        let tarpc = capabilities.protocols.get("tarpc").unwrap();
        assert_eq!(tarpc.latency_us, Some(50));
        assert_eq!(tarpc.status, Some("planned_v0.2.0".to_string()));

        let jsonrpc = capabilities.protocols.get("jsonrpc").unwrap();
        assert_eq!(jsonrpc.latency_us, Some(2000));

        // tarpc should be 40x faster
        assert!(jsonrpc.latency_us.unwrap() / tarpc.latency_us.unwrap() == 40);
    }

    #[tokio::test]
    async fn test_rpc_health() {
        let health = rpc_health().await.0;
        assert_eq!(health["status"], "healthy");
        assert_eq!(health["rpc"]["jsonrpc"], "available");
        assert_eq!(health["rpc"]["tarpc"], "planned_v0.2.0");
    }
}
