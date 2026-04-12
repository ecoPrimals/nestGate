// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability-Based Configuration Discovery
//!
//! This module implements capability-based service discovery for configuration,
//! eliminating hardcoded endpoints and enabling true primal sovereignty.
//!
//! # Architecture
//!
//! ```text
//! Configuration Loading Flow:
//!
//! 1. Capability Discovery (runtime, agnostic)
//!    ↓
//! 2. Environment Variables (deployment-specific)
//!    ↓
//! 3. Local Discovery (mDNS, service registry)
//!    ↓
//! 4. Intelligent Defaults (safe fallbacks)
//! ```
//!
//! # Examples
//!
//! ```rust,ignore
//! use nestgate_core::config::capability_discovery;
//!
//! # async fn example() -> nestgate_core::Result<()> {
//! // Discover storage service endpoint
//! let endpoint = capability_discovery::discover_service("storage")?;
//! println!("Found storage at: {}", endpoint.endpoint);
//!
//! // With fallback chain
//! let endpoint = capability_discovery::discover_with_fallback(
//!     "api",                          // Capability type
//!     "NESTGATE_API_ENDPOINT",        // Env var
//!     "http://localhost:8080"         // Default
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Sovereignty Compliance
//!
//! ✅ **Self-Knowledge Only**: Only announces own capabilities
//! ✅ **Runtime Discovery**: No hardcoded primal endpoints
//! ✅ **Agnostic**: Works across any deployment environment
//! ✅ **Fallback Safe**: Graceful degradation to defaults

use crate::constants::system::ecosystem_path_segment;
use nestgate_types::EnvSource;
use nestgate_types::ProcessEnv;
use nestgate_types::error::NestGateError;
use nestgate_types::error::Result;
use std::env;
use std::time::Duration;

// ==================== TYPES ====================

/// Service endpoint discovered through capability system
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceEndpoint {
    /// Service capability type (e.g., "storage", "compute", "api")
    pub capability: String,
    /// Full endpoint URL or address
    pub endpoint: String,
    /// Time-to-live for this discovery
    pub ttl: Duration,
    /// Source of this discovery
    pub source: DiscoverySource,
}

/// Source of a service discovery
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoverySource {
    /// Discovered through capability registry
    CapabilityRegistry,
    /// Loaded from environment variable
    Environment,
    /// Discovered via local mDNS
    LocalDiscovery,
    /// Using safe default
    Default,
}

// ==================== DISCOVERY FUNCTIONS ====================

/// Discover service endpoint through capability system using an injectable env source.
///
/// This is the primary discovery mechanism that respects primal sovereignty:
/// - No hardcoded endpoints
/// - Runtime discovery only
/// - Self-knowledge pattern
///
/// Production callers typically use [`discover_service`], which reads the real process environment.
/// Tests should pass [`MapEnv`](nestgate_types::MapEnv) here.
///
/// # Errors
///
/// Returns error if service cannot be discovered through any method
pub fn discover_service_with_env(
    capability: &str,
    env: &(impl EnvSource + ?Sized),
) -> Result<ServiceEndpoint> {
    // Try capability registry first (preferred)
    if let Ok(endpoint) = discover_from_capability_registry(capability) {
        return Ok(endpoint);
    }

    // Try environment variable
    let env_var = format!("NESTGATE_{}_ENDPOINT", capability.to_uppercase());
    if let Some(endpoint) = env.get(&env_var) {
        return Ok(ServiceEndpoint {
            capability: capability.to_string(),
            endpoint,
            ttl: Duration::from_secs(300),
            source: DiscoverySource::Environment,
        });
    }

    // Try local discovery (mDNS, etc.)
    if let Ok(endpoint) = discover_from_local(capability) {
        return Ok(endpoint);
    }

    // No service found
    Err(NestGateError::network_error(format!(
        "Service '{capability}' not found (tried: capability, environment, local discovery)"
    )))
}

/// Discover service endpoint through capability system using the real process environment.
///
/// For tests, use [`discover_service_with_env`] with [`MapEnv`](nestgate_types::MapEnv).
pub fn discover_service(capability: &str) -> Result<ServiceEndpoint> {
    discover_service_with_env(capability, &ProcessEnv)
}

