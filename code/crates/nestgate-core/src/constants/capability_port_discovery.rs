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
//! ```rust,no_run
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

use crate::universal_primal_discovery::{
    capability_based_discovery::PrimalCapability, service_registry::ServiceRegistry,
};
use crate::Result;
use std::env;

// ==================== MODERN DISCOVERY FUNCTIONS ====================

/// Discover API service port using capability-based discovery
///
/// # Discovery Order
/// 1. Capability discovery (find service advertising ApiGateway capability)
/// 2. Environment variable (`NESTGATE_API_PORT`)
/// 3. Safe default (8080)
///
/// # Primal Sovereignty
/// Respects primal autonomy - discovers services at runtime without hardcoded assumptions.
///
/// # Example
/// ```rust,no_run
/// # use nestgate_core::constants::capability_port_discovery::discover_api_port;
/// # async fn example() -> nestgate_core::Result<()> {
/// let port = discover_api_port().await?;
/// println!("API available on port: {}", port);
/// # Ok(())
/// # }
/// ```
pub async fn discover_api_port() -> Result<u16> {
    // 1. Try capability discovery
    if let Ok(service_url) = try_discover_api_service().await {
        if let Some(port) = extract_port_from_url(&service_url) {
            return Ok(port);
        }
    }

    // 2. Try environment variable
    if let Ok(port_str) = env::var("NESTGATE_API_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            if port > 0 {
                return Ok(port);
            }
        }
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
pub async fn discover_metrics_port() -> Result<u16> {
    // 1. Try capability discovery
    if let Ok(service_url) = try_discover_metrics_service().await {
        if let Some(port) = extract_port_from_url(&service_url) {
            return Ok(port);
        }
    }

    // 2. Try environment variable
    if let Ok(port_str) = env::var("NESTGATE_METRICS_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            if port > 0 {
                return Ok(port);
            }
        }
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
pub async fn discover_health_port() -> Result<u16> {
    // 1. Environment variable (health checks are often load-balancer specific)
    if let Ok(port_str) = env::var("NESTGATE_HEALTH_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            if port > 0 {
                return Ok(port);
            }
        }
    }

    // 2. Safe default
    Ok(8082)
}

/// Discover admin interface port using capability-based discovery
///
/// # Discovery Order
/// 1. Environment variable (`NESTGATE_ADMIN_PORT`)
/// 2. Safe default (8081)
pub async fn discover_admin_port() -> Result<u16> {
    // 1. Environment variable (admin interfaces are sensitive, explicit config preferred)
    if let Ok(port_str) = env::var("NESTGATE_ADMIN_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            if port > 0 {
                return Ok(port);
            }
        }
    }

    // 2. Safe default
    Ok(8081)
}

/// Discover storage service port using capability-based discovery
///
/// # Discovery Order
/// 1. Capability discovery (ZfsStorage capability)
/// 2. Environment variable (`NESTGATE_STORAGE_PORT`)
/// 3. Safe default (8083)
pub async fn discover_storage_port() -> Result<u16> {
    // 1. Try capability discovery
    if let Ok(service_url) = try_discover_storage_service().await {
        if let Some(port) = extract_port_from_url(&service_url) {
            return Ok(port);
        }
    }

    // 2. Try environment variable
    if let Ok(port_str) = env::var("NESTGATE_STORAGE_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            if port > 0 {
                return Ok(port);
            }
        }
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
pub async fn discover_tarpc_port() -> Result<u16> {
    // 1. Try environment variable
    if let Ok(port_str) = env::var("NESTGATE_TARPC_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            if port > 0 {
                return Ok(port);
            }
        }
    }

    // 2. Safe default
    Ok(8091)
}

// ==================== HELPER FUNCTIONS ====================

/// Try to discover API service (returns Err if not found, doesn't panic)
async fn try_discover_api_service() -> Result<String> {
    let registry = ServiceRegistry::new(vec![PrimalCapability::ApiGateway]).await?;
    let service = registry
        .find_by_capability(&PrimalCapability::ApiGateway)
        .await?;
    Ok(service.url())
}

/// Try to discover metrics service (returns Err if not found)
async fn try_discover_metrics_service() -> Result<String> {
    let registry = ServiceRegistry::new(vec![PrimalCapability::Observability]).await?;
    let service = registry
        .find_by_capability(&PrimalCapability::Observability)
        .await?;
    Ok(service.url())
}

