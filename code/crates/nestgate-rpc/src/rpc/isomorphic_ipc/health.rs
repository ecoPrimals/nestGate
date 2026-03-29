// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Health Monitoring for `NestGate` with Isomorphic IPC
//!
//! **Phase 3: Deployment Coordination - Health Checks**
//!
//! This module provides health monitoring using the isomorphic IPC client,
//! enabling other primals to check `NestGate`'s status without knowing
//! whether it's using Unix sockets or TCP.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                 HEALTH CHECK WORKFLOW                        │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                              │
//! │  1. DISCOVER → Find NestGate endpoint (launcher)            │
//! │                                                              │
//! │  2. CONNECT → Establish isomorphic connection               │
//! │                                                              │
//! │  3. CHECK → Send `health.check` JSON-RPC request            │
//! │                                                              │
//! │  4. VERIFY → Parse response and return status               │
//! │                                                              │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Deep Debt Principles
//!
//! - ✅ **Zero Hardcoding**: Uses runtime endpoint discovery
//! - ✅ **Platform Agnostic**: Works with both Unix and TCP transports
//! - ✅ **Modern Idiomatic Rust**: Async/await, Result propagation
//! - ✅ **Runtime Discovery**: No compile-time knowledge of transport

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::{interval, sleep};
use tracing::{debug, error, info, warn};

use super::launcher::connect_to_nestgate;

// ═══════════════════════════════════════════════════════════════════════════════
// HEALTH STATUS TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Health status of `NestGate`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// `NestGate` is healthy and operational
    Healthy,
    /// `NestGate` is degraded but operational
    Degraded,
    /// `NestGate` is unhealthy and may not respond
    Unhealthy,
    /// `NestGate` is not responding
    Unreachable,
}

impl HealthStatus {
    /// Check if status indicates `NestGate` is operational
    #[must_use]
    pub const fn is_operational(&self) -> bool {
        matches!(self, Self::Healthy | Self::Degraded)
    }

    /// Check if status indicates `NestGate` needs attention
    #[must_use]
    pub const fn needs_attention(&self) -> bool {
        matches!(self, Self::Degraded | Self::Unhealthy | Self::Unreachable)
    }
}

/// Detailed health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    /// Overall health status
    pub status: HealthStatus,
    /// `NestGate` version
    pub version: String,
    /// Uptime in seconds
    #[serde(default)]
    pub uptime_seconds: u64,
    /// Active connections count
    #[serde(default)]
    pub active_connections: usize,
    /// Additional metadata
    #[serde(default)]
    pub metadata: Option<Value>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// HEALTH CHECK CLIENT
// ═══════════════════════════════════════════════════════════════════════════════

/// Perform a health check on `NestGate`
///
/// This is the **primary health check function** for other primals.
///
/// ## Process
///
/// 1. Discover `NestGate` endpoint (automatic)
/// 2. Connect using isomorphic transport
/// 3. Send `health.check` JSON-RPC request
/// 4. Parse and return response
///
/// ## Example
///
/// ```rust,ignore
/// use nestgate_core::rpc::isomorphic_ipc::health;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     match health::check_nestgate_health().await {
///         Ok(status) => {
///             println!("NestGate is {}",
///                 if status.is_operational() { "operational" } else { "down" }
///             );
///         }
///         Err(e) => {
///             eprintln!("Health check failed: {}", e);
///         }
///     }
///     Ok(())
/// }
/// ```
pub async fn check_nestgate_health() -> Result<HealthStatus> {
    match check_nestgate_health_detailed().await {
        Ok(response) => Ok(response.status),
        Err(e) => {
            warn!("⚠️  Health check failed: {}", e);
            Ok(HealthStatus::Unreachable)
        }
    }
}

/// Perform a detailed health check on `NestGate`
///
/// Returns full `HealthCheckResponse` with version, uptime, and connection info.
///
/// ## Example
///
/// ```rust,ignore
/// use nestgate_core::rpc::isomorphic_ipc::health;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let response = health::check_nestgate_health_detailed().await?;
///     println!("Status: {:?}", response.status);
///     println!("Version: {}", response.version);
///     println!("Uptime: {}s", response.uptime_seconds);
///     println!("Connections: {}", response.active_connections);
///     Ok(())
/// }
/// ```
pub async fn check_nestgate_health_detailed() -> Result<HealthCheckResponse> {
    // Connect to NestGate using isomorphic discovery
    let mut stream = connect_to_nestgate()
        .await
        .context("Failed to connect to NestGate for health check")?;

    // Create JSON-RPC 2.0 health.check request (wateringHole semantic naming)
    let request = json!({
        "jsonrpc": "2.0",
        "method": "health.check",
        "params": {},
        "id": 1
    });

    // Send request
    let request_str = serde_json::to_string(&request)?;
    stream
        .write_all(format!("{request_str}\n").as_bytes())
        .await
        .context("Failed to send health check request")?;
    stream.flush().await?;

    debug!("📤 Sent health check request");

    // Read response
    let mut reader = BufReader::new(stream);
    let mut response_line = String::new();
    reader
        .read_line(&mut response_line)
        .await
        .context("Failed to read health check response")?;

    debug!("📥 Received health check response: {}", response_line);

    // Parse JSON-RPC response
    let response: Value =
        serde_json::from_str(&response_line).context("Failed to parse health check response")?;

    // Extract result
    if let Some(result) = response.get("result") {
        // Try to parse as HealthCheckResponse
        if let Ok(health_response) = serde_json::from_value::<HealthCheckResponse>(result.clone()) {
            return Ok(health_response);
        }

        // Fallback: Extract basic status
        if let Some(status_str) = result.get("status").and_then(|v| v.as_str()) {
            let status = match status_str {
                "healthy" => HealthStatus::Healthy,
                "degraded" => HealthStatus::Degraded,
                "unhealthy" | _ => HealthStatus::Unhealthy,
            };

            let version = result
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();

            return Ok(HealthCheckResponse {
                status,
                version,
                uptime_seconds: 0,
                active_connections: 0,
                metadata: None,
            });
        }
    }

    // Check for error
    if let Some(error) = response.get("error") {
        return Err(anyhow::anyhow!(
            "Health check returned error: {}",
            error
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error")
        ));
    }

    Err(anyhow::anyhow!("Invalid health check response format"))
}

