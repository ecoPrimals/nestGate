// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! # Isomorphic IPC Server
//!
//! **UNIVERSAL**: Automatically adapts to platform constraints\
//! **PATTERN**: Try→Detect→Adapt→Succeed\
//! **ZERO CONFIG**: No environment variables or flags required
//!
//! ## Philosophy
//!
//! The server should **discover its environment** and adapt automatically:
//! - Try Unix sockets first (optimal performance)
//! - Detect platform constraints (`SELinux`, lack of support)
//! - Adapt to TCP fallback (automatic, transparent)
//! - Succeed or fail with real error (clear diagnosis)
//!
//! This is **biological adaptation** - the binary learns its environment!
//!
//! ## Architecture
//!
//! ```text
//! Try→Detect→Adapt→Succeed Pattern:
//!
//! 1. TRY Unix socket server
//!    ↓
//! 2. DETECT if error is platform constraint
//!    ↓ (yes)                  ↓ (no)
//! 3. ADAPT to TCP fallback    FAIL with error
//!    ↓
//! 4. SUCCEED (Unix OR TCP)
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! // Requires RpcHandler impl; see fn new() doc for construction example
//! use nestgate_core::rpc::isomorphic_ipc::IsomorphicIpcServer;
//! let server = IsomorphicIpcServer::new("nestgate".into(), handler);
//! ```
//!
//! ## Expected Behavior
//!
//! **Linux (Unix sockets work)**:
//! ```text
//! [INFO] Starting IPC server (isomorphic mode)...
//! [INFO]    Service: nestgate
//! [INFO]    Trying Unix socket IPC (optimal)...
//! [INFO] Unix socket IPC active (optimal path)
//! ```
//!
//! **Android (Unix sockets blocked by `SELinux`)**:
//! ```text
//! [INFO] Starting IPC server (isomorphic mode)...
//! [INFO]    Service: nestgate
//! [INFO]    Trying Unix socket IPC (optimal)...
//! [WARN] Unix sockets unavailable: Permission denied
//! [WARN]    Detected platform constraint, adapting...
//! [INFO] Starting TCP IPC fallback (isomorphic mode)
//! [INFO] TCP IPC listening on 127.0.0.1:45763
//! ```
//!
//! **Real Error** (disk full, permissions, etc.):
//! ```text
//! [ERROR] Failed to start IPC server: No space left on device
//! ```
//!
//! ## Reference
//!
//! JSON-RPC method names follow wateringHole semantic naming (`health.check`,
//! `health.liveness`, `health.readiness`, `capabilities.list`, …); see
//! `SEMANTIC_METHOD_NAMING_STANDARD.md`.
//!
//! Pattern validated in orchestration provider v3.33.0

use anyhow::{Context, Result};
use bytes::Bytes;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use super::platform_detection::is_platform_constraint;
use super::tcp_fallback::{RpcHandler, TcpFallbackServer};

/// RAII guard that removes a Unix socket file and its PID sidecar on drop.
///
/// Prevents stale sockets from accumulating after crashes or normal shutdown.
/// See `CAPABILITY_BASED_DISCOVERY_STANDARD.md` v1.3.0 section 6.
struct SocketCleanupGuard {
    path: PathBuf,
}

impl Drop for SocketCleanupGuard {
    fn drop(&mut self) {
        remove_pid_file(&self.path);
        if self.path.exists() {
            if let Err(e) = std::fs::remove_file(&self.path) {
                warn!("Failed to remove socket file {}: {e}", self.path.display());
            } else {
                info!("Removed socket file: {}", self.path.display());
            }
        }
    }
}

/// Write a PID file alongside the socket (`{socket}.pid`) for liveness probing.
fn write_pid_file(socket_path: &std::path::Path) {
    let pid_path = socket_path.with_extension("pid");
    let pid = std::process::id();
    if let Err(e) = std::fs::write(&pid_path, pid.to_string()) {
        warn!("Failed to write PID file {}: {e}", pid_path.display());
    } else {
        debug!("PID file written: {} (pid {})", pid_path.display(), pid);
    }
}

/// Remove PID file sidecar for a socket path.
fn remove_pid_file(socket_path: &std::path::Path) {
    let pid_path = socket_path.with_extension("pid");
    if pid_path.exists() {
        let _ = std::fs::remove_file(&pid_path);
    }
}