/// Discover service with fallback to default
///
/// This provides the complete fallback chain for production use:
/// capability → environment → local → default
///
/// # Arguments
///
/// * `capability` - Service capability type to discover
/// * `env_var` - Environment variable name to check
/// * `default_endpoint` - Safe default if all discovery methods fail
///
/// # Errors
///
/// This function currently always returns [`Ok`]; the [`Result`] is reserved for future discovery
/// backends that may fail instead of falling back.
pub fn discover_with_fallback(
    capability: &str,
    env_var: &str,
    default_endpoint: &str,
) -> Result<ServiceEndpoint> {
    // Try full discovery chain
    if let Ok(endpoint) = discover_service(capability) {
        return Ok(endpoint);
    }

    // Check specific env var if provided
    if let Ok(value) = env::var(env_var) {
        return Ok(ServiceEndpoint {
            capability: capability.to_string(),
            endpoint: value,
            ttl: Duration::from_secs(300),
            source: DiscoverySource::Environment,
        });
    }

    // Use safe default
    Ok(ServiceEndpoint {
        capability: capability.to_string(),
        endpoint: default_endpoint.to_string(),
        ttl: Duration::from_secs(3600),
        source: DiscoverySource::Default,
    })
}

/// Announce own capabilities to discovery system
///
/// This implements the self-knowledge pattern:
/// - Announces what WE offer
/// - Does not assume anything about other primals
/// - Enables others to discover us
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::config::capability_discovery;
///
/// # async fn example() -> nestgate_core::Result<()> {
/// // Announce our API capability
/// capability_discovery::announce_capability(
///     "api",
///     "http://localhost:8080",
///     std::time::Duration::from_secs(60)
/// ).await?;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns [`NestGateError`] when the capability cannot be announced to the discovery system
/// (currently always returns [`Ok`] while integration is stubbed).
pub fn announce_capability(capability: &str, endpoint: &str, ttl: Duration) -> Result<()> {
    let manifest = serde_json::json!({
        "capability": capability,
        "endpoint": endpoint,
        "ttl_secs": ttl.as_secs(),
        "announced_at": chrono::Utc::now().to_rfc3339(),
        "pid": std::process::id(),
    });

    let manifest_dir = capability_manifest_dir();
    if let Err(e) = std::fs::create_dir_all(&manifest_dir) {
        tracing::warn!(
            "Could not create capability manifest directory {}: {e}",
            manifest_dir.display()
        );
        tracing::info!(
            "Announcing capability '{}' at '{}' (TTL: {:?}) (log only; manifest write failed)",
            capability,
            endpoint,
            ttl
        );
        return Ok(());
    }

    let manifest_path = manifest_dir.join(format!("{capability}.json"));
    match std::fs::write(
        &manifest_path,
        serde_json::to_vec_pretty(&manifest).unwrap_or_default(),
    ) {
        Ok(()) => {
            tracing::info!(
                "Announced capability '{}' at '{}' -> {}",
                capability,
                endpoint,
                manifest_path.display()
            );
        }
        Err(e) => {
            tracing::warn!(
                "Could not write capability manifest {}: {e}",
                manifest_path.display()
            );
        }
    }

    Ok(())
}

/// Directory where capability manifests are written for peer discovery.
///
/// Resolution: `$XDG_RUNTIME_DIR/<ecosystem>/nestgate/capabilities/` (see [`ecosystem_path_segment`]) or
/// `/tmp/nestgate-capabilities/` as fallback.
fn capability_manifest_dir() -> std::path::PathBuf {
    if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
        let p = std::path::Path::new(&xdg);
        if p.exists() {
            return p
                .join(ecosystem_path_segment())
                .join("nestgate")
                .join("capabilities");
        }
    }
    std::path::PathBuf::from("/tmp/nestgate-capabilities")
}

// ==================== INTERNAL DISCOVERY METHODS ====================

/// Discover from capability manifest files written by [`announce_capability`].
fn discover_from_capability_registry(capability: &str) -> Result<ServiceEndpoint> {
    let manifest_path = capability_manifest_dir().join(format!("{capability}.json"));
    let bytes = std::fs::read(&manifest_path).map_err(|_| {
        NestGateError::network_error(format!(
            "No capability manifest for '{capability}' at {}",
            manifest_path.display()
        ))
    })?;
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| {
        NestGateError::network_error(format!(
            "Invalid capability manifest for '{capability}': {e}"
        ))
    })?;
    let endpoint = value["endpoint"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::network_error(format!(
                "Capability manifest for '{capability}' missing 'endpoint' field"
            ))
        })?
        .to_string();
    let ttl_secs = value["ttl_secs"].as_u64().unwrap_or(60);

    Ok(ServiceEndpoint {
        capability: capability.to_string(),
        endpoint,
        ttl: Duration::from_secs(ttl_secs),
        source: DiscoverySource::CapabilityRegistry,
    })
}

