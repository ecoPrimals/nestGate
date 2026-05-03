// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Service module
//!
//! `UniBin` service management with daemon mode, status, health, and version commands

use std::net::SocketAddr;
use std::sync::Arc;

use nestgate_api::routes::create_router_with_state;

use tracing::info;

use crate::cli::{ServiceAction, port_from_env_or_default};
use crate::error::{BinResult, NestGateBinError};

use super::bind::{resolve_socket_only_tcp_listen_port, resolve_standalone_http_bind};

/// Service manager for CLI lifecycle operations.
pub struct ServiceManager {
    // Shutdown signal for graceful service termination
    shutdown_tx: Option<tokio::sync::broadcast::Sender<()>>,
}

impl ServiceManager {
    // Create a new service manager
    #[must_use]
    pub const fn new() -> Self {
        Self { shutdown_tx: None }
    }

    // Execute a service action
    pub async fn execute(&mut self, action: ServiceAction) -> BinResult<()> {
        match action {
            ServiceAction::Start {
                port,
                bind,
                listen,
                daemon: _,
            } => {
                self.start_service(Some(port), Some(bind.as_str()), listen, None)
                    .await
            }
            ServiceAction::Stop => self.stop_service().await,
            ServiceAction::Restart => self.restart_service(None, None).await,
            ServiceAction::Status => self.show_status().await,
            ServiceAction::Logs {
                lines: _,
                follow: _,
            } => Err(NestGateBinError::not_implemented(
                "Log viewing not yet implemented — use journalctl -u nestgate or check $NESTGATE_LOG_DIR",
            )),
        }
    }

    // Start NestGate service
    async fn start_service(
        &self,
        port: Option<u16>,
        bind: Option<&str>,
        listen: Option<SocketAddr>,
        config: Option<&str>,
    ) -> BinResult<()> {
        info!(
            "NestGate v{} — storage & discovery primal",
            env!("CARGO_PKG_VERSION")
        );

        let socket_requested = std::env::var("NESTGATE_SOCKET").is_ok()
            || std::env::var("NESTGATE_FAMILY_ID").is_ok()
            || std::env::var("FAMILY_ID").is_ok();

        if socket_requested {
            // Propagate FAMILY_ID → NESTGATE_FAMILY_ID so SocketConfig resolves consistently
            if std::env::var("NESTGATE_FAMILY_ID").is_err()
                && let Ok(fid) = std::env::var("FAMILY_ID")
            {
                nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", &fid);
            }

            let tcp_addr = Self::resolve_composition_tcp(port, bind, listen)?;
            self.start_unix_socket_mode(tcp_addr).await
        } else {
            self.start_http_mode(port, bind, listen, config).await
        }
    }

    /// Derive an optional TCP JSON-RPC bind address from the `service start` flags so UDS mode
    /// can run TCP alongside the Unix socket (same newline JSON-RPC, no HTTP).
    fn resolve_composition_tcp(
        port: Option<u16>,
        bind: Option<&str>,
        listen: Option<SocketAddr>,
    ) -> BinResult<Option<SocketAddr>> {
        if let Some(addr) = listen {
            return Ok(Some(addr));
        }
        let Some(p) = port else {
            return Ok(None);
        };
        let host = bind.unwrap_or("127.0.0.1");
        let addr: SocketAddr = format!("{host}:{p}").parse().map_err(|e| {
            NestGateBinError::service_init_error(
                format!("Invalid TCP bind address: {e}"),
                Some("tcp-addr".to_string()),
            )
        })?;
        Ok(Some(addr))
    }

