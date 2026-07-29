// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! CLI probe commands for daemon health, status, and version display.
//!
//! These free functions connect to the running daemon via the ecosystem socket
//! and display live information. Extracted from `service.rs` for single-responsibility.

use crate::error::BinResult;

/// Show daemon status (`UniBin` CLI command)
///
/// Resolves the ecosystem socket path, probes the daemon, and displays live
/// status when reachable. Falls back to static info when the daemon is offline.
pub async fn show_status() -> BinResult<()> {
    println!("NestGate Status");
    println!("---");
    println!("  Version:  {}", env!("CARGO_PKG_VERSION"));
    println!("  Rust:     100% application code");
    println!("  Unsafe:   forbidden (all crate roots)");

    match probe_daemon("health.check").await {
        Ok((socket_path, response)) => {
            println!("  Socket:   {socket_path}");
            let status_str = response
                .get("status")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            println!("  Daemon:   ONLINE ({status_str})");
        }
        Err(DaemonProbeError::NoSocket(reason)) => {
            println!("  Socket:   not configured ({reason})");
            println!("  Daemon:   OFFLINE");
        }
        Err(DaemonProbeError::NotRunning(path)) => {
            println!("  Socket:   {path} (not listening)");
            println!("  Daemon:   OFFLINE");
        }
        Err(DaemonProbeError::RpcError(path, err)) => {
            println!("  Socket:   {path}");
            println!("  Daemon:   ERROR ({err})");
        }
    }

    println!();
    Ok(())
}

/// Show health check (`UniBin` CLI command)
///
/// Connects to the running daemon via the ecosystem socket and issues a
/// `health.check` JSON-RPC call. Displays component-level health when
/// available, or clear guidance when the daemon is unreachable.
pub async fn show_health() -> BinResult<()> {
    println!("NestGate Health Check");
    println!("---");

    match probe_daemon("health.check").await {
        Ok((socket_path, response)) => {
            println!("  Socket:  {socket_path}");
            let status = response
                .get("status")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            println!("  Status:  {status}");

            if let Some(components) = response
                .get("components")
                .and_then(serde_json::Value::as_object)
            {
                for (name, val) in components {
                    let comp_status = val
                        .as_str()
                        .or_else(|| val.get("status").and_then(serde_json::Value::as_str))
                        .unwrap_or("unknown");
                    println!("    {name}: {comp_status}");
                }
            }

            if let Some(uptime) = response
                .get("uptime_seconds")
                .and_then(serde_json::Value::as_u64)
            {
                let hours = uptime / 3600;
                let mins = (uptime % 3600) / 60;
                let secs = uptime % 60;
                println!("  Uptime:  {hours}h {mins}m {secs}s");
            }
        }
        Err(DaemonProbeError::NoSocket(reason)) => {
            println!("  Daemon not configured: {reason}");
            println!("  Start with: nestgate service start");
        }
        Err(DaemonProbeError::NotRunning(path)) => {
            println!("  Daemon not running (socket {path} unreachable)");
            println!("  Start with: nestgate service start");
        }
        Err(DaemonProbeError::RpcError(path, err)) => {
            println!("  Socket:  {path}");
            println!("  Error:   {err}");
        }
    }

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

/// Errors from attempting to probe the running daemon over the ecosystem socket.
pub(crate) enum DaemonProbeError {
    NoSocket(String),
    NotRunning(String),
    RpcError(String, String),
}

/// Probe the running daemon by resolving the socket and calling a JSON-RPC method.
async fn probe_daemon(
    method: &str,
) -> std::result::Result<(String, serde_json::Value), DaemonProbeError> {
    let socket_config = nestgate_core::rpc::SocketConfig::from_environment()
        .map_err(|e| DaemonProbeError::NoSocket(e.to_string()))?;

    let path_display = socket_config.socket_path.display().to_string();

    if !socket_config.socket_path.exists() {
        return Err(DaemonProbeError::NotRunning(path_display));
    }

    let endpoint = nestgate_types::TransportEndpoint::uds(&socket_config.socket_path);
    let mut client = nestgate_core::rpc::JsonRpcClient::connect_transport(&endpoint)
        .await
        .map_err(|e| DaemonProbeError::NotRunning(format!("{path_display} ({e})")))?;

    let result = client
        .call(method, serde_json::json!({}))
        .await
        .map_err(|e| DaemonProbeError::RpcError(path_display.clone(), e.to_string()))?;

    Ok((path_display, result))
}