/// Discover from local network (mDNS, etc.)
fn discover_from_local(capability: &str) -> Result<ServiceEndpoint> {
    tracing::debug!("feature pending: mDNS/local discovery without nestgate-core DiscoveryBuilder");
    let _ = capability;
    Err(NestGateError::network_error(
        "Local mDNS discovery unavailable (nestgate-core decoupled)",
    ))
}

// ==================== HELPER FUNCTIONS ====================

/// Parse endpoint string into components
///
/// Validates and parses endpoints like:
/// - `http://localhost:8080`
/// - `localhost:8080`
/// - `192.168.1.100:9090`
///
/// # Errors
///
/// Returns [`NestGateError`] when the endpoint string is not a valid URL or `host:port` pair.
pub fn parse_endpoint(endpoint: &str) -> Result<(String, u16)> {
    // Handle full URLs
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        return parse_url_endpoint(endpoint);
    }

    // Handle host:port format
    parse_hostport_endpoint(endpoint)
}

fn parse_url_endpoint(url: &str) -> Result<(String, u16)> {
    // Simple URL parsing (for production, use url crate)
    let without_protocol = url
        .trim_start_matches("http://")
        .trim_start_matches("https://");

    parse_hostport_endpoint(without_protocol)
}

fn parse_hostport_endpoint(hostport: &str) -> Result<(String, u16)> {
    let parts: Vec<&str> = hostport.split(':').collect();

    if parts.len() != 2 {
        return Err(NestGateError::validation_error(format!(
            "Invalid endpoint format '{hostport}': expected 'host:port'"
        )));
    }

    let host = parts[0];
    let port = parts[1].parse::<u16>().map_err(|e| {
        NestGateError::validation_error(format!("Invalid port '{}': {}", parts[1], e))
    })?;

    if host.is_empty() {
        return Err(NestGateError::validation_error("Host cannot be empty"));
    }

    Ok((host.to_string(), port))
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_with_fallback_uses_default() {
        let result =
            discover_with_fallback("test_service", "NONEXISTENT_ENV_VAR", "localhost:9999");

        assert!(result.is_ok());
        let endpoint = result.unwrap();
        assert_eq!(endpoint.endpoint, "localhost:9999");
        assert_eq!(endpoint.source, DiscoverySource::Default);
    }

    #[test]
    fn test_discover_with_fallback_uses_env() {
        let orig = env::var("TEST_SERVICE_ENDPOINT").ok();
        crate::env_process::set_var("TEST_SERVICE_ENDPOINT", "envhost:8888");

        let result =
            discover_with_fallback("test_service", "TEST_SERVICE_ENDPOINT", "localhost:9999");

        match orig {
            Some(v) => crate::env_process::set_var("TEST_SERVICE_ENDPOINT", v),
            None => crate::env_process::remove_var("TEST_SERVICE_ENDPOINT"),
        }
        assert!(result.is_ok());
        let endpoint = result.unwrap();
        assert_eq!(endpoint.endpoint, "envhost:8888");
        assert_eq!(endpoint.source, DiscoverySource::Environment);
    }

    #[test]
    fn test_parse_endpoint_hostport() {
        let result = parse_endpoint("localhost:8080");
        assert!(result.is_ok());

        let (host, port) = result.unwrap();
        assert_eq!(host, "localhost");
        assert_eq!(port, 8080);
    }

    #[test]
    fn test_parse_endpoint_http_url() {
        let result = parse_endpoint("http://example.com:3000");
        assert!(result.is_ok());

        let (host, port) = result.unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 3000);
    }

    #[test]
    fn test_parse_endpoint_invalid() {
        assert!(parse_endpoint("invalid").is_err());
        assert!(parse_endpoint(":8080").is_err());
        assert!(parse_endpoint("localhost:").is_err());
        assert!(parse_endpoint("localhost:not_a_port").is_err());
    }

    #[test]
    fn test_announce_capability() {
        let result = announce_capability("test", "localhost:8080", Duration::from_secs(60));

        // Announcement may fail in test environment without mDNS service
        // Just verify it doesn't panic
        let _ = result;
    }
}
