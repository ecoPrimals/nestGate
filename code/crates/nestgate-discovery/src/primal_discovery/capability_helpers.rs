// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability-Based Discovery Helpers
//!
//! High-level helpers for discovering and connecting to primals by capability.
//! These replace hardcoded URLs and port numbers with runtime discovery.
//!
//! # Philosophy
//!
//! **Old Way** (hardcoded):
//! ```rust,ignore
//! let beardog_url = "http://localhost:3000"; // ❌ Hardcoded!
//! ```
//!
//! **New Way** (discovered):
//! ```rust,ignore
//! let security_url = discover_security().await?; // ✅ Discovered!
//! ```
//!
//! # Features
//!
//! - Capability-based queries (not hardcoded names!)
//! - Automatic fallback to environment variables
//! - Last-resort defaults for development
//! - Works across any infrastructure
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_core::primal_discovery::capability_helpers::*;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Discover orchestration service (e.g., Songbird)
//! let orchestration = discover_orchestration().await?;
//! println!("Orchestration at: {}", orchestration.endpoint);
//!
//! // Discover security capability provider
//! let security = discover_security().await?;
//! println!("Security at: {}", security.endpoint);
//!
//! // Or discover any capability
//! let service = discover_capability("compute").await?;
//! # Ok(())
//! # }
//! ```

use nestgate_config::constants::hardcoding::runtime_fallback_ports as fallback_ports;
use nestgate_config::constants::{get_api_port, get_dev_port};
use nestgate_types::error::{NestGateError, Result};

/// Last-resort host for capability bootstrap when discovery and env URLs are unset.
/// Prefer `NESTGATE_<CAPABILITY>_HOST` (e.g. `NESTGATE_ORCHESTRATION_HOST`), then
/// `NESTGATE_DISCOVERY_FALLBACK_HOST`, then loopback.
fn fallback_host_for_capability(capability: &str) -> String {
    let specific = format!("NESTGATE_{}_HOST", capability.to_uppercase());
    std::env::var(&specific)
        .or_else(|_| std::env::var("NESTGATE_DISCOVERY_FALLBACK_HOST"))
        .unwrap_or_else(|_| "127.0.0.1".to_string())
}

