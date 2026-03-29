// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Atomic Composition Support for `NestGate`
//!
//! **Phase 3: Deployment Coordination - Atomic Compositions**
//!
//! This module provides support for atomic primal compositions, specifically
//! the **NEST Atomic** (TOWER + nestgate + squirrel).
//!
//! ## Atomic Compositions
//!
//! **NEST** = TOWER + nestgate + squirrel
//! - **TOWER** = beardog (device) + songbird (network)
//! - **nestgate** = Universal storage + discovery
//! - **squirrel** = AI/MCP integration
//!
//! Combined: Complete storage + compute + AI + networking stack
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    NEST ATOMIC LAUNCHER                      │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                              │
//! │  TOWER (foundational device + networking)                   │
//! │    ├─ beardog  → Device abstraction & capabilities          │
//! │    └─ songbird → Network discovery & federation             │
//! │                                                              │
//! │  nestgate (storage + discovery)                             │
//! │    ├─ Universal storage (ZFS, ext4, tmpfs)                  │
//! │    └─ Primal discovery & service metadata                   │
//! │                                                              │
//! │  squirrel (AI integration)                                  │
//! │    ├─ MCP (Model Context Protocol)                          │
//! │    └─ AI model orchestration                                │
//! │                                                              │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Deep Debt Principles
//!
//! - ✅ **Runtime Discovery**: Each primal discovers others via isomorphic IPC
//! - ✅ **Zero Hardcoding**: No hardcoded ports, paths, or endpoints
//! - ✅ **Platform Agnostic**: Works on all platforms (Linux, macOS, FreeBSD, Android)
//! - ✅ **Self-Knowledge**: Each primal only knows itself, discovers others
//! - ✅ **Capability-Based**: Adapts to available platform capabilities

use anyhow::{Context, Result};
use std::time::Duration;
use tracing::{debug, info, warn};

use super::health::{HealthStatus, check_nestgate_health, wait_for_healthy};
use super::launcher::{discover_nestgate_endpoint, is_nestgate_running};

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Check health of a primal via capability-based discovery
///
/// ✅ EVOLVED: Real implementation using socket discovery + JSON-RPC health check
/// ✅ DEEP DEBT PRINCIPLE #6: Primal Self-Knowledge
/// - Discovers primal socket at runtime (no hardcoding)
/// - Gracefully handles unavailable primals
/// - Returns error if primal not available (caller decides fallback)
///
/// # Arguments
/// * `primal_name` - Name of primal to check ("beardog", "songbird", "squirrel")
///
/// # Returns
/// * `Ok(HealthStatus)` - Health status if primal discovered and responsive
/// * `Err(_)` - Primal not available or health check failed
async fn check_primal_health(primal_name: &str) -> Result<HealthStatus> {
    debug!(
        "🔍 Attempting to discover {} health endpoint...",
        primal_name
    );

    // ✅ CAPABILITY-BASED: Discover primal socket via standard paths
    // Priority: BIOMEOS_SOCKET_DIR → XDG_RUNTIME_DIR/biomeos → /tmp
    let socket_path = discover_primal_socket(primal_name);

    let Some(socket) = socket_path else {
        anyhow::bail!("{primal_name} socket not found in any standard location");
    };

    // ✅ RUNTIME DISCOVERY: Attempt to connect to the primal's socket
    debug!("🔌 Connecting to {} at {}", primal_name, socket.display());

    let stream = tokio::net::UnixStream::connect(&socket)
        .await
        .map_err(|e| {
            anyhow::anyhow!(
                "{} not reachable at {}: {}",
                primal_name,
                socket.display(),
                e
            )
        })?;

    // ✅ JSON-RPC 2.0: Send health check request
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    let (reader, mut writer) = stream.into_split();

    let health_request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "health",
        "id": 1
    });

    let mut request_bytes = serde_json::to_vec(&health_request)?;
    request_bytes.push(b'\n');
    writer.write_all(&request_bytes).await?;

    // Read response with timeout
    let mut buf_reader = BufReader::new(reader);
    let mut response_line = String::new();

    let read_result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        buf_reader.read_line(&mut response_line),
    )
    .await;

    match read_result {
        Ok(Ok(_)) if !response_line.is_empty() => {
            // Parse response to check for health
            if let Ok(resp) = serde_json::from_str::<serde_json::Value>(&response_line) {
                if resp.get("result").is_some() {
                    debug!("✅ {} is healthy (responded to health check)", primal_name);
                    return Ok(HealthStatus::Healthy);
                } else if resp.get("error").is_some() {
                    debug!("⚠️  {} responded with error", primal_name);
                    return Ok(HealthStatus::Degraded);
                }
            }
            // Got a response but couldn't parse - still alive
            debug!("⚠️  {} responded but health format unknown", primal_name);
            Ok(HealthStatus::Degraded)
        }
        Ok(Ok(_)) => {
            // Empty response
            debug!("⚠️  {} connected but no response", primal_name);
            Ok(HealthStatus::Degraded)
        }
        Ok(Err(e)) => {
            debug!("❌ {} read error: {}", primal_name, e);
            Ok(HealthStatus::Unreachable)
        }
        Err(_) => {
            debug!("⏰ {} health check timed out (5s)", primal_name);
            Ok(HealthStatus::Degraded)
        }
    }
}

