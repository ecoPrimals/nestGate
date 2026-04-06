// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! # 🌐 TCP IPC Fallback Server
//!
//! **ISOMORPHIC**: Automatic fallback when Unix sockets unavailable\
//! **PROTOCOL**: Same JSON-RPC 2.0 as Unix sockets\
//! **SECURITY**: Localhost only (127.0.0.1), same security as Unix sockets
//!
//! ## Philosophy
//!
//! This module implements the "ADAPT" phase of Try→Detect→Adapt→Succeed:
//! - Activates when Unix sockets fail due to platform constraints
//! - Uses same JSON-RPC 2.0 protocol (transparent to clients)
//! - Binds to localhost only (same security model as Unix sockets)
//! - Writes discovery file for client auto-discovery
//!
//! ## Why TCP on Localhost?
//!
//! **Security**: TCP on localhost (127.0.0.1) provides equivalent security to Unix sockets:
//! - Only local processes can connect
//! - No network exposure
//! - Firewall-friendly
//!
//! **Compatibility**: Works on ALL platforms:
//! - Linux (when Unix sockets blocked)
//! - Android (`SELinux` restrictions)
//! - Future platforms with similar constraints
//!
//! ## Discovery System
//!
//! Writes ephemeral port to discovery file:
//! ```text
//! Format: tcp:127.0.0.1:PORT
//! Locations: $XDG_RUNTIME_DIR, $HOME/.local/share, /tmp
//! ```
//!
//! Clients discover endpoint automatically (zero configuration).
//!
//! ## Reference
//!
//! Pattern validated in orchestration provider v3.33.0

use anyhow::{Context, Result};
use nestgate_config::constants::hardcoding::addresses::LOCALHOST_IPV4;
use nestgate_types::{EnvSource, ProcessEnv, env_var_or_default};
use serde_json::Value;
use std::fs::File;
use std::future::Future;
use std::io::Write as StdWrite;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info, warn};

/// Dyn-compatible RPC handler trait for JSON-RPC 2.0 request dispatch.
///
/// Used by both Unix socket and TCP fallback servers.
pub trait RpcHandler: Send + Sync {
    /// Handle a JSON-RPC 2.0 request, returning a JSON-RPC 2.0 response.
    fn handle_request(&self, request: Value) -> Pin<Box<dyn Future<Output = Value> + Send + '_>>;
}

/// TCP fallback server for isomorphic IPC
///
/// Provides JSON-RPC 2.0 over TCP when Unix sockets are unavailable.
#[derive(Clone)]
pub struct TcpFallbackServer {
    /// Service name (for discovery file). `Arc<str>` for zero-copy sharing.
    service_name: Arc<str>,
    /// RPC handler (shared with Unix socket server)
    handler: Arc<dyn RpcHandler>,
}

impl TcpFallbackServer {
    /// Create new TCP fallback server
    ///
    /// # Arguments
    ///
    /// * `service_name` - Name of service (for discovery file)
    /// * `handler` - RPC handler (same as Unix socket handler)
    pub fn new(service_name: impl Into<Arc<str>>, handler: Arc<dyn RpcHandler>) -> Self {
        Self {
            service_name: service_name.into(),
            handler,
        }
    }

    /// Start TCP fallback server
    ///
    /// **Binds to localhost:0** (ephemeral port for security)\
    /// **Writes discovery file** (for client auto-discovery)\
    /// **Accepts connections** (same loop as Unix socket server)
    ///
    /// # Returns
    ///
    /// Never returns (runs until process termination)
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if binding the TCP listener or reading its local address fails
    /// before the accept loop starts.
    pub async fn start(self: Arc<Self>) -> Result<()> {
        info!("🌐 Starting TCP IPC fallback (isomorphic mode)");
        info!("   Protocol: JSON-RPC 2.0 (same as Unix socket)");
        info!("   Security: Localhost only (127.0.0.1)");

        // Bind address configurable via NESTGATE_IPC_BIND_ADDRESS (default: loopback from config)
        let bind_addr =
            env_var_or_default(&ProcessEnv, "NESTGATE_IPC_BIND_ADDRESS", LOCALHOST_IPV4);
        let bind_socket = format!("{bind_addr}:0");

        // Bind to configurable address:0 (ephemeral port, OS assigns)
        let listener = TcpListener::bind(&bind_socket)
            .await
            .context("Failed to bind TCP socket for IPC fallback")?;

        let local_addr = listener
            .local_addr()
            .context("Failed to get local address")?;

        info!(
            "✅ TCP IPC listening on {} (resolved from bind pattern {} — ephemeral port)",
            local_addr, bind_socket
        );

        // Write discovery file for clients
        self.write_tcp_discovery_file(&local_addr)?;

        info!("   Status: READY ✅ (isomorphic TCP fallback active)");

        self.accept_loop(listener).await
    }

