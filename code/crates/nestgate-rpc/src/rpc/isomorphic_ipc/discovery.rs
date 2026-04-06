// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! # 🔍 IPC Endpoint Discovery
//!
//! **UNIVERSAL**: Discovers Unix socket OR TCP endpoints automatically\
//! **ZERO CONFIG**: No environment variables or flags required\
//! **ADAPTIVE**: Tries optimal path first, falls back gracefully
//!
//! ## Philosophy
//!
//! Endpoint discovery is **RUNTIME DATA**, not compile-time configuration:
//! - Try Unix socket first (optimal)
//! - Fall back to TCP discovery file (adaptive)
//! - Fail only if both unavailable (real error)
//!
//! This enables **isomorphic clients** that work on all platforms without changes.
//!
//! ## Discovery Strategy
//!
//! 1. **Try Unix Socket** (optimal):
//!    - Check `XDG_RUNTIME_DIR`
//!    - Fallback to /tmp
//!    - Verify socket file exists
//!
//! 2. **Try TCP Discovery File** (fallback):
//!    - Read discovery file (XDG-compliant locations)
//!    - Parse format: `tcp:127.0.0.1:PORT`
//!    - Return TCP endpoint
//!
//! 3. **Fail** (real error):
//!    - Service not running
//!    - Discovery files not found
//!
//! ## Reference
//!
//! Pattern validated in orchestration provider v3.33.0

use anyhow::Result;
use nestgate_types::{EnvSource, ProcessEnv};
use std::net::SocketAddr;
use std::path::PathBuf;
use tracing::{debug, info};

/// IPC endpoint type (polymorphic)
///
/// Represents either a Unix socket or TCP endpoint.
/// Clients use this to connect regardless of transport.
#[derive(Debug, Clone)]
pub enum IpcEndpoint {
    /// Unix socket path (optimal)
    UnixSocket(PathBuf),
    /// TCP local address (fallback - localhost only)
    TcpLocal(SocketAddr),
}

impl IpcEndpoint {
    /// Get endpoint description (for logging)
    #[must_use]
    pub fn description(&self) -> String {
        match self {
            Self::UnixSocket(path) => format!("Unix socket: {}", path.display()),
            Self::TcpLocal(addr) => format!("TCP (localhost): {addr}"),
        }
    }

    /// Check if endpoint is Unix socket
    #[must_use]
    pub const fn is_unix_socket(&self) -> bool {
        matches!(self, Self::UnixSocket(_))
    }

    /// Check if endpoint is TCP
    #[must_use]
    pub const fn is_tcp(&self) -> bool {
        matches!(self, Self::TcpLocal(_))
    }
}

/// Discover IPC endpoint for a service
///
/// **TRIES**:
/// 1. Unix socket (optimal)
/// 2. TCP discovery file (fallback)
///
/// **ZERO CONFIG**: No environment variables or flags required
///
/// # Arguments
///
/// * `service_name` - Name of service to discover
///
/// # Returns
///
/// * `Ok(IpcEndpoint)` - Discovered endpoint (Unix or TCP)
/// * `Err(_)` - Service not found or not running
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::rpc::isomorphic_ipc::discover_ipc_endpoint;
///
/// # async fn example() -> anyhow::Result<()> {
/// let endpoint = discover_ipc_endpoint("nestgate")?;
/// println!("Discovered: {}", endpoint.description());
/// # Ok(())
/// # }
/// ```
pub fn discover_ipc_endpoint(service_name: &str) -> Result<IpcEndpoint> {
    discover_ipc_endpoint_from_env(&ProcessEnv, service_name)
}

/// Like [`discover_ipc_endpoint`], but reads `XDG_RUNTIME_DIR` / `HOME` from an injectable [`EnvSource`].
pub fn discover_ipc_endpoint_from_env(
    env: &dyn EnvSource,
    service_name: &str,
) -> Result<IpcEndpoint> {
    info!("🔍 Discovering IPC endpoint for: {}", service_name);

    // 1. Try Unix socket first (optimal)
    debug!("   Trying Unix socket discovery...");
    if let Ok(socket_path) = discover_unix_socket_from_env(env, service_name) {
        if socket_path.exists() {
            info!("✅ Discovered Unix socket: {}", socket_path.display());
            return Ok(IpcEndpoint::UnixSocket(socket_path));
        }
        debug!(
            "   Unix socket path does not exist: {}",
            socket_path.display()
        );
    }

    // 2. Try TCP discovery file (fallback)
    debug!("   Trying TCP discovery file...");
    if let Ok(endpoint) = discover_tcp_endpoint_from_env(env, service_name) {
        info!("✅ Discovered TCP endpoint: {}", endpoint.description());
        return Ok(endpoint);
    }

    Err(anyhow::anyhow!(
        "Could not discover IPC endpoint for service: {service_name} (tried Unix socket and TCP discovery file)"
    ))
}

