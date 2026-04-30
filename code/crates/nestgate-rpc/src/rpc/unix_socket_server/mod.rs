// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! # JSON-RPC Unix Socket Server
//!
//! **DEPRECATED**: This module is deprecated as of v2.3.0
//!
//! ## Migration to Universal IPC Architecture
//!
//! **Connection logic has moved to the orchestration provider** (Universal IPC Layer)
//!
//! ### Why This Change?
//!
//! - **Separation of Concerns**: `NestGate` = Storage, orchestration layer = Communication
//! - **True Universality**: The orchestration IPC layer abstracts platform differences (Unix/Windows/etc.)
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
//! **After (Universal IPC via Orchestration Provider)**:
//! ```rust,ignore
//! // Register with the orchestration provider via JSON-RPC (works on ALL platforms!)
//! let client = JsonRpcClient::connect_unix("/run/capability/orchestration.sock").await?;
//! let response = client.call("ipc.register", json!({
//!     "service_id": "myservice",
//!     "capabilities": ["storage"],
//! })).await?;
//! ```
//!
//! ### What `NestGate` Still Provides
//!
//! - Service metadata storage (`service_metadata` module)
//! - Capability-based discovery
//! - Persistent service registry
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
//! **Ecosystem Unix IPC** — native Unix socket communication (`BIOMEOS_SOCKET_DIR` is the standard shared-socket path; see [`crate::rpc::socket_config`])
//!
//! Implements JSON-RPC 2.0 server over Unix sockets for efficient
//! capability-peer communication within the ecosystem runtime layout.
//!
//! ## Philosophy
//! - **Self-Knowledge**: Socket path from own environment ($`NESTGATE_FAMILY_ID`)
//! - **Runtime Discovery**: Discover the orchestration provider via the capability system
//! - **Zero Hardcoding**: All configuration from environment
//! - **Memory safety**: implemented with safe Rust throughout this path
//! - **Modern Async**: Native async/await with tokio
//!
//! ## Socket Path Pattern
//! ```text
//! /run/user/{uid}/nestgate-{family_id}.sock
//! ```
//!
//! ## Environment Variables
//! - `NESTGATE_FAMILY_ID` (required): Family identifier for socket path
//! - `NESTGATE_ORCHESTRATION_FAMILY_ID` (optional): For auto-registration with orchestration provider
//!
//! ## Usage (Deprecated)
//! ```rust,ignore
//! use nestgate_core::rpc::unix_socket_server::JsonRpcUnixServer;
//!
//! # async fn example() -> std::result::Result<(), nestgate_types::NestGateError> {
//! let family_id = std::env::var("NESTGATE_FAMILY_ID")?;
//! let server = JsonRpcUnixServer::new(&family_id).await?;
//! server.serve().await?;
//! # Ok(())
//! # }
//! ```

mod audit_handlers;
mod blob_handlers;
mod bonding_handlers;
mod dispatch;
pub(crate) mod external_handlers;
mod nat_handlers;
mod session_handlers;
mod storage_handlers;
mod template_handlers;
mod zfs_handlers;

use bytes::Bytes;
use dispatch::handle_request;
use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::borrow::Cow;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
#[cfg(unix)]
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::BufReader;
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
    /// Native encrypt-at-rest via ChaCha20-Poly1305. When `Some`, all
    /// `storage.store` data is encrypted before writing to disk and
    /// `storage.retrieve` auto-decrypts envelope payloads.
    pub(crate) encryption: Option<Arc<crate::rpc::storage_encryption::StorageEncryption>>,
}

impl StorageState {
    /// Create new storage state (templates/audit; filesystem-backed storage via dataset paths).
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
            family_id: None,
            storage_initialized: true,
            encryption: None,
        })
    }
}

/// JSON-RPC Unix socket server for ecosystem Unix IPC (standard layout under `BIOMEOS_SOCKET_DIR`; see socket config)
///
/// **DEPRECATED**: Use the orchestration provider's IPC service instead (Universal IPC Architecture)
///
/// Connection logic has moved to the orchestration provider for true platform universality.
/// See `UNIVERSAL_IPC_EVOLUTION_PLAN_JAN_19_2026.md` for migration guide.
pub struct JsonRpcUnixServer {
    socket_path: PathBuf,
    /// Family ID for primal identification (used in future multi-primal features)
    family_id: String,
    state: StorageState,
    /// Set in [`JsonRpcUnixServer::serve`] when `storage.sock` was installed (ecosystem `.../biomeos/` layout only).
    #[cfg(unix)]
    storage_capability_symlink_installed: AtomicBool,
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
        info!("NestGate JSON-RPC Unix Socket Server");
        info!("═══════════════════════════════════════════════════════════");
        socket_config.log_summary();