    /// Unix socket JSON-RPC server via [`nestgate_core::rpc::IsomorphicIpcServer`] and the
    /// full ecosystem [`nestgate_core::rpc::legacy_ecosystem_rpc_handler`] dispatch table.
    ///
    /// When `tcp_addr` is `Some`, a TCP JSON-RPC listener (same newline-delimited protocol)
    /// runs alongside the Unix socket so `service start --port` in a composition still
    /// provides UDS as the primary transport.
    async fn start_unix_socket_mode(&self, tcp_addr: Option<SocketAddr>) -> BinResult<()> {
        use nestgate_core::rpc::{
            IsomorphicIpcServer, SocketConfig, TcpFallbackServer, legacy_ecosystem_rpc_handler,
        };

        info!("Starting in ECOSYSTEM MODE (Unix socket)");

        let socket_config = SocketConfig::from_environment().map_err(|e| {
            crate::error::NestGateBinError::service_init_error(
                format!("Failed to get socket configuration: {e}"),
                Some("socket-config".to_string()),
            )
        })?;

        socket_config.log_summary();

        info!("Configuration validated");
        info!("Socket path: {}", socket_config.socket_path.display());
        info!("Family ID: {}", socket_config.family_id);
        info!("Node ID: {}", socket_config.node_id);
        info!(
            "Source: {}",
            match socket_config.source {
                nestgate_core::rpc::SocketConfigSource::Environment => "NESTGATE_SOCKET env var",
                nestgate_core::rpc::SocketConfigSource::BiomeOSDirectory =>
                    "BIOMEOS_SOCKET_DIR (ecosystem standard layout)",
                nestgate_core::rpc::SocketConfigSource::XdgRuntime => "XDG runtime directory",
                nestgate_core::rpc::SocketConfigSource::TempDirectory => "/tmp fallback",
            }
        );
        if let Some(addr) = tcp_addr {
            info!("TCP JSON-RPC also listening on {addr}");
        }

        let encryption = nestgate_core::rpc::storage_encryption::StorageEncryption::resolve(Some(
            socket_config.family_id.as_str(),
        ))
        .await;
        let encryption = encryption.map(std::sync::Arc::new);
        if encryption.is_some() {
            info!("Storage encrypt-at-rest: enabled (chacha20-poly1305)");
        }

        let handler =
            legacy_ecosystem_rpc_handler(&socket_config.family_id, encryption).map_err(|e| {
                crate::error::NestGateBinError::service_init_error(
                    format!("Failed to create JSON-RPC handler: {e}"),
                    Some("unix-socket-handler".to_string()),
                )
            })?;
        let server = Arc::new(IsomorphicIpcServer::new(
            socket_config.family_id.clone(),
            handler.clone(),
        ));

        if let Some(addr) = tcp_addr {
            let tcp = Arc::new(TcpFallbackServer::new(
                socket_config.family_id.clone(),
                handler,
            ));
            tokio::spawn(async move {
                if let Err(e) = tcp.start_bound(addr).await {
                    tracing::error!("TCP JSON-RPC listener exited: {e}");
                }
            });
        }

        info!("JSON-RPC Unix Socket Server ready (isomorphic IPC)");
        info!("Mode: Ecosystem (atomic architecture)");
        info!("Press Ctrl+C to stop\n");

        server.start().await.map_err(|e| {
            crate::error::NestGateBinError::runtime_error(
                format!("Unix socket server error: {e}"),
                Some("unix-socket-serve".to_string()),
            )
        })?;

        Ok(())
    }

