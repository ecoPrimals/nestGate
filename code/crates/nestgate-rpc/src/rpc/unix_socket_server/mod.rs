// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

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
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
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
    pub(crate) fn new() -> Result<Self> {
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
    pub fn new(family_id: &str) -> Result<Self> {
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
        let state = StorageState::new()?;
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
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`] if binding the Unix listener fails. Accept-loop errors are logged
    /// and do not stop the server.
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
    let mut line = Vec::new();

    loop {
        line.clear();
        let n = reader
            .read_until(b'\n', &mut line)
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to read request: {e}")))?;

        if n == 0 && line.is_empty() {
            // Connection closed
            break;
        }

        let trimmed = line.as_slice().trim_ascii();
        if trimmed.is_empty() {
            continue;
        }

        debug!("Received request: {}", String::from_utf8_lossy(trimmed));

        // Parse and handle request (from bytes: no UTF-8 `String` buffer for the line)
        let response = match serde_json::from_slice::<JsonRpcRequest>(trimmed) {
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
        "capabilities.list" => model_cache_handlers::capabilities_list(),
        "discover_capabilities" | "discover.capabilities" => {
            model_cache_handlers::discover_capabilities()
        }
        // Storage operations
        "storage.store" | "storage.put" => {
            storage_handlers::storage_store(request.params.as_ref(), state)
        }
        "storage.retrieve" | "storage.get" => {
            storage_handlers::storage_retrieve(request.params.as_ref(), state)
        }
        "storage.exists" => storage_handlers::storage_exists(request.params.as_ref(), state),
        "storage.delete" => storage_handlers::storage_delete(request.params.as_ref(), state),
        "storage.list" => storage_handlers::storage_list(request.params.as_ref(), state).await,
        "storage.stats" => storage_handlers::storage_stats(request.params.as_ref(), state).await,
        "storage.store_blob" => {
            storage_handlers::storage_store_blob(request.params.as_ref(), state)
        }
        "storage.retrieve_blob" => {
            storage_handlers::storage_retrieve_blob(request.params.as_ref(), state)
        }
        // Model cache operations (extracted to model_cache_handlers.rs)
        "model.register" => model_cache_handlers::model_register(request.params.as_ref()),
        "model.exists" => model_cache_handlers::model_exists(request.params.as_ref()),
        "model.locate" => model_cache_handlers::model_locate(request.params.as_ref()),
        "model.metadata" => model_cache_handlers::model_metadata(request.params.as_ref()),
        // Template operations
        "templates.store" => {
            template_handlers::templates_store(request.params.as_ref(), state).await
        }
        "templates.retrieve" => {
            template_handlers::templates_retrieve(request.params.as_ref(), state).await
        }
        "templates.list" => template_handlers::templates_list(request.params.as_ref(), state).await,
        "templates.community_top" => {
            template_handlers::templates_community_top(request.params.as_ref(), state).await
        }
        // Audit operations
        "audit.store_execution" => {
            audit_handlers::audit_store_execution(request.params.as_ref(), state).await
        }
        // NAT traversal persistence (relay-assisted coordinated punch protocol)
        "nat.store_traversal_info" => {
            nat_handlers::nat_store_traversal_info(request.params.as_ref(), state)
        }
        "nat.retrieve_traversal_info" => {
            nat_handlers::nat_retrieve_traversal_info(request.params.as_ref(), state)
        }
        // Known beacon persistence
        "beacon.store" => nat_handlers::beacon_store(request.params.as_ref(), state),
        "beacon.retrieve" => nat_handlers::beacon_retrieve(request.params.as_ref(), state),
        // Alias `nat.beacon` uses the same payload shape as `beacon.list`
        "beacon.list" | "nat.beacon" => {
            nat_handlers::beacon_list(request.params.as_ref(), state).await
        }
        "beacon.delete" => nat_handlers::beacon_delete(request.params.as_ref(), state),
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

/// Bridges the full ecosystem JSON-RPC dispatch table to [`crate::rpc::isomorphic_ipc::RpcHandler`]
/// for [`crate::rpc::isomorphic_ipc::IsomorphicIpcServer`] (same methods as [`JsonRpcUnixServer`]).
pub struct LegacyUnixJsonRpcHandler {
    state: Arc<StorageState>,
}

impl LegacyUnixJsonRpcHandler {
    /// Create a handler backed by the given storage/template/audit state.
    #[must_use]
    pub const fn new(state: Arc<StorageState>) -> Self {
        Self { state }
    }
}

impl crate::rpc::isomorphic_ipc::RpcHandler for LegacyUnixJsonRpcHandler {
    fn handle_request(&self, request: Value) -> Pin<Box<dyn Future<Output = Value> + Send + '_>> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            match serde_json::from_value::<JsonRpcRequest>(request) {
                Ok(req) => {
                    let resp = handle_request(req, &state).await;
                    serde_json::to_value(resp).unwrap_or_else(|_| {
                        json!({
                            "jsonrpc": "2.0",
                            "error": { "code": -32603, "message": "Internal error" },
                            "id": null
                        })
                    })
                }
                Err(e) => json!({
                    "jsonrpc": "2.0",
                    "error": {
                        "code": -32700,
                        "message": "Parse error",
                        "data": { "error": e.to_string() }
                    },
                    "id": null
                }),
            }
        })
    }
}

/// Build the same JSON-RPC handler surface as [`JsonRpcUnixServer`] for use with
/// [`crate::rpc::isomorphic_ipc::IsomorphicIpcServer`] (ecosystem socket layout via [`crate::rpc::socket_config::SocketConfig`]).
///
/// # Errors
///
/// Returns [`NestGateError`] if the storage backend used by the handler cannot be initialized.
pub fn legacy_ecosystem_rpc_handler(
    family_id: impl Into<String>,
) -> Result<Arc<dyn crate::rpc::isomorphic_ipc::RpcHandler>> {
    let mut state = StorageState::new()?;
    state.family_id = Some(family_id.into());
    Ok(Arc::new(LegacyUnixJsonRpcHandler::new(Arc::new(state))))
}

#[cfg(test)]
mod tests;