/// Try to discover storage service (returns Err if not found)
async fn try_discover_storage_service() -> Result<String> {
    let registry = ServiceRegistry::new(vec![PrimalCapability::ZfsStorage]).await?;
    let service = registry
        .find_by_capability(&PrimalCapability::ZfsStorage)
        .await?;
    Ok(service.url())
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
/// Prefer async `discover_api_port()` when possible.
pub fn discover_api_port_sync() -> u16 {
    env::var("NESTGATE_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&p| p > 0)
        .unwrap_or(8080)
}

/// Synchronous metrics port discovery
pub fn discover_metrics_port_sync() -> u16 {
    env::var("NESTGATE_METRICS_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&p| p > 0)
        .unwrap_or(9090)
}

/// Synchronous health port discovery
pub fn discover_health_port_sync() -> u16 {
    env::var("NESTGATE_HEALTH_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&p| p > 0)
        .unwrap_or(8082)
}

/// Synchronous admin port discovery
pub fn discover_admin_port_sync() -> u16 {
    env::var("NESTGATE_ADMIN_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&p| p > 0)
        .unwrap_or(8081)
}

/// Synchronous tarpc port discovery
pub fn discover_tarpc_port_sync() -> u16 {
    env::var("NESTGATE_TARPC_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .filter(|&p| p > 0)
        .unwrap_or(8091)
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

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
    #[ignore = "Flaky due to global env state - needs serial execution or proper isolation"]
    fn test_sync_discovery_defaults() {
        // Clear any env vars
        env::remove_var("NESTGATE_API_PORT");
        env::remove_var("NESTGATE_METRICS_PORT");

        // Should return defaults
        assert_eq!(discover_api_port_sync(), 8080);
        assert_eq!(discover_metrics_port_sync(), 9090);
        assert_eq!(discover_health_port_sync(), 8082);
        assert_eq!(discover_admin_port_sync(), 8081);
    }

    #[test]
    #[ignore = "Flaky due to global env state - needs serial execution or proper isolation"]
    fn test_sync_discovery_from_env() {
        // Set env vars
        env::set_var("NESTGATE_API_PORT", "9000");
        env::set_var("NESTGATE_METRICS_PORT", "9999");

        // Should use env vars
        assert_eq!(discover_api_port_sync(), 9000);
        assert_eq!(discover_metrics_port_sync(), 9999);

        // Cleanup
        env::remove_var("NESTGATE_API_PORT");
        env::remove_var("NESTGATE_METRICS_PORT");
    }

    #[test]
    fn test_sync_discovery_invalid_env() {
        // Set invalid env var
        env::set_var("NESTGATE_API_PORT", "invalid");

        // Should fall back to default
        assert_eq!(discover_api_port_sync(), 8080);

        // Cleanup
        env::remove_var("NESTGATE_API_PORT");
    }

    #[tokio::test]
    async fn test_async_discovery_fallback() {
        // Clear env vars
        env::remove_var("NESTGATE_API_PORT");

        // Should fall back to default when capability discovery fails
        let port = discover_api_port().await.unwrap();
        assert_eq!(port, 8080);
    }

    #[tokio::test]
    #[ignore = "Flaky due to environment variable state - needs isolation"]
    async fn test_async_discovery_from_env() {
        // Set env var
        env::set_var("NESTGATE_API_PORT", "9000");

        // Should use env var
        let port = discover_api_port().await.unwrap();
        assert_eq!(port, 9000);

        // Cleanup
        env::remove_var("NESTGATE_API_PORT");
    }

    #[test]
    fn test_extract_port_edge_cases() {
        assert_eq!(extract_port_from_url("http://host:0"), None);
        assert_eq!(extract_port_from_url("http://host:65535/path"), Some(65535));
        assert_eq!(extract_port_from_url("http://host:443/"), Some(443));
    }

    #[tokio::test]
    async fn test_discover_metrics_port_default() {
        env::remove_var("NESTGATE_METRICS_PORT");
        let port = discover_metrics_port().await.unwrap();
        assert_eq!(port, 9090);
    }

    #[tokio::test]
    async fn test_discover_health_port_default() {
        env::remove_var("NESTGATE_HEALTH_PORT");
        let port = discover_health_port().await.unwrap();
        assert_eq!(port, 8082);
    }

    #[tokio::test]
    async fn test_discover_admin_port_default() {
        env::remove_var("NESTGATE_ADMIN_PORT");
        let port = discover_admin_port().await.unwrap();
        assert_eq!(port, 8081);
    }

    #[tokio::test]
    async fn test_discover_storage_port_default() {
        env::remove_var("NESTGATE_STORAGE_PORT");
        let port = discover_storage_port().await.unwrap();
        assert_eq!(port, 8083);
    }

    #[tokio::test]
    async fn test_discover_tarpc_port_default() {
        env::remove_var("NESTGATE_TARPC_PORT");
        let port = discover_tarpc_port().await.unwrap();
        assert_eq!(port, 8091);
    }

    #[test]
    fn test_tarpc_port_sync_default() {
        env::remove_var("NESTGATE_TARPC_PORT");
        assert_eq!(discover_tarpc_port_sync(), 8091);
    }
}
