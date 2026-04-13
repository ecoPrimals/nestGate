// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Atomic Composition Support for `NestGate`
//!
//! **Phase 3: Deployment Coordination - Atomic Compositions**
//!
//! This module provides support for atomic primal compositions, specifically
//! the **NEST Atomic** (TOWER + `NestGate`).
//!
//! ## Atomic Compositions
//!
//! **NEST Atomic** = TOWER + `NestGate` (storage/permanence)
//! - **TOWER** = security capability + orchestration capability
//! - **`NestGate`** = universal storage, ZFS, permanence (this primal)
//!
//! ## Architecture
//!
//! ```text
//! NEST ATOMIC COMPOSITION
//!
//!   TOWER (foundational device + networking)
//!     security: crypto, key management, lineage
//!     orchestration: TLS, HTTP transport, discovery registry
//!
//!   NestGate (storage / permanence — this primal)
//!     Universal storage (ZFS, ext4, tmpfs)
//!     JSON-RPC on UDS (primary surface)
//!     Self-knowledge + capability-based peer discovery
//!
//!   [Dynamic overlays discovered at runtime via biomeOS]
//!     AI/MCP: meta-tier, not atomic member
//!     Any capability provider via Neural API routing
//! ```
//!
//! ## Deep Debt Principles
//!
//! - **Runtime Discovery**: Each primal discovers others via isomorphic IPC
//! - **Zero Hardcoding**: No hardcoded ports, paths, or endpoints
//! - **Platform Agnostic**: Works on all platforms (Linux, macOS, FreeBSD, Android)
//! - **Self-Knowledge**: Each primal only knows itself, discovers others
//! - **Capability-Based**: Adapts to available platform capabilities
//! - **Sovereignty**: `NestGate` delegates crypto to the security capability
//!   provider and network to the orchestration capability provider
//!
//! ## Audit Compliance Note
//!
//! Primal product names in this module's documentation describe **ecosystem
//! topology** — the NEST Atomic composition and TOWER architecture. These are
//! architectural role descriptions, not routing logic or hardcoded coupling.
//! Production code discovers peers via capability IPC and environment variables.

mod discovery;
#[cfg(test)]
mod tests;

use anyhow::{Context, Result};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, info, warn};

use super::health::{HealthStatus, check_nestgate_health, wait_for_healthy};
use super::launcher::{discover_nestgate_endpoint, is_nestgate_running};
use discovery::{discover_available_primals, discover_primal_socket, local_primal_id};

// ═══════════════════════════════════════════════════════════════════════════════
// ATOMIC COMPOSITION TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Atomic composition type.
///
/// Compositions define **required capabilities**, not specific primal names.
/// At runtime, capability discovery resolves which primals satisfy each role.
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
        /// Required capability roles
        required_capabilities: Vec<String>,
    },
}

/// Well-known capability roles for atomic compositions.
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
    /// Get human-readable name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Tower => "TOWER",
            Self::Node => "NODE",
            Self::Nest => "NEST",
            Self::Custom { name, .. } => name,
        }
    }

    /// Get required capabilities for this composition.
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

    /// Check if this composition requires the storage capability (`NestGate`'s role).
    #[must_use]
    pub fn requires_storage(&self) -> bool {
        self.required_capabilities()
            .contains(&capabilities::STORAGE)
    }
}

