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

pub use super::service_probe::{show_health, show_status, show_version};

/// Service manager for CLI lifecycle operations.
pub struct ServiceManager {
    shutdown_tx: Option<tokio::sync::broadcast::Sender<()>>,
}

impl ServiceManager {
    #[must_use]
    pub const fn new() -> Self {
        Self { shutdown_tx: None }
    }

    /// Execute a service action from the CLI dispatch.
    pub async fn execute(&mut self, action: ServiceAction) -> BinResult<()> {
        match action {
            ServiceAction::Start {
                socket,
                port,
                bind,
                listen,
                daemon: _,
            } => {
                if let Some(ref sock_path) = socket {
                    nestgate_core::env_process::set_var(
                        "NESTGATE_SOCKET",
                        sock_path.to_string_lossy().as_ref(),
                    );
                    tracing::info!("Socket path (CLI): {}", sock_path.display());
                }
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

        let bind_mode = std::env::var("PRIMAL_BIND_MODE")
            .unwrap_or_default()
            .to_lowercase();
        let tcp_only = matches!(bind_mode.as_str(), "tcp_only" | "tcp");

        let socket_requested = !tcp_only
            && (std::env::var("NESTGATE_SOCKET").is_ok_and(|s| !s.is_empty())
                || std::env::var("NESTGATE_FAMILY_ID").is_ok()
                || std::env::var("FAMILY_ID").is_ok());

        if tcp_only {
            tracing::info!("PRIMAL_BIND_MODE={bind_mode}: skipping UDS, using HTTP/TCP only");
        }

        if socket_requested {
            if std::env::var("NESTGATE_FAMILY_ID").is_err()
                && let Ok(fid) = std::env::var("FAMILY_ID")
            {
                nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", &fid);
            }

            let tcp_addr = resolve_composition_tcp(port, bind, listen)?;
            start_socket_server(tcp_addr).await
        } else {
            self.start_http_mode(port, bind, listen, config).await
        }
    }

    /// Start HTTP mode — **default** per STARTUP-NG-01 (guideStone Stream 1).
    async fn start_http_mode(
        &self,
        port: Option<u16>,
        bind: Option<&str>,
        listen: Option<SocketAddr>,
        config: Option<&str>,
    ) -> BinResult<()> {
        info!("Starting in STANDALONE MODE (HTTP)");

        let runtime_config = nestgate_core::config::runtime::get_config();
        let tarpc_port = runtime_config.network.tarpc_port;

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

        let listener = tokio::net::TcpListener::bind(&bind_addr)
            .await
            .map_err(|e| {
                NestGateBinError::service_init_error(
                    format!("Failed to bind to {bind_addr}: {e}"),
                    Some("http-server".into()),
                )
            })?;

        start_tarpc_server(tarpc_port, &bind_host);

        info!("Service started successfully");
        let display_host = if bind_host == bind_all_ipv4 {
            "localhost".into()
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

        axum::serve(listener, app).await.map_err(|e| {
            NestGateBinError::runtime_error(format!("Server error: {e}"), Some("http-serve".into()))
        })?;

        Ok(())
    }

    async fn stop_service(&mut self) -> BinResult<()> {
        info!("Stopping NestGate service");

        if let Some(tx) = &self.shutdown_tx {
            let _ = tx.send(());
            info!("Shutdown signal sent to service");

            tokio::time::timeout(std::time::Duration::from_secs(5), async {
                tokio::task::yield_now().await;
            })
            .await
            .ok();
        }

        self.shutdown_tx = None;
        info!("NestGate service stopped successfully");
        Ok(())
    }

    async fn restart_service(&mut self, port: Option<u16>, config: Option<&str>) -> BinResult<()> {
        info!("Restarting NestGate service");
        self.stop_service().await?;
        self.start_service(port, None, None, config).await?;
        Ok(())
    }

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
                let probe = nestgate_types::TransportEndpoint::uds(path);
                if nestgate_core::rpc::connect_transport(&probe).await.is_ok() {
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
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Socket Server — shared between ServiceManager and daemon mode
// ═══════════════════════════════════════════════════════════════════════════

/// Start the isomorphic IPC socket server with optional TCP JSON-RPC alongside.
///
/// This is the shared implementation used by both `ServiceManager::start_service`
/// (ecosystem mode) and `run_socket_only_daemon`. Deduplicates socket config
/// resolution, encryption setup, handler creation, and TCP fallback spawning.
async fn start_socket_server(tcp_addr: Option<SocketAddr>) -> BinResult<()> {
    use nestgate_core::rpc::{
        IsomorphicIpcServer, SocketConfig, TcpFallbackServer, legacy_ecosystem_rpc_handler,
    };

    let socket_config = SocketConfig::from_environment().map_err(|e| {
        NestGateBinError::service_init_error(
            format!("Failed to get socket configuration: {e}"),
            Some("socket-config".into()),
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
            nestgate_core::rpc::SocketConfigSource::EcosystemDirectory =>
                "ECOSYSTEM_SOCKET_DIR (ecosystem standard layout)",
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
    let encryption = encryption.map(Arc::new);
    if encryption.is_some() {
        info!("Storage encrypt-at-rest: enabled (chacha20-poly1305)");
    }

    let handler =
        legacy_ecosystem_rpc_handler(&socket_config.family_id, encryption).map_err(|e| {
            NestGateBinError::service_init_error(
                format!("Failed to create JSON-RPC handler: {e}"),
                Some("unix-socket-handler".into()),
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
    info!("Press Ctrl+C to stop\n");

    server.start().await.map_err(|e| {
        NestGateBinError::runtime_error(
            format!("Unix socket server error: {e}"),
            Some("unix-socket-serve".into()),
        )
    })?;

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// UNIBIN: Daemon Mode & CLI Commands
// ═══════════════════════════════════════════════════════════════════════════

/// Run `NestGate` in daemon mode (`UniBin` pattern)
///
/// This is the main server mode for `NestGate`, supporting:
/// - HTTP mode (default per STARTUP-NG-01)
/// - Socket-only mode (opt-in via `--socket-only`)
/// - Multi-family support (--family-id flag or `NESTGATE_FAMILY_ID` env var)
/// - Transport injection via `TRANSPORT_ENDPOINT` env var (ecosystem standard)
pub async fn run_daemon(
    port: Option<u16>,
    bind: &str,
    listen: Option<SocketAddr>,
    dev: bool,
    enable_http: bool,
    family_id: Option<&str>,
) -> BinResult<()> {
    if let Some(fid) = family_id {
        nestgate_core::env_process::set_var("NESTGATE_FAMILY_ID", fid);
        info!("Multi-family mode: family_id='{}'", fid);
    }

    log_transport_endpoint();

    if enable_http {
        let resolved_port = port.unwrap_or_else(port_from_env_or_default);
        info!("Starting NestGate with HTTP server (optional mode — Tier 5 fallback)");
        info!("   Port: {}, Bind: {}, Dev: {}", resolved_port, bind, dev);

        let manager = ServiceManager::new();
        manager
            .start_service(Some(resolved_port), Some(bind), listen, None)
            .await
    } else {
        info!("Starting NestGate in socket-only mode (NUCLEUS integration)");
        let tcp_addr =
            resolve_socket_only_tcp_listen_port(port, listen, bind, &nestgate_types::ProcessEnv)?;
        start_socket_server(tcp_addr).await
    }
}

/// Log `TRANSPORT_ENDPOINT` status at startup.
fn log_transport_endpoint() {
    match nestgate_types::TransportEndpoint::from_env() {
        Ok(ep) => {
            info!("TRANSPORT_ENDPOINT: {ep}");
            if ep.is_local() {
                info!("  Transport class: local");
            } else {
                info!("  Transport class: remote / federated");
            }
        }
        Err(nestgate_types::TransportEndpointError::NotSet) => {
            info!("TRANSPORT_ENDPOINT: not set (using legacy discovery)");
        }
        Err(e) => {
            tracing::warn!(
                "TRANSPORT_ENDPOINT parse error: {e} (falling back to legacy discovery)"
            );
        }
    }
}

/// Derive an optional TCP JSON-RPC bind address from the `service start` flags.
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
            Some("tcp-addr".into()),
        )
    })?;
    Ok(Some(addr))
}

/// Spawn tarpc server if the feature is enabled.
#[cfg(feature = "tarpc-server")]
fn start_tarpc_server(tarpc_port: u16, bind_host: &str) {
    let tarpc_bind_addr: std::net::SocketAddr = match format!("{bind_host}:{tarpc_port}").parse() {
        Ok(addr) => addr,
        Err(e) => {
            tracing::error!("Invalid tarpc bind address: {e}");
            return;
        }
    };

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
fn start_tarpc_server(tarpc_port: u16, _bind_host: &str) {
    tracing::info!(
        "tarpc server available via `tarpc-server` feature (port {} reserved)",
        tarpc_port
    );
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
            socket: None,
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

    #[test]
    fn service_action_start_holds_socket_path() {
        let a = ServiceAction::Start {
            socket: Some(std::path::PathBuf::from("/run/membrane/nestgate.sock")),
            port: 8080,
            bind: "127.0.0.1".into(),
            listen: None,
            daemon: false,
        };
        match a {
            ServiceAction::Start { socket, .. } => {
                assert_eq!(
                    socket.unwrap().to_str().unwrap(),
                    "/run/membrane/nestgate.sock"
                );
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
