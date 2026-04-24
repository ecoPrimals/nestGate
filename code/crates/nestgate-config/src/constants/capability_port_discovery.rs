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
use nestgate_types::{EnvSource, ProcessEnv};
use std::sync::OnceLock;

fn parse_positive_port(env: &(impl EnvSource + ?Sized), key: &str) -> Option<u16> {
    env.get(key).and_then(|s| s.parse().ok()).filter(|&p| p > 0)
}

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
    discover_api_port_from_env_source(&ProcessEnv)
}

/// Like [`discover_api_port`], but reads `NESTGATE_API_PORT` from `env`.
pub fn discover_api_port_from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<u16> {
    // 1. Try capability discovery
    if let Ok(service_url) = try_discover_api_service()
        && let Some(port) = extract_port_from_url(&service_url)
    {
        return Ok(port);
    }

    // 2. Try environment variable
    if let Some(port) = parse_positive_port(env, "NESTGATE_API_PORT") {
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
    discover_metrics_port_from_env_source(&ProcessEnv)
}

/// Like [`discover_metrics_port`], but reads `NESTGATE_METRICS_PORT` from `env`.
pub fn discover_metrics_port_from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<u16> {
    // 1. Try capability discovery
    if let Ok(service_url) = try_discover_metrics_service()
        && let Some(port) = extract_port_from_url(&service_url)
    {
        return Ok(port);
    }

    // 2. Try environment variable
    if let Some(port) = parse_positive_port(env, "NESTGATE_METRICS_PORT") {
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
    discover_health_port_from_env_source(&ProcessEnv)
}

/// Like [`discover_health_port`], but reads `NESTGATE_HEALTH_PORT` from `env`.
pub fn discover_health_port_from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<u16> {
    // 1. Environment variable (health checks are often load-balancer specific)
    if let Some(port) = parse_positive_port(env, "NESTGATE_HEALTH_PORT") {
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
    discover_admin_port_from_env_source(&ProcessEnv)
}

/// Like [`discover_admin_port`], but reads `NESTGATE_ADMIN_PORT` from `env`.
pub fn discover_admin_port_from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<u16> {
    // 1. Environment variable (admin interfaces are sensitive, explicit config preferred)
    if let Some(port) = parse_positive_port(env, "NESTGATE_ADMIN_PORT") {
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
    discover_storage_port_from_env_source(&ProcessEnv)
}

/// Like [`discover_storage_port`], but reads `NESTGATE_STORAGE_PORT` from `env`.
pub fn discover_storage_port_from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<u16> {
    // 1. Try capability discovery
    if let Ok(service_url) = try_discover_storage_service()
        && let Some(port) = extract_port_from_url(&service_url)
    {
        return Ok(port);
    }

    // 2. Try environment variable
    if let Some(port) = parse_positive_port(env, "NESTGATE_STORAGE_PORT") {
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
    discover_tarpc_port_from_env_source(&ProcessEnv)
}

/// Like [`discover_tarpc_port`], but reads `NESTGATE_TARPC_PORT` from `env`.
pub fn discover_tarpc_port_from_env_source(env: &(impl EnvSource + ?Sized)) -> Result<u16> {
    // 1. Try environment variable
    if let Some(port) = parse_positive_port(env, "NESTGATE_TARPC_PORT") {
        return Ok(port);
    }

    // 2. Safe default
    Ok(super::hardcoding::runtime_fallback_ports::TARPC)
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
    discover_api_port_sync_from_env_source(&ProcessEnv)
}

/// Like [`discover_api_port_sync`], but reads from `env`.
#[must_use]
pub fn discover_api_port_sync_from_env_source(env: &(impl EnvSource + ?Sized)) -> u16 {
    parse_positive_port(env, "NESTGATE_API_PORT").unwrap_or(8080)
}

/// Synchronous metrics port discovery
#[must_use]
pub fn discover_metrics_port_sync() -> u16 {
    discover_metrics_port_sync_from_env_source(&ProcessEnv)
}

/// Like [`discover_metrics_port_sync`], but reads from `env`.
#[must_use]
pub fn discover_metrics_port_sync_from_env_source(env: &(impl EnvSource + ?Sized)) -> u16 {
    parse_positive_port(env, "NESTGATE_METRICS_PORT").unwrap_or(9090)
}

/// Synchronous health port discovery
#[must_use]
pub fn discover_health_port_sync() -> u16 {
    discover_health_port_sync_from_env_source(&ProcessEnv)
}

/// Like [`discover_health_port_sync`], but reads from `env`.
#[must_use]
pub fn discover_health_port_sync_from_env_source(env: &(impl EnvSource + ?Sized)) -> u16 {
    parse_positive_port(env, "NESTGATE_HEALTH_PORT").unwrap_or(8082)
}

/// Synchronous admin port discovery
#[must_use]
pub fn discover_admin_port_sync() -> u16 {
    discover_admin_port_sync_from_env_source(&ProcessEnv)
}

/// Like [`discover_admin_port_sync`], but reads from `env`.
#[must_use]
pub fn discover_admin_port_sync_from_env_source(env: &(impl EnvSource + ?Sized)) -> u16 {
    parse_positive_port(env, "NESTGATE_ADMIN_PORT").unwrap_or(8081)
}

/// Synchronous tarpc port discovery
#[must_use]
pub fn discover_tarpc_port_sync() -> u16 {
    discover_tarpc_port_sync_from_env_source(&ProcessEnv)
}

/// Like [`discover_tarpc_port_sync`], but reads from `env`.
#[must_use]
pub fn discover_tarpc_port_sync_from_env_source(env: &(impl EnvSource + ?Sized)) -> u16 {
    parse_positive_port(env, "NESTGATE_TARPC_PORT")
        .unwrap_or(super::hardcoding::runtime_fallback_ports::TARPC)
}

// ==================== RUNTIME PORT RESOLVER ====================

/// Unified port resolver implementing the three-tier resolution strategy:
/// 1. Registered `CapabilityPortResolver` (runtime discovery)
/// 2. `NESTGATE_*` environment variable
/// 3. Ephemeral port binding (`TcpListener::bind("127.0.0.1:0")`) or safe default
///
/// Prefer this over importing raw port constants from `runtime_fallback_ports`.
pub struct RuntimePortResolver;

impl RuntimePortResolver {
    /// Resolve a port by capability id and env var name.
    ///
    /// Tries: capability resolver → env var → ephemeral bind.
    #[must_use]
    pub fn resolve(capability_id: &str, env_var: &str) -> u16 {
        Self::resolve_from_env_source(&ProcessEnv, capability_id, env_var)
    }

    /// Same as [`Self::resolve`] with an injectable `EnvSource`.
    pub fn resolve_from_env_source(
        env: &(impl EnvSource + ?Sized),
        capability_id: &str,
        env_var: &str,
    ) -> u16 {
        if let Some(resolver) = CAPABILITY_RESOLVER.get()
            && let Some(port) = resolver.resolve_service_port(capability_id)
            && port > 0
        {
            return port;
        }

        if let Some(port) = parse_positive_port(env, env_var) {
            return port;
        }

        Self::ephemeral_port()
    }

    /// Resolve a port using only env var (no capability id), with a static default fallback.
    ///
    /// Use this for ports that don't have a capability mapping (e.g. database ports which
    /// belong to external services, not primal capabilities).
    #[must_use]
    pub fn resolve_env_or_default(env_var: &str, default: u16) -> u16 {
        Self::resolve_env_or_default_from_env_source(&ProcessEnv, env_var, default)
    }

    /// Same as [`Self::resolve_env_or_default`] with an injectable `EnvSource`.
    pub fn resolve_env_or_default_from_env_source(
        env: &(impl EnvSource + ?Sized),
        env_var: &str,
        default: u16,
    ) -> u16 {
        parse_positive_port(env, env_var).unwrap_or(default)
    }

    /// Bind to an ephemeral port and return the OS-assigned number.
    /// Falls back to 0 if binding fails (callers should treat 0 as "unconfigured").
    fn ephemeral_port() -> u16 {
        std::net::TcpListener::bind("127.0.0.1:0")
            .ok()
            .and_then(|l| l.local_addr().ok())
            .map_or(0, |a| a.port())
    }
}

// ==================== TESTS ====================
//
// Resolver injection is covered in integration tests (separate binaries so `OnceLock` registration
// does not leak across tests): `tests/capability_port_discovery_*.rs`.

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;

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
        let env = MapEnv::new();
        assert_eq!(discover_api_port_sync_from_env_source(&env), 8080);
        assert_eq!(discover_metrics_port_sync_from_env_source(&env), 9090);
        assert_eq!(discover_health_port_sync_from_env_source(&env), 8082);
        assert_eq!(discover_admin_port_sync_from_env_source(&env), 8081);
    }

    #[test]
    fn test_sync_discovery_from_env() {
        let env = MapEnv::from([
            ("NESTGATE_API_PORT", "9000"),
            ("NESTGATE_METRICS_PORT", "9999"),
        ]);
        assert_eq!(discover_api_port_sync_from_env_source(&env), 9000);
        assert_eq!(discover_metrics_port_sync_from_env_source(&env), 9999);
    }

    #[test]
    fn test_sync_discovery_invalid_env() {
        let env = MapEnv::from([("NESTGATE_API_PORT", "invalid")]);
        assert_eq!(discover_api_port_sync_from_env_source(&env), 8080);
    }

    #[test]
    fn test_async_discovery_fallback() {
        let env = MapEnv::new();
        let port = discover_api_port_from_env_source(&env).unwrap();
        assert_eq!(port, 8080);
    }

    #[test]
    fn test_async_discovery_from_env() {
        let env = MapEnv::from([("NESTGATE_API_PORT", "9000")]);
        let port = discover_api_port_from_env_source(&env).unwrap();
        assert_eq!(port, 9000);
    }

    #[test]
    fn test_extract_port_edge_cases() {
        assert_eq!(extract_port_from_url("http://host:0"), None);
        assert_eq!(extract_port_from_url("http://host:65535/path"), Some(65535));
        assert_eq!(extract_port_from_url("http://host:443/"), Some(443));
    }

    #[test]
    fn test_discover_metrics_port_default() {
        let env = MapEnv::new();
        let port = discover_metrics_port_from_env_source(&env).unwrap();
        assert_eq!(port, 9090);
    }

    #[test]
    fn test_discover_health_port_default() {
        let env = MapEnv::new();
        let port = discover_health_port_from_env_source(&env).unwrap();
        assert_eq!(port, 8082);
    }

    #[test]
    fn test_discover_admin_port_default() {
        let env = MapEnv::new();
        let port = discover_admin_port_from_env_source(&env).unwrap();
        assert_eq!(port, 8081);
    }

    #[test]
    fn test_discover_storage_port_default() {
        let env = MapEnv::new();
        let port = discover_storage_port_from_env_source(&env).unwrap();
        assert_eq!(port, 8083);
    }

    #[test]
    fn test_discover_tarpc_port_default() {
        let env = MapEnv::new();
        let port = discover_tarpc_port_from_env_source(&env).unwrap();
        assert_eq!(port, 8091);
    }

    #[test]
    fn test_tarpc_port_sync_default() {
        let env = MapEnv::new();
        assert_eq!(discover_tarpc_port_sync_from_env_source(&env), 8091);
    }

    #[test]
    fn runtime_port_resolver_uses_env_var() {
        let env = MapEnv::from([("NESTGATE_API_PORT", "4242")]);
        let port = RuntimePortResolver::resolve_from_env_source(
            &env,
            capability_ids::API_GATEWAY,
            "NESTGATE_API_PORT",
        );
        assert_eq!(port, 4242);
    }

    #[test]
    fn runtime_port_resolver_ephemeral_when_nothing_set() {
        let env = MapEnv::new();
        let port = RuntimePortResolver::resolve_from_env_source(
            &env,
            "nonexistent.capability",
            "NONEXISTENT_ENV_VAR",
        );
        assert!(port > 0, "ephemeral port should be > 0");
    }

    #[test]
    fn runtime_port_resolver_env_or_default() {
        let env = MapEnv::new();
        let port =
            RuntimePortResolver::resolve_env_or_default_from_env_source(&env, "UNUSED_VAR", 5432);
        assert_eq!(port, 5432);
    }
}
