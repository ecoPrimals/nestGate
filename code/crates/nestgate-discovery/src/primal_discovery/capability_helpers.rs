// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unused_async,
    reason = "Async discovery API preserved for forward-compatible awaits; implementations are sync today"
)]

//! Capability-Based Discovery Helpers
//!
//! High-level helpers for discovering and connecting to primals by capability.
//! These replace hardcoded URLs and port numbers with runtime discovery.
//!
//! # Philosophy
//!
//! **Old Way** (hardcoded):
//! ```rust,ignore
//! let security_url = "http://localhost:3000"; // ❌ Hardcoded!
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
//! // Discover orchestration capability provider
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

use nestgate_config::constants::PortConfig;
use nestgate_config::constants::hardcoding::runtime_fallback_ports as fallback_ports;
use nestgate_types::error::{NestGateError, Result};
use nestgate_types::{EnvSource, ProcessEnv, env_parsed};

/// Last-resort host for capability bootstrap when discovery and env URLs are unset.
/// Prefer `NESTGATE_<CAPABILITY>_HOST` (e.g. `NESTGATE_ORCHESTRATION_HOST`), then
/// `NESTGATE_DISCOVERY_FALLBACK_HOST`, then loopback.
fn fallback_host_for_capability_from_env(
    env: &(impl EnvSource + ?Sized),
    capability: &str,
) -> String {
    let specific = format!("NESTGATE_{}_HOST", capability.to_uppercase());
    env.get(&specific)
        .or_else(|| env.get("NESTGATE_DISCOVERY_FALLBACK_HOST"))
        .unwrap_or_else(|| "127.0.0.1".to_string())
}

fn default_port_compute_from_env(env: &(impl EnvSource + ?Sized)) -> u16 {
    env_parsed(env, "NESTGATE_COMPUTE_PORT", fallback_ports::COMPUTE)
}

fn default_port_ai_from_env(env: &(impl EnvSource + ?Sized)) -> u16 {
    env_parsed(env, "NESTGATE_AI_PORT", fallback_ports::ADMIN)
}

