// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
//! // Discover security service (e.g., BearDog)
//! let security = discover_security().await?;
//! println!("Security at: {}", security.endpoint);
//!
//! // Or discover any capability
//! let service = discover_capability("compute").await?;
//! # Ok(())
//! # }
//! ```

use crate::discovery_mechanism::{DiscoveryBuilder, ServiceInfo};
use nestgate_types::error::{NestGateError, Result};

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
    /// Create from `ServiceInfo` (discovery)
    fn from_service_info(info: ServiceInfo) -> Self {
        Self {
            name: info.name.clone(),
            endpoint: info.endpoint.clone(),
            capabilities: info.capabilities,
            source: DiscoverySource::Discovery,
        }
    }

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

/// Discover orchestration service (e.g., Songbird)
///
/// **Priority**:
/// 1. Discovery (mDNS/Consul/k8s)
/// 2. `NESTGATE_CAPABILITY_ORCHESTRATION` environment variable
/// 3. `NESTGATE_SONGBIRD_URL` (deprecated, backward compat)
/// 4. Last-resort default: `http://127.0.0.1:8080`
///
/// # Errors
///
/// Returns error only if all discovery methods fail and no fallback is available.
pub async fn discover_orchestration() -> Result<DiscoveredService> {
    discover_capability_with_legacy("orchestration", "SONGBIRD", 8080).await
}

/// Discover security service (e.g., `BearDog`)
///
/// **Priority**:
/// 1. Discovery (mDNS/Consul/k8s)
/// 2. `NESTGATE_CAPABILITY_SECURITY` environment variable
/// 3. `NESTGATE_BEARDOG_URL` (deprecated, backward compat)
/// 4. Last-resort default: `http://127.0.0.1:3000`
pub async fn discover_security() -> Result<DiscoveredService> {
    discover_capability_with_legacy("security", "BEARDOG", 3000).await
}

/// Discover compute service (e.g., `ToadStool`)
///
/// **Priority**:
/// 1. Discovery (mDNS/Consul/k8s)
/// 2. `NESTGATE_CAPABILITY_COMPUTE` environment variable
/// 3. `NESTGATE_TOADSTOOL_URL` (deprecated, backward compat)
/// 4. Last-resort default: `http://127.0.0.1:7070`
pub async fn discover_compute() -> Result<DiscoveredService> {
    discover_capability_with_legacy("compute", "TOADSTOOL", 7070).await
}

/// Discover AI service (e.g., Squirrel)
///
/// **Priority**:
/// 1. Discovery (mDNS/Consul/k8s)
/// 2. `NESTGATE_CAPABILITY_AI` environment variable
/// 3. `NESTGATE_SQUIRREL_URL` (deprecated, backward compat)
/// 4. Last-resort default: `http://127.0.0.1:9000`
pub async fn discover_ai() -> Result<DiscoveredService> {
    discover_capability_with_legacy("ai", "SQUIRREL", 9000).await
}

/// Discover ecosystem service (e.g., `BiomeOS`)
///
/// **Priority**:
/// 1. Discovery (mDNS/Consul/k8s)
/// 2. `NESTGATE_CAPABILITY_ECOSYSTEM` environment variable
/// 3. `NESTGATE_BIOMEOS_URL` (deprecated, backward compat)
/// 4. Last-resort default: `http://127.0.0.1:6000`
pub async fn discover_ecosystem() -> Result<DiscoveredService> {
    discover_capability_with_legacy("ecosystem", "BIOMEOS", 6000).await
}

// ==================== CORE DISCOVERY LOGIC ====================

