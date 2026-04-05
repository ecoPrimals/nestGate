// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Modern Capability-Based Port Discovery
//!
//! Replaces hardcoded port constants with runtime capability discovery.
//!
//! # Migration Strategy
//!
//! **3-Layer Fallback** (following primal self-knowledge principles):
//! 1. **Capability Discovery** - Discover service via capabilities (primal autonomy)
//! 2. **Environment Variables** - Read from environment (runtime configuration)
//! 3. **Safe Defaults** - Use sensible fallback (backward compatibility)
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_core::constants::capability_port_discovery;
//!
//! #[tokio::main]
//! async fn main() -> nestgate_core::Result<()> {
//!     // ✅ Modern approach: 3-layer discovery
//!     let api_port = capability_port_discovery::discover_api_port().await?;
//!     println!("API port: {}", api_port);
//!     Ok(())
//! }
//! ```
//!
//! # Primal Sovereignty
//!
//! This module respects primal sovereignty:
//! - No assumptions about other primals' ports
//! - Runtime discovery preferred over hardcoding
//! - Environment-driven configuration
//! - Self-knowledge only (no external assumptions)

// Consolidation: `ServiceRegistry` / `PrimalCapability` may be shared from `nestgate-types`.
use nestgate_types::error::{NestGateError, Result};
use std::env;
use std::sync::OnceLock;

/// Capability identifiers passed to [`CapabilityPortResolver::resolve_service_port`].
///
/// Callers (for example `nestgate-discovery` at startup) register a resolver that maps these
/// strings to listening ports. `NestGate` config stays free of any dependency on the discovery crate.
pub mod capability_ids {
    /// HTTP API gateway (REST/OpenAPI).
    pub const API_GATEWAY: &str = "api.gateway";
    /// Prometheus-style metrics / observability scrape endpoint.
    pub const OBSERVABILITY_METRICS: &str = "observability.metrics";
    /// ZFS-backed storage service.
    pub const STORAGE_ZFS: &str = "storage.zfs";
}

/// Injected runtime resolver: maps a capability string to a TCP port (primal self-knowledge).
///
/// Implementations live outside `nestgate-config` (for example in `nestgate-discovery`) and are
/// registered once at process startup via [`register_capability_resolver`].
pub trait CapabilityPortResolver: Send + Sync {
    /// Returns the listening port for `capability`, or `None` if unknown.
    fn resolve_service_port(&self, capability: &str) -> Option<u16>;
}

static CAPABILITY_RESOLVER: OnceLock<Box<dyn CapabilityPortResolver>> = OnceLock::new();

/// Register the global capability port resolver (typically once at startup).
///
/// If a resolver was already registered, this call is ignored (first registration wins).
pub fn register_capability_resolver(resolver: Box<dyn CapabilityPortResolver>) {
    let _ = CAPABILITY_RESOLVER.set(resolver);
}

// ==================== MODERN DISCOVERY FUNCTIONS ====================

/// Discover API service port using capability-based discovery
///
/// # Discovery Order
/// 1. Capability discovery (find service advertising `ApiGateway` capability)
/// 2. Environment variable (`NESTGATE_API_PORT`)
/// 3. Safe default (8080)
///
/// # Primal Sovereignty
/// Respects primal autonomy - discovers services at runtime without hardcoded assumptions.
///
/// # Example
/// ```rust,ignore
/// # use nestgate_core::constants::capability_port_discovery::discover_api_port;
/// # async fn example() -> nestgate_core::Result<()> {
/// let port = discover_api_port().await?;
/// println!("API available on port: {}", port);
/// # Ok(())
/// # }
/// ```
pub fn discover_api_port() -> Result<u16> {
    // 1. Try capability discovery
    if let Ok(service_url) = try_discover_api_service()
        && let Some(port) = extract_port_from_url(&service_url)
    {
        return Ok(port);
    }

    // 2. Try environment variable
    if let Ok(port_str) = env::var("NESTGATE_API_PORT")
        && let Ok(port) = port_str.parse::<u16>()
        && port > 0
    {
        return Ok(port);
    }

    // 3. Safe default (maintains backward compatibility)
    Ok(8080)
}

/// Discover metrics service port using capability-based discovery
///
/// # Discovery Order
/// 1. Capability discovery (Observability capability)
/// 2. Environment variable (`NESTGATE_METRICS_PORT`)
/// 3. Safe default (9090)
pub fn discover_metrics_port() -> Result<u16> {
    // 1. Try capability discovery
    if let Ok(service_url) = try_discover_metrics_service()
        && let Some(port) = extract_port_from_url(&service_url)
    {
        return Ok(port);
    }

    // 2. Try environment variable
    if let Ok(port_str) = env::var("NESTGATE_METRICS_PORT")
        && let Ok(port) = port_str.parse::<u16>()
        && port > 0
    {
        return Ok(port);
    }

    // 3. Safe default
    Ok(9090)
}