/// Discover Unix socket path (XDG-compliant)
///
/// **Tries** (in order):
/// 1. `$XDG_RUNTIME_DIR/{service}.sock` (preferred)
/// 2. `/tmp/{service}.sock` (fallback)
///
/// # Arguments
///
/// * `service_name` - Name of service
///
/// # Returns
///
/// * `Ok(PathBuf)` - Expected socket path (may not exist yet)
/// * `Err(_)` - Could not determine path
fn discover_unix_socket_from_env(env: &dyn EnvSource, service_name: &str) -> Result<PathBuf> {
    // Try XDG_RUNTIME_DIR first (preferred)
    if let Some(runtime_dir) = env.get("XDG_RUNTIME_DIR") {
        let socket_path = PathBuf::from(format!("{runtime_dir}/{service_name}.sock"));
        debug!("   Unix socket candidate: {}", socket_path.display());
        return Ok(socket_path);
    }

    // Fallback to /tmp
    let socket_path = PathBuf::from(format!("/tmp/{service_name}.sock"));
    debug!(
        "   Unix socket candidate (fallback): {}",
        socket_path.display()
    );
    Ok(socket_path)
}

/// Discover TCP endpoint from discovery file
///
/// **Discovery file format**: `tcp:127.0.0.1:PORT`
///
/// **Locations** (tried in order):
/// 1. `$XDG_RUNTIME_DIR/{service}-ipc-port`
/// 2. `$HOME/.local/share/{service}-ipc-port`
/// 3. `/tmp/{service}-ipc-port`
///
/// # Arguments
///
/// * `service_name` - Name of service
///
/// # Returns
///
/// * `Ok(IpcEndpoint::TcpLocal)` - Discovered TCP endpoint
/// * `Err(_)` - No discovery file found
fn discover_tcp_endpoint_from_env(env: &dyn EnvSource, service_name: &str) -> Result<IpcEndpoint> {
    let discovery_files = get_tcp_discovery_file_candidates_from_env(env, service_name);

    for file in discovery_files {
        debug!("   Checking discovery file: {}", file.display());

        if let Ok(contents) = std::fs::read_to_string(&file) {
            // Parse format: tcp:127.0.0.1:PORT
            if let Some(addr_str) = contents.trim().strip_prefix("tcp:") {
                if let Ok(addr) = addr_str.parse::<SocketAddr>() {
                    debug!("✅ Parsed TCP endpoint from discovery file: {}", addr);
                    return Ok(IpcEndpoint::TcpLocal(addr));
                }
                debug!("⚠️  Invalid address format: {}", addr_str);
            } else {
                debug!("⚠️  Invalid discovery file format (expected tcp:127.0.0.1:PORT)");
            }
        }
    }

    Err(anyhow::anyhow!("No TCP discovery file found"))
}

/// Get candidate paths for TCP discovery file
///
/// **Returns** (in order of preference):
/// 1. `$XDG_RUNTIME_DIR/{service}-ipc-port`
/// 2. `$HOME/.local/share/{service}-ipc-port`
/// 3. `/tmp/{service}-ipc-port`
fn get_tcp_discovery_file_candidates_from_env(
    env: &dyn EnvSource,
    service_name: &str,
) -> Vec<PathBuf> {
    let mut candidates = Vec::new();

    // XDG_RUNTIME_DIR (preferred)
    if let Some(runtime_dir) = env.get("XDG_RUNTIME_DIR") {
        candidates.push(PathBuf::from(format!(
            "{runtime_dir}/{service_name}-ipc-port"
        )));
    }

    // HOME/.local/share (fallback)
    if let Some(home) = env.get("HOME") {
        candidates.push(PathBuf::from(format!(
            "{home}/.local/share/{service_name}-ipc-port"
        )));
    }

    // /tmp (last resort)
    candidates.push(PathBuf::from(format!("/tmp/{service_name}-ipc-port")));

    candidates
}