    /// Start TCP JSON-RPC listener on a fixed address (e.g. `UniBin` `daemon --port` / `--listen`).
    ///
    /// Same newline-delimited JSON-RPC 2.0 protocol as [`Self::start`]. Use alongside Unix
    /// socket IPC when both transports are desired.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if binding `addr` or reading the listener's local address fails
    /// before the accept loop starts.
    pub async fn start_bound(self: Arc<Self>, addr: SocketAddr) -> Result<()> {
        info!("🌐 Starting TCP JSON-RPC listener (isomorphic IPC, fixed address)");
        info!("   Protocol: JSON-RPC 2.0 (same as Unix socket)");

        let listener = TcpListener::bind(addr)
            .await
            .with_context(|| format!("Failed to bind TCP JSON-RPC listener on {addr}"))?;

        let local_addr = listener
            .local_addr()
            .context("Failed to get local address after bind")?;

        info!(
            "✅ TCP JSON-RPC listening on {} (requested {})",
            local_addr, addr
        );

        self.write_tcp_discovery_file(&local_addr)?;

        info!("   Status: READY ✅ (TCP JSON-RPC alongside Unix socket when enabled)");

        self.accept_loop(listener).await
    }

    async fn accept_loop(self: Arc<Self>, listener: TcpListener) -> Result<()> {
        // Accept connections (same pattern as Unix socket server)
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    debug!("📥 TCP client connected: {}", addr);
                    let handler = self.clone();

                    // Spawn task for each connection
                    tokio::spawn(async move {
                        if let Err(e) = handler.handle_tcp_connection(stream).await {
                            error!("TCP connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept TCP connection: {}", e);
                }
            }
        }
    }

    /// Handle TCP connection (JSON-RPC over TCP)
    ///
    /// **Protocol**: Line-delimited JSON-RPC 2.0\
    /// **Format**: Each request/response is a single JSON line\
    /// **Same as Unix sockets**: Identical protocol for transparency
    async fn handle_tcp_connection(&self, stream: TcpStream) -> Result<()> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = Vec::new();

        loop {
            line.clear();

            let n = match reader.read_until(b'\n', &mut line).await {
                Ok(n) => n,
                Err(e) => {
                    error!("❌ TCP read error: {}", e);
                    break;
                }
            };
            if n == 0 && line.is_empty() {
                debug!("📤 TCP client disconnected (EOF)");
                break;
            }

            debug!("📥 Received {} bytes", n);

            let trimmed = line.as_slice().trim_ascii();
            if trimmed.is_empty() {
                continue;
            }

            // Parse JSON from bytes (no intermediate `String` for the line buffer).
            match serde_json::from_slice::<Value>(trimmed) {
                Ok(request) => {
                    debug!("🔍 Parsed JSON-RPC request");

                    // Handle request (same as Unix socket)
                    let response = self.handler.handle_request(request).await;

                    // Send response (line-delimited JSON)
                    let response_str = serde_json::to_string(&response)
                        .context("Failed to serialize JSON-RPC response")?;

                    writer
                        .write_all(response_str.as_bytes())
                        .await
                        .context("Failed to write response")?;

                    writer
                        .write_all(b"\n")
                        .await
                        .context("Failed to write newline")?;

                    debug!("📤 Sent response ({} bytes)", response_str.len());
                }
                Err(e) => {
                    warn!("⚠️  Invalid JSON-RPC request: {}", e);
                    // Continue to next request (don't break connection)
                }
            }
        }

