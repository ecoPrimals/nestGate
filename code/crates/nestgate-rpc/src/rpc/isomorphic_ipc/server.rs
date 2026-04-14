// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! # 🔌 Isomorphic IPC Server
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
//! let server = IsomorphicIpcServer::new("nestgate".to_string(), handler);
//! ```
//!
//! ## Expected Behavior
//!
//! **Linux (Unix sockets work)**:
//! ```text
//! [INFO] 🔌 Starting IPC server (isomorphic mode)...
//! [INFO]    Service: nestgate
//! [INFO]    Trying Unix socket IPC (optimal)...
//! [INFO] ✅ Unix socket IPC active (optimal path)
//! ```
//!
//! **Android (Unix sockets blocked by `SELinux`)**:
//! ```text
//! [INFO] 🔌 Starting IPC server (isomorphic mode)...
//! [INFO]    Service: nestgate
//! [INFO]    Trying Unix socket IPC (optimal)...
//! [WARN] ⚠️  Unix sockets unavailable: Permission denied
//! [WARN]    Detected platform constraint, adapting...
//! [INFO] 🌐 Starting TCP IPC fallback (isomorphic mode)
//! [INFO] ✅ TCP IPC listening on 127.0.0.1:45763
//! ```
//!
//! **Real Error** (disk full, permissions, etc.):
//! ```text
//! [ERROR] ❌ Failed to start IPC server: No space left on device
//! ```
//!
//! ## Reference
//!
//! JSON-RPC method names follow wateringHole semantic naming (`health.check`,
//! `health.liveness`, `health.readiness`, `capabilities.list`, …); see
//! `SEMANTIC_METHOD_NAMING_STANDARD.md`.
//!
//! Pattern validated in orchestration provider v3.33.0