/// Isomorphic IPC server (Unix socket OR TCP fallback)
///
/// **Self-adapting** server that automatically chooses transport:
/// - Tries Unix sockets first (optimal)
/// - Falls back to TCP if platform constraints detected
/// - Same protocol (JSON-RPC 2.0) on both transports
pub struct IsomorphicIpcServer {
    /// Service name (for socket paths and discovery files). `Arc<str>` for
    /// zero-copy cloning when passed to fallback transports.
    service_name: Arc<str>,
    /// RPC handler (shared between Unix and TCP servers)
    handler: Arc<dyn RpcHandler>,
}

impl IsomorphicIpcServer {
    /// Create new isomorphic IPC server
    ///
    /// # Arguments
    ///
    /// * `service_name` - Name of service (for socket paths, discovery files)
    /// * `handler` - RPC handler (must implement `RpcHandler` trait)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use nestgate_core::rpc::isomorphic_ipc::IsomorphicIpcServer;
    /// use std::sync::Arc;
    ///
    /// # async fn example(handler: Arc<dyn nestgate_core::rpc::isomorphic_ipc::RpcHandler>) {
    /// let server = Arc::new(IsomorphicIpcServer::new(
    ///     "nestgate".into(),
    ///     handler,
    /// ));
    /// # }
    /// ```
    pub fn new(service_name: impl Into<Arc<str>>, handler: Arc<dyn RpcHandler>) -> Self {
        Self {
            service_name: service_name.into(),
            handler,
        }
    }

    /// Start isomorphic IPC server (Try→Detect→Adapt→Succeed)
    ///
    /// **AUTOMATIC ADAPTATION**:
    /// - Tries Unix socket first (optimal)
    /// - Detects platform constraints (`SELinux`, lack of support)
    /// - Adapts to TCP fallback (automatic)
    /// - Succeeds or fails with clear error
    ///
    /// **NEVER RETURNS** (runs until process termination)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Never returns (server runs until process termination)
    /// * `Err(_)` - Server failed to start (real error, not platform constraint)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use nestgate_core::rpc::isomorphic_ipc::IsomorphicIpcServer;
    /// # use std::sync::Arc;
    /// # async fn example(server: Arc<IsomorphicIpcServer>) -> anyhow::Result<()> {
    /// server.start().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if the Unix socket server fails to start with a non-platform
    /// error (for example bind or socket preparation), or if TCP fallback fails after a platform
    /// constraint is detected.
    pub async fn start(self: Arc<Self>) -> Result<()> {
        info!("Starting IPC server (isomorphic mode)...");
        info!("   Service: {}", self.service_name);
        info!("   Pattern: Try→Detect→Adapt→Succeed");

        if Self::is_tcp_only_bind_mode() {
            info!("   PRIMAL_BIND_MODE=tcp/tcp_only: skipping UDS, TCP fallback only");
            return self.start_tcp_fallback().await;
        }

        #[cfg(unix)]
        {
            // 1. TRY Unix socket first (optimal)
            info!("   Trying Unix socket IPC (optimal)...");
            match self.try_unix_server().await {
                Ok(()) => {
                    info!("Unix socket IPC active (optimal path)");
                    Ok(())
                }

                // 2. DETECT platform constraints
                Err(e) if is_platform_constraint(&e) => {
                    warn!("Unix sockets unavailable: {}", e);
                    warn!("   Detected platform constraint, adapting...");

                    // 3. ADAPT to TCP fallback
                    info!("Initiating TCP IPC fallback (isomorphic mode)");
                    self.start_tcp_fallback().await
                }

                // 4. Real error (not platform constraint)
                Err(e) => {
                    error!("Failed to start IPC server: {}", e);
                    error!("   This is a real error, not a platform constraint");
                    Err(e)
                }
            }
        }
        #[cfg(not(unix))]
        {
            info!("   Unix sockets not available on this platform, using TCP fallback");
            self.start_tcp_fallback().await
        }
    }

    /// `PRIMAL_BIND_MODE=tcp_only` or `tcp` means UDS should not be attempted.
    fn is_tcp_only_bind_mode() -> bool {
        matches!(
            std::env::var("PRIMAL_BIND_MODE")
                .unwrap_or_default()
                .to_lowercase()
                .as_str(),
            "tcp_only" | "tcp"
        )
    }