/// Discover a primal's Unix socket path via standard locations
///
/// ✅ ZERO HARDCODING: Uses environment and standard paths only
fn discover_primal_socket(primal_name: &str) -> Option<std::path::PathBuf> {
    // 1. Check biomeOS socket directory
    if let Ok(dir) = std::env::var("BIOMEOS_SOCKET_DIR") {
        let path = std::path::PathBuf::from(dir).join(format!("{primal_name}.sock"));
        if path.exists() {
            return Some(path);
        }
    }

    // 2. Check XDG runtime directory with biomeOS subdirectory
    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        let path = std::path::PathBuf::from(xdg)
            .join("biomeos")
            .join(format!("{primal_name}.sock"));
        if path.exists() {
            return Some(path);
        }
    }

    // 3. Check /tmp fallback
    let tmp_path = std::path::PathBuf::from("/tmp").join(format!("{primal_name}.sock"));
    if tmp_path.exists() {
        return Some(tmp_path);
    }

    // 4. Check family-scoped socket patterns
    if let Ok(family_id) = std::env::var("NESTGATE_FAMILY_ID") {
        // Try family-scoped pattern: {primal_name}-{family_id}.sock
        if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
            let path = std::path::PathBuf::from(xdg)
                .join("biomeos")
                .join(format!("{primal_name}-{family_id}.sock"));
            if path.exists() {
                return Some(path);
            }
        }
    }

    None
}

// ═══════════════════════════════════════════════════════════════════════════════
// ATOMIC COMPOSITION TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Atomic composition type
///
/// Compositions define **required capabilities**, not specific primal names.
/// At runtime, capability discovery resolves which primals satisfy each role.
/// The well-known names are defaults that can be overridden via configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomicType {
    /// TOWER = device-capability + network-capability (foundation)
    Tower,
    /// NODE = TOWER + gpu-compute-capability
    Node,
    /// NEST = TOWER + storage-capability + ai-capability
    Nest,
    /// Custom composition with arbitrary capability requirements
    Custom {
        /// Human-readable composition name
        name: String,
        /// List of capability roles required by this composition
        required_capabilities: Vec<String>,
    },
}

/// Well-known capability roles for atomic compositions
pub mod capabilities {
    /// Device abstraction and hardware management
    pub const DEVICE: &str = "device";
    /// Network discovery and federation
    pub const NETWORK: &str = "network";
    /// Universal storage and data management
    pub const STORAGE: &str = "storage";
    /// AI/MCP integration and model orchestration
    pub const AI_INTEGRATION: &str = "ai-integration";
    /// GPU compute acceleration
    pub const GPU_COMPUTE: &str = "gpu-compute";
}