        Ok(())
    }

    /// Write TCP discovery file (XDG-compliant)
    ///
    /// **Format**: `tcp:127.0.0.1:PORT`\
    /// **Locations**: Try in order:
    /// 1. `$XDG_RUNTIME_DIR/{service}-ipc-port` (preferred)
    /// 2. `$HOME/.local/share/{service}-ipc-port` (fallback)
    /// 3. `/tmp/{service}-ipc-port` (last resort)
    ///
    /// **Clients** read this file to discover TCP endpoint automatically.
    fn write_tcp_discovery_file(&self, addr: &SocketAddr) -> Result<()> {
        self.write_tcp_discovery_file_from_env_source(&ProcessEnv, addr)
    }

    /// Like [`Self::write_tcp_discovery_file`], but reads `XDG_RUNTIME_DIR` / `HOME` from `env`.
    pub(crate) fn write_tcp_discovery_file_from_env_source(
        &self,
        env: &dyn EnvSource,
        addr: &SocketAddr,
    ) -> Result<()> {
        // XDG-compliant discovery file paths (try in order)
        let discovery_dirs: [Option<String>; 3] = [
            env.get("XDG_RUNTIME_DIR"),
            env.get("HOME").map(|h| format!("{h}/.local/share")),
            Some("/tmp".to_string()),
        ];

        for dir in discovery_dirs
            .iter()
            .filter_map(|d: &Option<String>| d.as_ref())
        {
            let discovery_file = format!("{}/{}-ipc-port", dir, self.service_name);

            match File::create(&discovery_file) {
                Ok(mut f) => {
                    // Write in format: tcp:127.0.0.1:PORT
                    if let Err(e) = writeln!(f, "tcp:{addr}") {
                        warn!("⚠️  Failed to write discovery file: {}", e);
                        continue;
                    }

                    info!("📁 TCP discovery file: {}", discovery_file);
                    return Ok(());
                }
                Err(e) => {
                    debug!("⚠️  Could not create discovery file in {}: {}", dir, e);
                    // Try next directory
                }
            }
        }

        warn!("⚠️  Could not write TCP discovery file (clients may not find endpoint)");
        warn!("   Tried: XDG_RUNTIME_DIR, HOME/.local/share, /tmp");
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use nestgate_types::MapEnv;

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
    fn test_tcp_server_creation() {
        let handler = Arc::new(MockHandler);
        let server = TcpFallbackServer::new("test-service".to_string(), handler);
        // Server constructed - clone needed for spawn
        let _cloned = server.clone();
    }

    #[test]
    fn test_discovery_file_format() {
        // Discovery file should be: tcp:127.0.0.1:PORT
        let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        let expected = "tcp:127.0.0.1:12345";
        assert_eq!(format!("tcp:{}", addr), expected);
    }

    #[test]
    fn test_address_resolution() {
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        assert_eq!(addr.ip().to_string(), "127.0.0.1");
        assert_eq!(addr.port(), 0);
    }

    #[test]
    fn test_bind_address_from_env() {
        // Default when NESTGATE_IPC_BIND_ADDRESS not set
        let bind_addr =
            env_var_or_default(&ProcessEnv, "NESTGATE_IPC_BIND_ADDRESS", LOCALHOST_IPV4);
        assert!(!bind_addr.is_empty());
        assert!(bind_addr.contains('.') || bind_addr == "localhost" || bind_addr == "::1");
    }

    #[tokio::test]
    async fn test_mock_handler_response_structure() {
        let handler = MockHandler;
        let response = handler.handle_request(serde_json::json!({"id": 1})).await;
        assert_eq!(response["jsonrpc"], "2.0");
        assert!(response.get("result").is_some());
    }

    #[test]
    fn write_tcp_discovery_file_writes_tcp_prefix_line() {
        let dir = tempfile::tempdir().expect("tempdir");
        let env = MapEnv::from([("XDG_RUNTIME_DIR", dir.path().to_string_lossy().as_ref())]);
        let handler = Arc::new(MockHandler);
        let server = TcpFallbackServer::new("ng_tcp_cov".to_string(), handler);
        let addr: SocketAddr = "127.0.0.1:55055".parse().unwrap();
        server
            .write_tcp_discovery_file_from_env_source(&env, &addr)
            .expect("write");
        let path = dir.path().join("ng_tcp_cov-ipc-port");
        let contents = std::fs::read_to_string(&path).expect("read discovery");
        assert!(contents.trim().starts_with("tcp:"));
    }

    #[test]
    fn write_tcp_discovery_file_falls_back_to_home_local_share() {
        let dir = tempfile::tempdir().expect("tempdir");
        // Force the XDG_RUNTIME_DIR attempt to fail (`File::create` needs an existing parent).
        let bad_xdg = dir.path().join("missing").join("parent");
        let share = dir.path().join(".local/share");
        std::fs::create_dir_all(&share).expect("mkdir");
        let svc = "ng_tcp_home_fb_scoped";
        let env = MapEnv::from([
            ("XDG_RUNTIME_DIR", bad_xdg.to_string_lossy().as_ref()),
            ("HOME", dir.path().to_string_lossy().as_ref()),
        ]);
        let handler = Arc::new(MockHandler);
        let server = TcpFallbackServer::new(svc.to_string(), handler);
        let addr: SocketAddr = "127.0.0.1:44044".parse().unwrap();
        server
            .write_tcp_discovery_file_from_env_source(&env, &addr)
            .expect("write");
        let path = share.join(format!("{svc}-ipc-port"));
        let contents = std::fs::read_to_string(&path).expect("read discovery");
        assert!(contents.trim().starts_with("tcp:"));
    }
}