fn default_port_compute() -> u16 {
    std::env::var("NESTGATE_COMPUTE_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(fallback_ports::COMPUTE)
}

fn default_port_ai() -> u16 {
    std::env::var("NESTGATE_AI_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(fallback_ports::ADMIN)
}

fn default_port_ecosystem() -> u16 {
    std::env::var("NESTGATE_ECOSYSTEM_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(fallback_ports::ECOSYSTEM)
}

/// Discovered service information
#[derive(Debug, Clone)]
pub struct DiscoveredService {
    /// Service name (if known)
    pub name: String,
    /// Primary endpoint URL
    pub endpoint: String,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Discovery source (discovery/env/default)
    pub source: DiscoverySource,
}

/// Source of service discovery
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoverySource {
    /// Discovered via mDNS/Consul/k8s
    Discovery,
    /// Found in environment variable
    Environment,
    /// Using last-resort default
    Default,
}

impl DiscoveredService {
    /// Create from environment variable
    fn from_env(capability: &str, endpoint: String) -> Self {
        Self {
            name: format!("{capability}-service"),
            endpoint,
            capabilities: vec![capability.to_string()],
            source: DiscoverySource::Environment,
        }
    }

    /// Create from default
    fn from_default(capability: &str, endpoint: String) -> Self {
        Self {
            name: format!("{capability}-service"),
            endpoint,
            capabilities: vec![capability.to_string()],
            source: DiscoverySource::Default,
        }
    }
}

// ==================== HIGH-LEVEL DISCOVERY FUNCTIONS ====================

/// Discover orchestration service (e.g., Songbird).
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_ORCHESTRATION` env var (set by songBird discovery)
/// 2. `NESTGATE_SONGBIRD_URL` (deprecated, backward compat)
/// 3. Development default fallback
///
/// # Errors
///
/// Infallible — always falls back to a development default.
pub async fn discover_orchestration() -> Result<DiscoveredService> {
    Ok(discover_capability_with_legacy(
        "orchestration",
        "SONGBIRD",
        get_api_port,
    ))
}

/// Discover whichever primal provides the "security" capability.
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_SECURITY` env var (set by songBird discovery)
/// 2. `NESTGATE_BEARDOG_URL` (deprecated, backward compat)
/// 3. Development default fallback
pub async fn discover_security() -> Result<DiscoveredService> {
    Ok(discover_capability_with_legacy(
        "security",
        "BEARDOG",
        get_dev_port,
    ))
}

/// Discover compute service (e.g., `ToadStool`).
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_COMPUTE` env var (set by songBird discovery)
/// 2. `NESTGATE_TOADSTOOL_URL` (deprecated, backward compat)
/// 3. Development default fallback
pub async fn discover_compute() -> Result<DiscoveredService> {
    Ok(discover_capability_with_legacy(
        "compute",
        "TOADSTOOL",
        default_port_compute,
    ))
}

/// Discover AI service (e.g., Squirrel).
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_AI` env var (set by songBird discovery)
/// 2. `NESTGATE_SQUIRREL_URL` (deprecated, backward compat)
/// 3. Development default fallback
pub async fn discover_ai() -> Result<DiscoveredService> {
    Ok(discover_capability_with_legacy(
        "ai",
        "SQUIRREL",
        default_port_ai,
    ))
}

/// Discover ecosystem service (e.g., `BiomeOS`).
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_ECOSYSTEM` env var (set by songBird discovery)
/// 2. `NESTGATE_BIOMEOS_URL` (deprecated, backward compat)
/// 3. Development default fallback
pub async fn discover_ecosystem() -> Result<DiscoveredService> {
    Ok(discover_capability_with_legacy(
        "ecosystem",
        "BIOMEOS",
        default_port_ecosystem,
    ))
}

// ==================== CORE DISCOVERY LOGIC ====================

/// Discover any service by capability
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_{CAPABILITY}` environment variable (set by songBird or operator)
/// 2. Error (no fallback for arbitrary capabilities)
///
/// # Errors
///
/// Returns error if capability not found via environment.
pub async fn discover_capability(capability: &str) -> Result<DiscoveredService> {
    let env_var = format!("NESTGATE_CAPABILITY_{}", capability.to_uppercase());
    if let Ok(endpoint) = std::env::var(&env_var) {
        tracing::info!(
            "Discovered '{}' capability from environment: {}",
            capability,
            endpoint
        );
        return Ok(DiscoveredService::from_env(capability, endpoint));
    }

    Err(NestGateError::network_error(format!(
        "Capability '{capability}' not found. Set {env_var} or configure peer discovery."
    )))
}

/// Discover capability with legacy environment variable support.
///
/// Checks env vars in order: capability-based, legacy primal-specific, then
/// falls back to development default.
fn discover_capability_with_legacy(
    capability: &str,
    legacy_name: &str,
    default_port: impl FnOnce() -> u16,
) -> DiscoveredService {
    let capability_env_var = format!("NESTGATE_CAPABILITY_{}", capability.to_uppercase());
    if let Ok(endpoint) = std::env::var(&capability_env_var) {
        tracing::info!(
            "Discovered '{}' capability from environment: {}",
            capability,
            endpoint
        );
        return DiscoveredService::from_env(capability, endpoint);
    }

    let legacy_env_var = format!("NESTGATE_{legacy_name}_URL");
    if let Ok(endpoint) = std::env::var(&legacy_env_var) {
        tracing::warn!(
            "Using deprecated '{}'. Migrate to '{}'",
            legacy_env_var,
            capability_env_var
        );
        return DiscoveredService::from_env(capability, endpoint);
    }

    let host = fallback_host_for_capability(capability);
    let default_endpoint = format!("http://{}:{}", host, default_port());
    tracing::warn!(
        "No environment configuration for '{}'. Using development default: {}. \
         Set {} for production.",
        capability,
        default_endpoint,
        capability_env_var
    );

    DiscoveredService::from_default(capability, default_endpoint)
}

// ==================== UTILITY FUNCTIONS ====================

/// Check if a capability is available (via discovery or environment)
///
/// Returns `true` if the capability can be discovered.
/// Does not use last-resort defaults.
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::primal_discovery::capability_helpers::*;
///
/// # async fn example() -> anyhow::Result<()> {
/// if is_capability_available("orchestration") {
///     let service = discover_orchestration().await?;
///     // Use service
/// } else {
///     // Handle absence gracefully
/// }
/// # Ok(())
/// # }
/// ```
#[must_use]
pub fn is_capability_available(capability: &str) -> bool {
    let env_var = format!("NESTGATE_CAPABILITY_{}", capability.to_uppercase());
    std::env::var(&env_var).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discover_with_env_var() {
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("NESTGATE_CAPABILITY_TEST", "http://test:1234");

        let result = discover_capability("test").await;
        assert!(result.is_ok());

        let service = result.unwrap();
        assert_eq!(service.endpoint, "http://test:1234");
        assert_eq!(service.source, DiscoverySource::Environment);

        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::remove_var("NESTGATE_CAPABILITY_TEST");
    }

    #[tokio::test]
    async fn test_discover_without_config_falls_back() {
        // Orchestration should fall back to default
        let result = discover_orchestration().await;
        assert!(result.is_ok());

        let service = result.unwrap();
        assert_eq!(service.source, DiscoverySource::Default);
        assert!(service.endpoint.contains("127.0.0.1"));
    }

    #[tokio::test]
    async fn test_legacy_env_var_backward_compat() {
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("NESTGATE_BEARDOG_URL", "http://legacy:9999");

        let result = discover_security().await;
        assert!(result.is_ok());

        let service = result.unwrap();
        assert_eq!(service.endpoint, "http://legacy:9999");
        assert_eq!(service.source, DiscoverySource::Environment);

        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::remove_var("NESTGATE_BEARDOG_URL");
    }

    #[tokio::test]
    async fn test_is_capability_available() {
        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::set_var("NESTGATE_CAPABILITY_CUSTOM", "http://custom:5555");

        assert!(is_capability_available("custom"));
        assert!(!is_capability_available("nonexistent"));

        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::remove_var("NESTGATE_CAPABILITY_CUSTOM");
    }
}