    /// Try to start Unix socket server.
    ///
    /// Uses `tokio::signal::ctrl_c` so SIGINT/SIGTERM triggers graceful
    /// shutdown with socket file cleanup (prevents stale socket accumulation).
    #[cfg(unix)]
    async fn try_unix_server(&self) -> Result<()> {
        use tokio::net::UnixListener;

        let (socket_path, family_id) =
            match crate::rpc::socket_config::SocketConfig::from_environment() {
                Ok(cfg) => {
                    cfg.prepare_socket_path()
                        .map_err(|e| anyhow::anyhow!("Failed to prepare socket path: {e}"))?;
                    let fid = cfg.family_id.clone();
                    (cfg.socket_path, fid)
                }
                Err(e) => {
                    warn!(
                        "SocketConfig unavailable ({}), using legacy service-name path layout",
                        e
                    );
                    let socket_path = self.get_socket_path()?;
                    Self::prepare_socket_path(&socket_path)?;
                    (socket_path, "standalone".into())
                }
            };

        let listener = UnixListener::bind(&socket_path).context("Failed to bind Unix socket")?;

        info!("Unix socket bound: {}", socket_path.display());

        let _socket_guard = SocketCleanupGuard {
            path: socket_path.clone(),
        };

        write_pid_file(&socket_path);

        {
            let announce_socket = socket_path.clone();
            tokio::spawn(async move {
                if let Err(e) =
                    crate::rpc::primal_announce::announce_to_coordinator(&announce_socket).await
                {
                    warn!("primal.announce failed: {e}");
                }
            });
        }

        #[cfg(unix)]
        let _storage_capability_symlink_guard =
            crate::rpc::socket_config::StorageCapabilitySymlinkGuard::new(&socket_path, &family_id);

        let shutdown = tokio::signal::ctrl_c();
        tokio::pin!(shutdown);

        loop {
            tokio::select! {
                biased;
                _ = &mut shutdown => {
                    info!("Shutdown signal received, cleaning up socket");
                    break;
                }
                accept = listener.accept() => {
                    match accept {
                        Ok((stream, _addr)) => {
                            let handler = self.handler.clone();
                            tokio::spawn(async move {
                                if let Err(e) = Self::handle_unix_connection(stream, handler).await {
                                    error!("Unix connection error: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            error!("Failed to accept Unix connection: {}", e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle Unix socket connection (persistent keep-alive).
    ///
    /// Reads newline-delimited JSON-RPC requests in a loop until the client
    /// disconnects (EOF). Each response is flushed before reading the next
    /// request, enabling multi-request sessions on a single connection
    /// (e.g. `storage.store` then `storage.retrieve`).
    ///
    /// When BTSP is required (production: `FAMILY_ID` set, not `BIOMEOS_INSECURE`),
    /// the 4-step BTSP handshake runs first, delegating crypto to the security
    /// capability provider. Development connections proceed directly.
    #[cfg(unix)]
    pub(crate) async fn handle_unix_connection(
        stream: tokio::net::UnixStream,
        handler: Arc<dyn RpcHandler>,
    ) -> Result<()> {
        use tokio::io::BufReader;

        let (reader, mut writer) = stream.into_split();
        let mut raw_reader = BufReader::new(reader);

        crate::rpc::protocol::strip_ribocipher_prefix(&mut raw_reader).await;

        if crate::rpc::btsp_server_handshake::is_btsp_required() {
            // Peek buffered data. `{` may be plain JSON-RPC (biomeOS
            // composition) *or* a JSON-line BTSP ClientHello. Disambiguate:
            //   - `"jsonrpc"` / `"method"` → plain JSON-RPC, skip handshake
            //   - `"client_ephemeral_pub"` → JSON-line BTSP
            //   - non-`{` first byte → length-prefixed BTSP
            use tokio::io::AsyncBufReadExt;
            let peek = raw_reader
                .fill_buf()
                .await
                .map_or_else(|_| vec![], <[u8]>::to_vec);

            let is_plain_json_rpc = if peek.first() == Some(&b'{') {
                let s = String::from_utf8_lossy(&peek);
                s.contains("\"jsonrpc\"") || s.contains("\"method\"")
            } else {
                false
            };

            if is_plain_json_rpc {
                tracing::debug!("BTSP: peeked JSON-RPC request, bypassing handshake (restricted)");
                return Self::json_rpc_keep_alive_loop(
                    &mut raw_reader,
                    &mut writer,
                    &handler,
                    false,
                )
                .await;
            }

            debug!("BTSP: non-JSON-RPC first line detected, starting handshake");

            let family_id = std::env::var("NESTGATE_FAMILY_ID")
                .or_else(|_| std::env::var("FAMILY_ID"))
                .unwrap_or_default();

            let _session = crate::rpc::btsp_server_handshake::perform_handshake(
                &mut raw_reader,
                &mut writer,
                &family_id,
            )
            .await?;

            Self::post_handshake_phase3_or_plaintext(&mut raw_reader, &mut writer, &handler)
                .await?;
            return Ok(());
        }

        Self::json_rpc_keep_alive_loop(&mut raw_reader, &mut writer, &handler, true).await
    }

    /// After Phase 2 handshake, intercept the first message to check for
    /// `btsp.negotiate`. On success, switch to the encrypted frame loop;
    /// otherwise fall through to the plaintext keep-alive loop.
    async fn post_handshake_phase3_or_plaintext<R, W>(
        reader: &mut R,
        writer: &mut W,
        handler: &Arc<dyn RpcHandler>,
    ) -> Result<()>
    where
        R: tokio::io::AsyncBufReadExt + Unpin,
        W: tokio::io::AsyncWriteExt + Unpin,
    {
        let mut first_line = Vec::new();
        let n = reader
            .read_until(b'\n', &mut first_line)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read first post-handshake message: {e}"))?;

        if n == 0 {
            return Ok(());
        }

        let trimmed = first_line.as_slice().trim_ascii();
        if trimmed.is_empty() {
            return Self::json_rpc_keep_alive_loop(reader, writer, handler, true).await;
        }

        let parsed: Value = match serde_json::from_slice(trimmed) {
            Ok(v) => v,
            Err(_) => return Self::json_rpc_keep_alive_loop(reader, writer, handler, true).await,
        };

        let is_negotiate = parsed
            .get("method")
            .and_then(Value::as_str)
            .is_some_and(|m| m == "btsp.negotiate");

        if !is_negotiate {
            let response = handler.handle_request(parsed).await;
            let response_bytes: Bytes = serde_json::to_vec(&response).map(Bytes::from)?;
            writer.write_all(&response_bytes).await?;
            writer.write_all(b"\n").await?;
            writer.flush().await?;
            return Self::json_rpc_keep_alive_loop(reader, writer, handler, true).await;
        }

        let keys = crate::rpc::btsp_phase3::transport::try_phase3_negotiate(&parsed, writer, true)
            .await
            .map_err(|e| anyhow::anyhow!("BTSP Phase 3 negotiate failed: {e}"))?;

        let Some(session_keys) = keys else {
            return Self::json_rpc_keep_alive_loop(reader, writer, handler, true).await;
        };

        info!("BTSP Phase 3: encrypted channel established (isomorphic IPC)");

        let handler = handler.clone();
        crate::rpc::btsp_phase3::transport::run_encrypted_frame_loop(
            reader,
            writer,
            &session_keys,
            |request| {
                let h = handler.clone();
                async move { h.handle_request(request).await }
            },
        )
        .await
        .map_err(|e| anyhow::anyhow!("BTSP Phase 3 encrypted loop error: {e}"))?;

        Ok(())
    }

    const CONNECTION_IDLE_LIMIT: std::time::Duration = crate::rpc::protocol::CONNECTION_IDLE_LIMIT;

    /// Event-driven JSON-RPC keep-alive loop.
    ///
    /// Uses `tokio::select!` to multiplex between I/O readiness and a
    /// resettable idle timer rather than wrapping reads in a brute-force
    /// timeout. On idle expiry the client receives a `connection.closing`
    /// JSON-RPC notification before the socket is torn down, giving it the
    /// opportunity to reconnect or flush pending work.
    ///
    /// When `btsp_authenticated` is `false` (BTSP required but the client
    /// sent plain JSON-RPC), only BTSP-exempt methods (health, identity,
    /// capabilities) are dispatched; all others receive error -32604.
    async fn json_rpc_keep_alive_loop<R, W>(
        reader: &mut R,
        writer: &mut W,
        handler: &Arc<dyn RpcHandler>,
        btsp_authenticated: bool,
    ) -> Result<()>
    where
        R: tokio::io::AsyncBufReadExt + Unpin,
        W: tokio::io::AsyncWriteExt + Unpin,
    {
        let mut line = Vec::new();
        let mut requests_served: u64 = 0;

        let idle_timer = tokio::time::sleep(Self::CONNECTION_IDLE_LIMIT);
        tokio::pin!(idle_timer);

        loop {
            line.clear();

            tokio::select! {
                result = reader.read_until(b'\n', &mut line) => {
                    match result {
                        Ok(0) => break,
                        Ok(_) => {
                            idle_timer
                                .as_mut()
                                .reset(tokio::time::Instant::now() + Self::CONNECTION_IDLE_LIMIT);

                            let trimmed = line.as_slice().trim_ascii();
                            if trimmed.is_empty() {
                                continue;
                            }

                            requests_served += 1;

                            let response = match serde_json::from_slice::<Value>(trimmed) {
                                Ok(request) => {
                                    if btsp_authenticated {
                                        handler.handle_request(request).await
                                    } else {
                                        Self::dispatch_or_reject_unauth(request, handler)
                                            .await
                                    }
                                }
                                Err(e) => {
                                    warn!("Invalid JSON-RPC request: {}", e);
                                    {
                                        use nestgate_types::JsonRpcErrorCode;
                                        serde_json::json!({
                                            "jsonrpc": "2.0",
                                            "error": {
                                                "code": JsonRpcErrorCode::ParseError.code(),
                                                "message": JsonRpcErrorCode::ParseError.default_message(),
                                                "data": { "error": e.to_string() }
                                            },
                                            "id": null
                                        })
                                    }
                                }
                            };
                            let response_bytes: Bytes =
                                serde_json::to_vec(&response).map(Bytes::from)?;
                            writer.write_all(&response_bytes).await?;
                            writer.write_all(b"\n").await?;
                            writer.flush().await?;
                        }
                        Err(e) => {
                            error!("Unix socket read error: {}", e);
                            break;
                        }
                    }
                }
                () = &mut idle_timer => {
                    debug!(
                        requests_served,
                        idle_secs = Self::CONNECTION_IDLE_LIMIT.as_secs(),
                        "Connection idle — sending close notification"
                    );
                    let notification = serde_json::json!({
                        "jsonrpc": "2.0",
                        "method": "connection.closing",
                        "params": {
                            "reason": "idle",
                            "idle_timeout_secs": Self::CONNECTION_IDLE_LIMIT.as_secs(),
                            "requests_served": requests_served
                        }
                    });
                    if let Ok(bytes) = serde_json::to_vec(&notification) {
                        let _ = writer.write_all(&bytes).await;
                        let _ = writer.write_all(b"\n").await;
                        let _ = writer.flush().await;
                    }
                    break;
                }
            }
        }

        debug!(requests_served, "Connection closed");
        Ok(())
    }

    /// Dispatch a request on an unauthenticated (BTSP-bypassed) connection.
    ///
    /// Only BTSP-exempt methods are forwarded to the handler; everything else
    /// gets a `-32604 BTSP authentication required` error.
    async fn dispatch_or_reject_unauth(request: Value, handler: &Arc<dyn RpcHandler>) -> Value {
        let method_raw = request.get("method").and_then(Value::as_str).unwrap_or("");
        let method = crate::rpc::protocol::normalize_method(method_raw);
        if crate::rpc::is_btsp_exempt_method(&method) {
            return handler.handle_request(request).await;
        }
        let id = request.get("id").cloned().unwrap_or(Value::Null);
        warn!(
            method = method_raw,
            "Rejecting unauthenticated call to BTSP-gated method"
        );
        {
            use nestgate_types::JsonRpcErrorCode;
            serde_json::json!({
                "jsonrpc": "2.0",
                "error": {
                    "code": JsonRpcErrorCode::AuthRequired.code(),
                    "message": JsonRpcErrorCode::AuthRequired.default_message(),
                    "data": { "method": method_raw }
                },
                "id": id
            })
        }
    }

    /// Get socket path — checks `NESTGATE_SOCKET` first, then XDG, then temp dir.
    fn get_socket_path(&self) -> Result<std::path::PathBuf> {
        if let Ok(explicit) = std::env::var("NESTGATE_SOCKET") {
            return Ok(std::path::PathBuf::from(explicit));
        }

        if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
            return Ok(std::path::PathBuf::from(format!(
                "{}/{}.sock",
                runtime_dir, self.service_name
            )));
        }

        Ok(std::env::temp_dir().join(format!("{}.sock", self.service_name)))
    }

    /// Prepare socket path (create dirs, remove old socket)
    fn prepare_socket_path(socket_path: &std::path::Path) -> Result<()> {
        // Create parent directory if needed
        if let Some(parent) = socket_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Remove old socket if exists
        if socket_path.exists() {
            std::fs::remove_file(socket_path)?;
        }

        Ok(())
    }

    /// Start TCP fallback server
    ///
    /// **FALLBACK**: Activated when Unix sockets unavailable due to platform constraints
    ///
    /// **SECURITY**: Binds to localhost:0 (ephemeral port, same security as Unix sockets)
    ///
    /// **PROTOCOL**: Same JSON-RPC 2.0 as Unix sockets (transparent to clients)
    async fn start_tcp_fallback(self: Arc<Self>) -> Result<()> {
        let tcp_server = Arc::new(TcpFallbackServer::new(
            Arc::clone(&self.service_name),
            self.handler.clone(),
        ));

        tcp_server.start().await
    }
}

#[cfg(test)]
mod server_tests;
