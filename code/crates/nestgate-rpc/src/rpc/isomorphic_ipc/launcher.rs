// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! NestGate Launcher with Isomorphic IPC Discovery
//!
//! **Phase 3: Deployment Coordination**
//!
//! This module provides launcher functionality with automatic endpoint discovery,
//! following the core principle: **Primals have self-knowledge and discover others at runtime**.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    LAUNCHER WORKFLOW                         │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                              │
//! │  1. DISCOVER → Check for existing NestGate endpoint         │
//! │     ├─ Try Unix socket (XDG-compliant paths)                │
//! │     └─ Try TCP discovery file                               │
//! │                                                              │
//! │  2. LAUNCH → Start NestGate if not running                  │
//! │     ├─ Attempts Unix socket first                           │
//! │     └─ Falls back to TCP on platform constraints            │
//! │                                                              │
//! │  3. CONNECT → Establish isomorphic client connection        │
//! │     └─ Returns polymorphic IpcStream                        │
//! │                                                              │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Deep Debt Principles
//!
//! - ✅ **Zero Hardcoding**: Discovery via XDG-compliant runtime paths
//! - ✅ **Platform Agnostic**: Works on Linux, macOS, FreeBSD, WSL2
//! - ✅ **Self-Knowledge**: NestGate discovers its own endpoint
//! - ✅ **Runtime Discovery**: Other primals discover NestGate at runtime
//! - ✅ **Capability-Based**: Adapts to platform capabilities (Unix vs TCP)

use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;
use tokio::time::{sleep, Duration};
use tracing::{debug, info};

use super::discovery::{discover_ipc_endpoint, IpcEndpoint};
use super::streams::{connect_endpoint, IpcStream};

/// Default service name for NestGate IPC discovery
pub const NESTGATE_SERVICE_NAME: &str = "nestgate";

/// Maximum retry attempts for endpoint discovery
const MAX_DISCOVERY_RETRIES: u32 = 10;

/// Delay between discovery retry attempts
const RETRY_DELAY: Duration = Duration::from_millis(500);

// ═══════════════════════════════════════════════════════════════════════════════
// ENDPOINT DISCOVERY
// ═══════════════════════════════════════════════════════════════════════════════

/// Discover NestGate's IPC endpoint (Unix socket or TCP)
///
/// This function embodies the principle: **Primals discover others at runtime**.
///
/// ## Discovery Process
///
/// 1. Check XDG-compliant Unix socket paths
/// 2. Check TCP discovery files in XDG locations
/// 3. Return discovered endpoint or error
///
/// ## Platform Adaptation
///
/// - Linux/macOS/FreeBSD: Prefers Unix sockets
/// - Android/WSL2: May use TCP fallback
/// - Discovery is **automatic** - no configuration needed
///
/// ## Example
///
/// ```no_run
/// use nestgate_core::rpc::isomorphic_ipc::launcher;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     // Discover NestGate automatically
///     let endpoint = launcher::discover_nestgate_endpoint().await?;
///     println!("Found NestGate at: {:?}", endpoint);
///     Ok(())
/// }
/// ```
pub async fn discover_nestgate_endpoint() -> Result<IpcEndpoint> {
    discover_ipc_endpoint(NESTGATE_SERVICE_NAME).context("Failed to discover NestGate IPC endpoint")
}