/// Discover health check port using capability-based discovery
///
/// # Discovery Order
/// 1. Capability discovery
/// 2. Environment variable (`NESTGATE_HEALTH_PORT`)
/// 3. Safe default (8082)
pub fn discover_health_port() -> Result<u16> {
    // 1. Environment variable (health checks are often load-balancer specific)
    if let Ok(port_str) = env::var("NESTGATE_HEALTH_PORT")
        && let Ok(port) = port_str.parse::<u16>()
        && port > 0
    {
        return Ok(port);
    }

    // 2. Safe default
    Ok(8082)
}

/// Discover admin interface port using capability-based discovery
///
/// # Discovery Order
/// 1. Environment variable (`NESTGATE_ADMIN_PORT`)
/// 2. Safe default (8081)
pub fn discover_admin_port() -> Result<u16> {
    // 1. Environment variable (admin interfaces are sensitive, explicit config preferred)
    if let Ok(port_str) = env::var("NESTGATE_ADMIN_PORT")
        && let Ok(port) = port_str.parse::<u16>()
        && port > 0
    {
        return Ok(port);
    }

    // 2. Safe default
    Ok(8081)
}

/// Discover storage service port using capability-based discovery
///
/// # Discovery Order
/// 1. Capability discovery (`ZfsStorage` capability)
/// 2. Environment variable (`NESTGATE_STORAGE_PORT`)
/// 3. Safe default (8083)
pub fn discover_storage_port() -> Result<u16> {
    // 1. Try capability discovery
    if let Ok(service_url) = try_discover_storage_service()
        && let Some(port) = extract_port_from_url(&service_url)
    {
        return Ok(port);
    }

    // 2. Try environment variable
    if let Ok(port_str) = env::var("NESTGATE_STORAGE_PORT")
        && let Ok(port) = port_str.parse::<u16>()
        && port > 0
    {
        return Ok(port);
    }

    // 3. Safe default
    Ok(8083)
}

/// Discover tarpc RPC service port using capability-based discovery
///
/// # Discovery Order
/// 1. Environment variable (`NESTGATE_TARPC_PORT`)
/// 2. Safe default (8091)
///
/// # Primal Sovereignty
/// tarpc is Rust-native high-performance RPC - discovered at runtime for flexibility
pub fn discover_tarpc_port() -> Result<u16> {
    // 1. Try environment variable
    if let Ok(port_str) = env::var("NESTGATE_TARPC_PORT")
        && let Ok(port) = port_str.parse::<u16>()
        && port > 0
    {
        return Ok(port);
    }

    // 2. Safe default
    Ok(8091)
}

// ==================== HELPER FUNCTIONS ====================

/// Try to discover API service (returns Err if not found, doesn't panic)
fn try_discover_api_service() -> Result<String> {
    if let Some(resolver) = CAPABILITY_RESOLVER.get()
        && let Some(port) = resolver.resolve_service_port(capability_ids::API_GATEWAY)
        && port > 0
    {
        return Ok(format!("http://localhost:{port}"));
    }
    Err(NestGateError::network_error(
        "Capability discovery: no API gateway port from resolver",
    ))
}

/// Try to discover metrics service (returns Err if not found)
fn try_discover_metrics_service() -> Result<String> {
    if let Some(resolver) = CAPABILITY_RESOLVER.get()
        && let Some(port) = resolver.resolve_service_port(capability_ids::OBSERVABILITY_METRICS)
        && port > 0
    {
        return Ok(format!("http://localhost:{port}"));
    }
    Err(NestGateError::network_error(
        "Capability discovery: no metrics port from resolver",
    ))
}

/// Try to discover storage service (returns Err if not found)
fn try_discover_storage_service() -> Result<String> {
    if let Some(resolver) = CAPABILITY_RESOLVER.get()
        && let Some(port) = resolver.resolve_service_port(capability_ids::STORAGE_ZFS)
        && port > 0
    {
        return Ok(format!("http://localhost:{port}"));
    }
    Err(NestGateError::network_error(
        "Capability discovery: no storage port from resolver",
    ))
}

/// Extract port number from URL string
///
/// # Examples
/// - `http://localhost:8080` → Some(8080)
/// - `https://api.example.com:443` → Some(443)
/// - `http://127.0.0.1` → None (no explicit port)
fn extract_port_from_url(url: &str) -> Option<u16> {
    // Simple port extraction - look for ":port" pattern
    url.split(':')
        .next_back()
        .and_then(|port_str| {
            // Remove trailing slashes or paths
            port_str
                .split('/')
                .next()
                .and_then(|clean_port| clean_port.parse::<u16>().ok())
        })
        .filter(|&port| port > 0)
}

// ==================== SYNCHRONOUS FALLBACKS ====================