/// Status of an atomic composition.
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
    /// Check if atomic is fully operational.
    #[must_use]
    pub fn is_operational(&self) -> bool {
        self.overall_health.is_operational()
            && self
                .component_statuses
                .iter()
                .all(|(_, status)| status.is_operational())
    }

    /// Get components that need attention.
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
// HEALTH CHECK HELPERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Check health of a primal via socket discovery + JSON-RPC.
async fn check_primal_health(primal_name: &str) -> Result<HealthStatus> {
    debug!("Attempting to discover {} health endpoint...", primal_name);

    let Some(socket) = discover_primal_socket(primal_name) else {
        anyhow::bail!("{primal_name} socket not found in any standard location");
    };

    debug!("Connecting to {} at {}", primal_name, socket.display());

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

    let (reader, mut writer) = stream.into_split();

    let health_request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "health",
        "id": 1
    });

    let mut request_bytes = serde_json::to_vec(&health_request)?;
    request_bytes.push(b'\n');
    writer.write_all(&request_bytes).await?;

    let mut buf_reader = BufReader::new(reader);
    let mut response_line = String::new();

    let read_result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        buf_reader.read_line(&mut response_line),
    )
    .await;

    match read_result {
        Ok(Ok(_)) if !response_line.is_empty() => {
            if let Ok(resp) = serde_json::from_str::<serde_json::Value>(&response_line) {
                if resp.get("result").is_some() {
                    debug!("{} is healthy (responded to health check)", primal_name);
                    return Ok(HealthStatus::Healthy);
                } else if resp.get("error").is_some() {
                    debug!("{} responded with error", primal_name);
                    return Ok(HealthStatus::Degraded);
                }
            }
            debug!("{} responded but health format unknown", primal_name);
            Ok(HealthStatus::Degraded)
        }
        Ok(Ok(_)) => {
            debug!("{} connected but no response", primal_name);
            Ok(HealthStatus::Degraded)
        }
        Ok(Err(e)) => {
            debug!("{} read error: {}", primal_name, e);
            Ok(HealthStatus::Unreachable)
        }
        Err(_) => {
            debug!("{} health check timed out (5s)", primal_name);
            Ok(HealthStatus::Degraded)
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC COMPOSITION API
// ═══════════════════════════════════════════════════════════════════════════════

/// Verify `NestGate` is healthy and operational.
///
/// # Errors
///
/// Returns [`anyhow::Error`] if `NestGate` is not running (discovery fails) or the reported
/// health status is not operational.
pub async fn verify_nestgate_health() -> Result<()> {
    info!("Verifying NestGate health...");

    if !is_nestgate_running().await {
        return Err(anyhow::anyhow!("NestGate is not running"));
    }

    let status = check_nestgate_health()
        .await
        .context("Failed to check NestGate health")?;

    if !status.is_operational() {
        return Err(anyhow::anyhow!("NestGate is not operational: {status:?}"));
    }

    info!("NestGate health verified: {:?}", status);
    Ok(())
}

/// Wait for `NestGate` to start and become healthy.
///
/// # Errors
///
/// Returns [`anyhow::Error`] when [`wait_for_healthy`] times out before `NestGate` reports healthy.
pub async fn wait_for_nestgate(timeout: Duration) -> Result<()> {
    info!("Waiting for NestGate to start (timeout: {:?})...", timeout);
    wait_for_healthy(timeout)
        .await
        .context("Timeout waiting for NestGate")
}

/// Verify an atomic composition's health by discovering primals at runtime.
///
/// # Errors
///
/// The current implementation always returns [`Ok`]; the [`Result`] is reserved for future
/// discovery or health-check failures.
pub async fn verify_composition_health(composition: &AtomicType) -> Result<AtomicStatus> {
    info!(
        "Verifying {} atomic composition health...",
        composition.name()
    );

    let mut component_statuses = Vec::new();
    let self_pkg = local_primal_id();

    let nestgate_status = check_nestgate_health().await.unwrap_or_else(|e| {
        warn!("Failed to check NestGate health: {}", e);
        HealthStatus::Unreachable
    });
    component_statuses.push((format!("{self_pkg} (self)"), nestgate_status));

    let discovered_primals = discover_available_primals();

    for primal_name in &discovered_primals {
        if primal_name == &self_pkg {
            continue;
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

    info!("{} health: {:?}", composition.name(), status.overall_health);
    for (component, component_status) in &status.component_statuses {
        info!("  - {}: {:?}", component, component_status);
    }

    Ok(status)
}

/// Backward-compatible wrapper for NEST composition health check.
///
/// # Errors
///
/// Same as [`verify_composition_health`]: currently always [`Ok`].
pub async fn verify_nest_health() -> Result<AtomicStatus> {
    verify_composition_health(&AtomicType::Nest).await
}

/// Get `NestGate`'s endpoint for atomic composition coordination.
///
/// # Errors
///
/// Returns [`anyhow::Error`] if [`super::launcher::discover_nestgate_endpoint`] fails (endpoint
/// not found).
pub async fn get_nestgate_endpoint_for_atomic() -> Result<String> {
    let endpoint = discover_nestgate_endpoint().await?;
    Ok(format!("{endpoint:?}"))
}