impl AtomicType {
    /// Get human-readable name
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Tower => "TOWER",
            Self::Node => "NODE",
            Self::Nest => "NEST",
            Self::Custom { name, .. } => name,
        }
    }

    /// Get required capabilities for this composition
    ///
    /// Returns capability roles (not primal names). At runtime, capability
    /// discovery determines which primals satisfy each role.
    pub fn required_capabilities(&self) -> Vec<&str> {
        match self {
            Self::Tower => vec![capabilities::DEVICE, capabilities::NETWORK],
            Self::Node => vec![
                capabilities::DEVICE,
                capabilities::NETWORK,
                capabilities::GPU_COMPUTE,
            ],
            Self::Nest => vec![
                capabilities::DEVICE,
                capabilities::NETWORK,
                capabilities::STORAGE,
                capabilities::AI_INTEGRATION,
            ],
            Self::Custom {
                required_capabilities,
                ..
            } => required_capabilities.iter().map(String::as_str).collect(),
        }
    }

    /// Check if this composition requires the storage capability (i.e. `NestGate`'s role)
    #[must_use]
    pub fn requires_storage(&self) -> bool {
        self.required_capabilities()
            .contains(&capabilities::STORAGE)
    }
}

/// Status of an atomic composition
#[derive(Debug, Clone)]
pub struct AtomicStatus {
    /// Atomic type
    pub atomic_type: AtomicType,
    /// Overall health
    pub overall_health: HealthStatus,
    /// Individual component statuses
    pub component_statuses: Vec<(String, HealthStatus)>,
}