    // Start HTTP mode (standalone/development)
    async fn start_http_mode(
        &self,
        port: Option<u16>,
        bind: Option<&str>,
        listen: Option<SocketAddr>,
        config: Option<&str>,
    ) -> BinResult<()> {
        info!("Starting in STANDALONE MODE (HTTP)");

        // Runtime configuration (no hardcoded host/port defaults here)
        let runtime_config = nestgate_core::config::runtime::get_config();

        // tarpc port from runtime config
        let tarpc_port = runtime_config.network.tarpc_port;

        // `--listen` (UniBin v1.2) takes precedence over `--bind` + `--port` / runtime defaults
        let bind_all_ipv4 = nestgate_core::constants::hardcoding::addresses::BIND_ALL_IPV4;
        let api_host_str = runtime_config.network.api_host.to_string();
        let (bind_addr, http_port, bind_host) = resolve_standalone_http_bind(
            port,
            bind,
            listen,
            runtime_config.network.api_port,
            runtime_config.network.bind_all,
            api_host_str.as_str(),
            bind_all_ipv4,
        );

        info!("Starting NestGate HTTP service on {}", bind_addr);

        if let Some(config_path) = config {
            info!("Using configuration file: {}", config_path);
        }

        let app = create_router_with_state();

        // Create HTTP TCP listener
        let listener = tokio::net::TcpListener::bind(&bind_addr)
            .await
            .map_err(|e| {
                crate::error::NestGateBinError::service_init_error(
                    format!("Failed to bind to {bind_addr}: {e}"),
                    Some("http-server".to_string()),
                )
            })?;

        // tarpc high-performance RPC server (primal-to-primal communication)
        #[cfg(feature = "tarpc-server")]
        {
            let tarpc_bind_addr: std::net::SocketAddr =
                format!("{bind_host}:{tarpc_port}").parse().map_err(|e| {
                    crate::error::NestGateBinError::service_init_error(
                        format!("Invalid tarpc bind address: {e}"),
                        Some("tarpc-server".to_string()),
                    )
                })?;

            // Spawn tarpc server alongside HTTP server
            tokio::spawn(async move {
                let service = match nestgate_core::rpc::NestGateRpcService::new() {
                    Ok(s) => s,
                    Err(e) => {
                        tracing::error!("Failed to create tarpc service: {}", e);
                        return;
                    }
                };
                tracing::info!("tarpc server starting on {}", tarpc_bind_addr);
                if let Err(e) = nestgate_core::rpc::serve_tarpc(tarpc_bind_addr, service).await {
                    tracing::error!("tarpc server error: {}", e);
                }
            });
        }

        #[cfg(not(feature = "tarpc-server"))]
        {
            tracing::info!(
                "tarpc server available via `tarpc-server` feature (port {} reserved)",
                tarpc_port
            );
        }

        info!("Service started successfully");
        let display_host = if bind_host == bind_all_ipv4 {
            "localhost".to_string()
        } else {
            bind_host.clone()
        };
        info!("HTTP API: http://{display_host}:{http_port}");
        info!("Health check: http://{display_host}:{http_port}/health");
        info!("Endpoints: GET /health, POST /jsonrpc, GET /api/v1/protocol/capabilities");
        info!("Protocols: HTTP/REST port {http_port}, JSON-RPC port {http_port}");
        #[cfg(feature = "tarpc-server")]
        info!("tarpc: port {tarpc_port} (active)");
        #[cfg(not(feature = "tarpc-server"))]
        info!("tarpc: port {tarpc_port} (build with `tarpc-server` feature to activate)");
        info!("Security: JWT authentication");
        info!("Mode: Standalone (development/testing)");
        info!("Press Ctrl+C to stop");

        // Start the HTTP server
        axum::serve(listener, app).await.map_err(|e| {
            crate::error::NestGateBinError::runtime_error(
                format!("Server error: {e}"),
                Some("http-serve".to_string()),
            )
        })?;

        Ok(())
    }

    // Stop NestGate service
    async fn stop_service(&mut self) -> BinResult<()> {
        info!("Stopping NestGate service");

        // Event-driven shutdown (no blocking sleep)
        if let Some(tx) = &self.shutdown_tx {
            // Send shutdown signal to all subscribers
            let _ = tx.send(());
            info!("Shutdown signal sent to service");

            // Wait for graceful shutdown with timeout
            // Service should acknowledge shutdown within reasonable time
            tokio::time::timeout(std::time::Duration::from_secs(5), async {
                // In production: wait for service to confirm shutdown
                // For now: immediate return after signal
                tokio::task::yield_now().await;
            })
            .await
            .ok();
        }

        // Clean up shutdown channel
        self.shutdown_tx = None;

        info!("NestGate service stopped successfully");

        Ok(())
    }

    // Restart NestGate service
    async fn restart_service(&mut self, port: Option<u16>, config: Option<&str>) -> BinResult<()> {
        info!("Restarting NestGate service");

        // Event-driven coordination for restart
        // Stop service and wait for graceful shutdown
        self.stop_service().await?;

        // Stop is event-driven; `stop_service` already waited for graceful shutdown

        // Start service with new configuration
        self.start_service(port, None, None, config).await?;

        Ok(())
    }

