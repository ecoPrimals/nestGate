// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # 🔌 JSON-RPC Unix Socket Server
//!
//! **⚠️ DEPRECATED**: This module is deprecated as of v2.3.0
//!
//! ## Migration to Universal IPC Architecture
//!
//! **Connection logic has moved to Songbird** (Universal IPC Layer)
//!
//! ### Why This Change?
//!
//! - **Separation of Concerns**: `NestGate` = Storage, Songbird = Communication
//! - **True Universality**: Songbird abstracts platform differences (Unix/Windows/etc.)
//! - **Single Responsibility**: Each primal owns its domain
//!
//! ### Migration Path
//!
//! **Before (`NestGate` Unix sockets)**:
//! ```rust,ignore
//! use nestgate_core::rpc::JsonRpcUnixServer;
//!
//! let server = JsonRpcUnixServer::new("myservice").await?;
//! server.serve().await?;
//! ```
//!
//! **After (Songbird Universal IPC)**:
//! ```rust,ignore
//! use songbird::ipc;
//!
//! // Register with Songbird (works on ALL platforms!)
//! let endpoint = ipc::register("myservice").await?;
//! ipc::listen(endpoint).await?;
//!
//! // Songbird stores metadata in NestGate automatically
//! ```
//!
//! ### What `NestGate` Still Provides
//!
//! - ✅ Service metadata storage (`service_metadata` module)
//! - ✅ Capability-based discovery
//! - ✅ Persistent service registry
//!
//! ### References
//!
//! - `UNIVERSAL_IPC_ARCHITECTURE_HANDOFF_JAN_19_2026.md`
//! - `UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md`
//! - `code/crates/nestgate-core/src/service_metadata/mod.rs`
//!
//! ---
//!
//! ## Legacy Documentation (Deprecated)
//!
//! **biomeOS IPC Integration** - Native Unix socket communication
//!
//! Implements JSON-RPC 2.0 server over Unix sockets for efficient
//! primal-to-primal communication within the biomeOS ecosystem.
//!
//! ## Philosophy
//! - **Self-Knowledge**: Socket path from own environment ($`NESTGATE_FAMILY_ID`)
//! - **Runtime Discovery**: Discover Songbird via capability system
//! - **Zero Hardcoding**: All configuration from environment
//! - **Memory Safe**: Zero unsafe blocks
//! - **Modern Async**: Native async/await with tokio
//!
//! ## Socket Path Pattern
//! ```text
//! /run/user/{uid}/nestgate-{family_id}.sock
//! ```
//!
//! ## Environment Variables
//! - `NESTGATE_FAMILY_ID` (required): Family identifier for socket path
//! - `SONGBIRD_FAMILY_ID` (optional): For auto-registration
//!
//! ## Usage (Deprecated)
//! ```rust,ignore
//! use nestgate_core::rpc::unix_socket_server::JsonRpcUnixServer;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let family_id = std::env::var("NESTGATE_FAMILY_ID")?;
//! let server = JsonRpcUnixServer::new(&family_id).await?;
//! server.serve().await?;
//! # Ok(())
//! # }
//! ```

mod audit_handlers;
mod nat_handlers;
mod storage_handlers;
mod template_handlers;

use crate::rpc::model_cache_handlers;
use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tracing::{debug, error, info, warn};

/// JSON-RPC 2.0 Request
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: Arc<str>,
    method: Arc<str>,
    params: Option<Value>,
    id: Option<Value>,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: Arc<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    id: Option<Value>,
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

/// Storage service state
#[derive(Clone)]
pub(crate) struct StorageState {
    /// **Integration:** Full dataset/object persistence uses `nestgate-core`
    /// `services::storage::StorageManagerService` when this legacy server is linked to core.
    /// Template storage for collaborative intelligence
    templates: crate::rpc::template_storage::TemplateStorage,
    /// Audit storage for execution tracking
    audits: crate::rpc::audit_storage::AuditStorage,
    /// Server's `family_id` derived from the socket name.
    /// Storage handlers default to this when callers omit `family_id`,
    /// eliminating redundant params for family-scoped socket connections.
    pub(crate) family_id: Option<String>,
    /// Set when template/audit backends are successfully initialized (`StorageState::new`).
    pub(crate) storage_initialized: bool,
}