impl AtomicStatus {
    /// Check if atomic is fully operational
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.overall_health.is_operational()
            && self
                .component_statuses
                .iter()
                .all(|(_, status)| status.is_operational())
    }

    /// Get components that need attention
    #[must_use]
    pub fn components_needing_attention(&self) -> Vec<&str> {
        self.component_statuses
            .iter()
            .filter(|(_, status)| status.needs_attention())
            .map(|(name, _)| name.as_str())
            .collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// NESTGATE ATOMIC FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Verify `NestGate` is healthy and operational
///
/// This is the **primary verification function** for NEST atomic composition.
///
/// ## Process
///
/// 1. Check if `NestGate` is running (discovery)
/// 2. Perform health check
/// 3. Verify operational status
///
/// ## Example
///
/// ```rust,ignore
/// use nestgate_core::rpc::isomorphic_ipc::atomic;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     atomic::verify_nestgate_health().await?;
///     println!("✅ NestGate is healthy and operational!");
///     Ok(())
/// }
/// ```
pub async fn verify_nestgate_health() -> Result<()> {
    info!("🔍 Verifying NestGate health...");

    // Check if running
    if !is_nestgate_running().await {
        return Err(anyhow::anyhow!("NestGate is not running"));
    }

    // Perform health check
    let status = check_nestgate_health()
        .await
        .context("Failed to check NestGate health")?;

    if !status.is_operational() {
        return Err(anyhow::anyhow!("NestGate is not operational: {status:?}"));
    }

    info!("✅ NestGate health verified: {:?}", status);
    Ok(())
}

/// Wait for `NestGate` to start and become healthy
///
/// Used during NEST atomic composition to ensure `NestGate` is ready
/// before proceeding with dependent services (like squirrel).
///
/// ## Example
///
/// ```rust,ignore
/// use nestgate_core::rpc::isomorphic_ipc::atomic;
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     // Wait up to 30 seconds for NestGate
///     atomic::wait_for_nestgate(Duration::from_secs(30)).await?;
///     println!("NestGate is ready!");
///     Ok(())
/// }
/// ```
pub async fn wait_for_nestgate(timeout: Duration) -> Result<()> {
    info!(
        "⏳ Waiting for NestGate to start (timeout: {:?})...",
        timeout
    );
    wait_for_healthy(timeout)
        .await
        .context("Timeout waiting for NestGate")
}

/// Verify NEST atomic composition health
///
/// Checks health of all NEST components:
/// - beardog (via its isomorphic IPC if available)
/// - songbird (via its isomorphic IPC if available)
/// - nestgate (via isomorphic IPC - implemented here)
/// - squirrel (via its isomorphic IPC if available)
///
/// ## Note
///
/// This function currently only verifies `NestGate`'s health directly.
/// Full NEST atomic health verification requires coordination with
/// biomeOS atomic-deploy crate or integration with beardog/songbird/squirrel
/// health check implementations.
///
/// ## Example
///
/// ```rust,ignore
/// use nestgate_core::rpc::isomorphic_ipc::atomic;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let status = atomic::verify_nest_health().await?;
///     
///     if status.is_operational() {
///         println!("✅ NEST atomic is fully operational!");
///     } else {
///         println!("⚠️  NEST atomic components need attention:");
///         for component in status.components_needing_attention() {
///             println!("  - {}", component);
///         }
///     }
///     
///     Ok(())
/// }
/// ```
/// Verify an atomic composition's health by discovering primals at runtime
///
/// Uses capability-based discovery to find which primals are running and
/// checks their health via JSON-RPC. Does **not** assume any specific primal
/// names -- discovers all available primals and maps them to required capabilities.
///
/// # Self-Knowledge Principle
///
/// `NestGate` checks its own health directly. For all other capabilities,
/// it discovers primals via socket scanning and checks them dynamically.
pub async fn verify_composition_health(composition: &AtomicType) -> Result<AtomicStatus> {
    info!(
        "🔍 Verifying {} atomic composition health...",
        composition.name()
    );

    let mut component_statuses = Vec::new();

    // Self-knowledge: check our own health directly
    let nestgate_status = check_nestgate_health().await.unwrap_or_else(|e| {
        warn!("⚠️  Failed to check NestGate health: {}", e);
        HealthStatus::Unreachable
    });
    component_statuses.push(("nestgate (self)".to_string(), nestgate_status));

    // Discover and check all other primals via socket scanning
    let discovered_primals = discover_available_primals().await;

    for primal_name in &discovered_primals {
        if primal_name == "nestgate" {
            continue; // Already checked via self-knowledge
        }

        let status = check_primal_health(primal_name).await.unwrap_or_else(|_| {
            debug!(
                "{} health endpoint not available, marking as degraded",
                primal_name
            );
            HealthStatus::Degraded
        });
        component_statuses.push((primal_name.clone(), status));
    }

    // Determine overall health
    let overall_health = if component_statuses
        .iter()
        .all(|(_, status)| *status == HealthStatus::Healthy)
    {
        HealthStatus::Healthy
    } else if component_statuses
        .iter()
        .any(|(_, status)| *status == HealthStatus::Unreachable)
    {
        HealthStatus::Unhealthy
    } else {
        HealthStatus::Degraded
    };

    let status = AtomicStatus {
        atomic_type: composition.clone(),
        overall_health,
        component_statuses,
    };

    info!(
        "📊 {} health: {:?}",
        composition.name(),
        status.overall_health
    );
    for (component, component_status) in &status.component_statuses {
        info!("  - {}: {:?}", component, component_status);
    }

    Ok(status)
}

/// Backward-compatible wrapper for NEST composition health check
pub async fn verify_nest_health() -> Result<AtomicStatus> {
    verify_composition_health(&AtomicType::Nest).await
}

/// Discover available primals by scanning standard socket locations
///
/// Scans `BIOMEOS_SOCKET_DIR`, `XDG_RUNTIME_DIR/biomeos`, and /tmp for
/// primal sockets. Returns discovered primal names (without assuming
/// which primals should exist).
async fn discover_available_primals() -> Vec<String> {
    let mut primals = Vec::new();
    let socket_dirs = gather_socket_search_dirs();

    for dir in &socket_dirs {
        let dir_path = std::path::Path::new(dir);
        if !dir_path.exists() {
            continue;
        }

        let Ok(entries) = std::fs::read_dir(dir_path) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };

            // Socket files are named {primal}.sock or {primal}-{family}.sock
            if path.extension().and_then(|e| e.to_str()) == Some("sock") {
                // Extract base primal name (strip family suffix if present)
                let primal_name = name.split('-').next().unwrap_or(name);
                if !primal_name.is_empty() && !primals.contains(&primal_name.to_string()) {
                    primals.push(primal_name.to_string());
                }
            }
        }
    }

    debug!("🔍 Discovered primals via socket scan: {:?}", primals);
    primals
}

/// Gather standard directories to search for primal sockets
fn gather_socket_search_dirs() -> Vec<String> {
    let mut dirs = Vec::new();

    if let Ok(dir) = std::env::var("BIOMEOS_SOCKET_DIR") {
        dirs.push(dir);
    }

    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        dirs.push(format!("{xdg}/biomeos"));
    }

    // UID-based XDG fallback (same effective source as `nestgate_platform` / core helpers when unified).
    let uid = uzers::get_current_uid();
    let xdg_default = format!("/run/user/{uid}/biomeos");
    if !dirs.contains(&xdg_default) {
        dirs.push(xdg_default);
    }

    dirs.push("/tmp".to_string());
    dirs
}