        // Prepare socket path (create dirs, remove old socket)
        socket_config.prepare_socket_path()?;

        let socket_path = socket_config.socket_path;

        // Initialize persistent storage backend
        info!("Initializing persistent storage backend...");
        let state = StorageState::new()?;
        info!("Storage backend initialized");

        Ok(Self {
            socket_path,
            family_id: family_id.to_string(),
            state,
            #[cfg(unix)]
            storage_capability_symlink_installed: AtomicBool::new(false),
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
                format!("Failed to bind Unix socket: {e}"),
            )
        })?;

        #[cfg(unix)]
        {
            let installed = crate::rpc::socket_config::install_storage_capability_symlink(
                &self.socket_path,
                &self.family_id,
            );
            self.storage_capability_symlink_installed
                .store(installed, Ordering::SeqCst);
        }

        info!("═══════════════════════════════════════════════════════════");
        info!("NestGate ready!");
        info!("   Socket: {}", self.socket_path.display());
        info!("   Family: {}", self.family_id);
        info!("   Protocol: JSON-RPC 2.0 over Unix socket");
        info!("═══════════════════════════════════════════════════════════");
        info!(
            "Test with: echo '{{\"jsonrpc\":\"2.0\",\"method\":\"storage.list\",\"id\":1}}' | nc -U {}",
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
        #[cfg(unix)]
        crate::rpc::socket_config::remove_storage_capability_symlink(
            &self.socket_path,
            &self.family_id,
            self.storage_capability_symlink_installed
                .load(Ordering::SeqCst),
        );
        // Clean up socket file
        if self.socket_path.exists()
            && let Err(e) = std::fs::remove_file(&self.socket_path)
        {
            warn!("Failed to remove socket file: {}", e);
        }
    }
}

/// Handle a single Unix socket connection.
///
/// When BTSP is required (production), runs the 4-step handshake before
/// entering the JSON-RPC dispatch loop. Development connections proceed
/// directly.
async fn handle_connection(stream: UnixStream, state: Arc<StorageState>) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut raw_reader = BufReader::new(reader);

    if crate::rpc::btsp_server_handshake::is_btsp_required() {
        // Peek the first byte via BufReader: `{` (0x7B) means plain
        // JSON-RPC from biomeOS composition; anything else triggers
        // BTSP handshake.
        use tokio::io::AsyncBufReadExt;
        let is_json_rpc = match raw_reader.fill_buf().await {
            Ok(buf) if !buf.is_empty() => buf[0] == b'{',
            _ => false,
        };

        if is_json_rpc {
            tracing::debug!("BTSP: first byte is '{{', bypassing handshake (JSON-RPC)");
            return json_rpc_loop(&mut raw_reader, &mut writer, &state).await;
        }

        let family_id = state.family_id.as_deref().unwrap_or("default").to_string();

        let _session = crate::rpc::btsp_server_handshake::perform_handshake(
            &mut raw_reader,
            &mut writer,
            &family_id,
        )
        .await?;

        return json_rpc_loop(&mut raw_reader, &mut writer, &state).await;
    }

    json_rpc_loop(&mut raw_reader, &mut writer, &state).await
}

/// Persistent newline-delimited JSON-RPC read/dispatch/write loop (keep-alive).
///
/// Reads requests until the client disconnects (EOF). Each request is
/// dispatched, the response written, and the writer flushed before reading
/// the next request. This allows multiple sequential calls on a single
/// connection — the pattern required by ecosystem composition parity
/// (e.g. `storage.store` then `storage.retrieve` on the same socket).
///
/// Shared between the BTSP-authenticated and development (plaintext) code paths.
///
/// Maximum idle time before a keep-alive connection is closed. The timer
/// resets on every successful request so active connections are never reaped.
const CONNECTION_IDLE_LIMIT: std::time::Duration = std::time::Duration::from_secs(300);