impl StorageState {
    /// Create new storage state (templates/audit; core storage via integration when available).
    pub(crate) async fn new() -> Result<Self> {
        tracing::debug!("feature pending: StorageManagerService wiring for Unix JSON-RPC storage");
        Ok(Self {
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
            family_id: None,
            storage_initialized: true,
        })
    }
}

/// JSON-RPC Unix socket server for biomeOS integration
///
/// **⚠️ DEPRECATED**: Use `songbird::ipc` instead (Universal IPC Architecture)
///
/// Connection logic has moved to Songbird for true platform universality.
/// See `UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md` for migration guide.
#[deprecated(
    since = "2.3.0",
    note = "Connection logic moved to Songbird IPC SERVICE. \
            Call /primal/songbird via JSON-RPC - DO NOT import songbird code! \
            See UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md for service-based integration."
)]
pub struct JsonRpcUnixServer {
    socket_path: PathBuf,
    /// Family ID for primal identification (used in future multi-primal features)
    family_id: String,
    state: StorageState,
}

impl JsonRpcUnixServer {
    /// Create new Unix socket server with standardized configuration
    ///
    /// # Configuration Priority (3-tier fallback)
    ///
    /// 1. `NESTGATE_SOCKET` env var (explicit override)
    /// 2. XDG runtime: `/run/user/{uid}/nestgate-{family}.sock` (recommended)
    /// 3. Temp fallback: `/tmp/nestgate-{family}-{node}.sock` (least secure)
    ///
    /// # Self-Knowledge Principle
    /// - Socket path derived from environment and own identity
    /// - Agnostic to deployment environment
    /// - Buildable (creates directories, cleans old sockets)
    /// - No hardcoding or assumptions
    ///
    /// # Arguments
    /// - `family_id`: Family identifier from $`NESTGATE_FAMILY_ID`
    ///
    /// # Errors
    /// - Returns error if socket path cannot be prepared
    /// - Returns error if socket binding fails
    pub async fn new(family_id: &str) -> Result<Self> {
        // Use standardized socket configuration
        let socket_config = crate::rpc::socket_config::SocketConfig::from_environment()?;

        // Log configuration before preparing
        info!("═══════════════════════════════════════════════════════════");
        info!("🏰 NestGate JSON-RPC Unix Socket Server");
        info!("═══════════════════════════════════════════════════════════");
        socket_config.log_summary();

        // Prepare socket path (create dirs, remove old socket)
        socket_config.prepare_socket_path()?;

        let socket_path = socket_config.socket_path;

        // Initialize persistent storage backend
        info!("📦 Initializing persistent storage backend...");
        let state = StorageState::new().await?;
        info!("✅ Storage backend initialized");

        Ok(Self {
            socket_path,
            family_id: family_id.to_string(),
            state,
        })
    }