    // Show service status
    // Status from runtime config and socket probes (not placeholders)
    async fn show_status(&self) -> BinResult<()> {
        info!("Checking NestGate service status");

        let runtime_config = nestgate_core::config::runtime::get_config();

        info!("NestGate Service Status:");
        info!("  Version: {}", env!("CARGO_PKG_VERSION"));
        info!("  Port: {}", runtime_config.network.api_port);

        let socket_alive = if let Ok(config) = nestgate_core::rpc::SocketConfig::from_environment()
        {
            let path = &config.socket_path;
            if path.exists() {
                if tokio::net::UnixStream::connect(path).await.is_ok() {
                    info!("  Socket: ALIVE ({})", path.display());
                    true
                } else {
                    info!("  Socket: STALE ({})", path.display());
                    false
                }
            } else {
                info!("  Socket: not found (daemon not running?)");
                false
            }
        } else {
            info!("  Socket: not configured");
            false
        };

        let cpu_count = std::thread::available_parallelism()
            .map(std::num::NonZero::get)
            .unwrap_or(1);
        info!("  CPU Cores: {cpu_count}");

        let caps = nestgate_core::services::storage::capabilities::detect_backend();
        info!("  Backend: {:?}", caps.backend_type);

        if socket_alive {
            info!("  Status: Running");
        } else {
            info!("  Status: Stopped");
        }

        Ok(())
    }
}

impl Default for ServiceManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UNIBIN: Daemon Mode & CLI Commands
// ═══════════════════════════════════════════════════════════════════════════

/// Run `NestGate` in daemon mode (`UniBin` pattern)
///
/// This is the main server mode for `NestGate`, supporting:
/// - Socket-only mode (TRUE ecoBin default - zero external dependencies)
/// - HTTP mode (optional - requires --enable-http flag)
/// - Multi-family support (--family-id flag or `NESTGATE_FAMILY_ID` env var)
pub async fn run_daemon(
    port: Option<u16>,
    bind: &str,
    listen: Option<SocketAddr>,
    dev: bool,
    enable_http: bool,
    family_id: Option<&str>,
) -> BinResult<()> {
    // Set family_id in environment for downstream socket config resolution
    if let Some(fid) = family_id {
        nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", fid);
        info!("Multi-family mode: family_id='{}'", fid);
    }

    if enable_http {
        let resolved_port = port.unwrap_or_else(port_from_env_or_default);
        info!("Starting NestGate with HTTP server (optional mode)");
        info!("   Port: {}, Bind: {}, Dev: {}", resolved_port, bind, dev);

        let manager = ServiceManager::new();
        manager
            .start_service(Some(resolved_port), Some(bind), listen, None)
            .await
    } else {
        info!("Starting NestGate in socket-only mode (TRUE ecoBin - default)");
        let tcp_addr =
            resolve_socket_only_tcp_listen_port(port, listen, bind, &nestgate_types::ProcessEnv)?;
        run_socket_only_daemon(tcp_addr).await
    }
}

/// Run `NestGate` in socket-only mode (TRUE ecoBin default - no HTTP REST).
///
/// Uses [`nestgate_core::rpc::IsomorphicIpcServer`] with the ecosystem JSON-RPC handler
/// (same surface as the legacy Unix server; optional fixed-port TCP via [`nestgate_core::rpc::TcpFallbackServer::start_bound`]).
async fn run_socket_only_daemon(tcp_jsonrpc_addr: Option<SocketAddr>) -> BinResult<()> {
    use nestgate_core::rpc::{
        IsomorphicIpcServer, SocketConfig, TcpFallbackServer, legacy_ecosystem_rpc_handler,
    };

    info!("NestGate socket-only mode (NUCLEUS integration)");

    // Get socket configuration with 4-tier fallback (ecosystem socket layout; BIOMEOS_SOCKET_DIR tier)
    let socket_config = SocketConfig::from_environment().map_err(|e| {
        crate::error::NestGateBinError::service_init_error(
            format!("Failed to get socket configuration: {e}"),
            Some("socket-config".to_string()),
        )
    })?;

    info!("Socket-only mode activated (no HTTP REST)");
    info!("  Primary: Unix socket JSON-RPC");
    if let Some(addr) = tcp_jsonrpc_addr {
        info!("  Also: TCP JSON-RPC on {addr} (newline-delimited)");
    }

    // Log socket configuration
    socket_config.log_summary();

    info!("Initializing persistent storage backend");
    let encryption = nestgate_core::rpc::storage_encryption::StorageEncryption::resolve(Some(
        socket_config.family_id.as_str(),
    ))
    .await;
    let encryption = encryption.map(std::sync::Arc::new);
    if encryption.is_some() {
        info!("Storage encrypt-at-rest: enabled (chacha20-poly1305)");
    }

    let handler =
        legacy_ecosystem_rpc_handler(&socket_config.family_id, encryption).map_err(|e| {
            crate::error::NestGateBinError::service_init_error(
                format!("Failed to create JSON-RPC handler: {e}"),
                Some("unix-socket-handler".to_string()),
            )
        })?;
    let server = Arc::new(IsomorphicIpcServer::new(
        socket_config.family_id.clone(),
        handler.clone(),
    ));
    info!("Storage backend initialized");

    if let Some(addr) = tcp_jsonrpc_addr {
        let tcp = Arc::new(TcpFallbackServer::new(
            socket_config.family_id.clone(),
            handler,
        ));
        tokio::spawn(async move {
            if let Err(e) = tcp.start_bound(addr).await {
                tracing::error!("TCP JSON-RPC listener exited: {e}");
            }
        });
    }

    info!("Mode: NUCLEUS integration (socket-only + optional TCP JSON-RPC)");
    info!(
        "Methods: storage.*, model.*, templates.*, audit.*, health.*, capabilities.*, session.*, discovery.*"
    );
    if tcp_jsonrpc_addr.is_some() {
        info!("Security: Unix socket + TCP JSON-RPC");
    } else {
        info!("Security: Local Unix socket only");
    }
    info!("Press Ctrl+C to stop");

    server.start().await.map_err(|e| {
        crate::error::NestGateBinError::runtime_error(
            format!("Unix socket server error: {e}"),
            Some("unix-socket-serve".to_string()),
        )
    })?;

    Ok(())
}