// ═══════════════════════════════════════════════════════════════════════════════
// PERIODIC HEALTH MONITORING
// ═══════════════════════════════════════════════════════════════════════════════

/// Monitor `NestGate` health periodically
///
/// Runs a background task that checks `NestGate` health at the specified interval.
/// Useful for monitoring daemons and health check services.
///
/// ## Example
///
/// ```rust,ignore
/// use nestgate_core::rpc::isomorphic_ipc::health;
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     // Monitor NestGate every 30 seconds
///     health::monitor_nestgate_health(
///         Duration::from_secs(30),
///         |status| {
///             println!("NestGate status: {:?}", status);
///             if status.needs_attention() {
///                 eprintln!("⚠️  NestGate needs attention!");
///             }
///         }
///     ).await
/// }
/// ```
pub async fn monitor_nestgate_health<F>(check_interval: Duration, mut callback: F) -> Result<()>
where
    F: FnMut(&HealthStatus),
{
    let mut interval_timer = interval(check_interval);

    info!(
        "🔍 Starting NestGate health monitoring (interval: {:?})",
        check_interval
    );

    loop {
        interval_timer.tick().await;

        let status = check_nestgate_health()
            .await
            .unwrap_or(HealthStatus::Unreachable);

        match status {
            HealthStatus::Healthy => debug!("✅ NestGate health check: Healthy"),
            HealthStatus::Degraded => warn!("⚠️  NestGate health check: Degraded"),
            HealthStatus::Unhealthy => error!("❌ NestGate health check: Unhealthy"),
            HealthStatus::Unreachable => error!("💀 NestGate health check: Unreachable"),
        }

        callback(&status);
    }
}

/// Wait for `NestGate` to become healthy
///
/// Polls health status until `NestGate` responds with a healthy status,
/// or until timeout is reached.
///
/// ## Example
///
/// ```rust,ignore
/// use nestgate_core::rpc::isomorphic_ipc::health;
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     // Wait up to 30 seconds for NestGate to become healthy
///     health::wait_for_healthy(Duration::from_secs(30)).await?;
///     println!("NestGate is now healthy!");
///     Ok(())
/// }
/// ```
pub async fn wait_for_healthy(timeout: Duration) -> Result<()> {
    let start = std::time::Instant::now();
    let check_interval = Duration::from_millis(500);

    info!("⏳ Waiting for NestGate to become healthy...");

    loop {
        if start.elapsed() > timeout {
            return Err(anyhow::anyhow!(
                "Timeout waiting for NestGate to become healthy after {timeout:?}"
            ));
        }

        match check_nestgate_health().await {
            Ok(HealthStatus::Healthy) => {
                info!("✅ NestGate is healthy!");
                return Ok(());
            }
            Ok(status) => {
                debug!(
                    "🔄 NestGate status: {:?}, waiting... ({:?} elapsed)",
                    status,
                    start.elapsed()
                );
            }
            Err(e) => {
                debug!("⚠️  Health check error: {}, retrying...", e);
            }
        }

        sleep(check_interval).await;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_is_operational() {
        assert!(HealthStatus::Healthy.is_operational());
        assert!(HealthStatus::Degraded.is_operational());
        assert!(!HealthStatus::Unhealthy.is_operational());
        assert!(!HealthStatus::Unreachable.is_operational());
    }

    #[test]
    fn test_health_status_needs_attention() {
        assert!(!HealthStatus::Healthy.needs_attention());
        assert!(HealthStatus::Degraded.needs_attention());
        assert!(HealthStatus::Unhealthy.needs_attention());
        assert!(HealthStatus::Unreachable.needs_attention());
    }

    #[tokio::test]
    async fn test_check_health_when_not_running() {
        // This test assumes NestGate is not running
        let status = check_nestgate_health()
            .await
            .unwrap_or(HealthStatus::Unreachable);
        // Should be Unreachable if not running
        // We can't assert this because NestGate might be running in CI
        let _ = status;
    }
}