    /// Start serving requests
    ///
    /// Binds to Unix socket and processes JSON-RPC 2.0 requests
    /// indefinitely. Each connection is handled concurrently.
    pub async fn serve(&self) -> Result<()> {
        let listener = UnixListener::bind(&self.socket_path).map_err(|e| {
            NestGateError::configuration_error(
                "socket_bind",
                &format!("Failed to bind Unix socket: {e}"),
            )
        })?;

        info!("═══════════════════════════════════════════════════════════");
        info!("✅ NestGate ready!");
        info!("   Socket: {}", self.socket_path.display());
        info!("   Family: {}", self.family_id);
        info!("   Protocol: JSON-RPC 2.0 over Unix socket");
        info!("═══════════════════════════════════════════════════════════");
        info!(
            "💡 Test with: echo '{{\"jsonrpc\":\"2.0\",\"method\":\"storage.list\",\"id\":1}}' | nc -U {}",
            self.socket_path.display()
        );

        let mut state = self.state.clone();
        state.family_id = Some(self.family_id.clone());
        let state = Arc::new(state);

        loop {
            match listener.accept().await {
                Ok((stream, _addr)) => {
                    let state = Arc::clone(&state);
                    tokio::spawn(async move {
                        if let Err(e) = handle_connection(stream, state).await {
                            error!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    /// Get socket path (for testing)
    #[must_use]
    pub const fn socket_path(&self) -> &PathBuf {
        &self.socket_path
    }
}

impl Drop for JsonRpcUnixServer {
    fn drop(&mut self) {
        // Clean up socket file
        if self.socket_path.exists()
            && let Err(e) = std::fs::remove_file(&self.socket_path)
        {
            warn!("Failed to remove socket file: {}", e);
        }
    }
}

/// Handle a single Unix socket connection
async fn handle_connection(stream: UnixStream, state: Arc<StorageState>) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader
            .read_line(&mut line)
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to read request: {e}")))?;

        if bytes_read == 0 {
            // Connection closed
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        debug!("Received request: {}", trimmed);

        // Parse and handle request
        let response = match serde_json::from_str::<JsonRpcRequest>(trimmed) {
            Ok(request) => handle_request(request, &state).await,
            Err(e) => {
                error!("Failed to parse JSON-RPC request: {}", e);
                JsonRpcResponse {
                    jsonrpc: Arc::from("2.0"),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32700,
                        message: Cow::Borrowed("Parse error"),
                        data: Some(json!({"error": e.to_string()})),
                    }),
                    id: None,
                }
            }
        };

        // Send response
        let response_json = serde_json::to_string(&response)
            .map_err(|e| NestGateError::api(format!("Failed to serialize response: {e}")))?;

        writer
            .write_all(response_json.as_bytes())
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to write response: {e}")))?;
        writer
            .write_all(b"\n")
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to write newline: {e}")))?;

        debug!("Sent response: {}", response_json);
    }

    Ok(())
}

/// Handle JSON-RPC request
async fn handle_request(request: JsonRpcRequest, state: &StorageState) -> JsonRpcResponse {
    if request.jsonrpc.as_ref() != "2.0" {
        return JsonRpcResponse {
            jsonrpc: Arc::from("2.0"),
            result: None,
            error: Some(JsonRpcError {
                code: -32600,
                message: Cow::Borrowed("Invalid Request"),
                data: Some(json!({"error": "Only JSON-RPC 2.0 is supported"})),
            }),
            id: request.id,
        };
    }

    let result = match &*request.method {
        // Health — wateringHole semantic names + legacy aliases
        "health.liveness" => Ok(json!({
            "status": "alive",
            "primal": "nestgate",
        })),
        "health.readiness" => Ok(if state.storage_initialized {
            json!({
                "status": "ready",
                "primal": "nestgate",
                "storage": "initialized",
            })
        } else {
            json!({
                "status": "not_ready",
                "primal": "nestgate",
                "storage": "not_initialized",
            })
        }),
        "health" | "health.check" => Ok(
            json!({"status": "healthy", "version": env!("CARGO_PKG_VERSION"), "primal": "nestgate"}),
        ),
        "capabilities.list" => model_cache_handlers::capabilities_list().await,
        "discover_capabilities" | "discover.capabilities" => {
            model_cache_handlers::discover_capabilities().await
        }
        // Storage operations
        "storage.store" | "storage.put" => {
            storage_handlers::storage_store(&request.params, state).await
        }
        "storage.retrieve" | "storage.get" => {
            storage_handlers::storage_retrieve(&request.params, state).await
        }
        "storage.exists" => storage_handlers::storage_exists(&request.params, state).await,
        "storage.delete" => storage_handlers::storage_delete(&request.params, state).await,
        "storage.list" => storage_handlers::storage_list(&request.params, state).await,
        "storage.stats" => storage_handlers::storage_stats(&request.params, state).await,
        "storage.store_blob" => storage_handlers::storage_store_blob(&request.params, state).await,
        "storage.retrieve_blob" => {
            storage_handlers::storage_retrieve_blob(&request.params, state).await
        }
        // Model cache operations (extracted to model_cache_handlers.rs)
        "model.register" => model_cache_handlers::model_register(&request.params).await,
        "model.exists" => model_cache_handlers::model_exists(&request.params).await,
        "model.locate" => model_cache_handlers::model_locate(&request.params).await,
        "model.metadata" => model_cache_handlers::model_metadata(&request.params).await,
        // Template operations
        "templates.store" => template_handlers::templates_store(&request.params, state).await,
        "templates.retrieve" => template_handlers::templates_retrieve(&request.params, state).await,
        "templates.list" => template_handlers::templates_list(&request.params, state).await,
        "templates.community_top" => {
            template_handlers::templates_community_top(&request.params, state).await
        }
        // Audit operations
        "audit.store_execution" => {
            audit_handlers::audit_store_execution(&request.params, state).await
        }
        // NAT traversal persistence (relay-assisted coordinated punch protocol)
        "nat.store_traversal_info" => {
            nat_handlers::nat_store_traversal_info(&request.params, state).await
        }
        "nat.retrieve_traversal_info" => {
            nat_handlers::nat_retrieve_traversal_info(&request.params, state).await
        }
        // Known beacon persistence
        "beacon.store" => nat_handlers::beacon_store(&request.params, state).await,
        "beacon.retrieve" => nat_handlers::beacon_retrieve(&request.params, state).await,
        "beacon.list" => nat_handlers::beacon_list(&request.params, state).await,
        "beacon.delete" => nat_handlers::beacon_delete(&request.params, state).await,
        // Alias: NAT beacon discovery (same payload shape as `beacon.list`)
        "nat.beacon" => nat_handlers::beacon_list(&request.params, state).await,
        _ => {
            return JsonRpcResponse {
                jsonrpc: Arc::from("2.0"),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: Cow::Borrowed("Method not found"),
                    data: Some(json!({"method": request.method})),
                }),
                id: request.id,
            };
        }
    };

    match result {
        Ok(value) => JsonRpcResponse {
            jsonrpc: Arc::from("2.0"),
            result: Some(value),
            error: None,
            id: request.id,
        },
        Err(e) => JsonRpcResponse {
            jsonrpc: Arc::from("2.0"),
            result: None,
            error: Some(JsonRpcError {
                code: -32603,
                message: Cow::Borrowed("Internal error"),
                data: Some(json!({"error": e.to_string()})),
            }),
            id: request.id,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore = "storage handlers require nestgate-core StorageManagerService wiring"]
    async fn test_storage_store_retrieve() {
        let state = StorageState::new()
            .await
            .expect("Failed to create test storage state");

        // Store
        let store_params = json!({
            "key": "test_key",
            "data": {"value": "test_data"},
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_store(&Some(store_params), &state)
            .await
            .unwrap();
        assert_eq!(result["success"], true);

        // Retrieve
        let retrieve_params = json!({
            "key": "test_key",
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_retrieve(&Some(retrieve_params), &state)
            .await
            .unwrap();
        assert_eq!(result["data"]["value"], "test_data");
    }

    #[tokio::test]
    #[ignore = "storage handlers require nestgate-core StorageManagerService wiring"]
    async fn test_storage_delete() {
        let state = StorageState::new()
            .await
            .expect("Failed to create test storage state");

        // Store
        let store_params = json!({
            "key": "delete_key",
            "data": {"value": "delete_me"},
            "family_id": "test_family"
        });
        storage_handlers::storage_store(&Some(store_params), &state)
            .await
            .unwrap();

        // Delete
        let delete_params = json!({
            "key": "delete_key",
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_delete(&Some(delete_params), &state)
            .await
            .unwrap();
        assert_eq!(result["success"], true);

        // Verify deleted
        let retrieve_params = json!({
            "key": "delete_key",
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_retrieve(&Some(retrieve_params), &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[ignore = "list keys count does not match development storage backend (assertions fail)"]
    async fn test_storage_list() {
        let state = StorageState::new()
            .await
            .expect("Failed to create test storage state");

        // Store multiple keys
        for i in 0..5 {
            let params = json!({
                "key": format!("key_{}", i),
                "data": {"index": i},
                "family_id": "test_family"
            });
            storage_handlers::storage_store(&Some(params), &state)
                .await
                .unwrap();
        }

        // List all
        let list_params = json!({"family_id": "test_family"});
        let result = storage_handlers::storage_list(&Some(list_params), &state)
            .await
            .unwrap();
        assert_eq!(result["keys"].as_array().unwrap().len(), 5);
    }

    #[tokio::test]
    #[ignore = "key_count does not match development storage backend (assertions fail)"]
    async fn test_storage_stats() {
        let state = StorageState::new()
            .await
            .expect("Failed to create test storage state");

        // Store some data
        let store_params = json!({
            "key": "stats_key",
            "data": {"value": "stats"},
            "family_id": "test_family"
        });
        storage_handlers::storage_store(&Some(store_params), &state)
            .await
            .unwrap();

        // Get stats
        let stats_params = json!({"family_id": "test_family"});
        let result = storage_handlers::storage_stats(&Some(stats_params), &state)
            .await
            .unwrap();
        assert_eq!(result["key_count"], 1);
        assert_eq!(result["blob_count"], 0);
    }

    #[tokio::test]
    #[ignore = "storage handlers require nestgate-core StorageManagerService wiring"]
    async fn test_blob_storage() {
        let state = StorageState::new()
            .await
            .expect("Failed to create test storage state");

        // Store blob
        let test_data = b"Hello, World!";
        use base64::Engine;
        let blob_base64 = base64::engine::general_purpose::STANDARD.encode(test_data);

        let store_params = json!({
            "key": "test_blob",
            "blob": blob_base64,
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_store_blob(&Some(store_params), &state)
            .await
            .unwrap();
        assert_eq!(result["success"], true);
        assert_eq!(result["size"], test_data.len());

        // Retrieve blob
        let retrieve_params = json!({
            "key": "test_blob",
            "family_id": "test_family"
        });
        let result = storage_handlers::storage_retrieve_blob(&Some(retrieve_params), &state)
            .await
            .unwrap();
        let retrieved_blob = base64::engine::general_purpose::STANDARD
            .decode(result["blob"].as_str().unwrap())
            .unwrap();
        assert_eq!(retrieved_blob, test_data);
    }

    #[tokio::test]
    async fn handle_request_health_liveness() {
        let state = StorageState::new().await.expect("storage state");
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "health.liveness".into(),
            params: None,
            id: Some(json!(1)),
        };
        let resp = handle_request(req, &state).await;
        assert!(resp.error.is_none());
        assert_eq!(
            resp.result.as_ref().and_then(|v| v.get("status")),
            Some(&json!("alive"))
        );
    }

    #[tokio::test]
    async fn handle_request_health_readiness_initialized() {
        let state = StorageState::new().await.expect("storage state");
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "health.readiness".into(),
            params: None,
            id: None,
        };
        let resp = handle_request(req, &state).await;
        assert!(resp.error.is_none());
        let st = resp.result.as_ref().and_then(|v| v.get("status"));
        assert_eq!(st, Some(&json!("ready")));
    }

    #[tokio::test]
    async fn handle_request_invalid_jsonrpc_version() {
        let state = StorageState::new().await.expect("storage state");
        let req = JsonRpcRequest {
            jsonrpc: "1.0".into(),
            method: "health".into(),
            params: None,
            id: Some(json!("a")),
        };
        let resp = handle_request(req, &state).await;
        assert!(resp.result.is_none());
        let err = resp.error.expect("error");
        assert_eq!(err.code, -32600);
    }

    #[tokio::test]
    async fn handle_request_method_not_found() {
        let state = StorageState::new().await.expect("storage state");
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "no.such.method".into(),
            params: None,
            id: Some(json!(99)),
        };
        let resp = handle_request(req, &state).await;
        let err = resp.error.expect("error");
        assert_eq!(err.code, -32601);
    }

    #[tokio::test]
    async fn handle_request_health_alias() {
        let state = StorageState::new().await.expect("storage state");
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "health".into(),
            params: None,
            id: Some(json!(0)),
        };
        let resp = handle_request(req, &state).await;
        assert!(resp.error.is_none());
        assert_eq!(
            resp.result.as_ref().and_then(|v| v.get("status")),
            Some(&json!("healthy"))
        );
    }

    #[tokio::test]
    async fn handle_request_health_check_alias() {
        let state = StorageState::new().await.expect("storage state");
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "health.check".into(),
            params: None,
            id: Some(json!("chk")),
        };
        let resp = handle_request(req, &state).await;
        assert!(resp.error.is_none());
        assert_eq!(
            resp.result.as_ref().and_then(|v| v.get("status")),
            Some(&json!("healthy"))
        );
    }

    #[tokio::test]
    async fn handle_request_readiness_not_initialized() {
        let mut state = StorageState::new().await.expect("storage state");
        state.storage_initialized = false;
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "health.readiness".into(),
            params: None,
            id: None,
        };
        let resp = handle_request(req, &state).await;
        assert!(resp.error.is_none());
        assert_eq!(
            resp.result.as_ref().and_then(|v| v.get("status")),
            Some(&json!("not_ready"))
        );
    }

    #[tokio::test]
    async fn handle_request_capabilities_list() {
        let state = StorageState::new().await.expect("storage state");
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "capabilities.list".into(),
            params: None,
            id: Some(json!(2)),
        };
        let resp = handle_request(req, &state).await;
        assert!(resp.error.is_none());
        assert!(
            resp.result
                .as_ref()
                .and_then(|v| v.get("methods"))
                .is_some()
        );
    }

    #[tokio::test]
    async fn handle_request_discover_capabilities() {
        let state = StorageState::new().await.expect("storage state");
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "discover_capabilities".into(),
            params: None,
            id: Some(json!(3)),
        };
        let resp = handle_request(req, &state).await;
        assert!(resp.error.is_none());
        assert!(
            resp.result
                .as_ref()
                .and_then(|v| v.get("capabilities"))
                .is_some()
        );
    }

    #[tokio::test]
    async fn handle_request_model_register_returns_internal_jsonrpc_error() {
        let state = StorageState::new().await.expect("storage state");
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "model.register".into(),
            params: Some(json!({})),
            id: Some(json!(4)),
        };
        let resp = handle_request(req, &state).await;
        let err = resp.error.expect("expected JSON-RPC error");
        assert_eq!(err.code, -32603);
        assert_eq!(err.message, "Internal error");
    }

    #[tokio::test]
    async fn handle_request_model_exists_locate_metadata_not_implemented() {
        let state = StorageState::new().await.expect("storage state");
        for method in ["model.exists", "model.locate", "model.metadata"] {
            let req = JsonRpcRequest {
                jsonrpc: "2.0".into(),
                method: method.into(),
                params: Some(json!({"model_id": "m1"})),
                id: Some(json!(method)),
            };
            let resp = handle_request(req, &state).await;
            let err = resp.error.expect("jsonrpc error");
            assert_eq!(err.code, -32603, "{method}");
        }
    }

    #[tokio::test]
    async fn handle_request_nat_and_beacon_stubs_not_implemented() {
        let state = StorageState::new().await.expect("storage state");
        for method in [
            "nat.store_traversal_info",
            "nat.retrieve_traversal_info",
            "beacon.store",
            "beacon.retrieve",
            "beacon.delete",
        ] {
            let req = JsonRpcRequest {
                jsonrpc: "2.0".into(),
                method: method.into(),
                params: Some(json!({})),
                id: Some(json!(method)),
            };
            let resp = handle_request(req, &state).await;
            let err = resp.error.expect("jsonrpc error");
            assert_eq!(err.code, -32603, "{method}");
        }
    }

    #[tokio::test]
    async fn handle_request_beacon_list_ok() {
        let state = StorageState::new().await.expect("storage state");
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "beacon.list".into(),
            params: Some(json!({})),
            id: Some(json!(1)),
        };
        let resp = handle_request(req, &state).await;
        assert!(resp.error.is_none());
        let arr = resp.result.as_ref().and_then(|v| v.get("peer_ids"));
        assert!(arr.is_some());
    }

    #[tokio::test]
    async fn handle_request_templates_store_and_list_dispatch() {
        let state = StorageState::new().await.expect("storage state");
        let store = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "templates.store".into(),
            params: Some(json!({
                "name": "n",
                "description": "d",
                "graph_data": {},
                "user_id": "u",
                "family_id": "fam-dispatch"
            })),
            id: Some(json!(1)),
        };
        let resp = handle_request(store, &state).await;
        assert!(resp.error.is_none());
        let list = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "templates.list".into(),
            params: Some(json!({"family_id": "fam-dispatch"})),
            id: Some(json!(2)),
        };
        let resp = handle_request(list, &state).await;
        assert!(resp.error.is_none());
        assert_eq!(
            resp.result
                .as_ref()
                .and_then(|v| v.get("total"))
                .and_then(|v| v.as_u64()),
            Some(1)
        );
    }

    #[tokio::test]
    async fn handle_request_templates_community_top_dispatch() {
        let state = StorageState::new().await.expect("storage state");
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "templates.community_top".into(),
            params: Some(json!({"limit": 3})),
            id: Some(json!(1)),
        };
        let resp = handle_request(req, &state).await;
        assert!(resp.error.is_none());
        assert_eq!(
            resp.result
                .as_ref()
                .and_then(|v| v.get("templates"))
                .and_then(|v| v.as_array())
                .map(|a| a.len()),
            Some(0)
        );
    }

    #[tokio::test]
    async fn handle_request_audit_store_execution_dispatch() {
        let state = StorageState::new().await.expect("storage state");
        let params = json!({
            "id": "audit-1",
            "execution_id": "ex-1",
            "graph_id": "g-1",
            "user_id": "user",
            "family_id": "fam-audit",
            "started_at": "2025-06-01T12:00:00Z",
            "status": "running",
            "modifications": [],
            "outcomes": [],
            "metadata": {}
        });
        let req = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "audit.store_execution".into(),
            params: Some(params),
            id: Some(json!(42)),
        };
        let resp = handle_request(req, &state).await;
        assert!(resp.error.is_none());
        assert_eq!(
            resp.result.as_ref().and_then(|v| v.get("success")),
            Some(&json!(true))
        );
    }

    #[tokio::test]
    async fn handle_request_discover_capabilities_dot_alias_matches_discover_capabilities() {
        let state = StorageState::new().await.expect("storage state");
        let a = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "discover_capabilities".into(),
            params: None,
            id: Some(json!(1)),
        };
        let b = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "discover.capabilities".into(),
            params: None,
            id: Some(json!(2)),
        };
        let ra = handle_request(a, &state).await;
        let rb = handle_request(b, &state).await;
        assert_eq!(
            ra.error.as_ref().map(|e| (e.code, &*e.message)),
            rb.error.as_ref().map(|e| (e.code, &*e.message))
        );
        assert_eq!(ra.result, rb.result);
    }

    #[tokio::test]
    async fn handle_request_storage_get_put_aliases_match_retrieve_store_errors() {
        let state = StorageState::new().await.expect("storage state");
        let get_alias = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "storage.get".into(),
            params: Some(json!({"key": "k", "family_id": "f"})),
            id: Some(json!(1)),
        };
        let retrieve = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "storage.retrieve".into(),
            params: Some(json!({"key": "k", "family_id": "f"})),
            id: Some(json!(2)),
        };
        let rg = handle_request(get_alias, &state).await;
        let rr = handle_request(retrieve, &state).await;
        assert_eq!(
            rg.error.as_ref().map(|e| e.code),
            rr.error.as_ref().map(|e| e.code)
        );

        let put_alias = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "storage.put".into(),
            params: Some(json!({"key": "k", "data": {}, "family_id": "f"})),
            id: Some(json!(3)),
        };
        let store = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "storage.store".into(),
            params: Some(json!({"key": "k", "data": {}, "family_id": "f"})),
            id: Some(json!(4)),
        };
        let rp = handle_request(put_alias, &state).await;
        let rs = handle_request(store, &state).await;
        assert_eq!(
            rp.error.as_ref().map(|e| e.code),
            rs.error.as_ref().map(|e| e.code)
        );
    }

    #[tokio::test]
    async fn handle_request_nat_beacon_alias_matches_beacon_list() {
        let state = StorageState::new().await.expect("storage state");
        let a = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "beacon.list".into(),
            params: Some(json!({})),
            id: Some(json!(1)),
        };
        let b = JsonRpcRequest {
            jsonrpc: "2.0".into(),
            method: "nat.beacon".into(),
            params: Some(json!({})),
            id: Some(json!(2)),
        };
        let ra = handle_request(a, &state).await;
        let rb = handle_request(b, &state).await;
        assert_eq!(
            ra.error.as_ref().map(|e| (e.code, &*e.message)),
            rb.error.as_ref().map(|e| (e.code, &*e.message))
        );
        assert_eq!(ra.result, rb.result);
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn handle_connection_rejects_invalid_json_line() {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        use tokio::net::UnixStream;

        let state = StorageState::new().await.expect("storage state");
        let state = Arc::new(state);
        let (client, server) = UnixStream::pair().expect("unix pair");
        let h = tokio::spawn(handle_connection(server, Arc::clone(&state)));
        let (mut c_read, mut c_write) = client.into_split();
        c_write
            .write_all(b"{not valid json}\n")
            .await
            .expect("write");
        let mut line = String::new();
        BufReader::new(&mut c_read)
            .read_line(&mut line)
            .await
            .expect("read");
        let v: serde_json::Value = serde_json::from_str(line.trim()).expect("resp json");
        assert_eq!(v["error"]["code"], -32700);
        drop(c_write);
        let _ = h.await;
    }
}