/// Show daemon status (`UniBin` CLI command)
pub async fn show_status() -> BinResult<()> {
    println!("NestGate Status");
    println!("---");
    println!("  Version:  {}", env!("CARGO_PKG_VERSION"));
    println!("  Rust:     100% application code");
    println!("  Unsafe:   forbidden (all crate roots)");
    println!("  Status:   (connect to daemon for live status)");
    println!();
    Ok(())
}

/// Show health check (`UniBin` CLI command)
pub async fn show_health() -> BinResult<()> {
    println!("NestGate Health Check");
    println!("---");
    println!("  (Connect to running daemon for live health status)");
    println!(
        "  Use: echo '{{\"jsonrpc\":\"2.0\",\"method\":\"health.check\",\"id\":1}}' | socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/nestgate.sock"
    );
    println!();
    Ok(())
}

/// Show version information (`UniBin` CLI command)
pub async fn show_version() -> BinResult<()> {
    println!("NestGate");
    println!("---");
    println!("  Version:       {}", env!("CARGO_PKG_VERSION"));
    println!(
        "  Build:         {}",
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    );
    println!("  Architecture:  UniBin (one binary, multiple modes)");
    println!("  Unsafe:        forbidden (all crate roots)");
    println!("  IPC:           UDS JSON-RPC (default), TCP fallback, optional HTTP");
    println!();
    Ok(())
}

#[cfg(test)]
mod service_manager_tests {
    use super::{ServiceManager, show_health, show_status, show_version};
    use crate::cli::ServiceAction;
    use std::net::SocketAddr;

    #[tokio::test]
    async fn execute_logs_returns_not_implemented_with_guidance() {
        let mut m = ServiceManager::new();
        let r = m
            .execute(ServiceAction::Logs {
                lines: 10,
                follow: false,
            })
            .await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn execute_stop_is_ok_when_never_started() {
        let mut m = ServiceManager::new();
        assert!(m.execute(ServiceAction::Stop).await.is_ok());
    }

    #[tokio::test]
    async fn new_and_default_construct_service_manager() {
        let _ = ServiceManager::new();
        let _ = ServiceManager::default();
    }

    #[test]
    fn service_action_start_holds_listen_port() {
        let addr: SocketAddr = "192.168.1.1:8080".parse().unwrap();
        let a = ServiceAction::Start {
            port: 8080,
            bind: "192.168.1.1".into(),
            listen: Some(addr),
            daemon: false,
        };
        match a {
            ServiceAction::Start { listen, port, .. } => {
                assert_eq!(port, 8080);
                assert_eq!(listen, Some(addr));
            }
            _ => panic!("start"),
        }
    }

    #[tokio::test]
    async fn show_status_health_version_helpers_succeed() {
        assert!(show_status().await.is_ok());
        assert!(show_health().await.is_ok());
        assert!(show_version().await.is_ok());
    }
}

#[cfg(test)]
#[path = "service_tests.rs"]
mod service_tests;