#[cfg(test)]
mod tests {

    use super::*;
    use nestgate_types::{MapEnv, ProcessEnv};

    #[test]
    fn test_discover_unix_socket() {
        let path = discover_unix_socket_from_env(&ProcessEnv, "nestgate").unwrap();
        let path_str = path.to_string_lossy();

        // Should contain service name
        assert!(path_str.contains("nestgate"));

        // Should end with .sock
        assert!(path_str.ends_with(".sock"));
    }

    #[test]
    fn test_discovery_file_candidates() {
        let candidates = get_tcp_discovery_file_candidates_from_env(&ProcessEnv, "nestgate");

        // Should have at least /tmp fallback
        assert!(!candidates.is_empty());

        // Last candidate should be /tmp
        let last = candidates.last().unwrap();
        assert!(last.to_string_lossy().starts_with("/tmp/"));

        // All should contain service name
        for candidate in &candidates {
            assert!(candidate.to_string_lossy().contains("nestgate"));
        }
    }

    #[test]
    fn test_endpoint_description() {
        let unix_ep = IpcEndpoint::UnixSocket(PathBuf::from("/tmp/test.sock"));
        assert!(unix_ep.description().contains("/tmp/test.sock"));

        let tcp_ep = IpcEndpoint::TcpLocal("127.0.0.1:12345".parse().unwrap());
        assert!(tcp_ep.description().contains("127.0.0.1:12345"));
    }

    #[test]
    fn test_endpoint_type_checks() {
        let unix_ep = IpcEndpoint::UnixSocket(PathBuf::from("/tmp/test.sock"));
        assert!(unix_ep.is_unix_socket());
        assert!(!unix_ep.is_tcp());

        let tcp_ep = IpcEndpoint::TcpLocal("127.0.0.1:12345".parse().unwrap());
        assert!(tcp_ep.is_tcp());
        assert!(!tcp_ep.is_unix_socket());
    }

    #[test]
    fn discover_ipc_endpoint_prefers_existing_unix_socket() {
        let dir = tempfile::tempdir().expect("tempdir");
        let sock = dir.path().join("disc_mysvc.sock");
        std::fs::write(&sock, b"x").unwrap();
        let rt = dir.path().to_string_lossy();
        let env = MapEnv::from([("XDG_RUNTIME_DIR", rt.as_ref())]);
        let ep = discover_ipc_endpoint_from_env(&env, "disc_mysvc").expect("discover");
        assert!(matches!(ep, IpcEndpoint::UnixSocket(ref p) if p == &sock));
    }

    #[test]
    fn discover_ipc_endpoint_falls_back_to_tcp_discovery_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let disc = dir.path().join("disc_tcp_svc-ipc-port");
        std::fs::write(&disc, "tcp:127.0.0.1:54321\n").unwrap();
        let rt = dir.path().to_string_lossy();
        let env = MapEnv::from([("XDG_RUNTIME_DIR", rt.as_ref())]);
        let ep = discover_ipc_endpoint_from_env(&env, "disc_tcp_svc").expect("discover");
        match ep {
            IpcEndpoint::TcpLocal(a) => assert_eq!(a.port(), 54321),
            _ => panic!("expected TCP"),
        }
    }

    #[test]
    fn discover_tcp_endpoint_skips_malformed_discovery_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        let disc = dir.path().join("bad_tcp-ipc-port");
        std::fs::write(&disc, "not-tcp-prefix\n").unwrap();
        let rt = dir.path().to_string_lossy();
        let env = MapEnv::from([("XDG_RUNTIME_DIR", rt.as_ref())]);
        let r = discover_tcp_endpoint_from_env(&env, "bad_tcp");
        assert!(r.is_err());
    }

    #[test]
    fn discover_tcp_endpoint_skips_unparseable_address() {
        let dir = tempfile::tempdir().expect("tempdir");
        let disc = dir.path().join("bad_addr-ipc-port");
        std::fs::write(&disc, "tcp:not_a_socket_addr\n").unwrap();
        let rt = dir.path().to_string_lossy();
        let env = MapEnv::from([("XDG_RUNTIME_DIR", rt.as_ref())]);
        let r = discover_tcp_endpoint_from_env(&env, "bad_addr");
        assert!(r.is_err());
    }
}