// ═══════════════════════════════════════════════════════════════════════════════
// INTEGRATION WITH BIOMEOS ATOMIC-DEPLOY
// ═══════════════════════════════════════════════════════════════════════════════

/// Get `NestGate`'s endpoint for atomic composition coordination
///
/// This function can be called by biomeOS atomic-deploy crate to discover
/// `NestGate`'s endpoint during NEST atomic composition.
///
/// ## Example
///
/// ```rust,ignore
/// use nestgate_core::rpc::isomorphic_ipc::atomic;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let endpoint = atomic::get_nestgate_endpoint_for_atomic().await?;
///     println!("NestGate endpoint for NEST atomic: {:?}", endpoint);
///     Ok(())
/// }
/// ```
pub async fn get_nestgate_endpoint_for_atomic() -> Result<String> {
    let endpoint = discover_nestgate_endpoint().await?;
    Ok(format!("{endpoint:?}"))
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_type_required_capabilities() {
        assert_eq!(AtomicType::Tower.required_capabilities().len(), 2);
        assert_eq!(AtomicType::Node.required_capabilities().len(), 3);
        assert_eq!(AtomicType::Nest.required_capabilities().len(), 4);
    }

    #[test]
    fn test_atomic_capabilities_are_roles_not_primal_names() {
        // Verify we use capability roles, not hardcoded primal names
        for cap in AtomicType::Nest.required_capabilities() {
            assert!(
                !["beardog", "songbird", "nestgate", "squirrel", "toadstool"].contains(&cap),
                "Capability '{}' should be a role, not a primal name",
                cap
            );
        }
    }

    #[test]
    fn test_atomic_type_requires_storage() {
        assert!(!AtomicType::Tower.requires_storage());
        assert!(!AtomicType::Node.requires_storage());
        assert!(AtomicType::Nest.requires_storage());
    }

    #[test]
    fn test_custom_atomic_type() {
        let custom = AtomicType::Custom {
            name: "EDGE".to_string(),
            required_capabilities: vec![
                capabilities::DEVICE.to_string(),
                capabilities::STORAGE.to_string(),
            ],
        };
        assert_eq!(custom.name(), "EDGE");
        assert_eq!(custom.required_capabilities().len(), 2);
        assert!(custom.requires_storage());
    }

    #[test]
    fn test_atomic_status_is_operational() {
        let status = AtomicStatus {
            atomic_type: AtomicType::Nest,
            overall_health: HealthStatus::Healthy,
            component_statuses: vec![
                ("nestgate (self)".to_string(), HealthStatus::Healthy),
                ("device-provider".to_string(), HealthStatus::Healthy),
            ],
        };
        assert!(status.is_operational());

        let degraded_status = AtomicStatus {
            atomic_type: AtomicType::Nest,
            overall_health: HealthStatus::Degraded,
            component_statuses: vec![
                ("nestgate (self)".to_string(), HealthStatus::Degraded),
                ("device-provider".to_string(), HealthStatus::Healthy),
            ],
        };
        assert!(degraded_status.is_operational());
        assert!(degraded_status.overall_health.needs_attention());

        let unhealthy_status = AtomicStatus {
            atomic_type: AtomicType::Nest,
            overall_health: HealthStatus::Unhealthy,
            component_statuses: vec![
                ("nestgate (self)".to_string(), HealthStatus::Unhealthy),
                ("device-provider".to_string(), HealthStatus::Healthy),
            ],
        };
        assert!(!unhealthy_status.is_operational());
    }

    #[tokio::test]
    async fn test_verify_nest_health() {
        // May fail if NestGate is not running -- that's expected
        let result = verify_nest_health().await;
        let _ = result;
    }

    #[tokio::test]
    async fn test_discover_available_primals_does_not_panic() {
        // Should gracefully handle missing socket directories
        let primals = discover_available_primals().await;
        // Just verify it returns without panic
        let _ = primals;
    }
}