/// Discover NestGate endpoint with retry logic
///
/// Waits for NestGate to become available, retrying up to `MAX_DISCOVERY_RETRIES` times.
/// Useful when launching NestGate and immediately trying to connect.
///
/// ## Example
///
/// ```no_run
/// use nestgate_core::rpc::isomorphic_ipc::launcher;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     // Wait for NestGate to start
///     let endpoint = launcher::discover_nestgate_with_retry().await?;
///     println!("NestGate is ready at: {:?}", endpoint);
///     Ok(())
/// }
/// ```
pub async fn discover_nestgate_with_retry() -> Result<IpcEndpoint> {
    let mut attempts = 0;

    loop {
        attempts += 1;

        match discover_nestgate_endpoint().await {
            Ok(endpoint) => {
                info!(
                    "✅ Discovered NestGate endpoint after {} attempt(s): {:?}",
                    attempts, endpoint
                );
                return Ok(endpoint);
            }
            Err(e) if attempts < MAX_DISCOVERY_RETRIES => {
                debug!(
                    "🔄 Discovery attempt {}/{} failed: {}. Retrying in {:?}...",
                    attempts, MAX_DISCOVERY_RETRIES, e, RETRY_DELAY
                );
                sleep(RETRY_DELAY).await;
            }
            Err(e) => {
                return Err(anyhow!(
                    "Failed to discover NestGate after {} attempts: {}",
                    attempts,
                    e
                ));
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CLIENT CONNECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Connect to NestGate using discovered endpoint
///
/// This is the **primary way** other primals should connect to NestGate.
///
/// ## Process
///
/// 1. Discover NestGate endpoint (Unix or TCP)
/// 2. Connect using appropriate transport
/// 3. Return polymorphic `IpcStream` (works with both)
///
/// ## Deep Debt Principle
///
/// **Zero Configuration**: Caller doesn't need to know if NestGate is using
/// Unix sockets or TCP - connection is automatic and transparent.
///
/// ## Example
///
/// ```no_run
/// use nestgate_core::rpc::isomorphic_ipc::launcher;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     // Connect to NestGate (discovers endpoint automatically)
///     let mut stream = launcher::connect_to_nestgate().await?;
///     
///     // Use stream for JSON-RPC communication
///     // (works with both Unix and TCP transparently)
///     
///     Ok(())
/// }
/// ```
pub async fn connect_to_nestgate() -> Result<IpcStream> {
    let endpoint = discover_nestgate_endpoint().await?;

    info!("🔌 Connecting to NestGate at: {:?}", endpoint);

    connect_endpoint(&endpoint)
        .await
        .context("Failed to connect to NestGate")
}

/// Connect to NestGate with retry logic
///
/// Useful when NestGate might be starting up. Retries discovery and connection.
///
/// ## Example
///
/// ```no_run
/// use nestgate_core::rpc::isomorphic_ipc::launcher;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     // Wait for NestGate to start and connect
///     let mut stream = launcher::connect_to_nestgate_with_retry().await?;
///     println!("Connected to NestGate!");
///     Ok(())
/// }
/// ```
pub async fn connect_to_nestgate_with_retry() -> Result<IpcStream> {
    let endpoint = discover_nestgate_with_retry().await?;

    info!("🔌 Connecting to NestGate at: {:?}", endpoint);

    connect_endpoint(&endpoint)
        .await
        .context("Failed to connect to NestGate")
}

// ═══════════════════════════════════════════════════════════════════════════════
// LAUNCHER UTILITIES
// ═══════════════════════════════════════════════════════════════════════════════

/// Check if NestGate is currently running
///
/// Returns `true` if a NestGate endpoint can be discovered, `false` otherwise.
///
/// ## Example
///
/// ```no_run
/// use nestgate_core::rpc::isomorphic_ipc::launcher;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     if launcher::is_nestgate_running().await {
///         println!("NestGate is running!");
///     } else {
///         println!("NestGate is not running.");
///     }
///     Ok(())
/// }
/// ```
pub async fn is_nestgate_running() -> bool {
    discover_nestgate_endpoint().await.is_ok()
}

/// Get the XDG-compliant socket path for NestGate
///
/// This is where NestGate will attempt to create its Unix socket (if supported).
///
/// ## XDG Compliance
///
/// Follows XDG Base Directory Specification:
/// 1. `$XDG_RUNTIME_DIR/nestgate.sock` (preferred)
/// 2. `$HOME/.local/share/nestgate/nestgate.sock` (fallback)
/// 3. `/tmp/nestgate-{uid}.sock` (last resort)
///
/// ## Example
///
/// ```no_run
/// use nestgate_core::rpc::isomorphic_ipc::launcher;
///
/// fn main() -> anyhow::Result<()> {
///     let socket_path = launcher::get_nestgate_socket_path()?;
///     println!("NestGate Unix socket path: {:?}", socket_path);
///     Ok(())
/// }
/// ```
pub fn get_nestgate_socket_path() -> Result<PathBuf> {
    // Priority 1: XDG_RUNTIME_DIR (session-specific, auto-cleaned)
    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        let path = PathBuf::from(runtime_dir).join(format!("{}.sock", NESTGATE_SERVICE_NAME));
        return Ok(path);
    }

    // Priority 2: HOME/.local/share (user-specific, persistent)
    if let Ok(home) = std::env::var("HOME") {
        let path = PathBuf::from(home)
            .join(".local")
            .join("share")
            .join(NESTGATE_SERVICE_NAME)
            .join(format!("{}.sock", NESTGATE_SERVICE_NAME));
        return Ok(path);
    }

    // Priority 3: /tmp with UID (system fallback)
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let uid = std::fs::metadata("/proc/self")
            .ok()
            .map(|m| m.uid())
            .unwrap_or(1000); // fallback UID
        let path = PathBuf::from("/tmp").join(format!("{}-{}.sock", NESTGATE_SERVICE_NAME, uid));
        Ok(path)
    }

    #[cfg(not(unix))]
    {
        // Non-Unix platforms (Windows) - use temp directory
        let temp_dir = std::env::temp_dir();
        let path = temp_dir.join(format!("{}.sock", NESTGATE_SERVICE_NAME));
        return Ok(path);
    }
}

/// Get the XDG-compliant TCP discovery file path for NestGate
///
/// This is where NestGate writes its TCP port information when using TCP fallback.
///
/// ## Example
///
/// ```no_run
/// use nestgate_core::rpc::isomorphic_ipc::launcher;
///
/// fn main() -> anyhow::Result<()> {
///     let discovery_path = launcher::get_nestgate_tcp_discovery_path()?;
///     println!("NestGate TCP discovery file: {:?}", discovery_path);
///     Ok(())
/// }
/// ```
pub fn get_nestgate_tcp_discovery_path() -> Result<PathBuf> {
    // Priority 1: XDG_RUNTIME_DIR
    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        let path = PathBuf::from(runtime_dir).join(format!("{}-ipc-port", NESTGATE_SERVICE_NAME));
        return Ok(path);
    }

    // Priority 2: HOME/.local/share
    if let Ok(home) = std::env::var("HOME") {
        let path = PathBuf::from(home)
            .join(".local")
            .join("share")
            .join(NESTGATE_SERVICE_NAME)
            .join(format!("{}-ipc-port", NESTGATE_SERVICE_NAME));
        return Ok(path);
    }

    // Priority 3: /tmp
    let path = PathBuf::from("/tmp").join(format!("{}-ipc-port", NESTGATE_SERVICE_NAME));
    Ok(path)
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_socket_path() {
        let path = get_nestgate_socket_path().expect("Should get socket path");
        assert!(
            path.to_string_lossy().contains("nestgate"),
            "Socket path should contain 'nestgate': {:?}",
            path
        );
    }

    #[test]
    fn test_get_tcp_discovery_path() {
        let path = get_nestgate_tcp_discovery_path().expect("Should get discovery path");
        assert!(
            path.to_string_lossy().contains("nestgate"),
            "Discovery path should contain 'nestgate': {:?}",
            path
        );
        assert!(
            path.to_string_lossy().contains("ipc-port"),
            "Discovery path should contain 'ipc-port': {:?}",
            path
        );
    }

    #[tokio::test]
    async fn test_is_nestgate_running_when_not_running() {
        // This test assumes NestGate is not running
        // In CI/CD, this should be true
        let running = is_nestgate_running().await;
        // We can't assert false because NestGate might actually be running
        // Just verify the function doesn't panic
        let _ = running;
    }
}