/// Synchronous port discovery (for contexts where async is not available)
///
/// Uses only environment variables and defaults (no capability discovery).
/// Prefer [`discover_api_port`] when the full discovery chain is needed.
#[must_use]
pub fn discover_api_port_sync() -> u16 {
    env::var("NESTGATE_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&p| p > 0)
        .unwrap_or(8080)
}

/// Synchronous metrics port discovery
#[must_use]
pub fn discover_metrics_port_sync() -> u16 {
    env::var("NESTGATE_METRICS_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&p| p > 0)
        .unwrap_or(9090)
}

/// Synchronous health port discovery
#[must_use]
pub fn discover_health_port_sync() -> u16 {
    env::var("NESTGATE_HEALTH_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&p| p > 0)
        .unwrap_or(8082)
}

/// Synchronous admin port discovery
#[must_use]
pub fn discover_admin_port_sync() -> u16 {
    env::var("NESTGATE_ADMIN_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&p| p > 0)
        .unwrap_or(8081)
}

/// Synchronous tarpc port discovery
#[must_use]
pub fn discover_tarpc_port_sync() -> u16 {
    env::var("NESTGATE_TARPC_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&p| p > 0)
        .unwrap_or(8091)
}

// ==================== TESTS ====================
//
// Resolver injection is covered in integration tests (separate binaries so `OnceLock` registration
// does not leak across tests): `tests/capability_port_discovery_*.rs`.

#[cfg(test)]
mod tests {
    use super::*;
    use temp_env::{with_var, with_var_unset, with_vars};

    #[test]
    fn test_extract_port_from_url() {
        assert_eq!(extract_port_from_url("http://localhost:8080"), Some(8080));
        assert_eq!(
            extract_port_from_url("https://api.example.com:443"),
            Some(443)
        );
        assert_eq!(
            extract_port_from_url("http://127.0.0.1:3000/api"),
            Some(3000)
        );
        assert_eq!(extract_port_from_url("http://example.com"), None);
    }

    #[test]
    fn test_sync_discovery_defaults() {
        with_vars(
            vec![
                ("NESTGATE_API_PORT", None::<&str>),
                ("NESTGATE_METRICS_PORT", None::<&str>),
                ("NESTGATE_HEALTH_PORT", None::<&str>),
                ("NESTGATE_ADMIN_PORT", None::<&str>),
            ],
            || {
                assert_eq!(discover_api_port_sync(), 8080);
                assert_eq!(discover_metrics_port_sync(), 9090);
                assert_eq!(discover_health_port_sync(), 8082);
                assert_eq!(discover_admin_port_sync(), 8081);
            },
        );
    }

    #[test]
    fn test_sync_discovery_from_env() {
        with_vars(
            vec![
                ("NESTGATE_API_PORT", Some("9000")),
                ("NESTGATE_METRICS_PORT", Some("9999")),
            ],
            || {
                assert_eq!(discover_api_port_sync(), 9000);
                assert_eq!(discover_metrics_port_sync(), 9999);
            },
        );
    }

    #[test]
    fn test_sync_discovery_invalid_env() {
        with_var("NESTGATE_API_PORT", Some("invalid"), || {
            assert_eq!(discover_api_port_sync(), 8080);
        });
    }

    #[test]
    fn test_async_discovery_fallback() {
        with_var_unset("NESTGATE_API_PORT", || {
            let port = discover_api_port().unwrap();
            assert_eq!(port, 8080);
        });
    }

    #[test]
    fn test_async_discovery_from_env() {
        with_vars(vec![("NESTGATE_API_PORT", Some("9000"))], || {
            let port = discover_api_port().unwrap();
            assert_eq!(port, 9000);
        });
    }

    #[test]
    fn test_extract_port_edge_cases() {
        assert_eq!(extract_port_from_url("http://host:0"), None);
        assert_eq!(extract_port_from_url("http://host:65535/path"), Some(65535));
        assert_eq!(extract_port_from_url("http://host:443/"), Some(443));
    }

    #[test]
    fn test_discover_metrics_port_default() {
        with_var_unset("NESTGATE_METRICS_PORT", || {
            let port = discover_metrics_port().unwrap();
            assert_eq!(port, 9090);
        });
    }

    #[test]
    fn test_discover_health_port_default() {
        with_var_unset("NESTGATE_HEALTH_PORT", || {
            let port = discover_health_port().unwrap();
            assert_eq!(port, 8082);
        });
    }

    #[test]
    fn test_discover_admin_port_default() {
        with_var_unset("NESTGATE_ADMIN_PORT", || {
            let port = discover_admin_port().unwrap();
            assert_eq!(port, 8081);
        });
    }

    #[test]
    fn test_discover_storage_port_default() {
        with_var_unset("NESTGATE_STORAGE_PORT", || {
            let port = discover_storage_port().unwrap();
            assert_eq!(port, 8083);
        });
    }

    #[test]
    fn test_discover_tarpc_port_default() {
        with_var_unset("NESTGATE_TARPC_PORT", || {
            let port = discover_tarpc_port().unwrap();
            assert_eq!(port, 8091);
        });
    }

    #[test]
    fn test_tarpc_port_sync_default() {
        with_var_unset("NESTGATE_TARPC_PORT", || {
            assert_eq!(discover_tarpc_port_sync(), 8091);
        });
    }
}