fn default_port_ecosystem_from_env(env: &(impl EnvSource + ?Sized)) -> u16 {
    env_parsed(env, "NESTGATE_ECOSYSTEM_PORT", fallback_ports::ECOSYSTEM)
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
    /// Discovered via capability IPC or orchestration provider
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

/// Discover orchestration capability provider.
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_ORCHESTRATION` (from peer discovery or operator)
/// 2. Development default fallback
///
/// # Errors
///
/// Infallible — always falls back to a development default.
pub async fn discover_orchestration() -> Result<DiscoveredService> {
    discover_orchestration_from_env(&ProcessEnv).await
}

/// Like [`discover_orchestration`], but reads capability and port env vars from an injectable source.
pub async fn discover_orchestration_from_env(
    env: &(impl EnvSource + ?Sized),
) -> Result<DiscoveredService> {
    Ok(discover_capability_with_default_from_env(
        env,
        "orchestration",
        |e| PortConfig::from_env_source(e).get_api_port(),
    ))
}

/// Discover whichever peer provides the `security` capability.
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_SECURITY`
/// 2. Development default fallback
pub async fn discover_security() -> Result<DiscoveredService> {
    discover_security_from_env(&ProcessEnv).await
}

/// Like [`discover_security`], but reads capability and port env vars from an injectable source.
pub async fn discover_security_from_env(
    env: &(impl EnvSource + ?Sized),
) -> Result<DiscoveredService> {
    Ok(discover_capability_with_default_from_env(
        env,
        "security",
        |e| PortConfig::from_env_source(e).get_dev_port(),
    ))
}

/// Discover compute capability provider.
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_COMPUTE`
/// 2. Development default fallback
pub async fn discover_compute() -> Result<DiscoveredService> {
    discover_compute_from_env(&ProcessEnv).await
}

/// Like [`discover_compute`], but reads capability and port env vars from an injectable source.
pub async fn discover_compute_from_env(
    env: &(impl EnvSource + ?Sized),
) -> Result<DiscoveredService> {
    Ok(discover_capability_with_default_from_env(
        env,
        "compute",
        default_port_compute_from_env,
    ))
}

/// Discover AI capability provider.
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_AI`
/// 2. Development default fallback
pub async fn discover_ai() -> Result<DiscoveredService> {
    discover_ai_from_env(&ProcessEnv).await
}

/// Like [`discover_ai`], but reads capability and port env vars from an injectable source.
pub async fn discover_ai_from_env(env: &(impl EnvSource + ?Sized)) -> Result<DiscoveredService> {
    Ok(discover_capability_with_default_from_env(
        env,
        "ai",
        default_port_ai_from_env,
    ))
}

/// Discover ecosystem capability provider.
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_ECOSYSTEM`
/// 2. Development default fallback
pub async fn discover_ecosystem() -> Result<DiscoveredService> {
    discover_ecosystem_from_env(&ProcessEnv).await
}

/// Like [`discover_ecosystem`], but reads capability and port env vars from an injectable source.
pub async fn discover_ecosystem_from_env(
    env: &(impl EnvSource + ?Sized),
) -> Result<DiscoveredService> {
    Ok(discover_capability_with_default_from_env(
        env,
        "ecosystem",
        default_port_ecosystem_from_env,
    ))
}

// ==================== CORE DISCOVERY LOGIC ====================

/// Discover any service by capability
///
/// **Priority**:
/// 1. `NESTGATE_CAPABILITY_{CAPABILITY}` environment variable (set by the orchestration provider or operator)
/// 2. Error (no fallback for arbitrary capabilities)
///
/// # Errors
///
/// Returns error if capability not found via environment.
pub async fn discover_capability(capability: &str) -> Result<DiscoveredService> {
    discover_capability_from_env(&ProcessEnv, capability).await
}

/// Like [`discover_capability`], but reads from an injectable [`EnvSource`].
pub async fn discover_capability_from_env(
    env: &(impl EnvSource + ?Sized),
    capability: &str,
) -> Result<DiscoveredService> {
    let env_var = format!("NESTGATE_CAPABILITY_{}", capability.to_uppercase());
    if let Some(endpoint) = env.get(&env_var) {
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

/// Resolve a known capability: `NESTGATE_CAPABILITY_{CAPABILITY}` or development default.
fn discover_capability_with_default_from_env<E: EnvSource + ?Sized>(
    env: &E,
    capability: &str,
    default_port: impl FnOnce(&E) -> u16,
) -> DiscoveredService {
    let capability_env_var = format!("NESTGATE_CAPABILITY_{}", capability.to_uppercase());
    if let Some(endpoint) = env.get(&capability_env_var) {
        tracing::info!(
            "Discovered '{}' capability from environment: {}",
            capability,
            endpoint
        );
        return DiscoveredService::from_env(capability, endpoint);
    }

    let host = fallback_host_for_capability_from_env(env, capability);
    let default_endpoint = format!("http://{}:{}", host, default_port(env));
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
    is_capability_available_from_env(&ProcessEnv, capability)
}

/// Like [`is_capability_available`], but reads from an injectable [`EnvSource`].
pub fn is_capability_available_from_env(env: &(impl EnvSource + ?Sized), capability: &str) -> bool {
    let env_var = format!("NESTGATE_CAPABILITY_{}", capability.to_uppercase());
    env.get(&env_var).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;

    #[tokio::test]
    async fn test_discover_with_env_var() {
        let env = MapEnv::from([("NESTGATE_CAPABILITY_TEST", "http://test:1234")]);
        let result = discover_capability_from_env(&env, "test").await;
        let service = match result {
            Ok(s) => s,
            Err(e) => panic!("discover_capability_from_env: {e:?}"),
        };
        assert_eq!(service.endpoint, "http://test:1234");
        assert_eq!(service.source, DiscoverySource::Environment);
    }

    #[tokio::test]
    async fn discover_capability_missing_returns_network_error() {
        let env = MapEnv::new();
        let result = discover_capability_from_env(&env, "absent_capability").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_discover_without_config_falls_back() {
        // Orchestration should fall back to default
        let env = MapEnv::new();
        let result = discover_orchestration_from_env(&env).await;
        let service = match result {
            Ok(s) => s,
            Err(e) => panic!("discover_orchestration_from_env: {e:?}"),
        };
        assert_eq!(service.source, DiscoverySource::Default);
        assert!(service.endpoint.contains("127.0.0.1"));
    }

    #[tokio::test]
    async fn discover_orchestration_uses_specific_capability_host_env() {
        let env = MapEnv::from([("NESTGATE_ORCHESTRATION_HOST", "10.10.0.1")]);
        let result = discover_orchestration_from_env(&env).await;
        let service = match result {
            Ok(s) => s,
            Err(e) => panic!("discover_orchestration_from_env: {e:?}"),
        };
        assert_eq!(service.source, DiscoverySource::Default);
        assert!(service.endpoint.contains("10.10.0.1"));
    }

    #[tokio::test]
    async fn discover_orchestration_prefers_discovery_fallback_host() {
        let env = MapEnv::from([("NESTGATE_DISCOVERY_FALLBACK_HOST", "192.168.0.5")]);
        let result = discover_orchestration_from_env(&env).await;
        let service = match result {
            Ok(s) => s,
            Err(e) => panic!("discover_orchestration_from_env: {e:?}"),
        };
        assert!(service.endpoint.contains("192.168.0.5"));
    }

    #[tokio::test]
    async fn discover_compute_from_capability_env() {
        let env = MapEnv::from([("NESTGATE_CAPABILITY_COMPUTE", "http://compute:9000")]);
        let result = discover_compute_from_env(&env).await;
        let service = match result {
            Ok(s) => s,
            Err(e) => panic!("discover_compute_from_env: {e:?}"),
        };
        assert_eq!(service.endpoint, "http://compute:9000");
        assert_eq!(service.source, DiscoverySource::Environment);
    }

    #[tokio::test]
    async fn discover_ai_respects_nestgate_ai_port() {
        let env = MapEnv::from([("NESTGATE_AI_PORT", "7654")]);
        let result = discover_ai_from_env(&env).await;
        let service = match result {
            Ok(s) => s,
            Err(e) => panic!("discover_ai_from_env: {e:?}"),
        };
        assert_eq!(service.source, DiscoverySource::Default);
        assert!(service.endpoint.ends_with(":7654"));
    }

    #[tokio::test]
    async fn discover_ecosystem_fallback_has_default_shape() {
        let env = MapEnv::new();
        let result = discover_ecosystem_from_env(&env).await;
        let service = match result {
            Ok(s) => s,
            Err(e) => panic!("discover_ecosystem_from_env: {e:?}"),
        };
        assert_eq!(service.source, DiscoverySource::Default);
        assert!(service.endpoint.starts_with("http://"));
    }

    #[tokio::test]
    async fn test_primal_name_url_env_vars_are_not_used() {
        let env = MapEnv::from([("NESTGATE_LEGACY_PRIMAL_URL", "http://legacy:9999")]);
        let result = discover_security_from_env(&env).await;
        let service = match result {
            Ok(s) => s,
            Err(e) => panic!("discover_security_from_env: {e:?}"),
        };
        assert_eq!(service.source, DiscoverySource::Default);
        assert!(service.endpoint.contains("127.0.0.1"));
    }

    #[tokio::test]
    async fn test_discover_security_from_capability_env() {
        let env = MapEnv::from([("NESTGATE_CAPABILITY_SECURITY", "http://sec:7777")]);
        let result = discover_security_from_env(&env).await;
        let service = match result {
            Ok(s) => s,
            Err(e) => panic!("discover_security_from_env: {e:?}"),
        };
        assert_eq!(service.endpoint, "http://sec:7777");
        assert_eq!(service.source, DiscoverySource::Environment);
    }

    #[tokio::test]
    async fn test_is_capability_available() {
        let env = MapEnv::from([("NESTGATE_CAPABILITY_CUSTOM", "http://custom:5555")]);
        assert!(is_capability_available_from_env(&env, "custom"));
        assert!(!is_capability_available_from_env(&env, "nonexistent"));
    }
}