/// Event-driven JSON-RPC keep-alive loop.
///
/// Uses `tokio::select!` to multiplex between I/O readiness and a resettable
/// idle timer. On idle expiry the client receives a `connection.closing`
/// notification before the socket is torn down.
async fn json_rpc_loop<R, W>(reader: &mut R, writer: &mut W, state: &StorageState) -> Result<()>
where
    R: tokio::io::AsyncBufReadExt + Unpin,
    W: tokio::io::AsyncWriteExt + Unpin,
{
    let mut line = Vec::new();
    let mut requests_served: u64 = 0;

    let idle_timer = tokio::time::sleep(CONNECTION_IDLE_LIMIT);
    tokio::pin!(idle_timer);

    loop {
        line.clear();

        tokio::select! {
            result = reader.read_until(b'\n', &mut line) => {
                match result {
                    Ok(0) => break,
                    Ok(_) => {
                        idle_timer.as_mut().reset(
                            tokio::time::Instant::now() + CONNECTION_IDLE_LIMIT,
                        );

                        let trimmed = line.as_slice().trim_ascii();
                        if trimmed.is_empty() {
                            continue;
                        }

                        requests_served += 1;
                        debug!("Received request: {}", String::from_utf8_lossy(trimmed));

                        let response = match serde_json::from_slice::<JsonRpcRequest>(trimmed) {
                            Ok(request) => handle_request(request, state).await,
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

                        let response_bytes: Bytes = serde_json::to_vec(&response)
                            .map(Bytes::from)
                            .map_err(|e| {
                                NestGateError::api(format!("Failed to serialize response: {e}"))
                            })?;

                        writer
                            .write_all(&response_bytes)
                            .await
                            .map_err(|e| {
                                NestGateError::io_error(format!("Failed to write response: {e}"))
                            })?;
                        writer.write_all(b"\n").await.map_err(|e| {
                            NestGateError::io_error(format!("Failed to write newline: {e}"))
                        })?;
                        writer.flush().await.map_err(|e| {
                            NestGateError::io_error(format!("Failed to flush response: {e}"))
                        })?;

                        debug!("Sent response ({} bytes)", response_bytes.len());
                    }
                    Err(e) => {
                        return Err(NestGateError::io_error(format!(
                            "Failed to read request: {e}"
                        )));
                    }
                }
            }
            () = &mut idle_timer => {
                debug!(
                    requests_served,
                    idle_secs = CONNECTION_IDLE_LIMIT.as_secs(),
                    "Connection idle — sending close notification"
                );
                let notification = json!({
                    "jsonrpc": "2.0",
                    "method": "connection.closing",
                    "params": {
                        "reason": "idle",
                        "idle_timeout_secs": CONNECTION_IDLE_LIMIT.as_secs(),
                        "requests_served": requests_served
                    }
                });
                let bytes: Bytes = serde_json::to_vec(&notification)
                    .map(Bytes::from)
                    .unwrap_or_default();
                let _ = writer.write_all(&bytes).await;
                let _ = writer.write_all(b"\n").await;
                let _ = writer.flush().await;
                break;
            }
        }
    }

    debug!(requests_served, "Connection closed");
    Ok(())
}

/// Bridges the full ecosystem JSON-RPC dispatch table to [`crate::rpc::isomorphic_ipc::RpcHandler`]
/// for [`crate::rpc::isomorphic_ipc::IsomorphicIpcServer`] (same methods as [`JsonRpcUnixServer`]).
pub struct LegacyUnixJsonRpcHandler {
    state: Arc<StorageState>,
}

impl LegacyUnixJsonRpcHandler {
    /// Create a handler backed by the given storage/template/audit state.
    #[must_use]
    pub(crate) const fn new(state: Arc<StorageState>) -> Self {
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

/// Build the same JSON-RPC handler surface as [`JsonRpcUnixServer`].
///
/// Intended for [`crate::rpc::isomorphic_ipc::IsomorphicIpcServer`] using ecosystem paths from
/// [`crate::rpc::socket_config::SocketConfig`].
///
/// # Errors
///
/// Returns [`NestGateError`] if the storage backend used by the handler cannot be initialized.
pub fn legacy_ecosystem_rpc_handler(
    family_id: impl Into<String>,
    encryption: Option<Arc<crate::rpc::storage_encryption::StorageEncryption>>,
) -> Result<Arc<dyn crate::rpc::isomorphic_ipc::RpcHandler>> {
    let mut state = StorageState::new()?;
    state.family_id = Some(family_id.into());
    state.encryption = encryption;
    Ok(Arc::new(LegacyUnixJsonRpcHandler::new(Arc::new(state))))
}

#[cfg(test)]
mod tests;