/// Discover any service by capability
///
/// **Priority**:
/// 1. Discovery (mDNS/Consul/k8s)
/// 2. `NESTGATE_CAPABILITY_{CAPABILITY}` environment variable
/// 3. Error (no last-resort default for arbitrary capabilities)
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::primal_discovery::capability_helpers::*;
///
/// # async fn example() -> anyhow::Result<()> {
/// // Discover storage capability
/// let storage = discover_capability("storage").await?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns error if capability not found via discovery or environment.
pub async fn discover_capability(capability: &str) -> Result<DiscoveredService> {
    // Try discovery first
    if let Ok(service) = try_discovery(capability).await {
        return Ok(service);
    }

    // Try capability-based environment variable
    let env_var = format!("NESTGATE_CAPABILITY_{}", capability.to_uppercase());
    if let Ok(endpoint) = std::env::var(&env_var) {
        tracing::info!(
            "Discovered '{}' capability from environment: {}",
            capability,
            endpoint
        );
        return Ok(DiscoveredService::from_env(capability, endpoint));
    }

    // No fallback for arbitrary capabilities
    Err(NestGateError::network_error(&format!(
        "Capability '{capability}' not found via discovery or environment. Set {env_var} or use service discovery."
    )))
}

/// Discover capability with legacy environment variable support
///
/// Internal helper that provides backward compatibility with deprecated
/// primal-specific environment variables.
async fn discover_capability_with_legacy(
    capability: &str,
    legacy_name: &str,
    default_port: u16,
) -> Result<DiscoveredService> {
    // Try discovery first
    if let Ok(service) = try_discovery(capability).await {
        return Ok(service);
    }

    // Try capability-based environment variable (NEW)
    let capability_env_var = format!("NESTGATE_CAPABILITY_{}", capability.to_uppercase());
    if let Ok(endpoint) = std::env::var(&capability_env_var) {
        tracing::info!(
            "Discovered '{}' capability from environment: {}",
            capability,
            endpoint
        );
        return Ok(DiscoveredService::from_env(capability, endpoint));
    }

    // Try legacy primal-specific environment variable (DEPRECATED)
    let legacy_env_var = format!("NESTGATE_{legacy_name}_URL");
    if let Ok(endpoint) = std::env::var(&legacy_env_var) {
        tracing::warn!(
            "Using deprecated environment variable '{}'. Please migrate to '{}'",
            legacy_env_var,
            capability_env_var
        );
        return Ok(DiscoveredService::from_env(capability, endpoint));
    }

    // Last-resort default for development
    let default_endpoint = format!("http://127.0.0.1:{default_port}");
    tracing::warn!(
        "No discovery or environment configuration for '{}' capability. Using development default: {}. \
         In production, use service discovery or set {} environment variable.",
        capability,
        default_endpoint,
        capability_env_var
    );

    Ok(DiscoveredService::from_default(
        capability,
        default_endpoint,
    ))
}

/// Try to discover service via discovery mechanism
async fn try_discovery(capability: &str) -> Result<DiscoveredService> {
    // Auto-detect discovery mechanism
    let discovery = DiscoveryBuilder::default().detect().await?;

    // Query by capability
    let services = discovery.find_by_capability(capability.to_string()).await?;

    // Return first service found
    if let Some(service) = services.first() {
        tracing::info!(
            "Discovered '{}' capability via discovery: {} at {}",
            capability,
            service.name,
            service.endpoint
        );
        Ok(DiscoveredService::from_service_info(service.clone()))
    } else {
        Err(NestGateError::network_error(&format!(
            "No service found providing '{capability}' capability"
        )))
    }
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
/// if is_capability_available("orchestration").await {
///     let service = discover_orchestration().await?;
///     // Use service
/// } else {
///     // Handle absence gracefully
/// }
/// # Ok(())
/// # }
/// ```
pub async fn is_capability_available(capability: &str) -> bool {
    // Try discovery
    if try_discovery(capability).await.is_ok() {
        return true;
    }

    // Try environment
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

        assert!(is_capability_available("custom").await);
        assert!(!is_capability_available("nonexistent").await);

        // SAFETY: single-threaded test context.
        nestgate_platform::env_process::remove_var("NESTGATE_CAPABILITY_CUSTOM");
    }
}