use anyhow::Result;
use bytes::Bytes;
use serde_json::Value;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use super::platform_detection::is_platform_constraint;
use super::tcp_fallback::{RpcHandler, TcpFallbackServer};

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
    ///     "nestgate".to_string(),
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

    /// Try to start Unix socket server
    ///
    /// **INTEGRATED**: Now uses the existing Unix socket infrastructure
    /// via the `UnixSocketRpcHandler` adapter.
    async fn try_unix_server(&self) -> Result<()> {
        use tokio::net::UnixListener;

        // Prefer production [`SocketConfig`] (NESTGATE_SOCKET / BIOMEOS_SOCKET_DIR / XDG / tmp) so behavior
        // matches [`crate::rpc::unix_socket_server::JsonRpcUnixServer`] and ecosystem clients.
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
                    self.prepare_socket_path(&socket_path)?;
                    (socket_path, "standalone".to_string())
                }
            };

        // Bind to Unix socket
        let listener = UnixListener::bind(&socket_path)
            .map_err(|e| anyhow::anyhow!("Failed to bind Unix socket: {e}"))?;

        info!("Unix socket bound: {}", socket_path.display());

        #[cfg(unix)]
        let _storage_capability_symlink_guard =
            crate::rpc::socket_config::StorageCapabilitySymlinkGuard::new(&socket_path, &family_id);

        // Accept connections
        loop {
            match listener.accept().await {
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
    pub(crate) async fn handle_unix_connection(
        stream: tokio::net::UnixStream,
        handler: Arc<dyn RpcHandler>,
    ) -> Result<()> {
        use tokio::io::BufReader;

        let (reader, mut writer) = stream.into_split();

        if crate::rpc::btsp_server_handshake::is_btsp_required() {
            let family_id = std::env::var("FAMILY_ID")
                .or_else(|_| std::env::var("BIOMEOS_FAMILY_ID"))
                .or_else(|_| std::env::var("NESTGATE_FAMILY_ID"))
                .unwrap_or_default();

            let mut raw_reader = tokio::io::BufReader::new(reader);
            let _session = crate::rpc::btsp_server_handshake::perform_handshake(
                &mut raw_reader,
                &mut writer,
                &family_id,
            )
            .await?;

            Self::json_rpc_keep_alive_loop(&mut raw_reader, &mut writer, &handler).await?;
            return Ok(());
        }

        let mut reader = BufReader::new(reader);
        Self::json_rpc_keep_alive_loop(&mut reader, &mut writer, &handler).await
    }

    /// Maximum idle time before a keep-alive connection is closed.
    ///
    /// The timer resets on every successful request, so active connections
    /// are never reaped. Only truly idle (half-open, abandoned) connections
    /// are affected.
    const CONNECTION_IDLE_LIMIT: std::time::Duration = std::time::Duration::from_secs(300);

    /// Event-driven JSON-RPC keep-alive loop.
    ///
    /// Uses `tokio::select!` to multiplex between I/O readiness and a
    /// resettable idle timer rather than wrapping reads in a brute-force
    /// timeout. On idle expiry the client receives a `connection.closing`
    /// JSON-RPC notification before the socket is torn down, giving it the
    /// opportunity to reconnect or flush pending work.
    async fn json_rpc_keep_alive_loop<R, W>(
        reader: &mut R,
        writer: &mut W,
        handler: &Arc<dyn RpcHandler>,
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
                                Ok(request) => handler.handle_request(request).await,
                                Err(e) => {
                                    warn!("Invalid JSON-RPC request: {}", e);
                                    serde_json::json!({
                                        "jsonrpc": "2.0",
                                        "error": {
                                            "code": -32700,
                                            "message": "Parse error",
                                            "data": { "error": e.to_string() }
                                        },
                                        "id": null
                                    })
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

    /// Get socket path (XDG-compliant)
    fn get_socket_path(&self) -> Result<std::path::PathBuf> {
        // Try XDG_RUNTIME_DIR first (preferred)
        if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
            return Ok(std::path::PathBuf::from(format!(
                "{}/{}.sock",
                runtime_dir, self.service_name
            )));
        }

        // Fallback to /tmp
        Ok(std::path::PathBuf::from(format!(
            "/tmp/{}.sock",
            self.service_name
        )))
    }

    /// Prepare socket path (create dirs, remove old socket)
    fn prepare_socket_path(&self, socket_path: &std::path::Path) -> Result<()> {
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
mod tests {

    use super::*;
    use serde_json::Value;
    use std::future::Future;
    use std::pin::Pin;

    /// Mock RPC handler for testing
    struct MockHandler;

    impl RpcHandler for MockHandler {
        fn handle_request(
            &self,
            _request: Value,
        ) -> Pin<Box<dyn Future<Output = Value> + Send + '_>> {
            Box::pin(async move {
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "result": "ok",
                    "id": 1
                })
            })
        }
    }

    #[test]
    fn test_server_creation() {
        let handler = Arc::new(MockHandler);
        let _server = IsomorphicIpcServer::new("test-service".to_string(), handler);
        // Server constructed successfully - handler is stored
    }

    #[tokio::test]
    async fn test_mock_handler_returns_valid_json_rpc() {
        let handler = MockHandler;
        let request = serde_json::json!({"jsonrpc": "2.0", "method": "test", "id": 1});
        let response = handler.handle_request(request).await;
        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(response["result"], "ok");
        assert_eq!(response["id"], 1);
    }

    #[test]
    fn test_server_creation_with_different_service_names() {
        let handler = Arc::new(MockHandler);
        let _server1 = IsomorphicIpcServer::new("nestgate".to_string(), handler.clone());
        let _server2 = IsomorphicIpcServer::new("test-svc-123".to_string(), handler);
    }

    #[tokio::test]
    async fn test_mock_handler_handles_empty_request() {
        let handler = MockHandler;
        let request = serde_json::json!({});
        let response = handler.handle_request(request).await;
        assert!(response.get("jsonrpc").is_some());
    }

    #[test]
    fn get_socket_path_ends_with_service_sock() {
        let handler = Arc::new(MockHandler);
        let server = IsomorphicIpcServer::new("svc-name-test".to_string(), handler);
        let p = server.get_socket_path().expect("path");
        assert!(
            p.to_string_lossy().ends_with("svc-name-test.sock"),
            "got {p:?}"
        );
    }

    #[test]
    fn prepare_socket_path_creates_parent_and_removes_stale_socket() {
        let dir = tempfile::tempdir().expect("tempdir");
        let sock = dir.path().join("nested").join("test.sock");
        let handler = Arc::new(MockHandler);
        let server = IsomorphicIpcServer::new("x".to_string(), handler);
        server.prepare_socket_path(&sock).expect("prepare");
        assert!(sock.parent().expect("parent").is_dir());
        std::fs::write(&sock, b"x").unwrap();
        server.prepare_socket_path(&sock).expect("prepare again");
        assert!(!sock.exists());
    }
}
