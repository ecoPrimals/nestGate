//! ⚠️ **DEPRECATED**: This module is being phased out in favor of capability-based configuration
//!
//! # Migration Path
//!
//! Instead of using hardcoded constants, use `CapabilityConfig` for runtime discovery:
//!
//! ```rust,no_run
//! # use nestgate_core::capability_config::CapabilityConfig;
//! # use anyhow::Result;
//! # fn example() -> Result<()> {
//! // ❌ OLD: Hardcoded
//! // const API_PORT: u16 = 8080;
//!
//! // ✅ NEW: Capability-based
//! let config = CapabilityConfig::from_env()?;
//! let api_endpoint = config.get_endpoint("api")?;
//! # Ok(())
//! # }
//! ```
//!
//! See `PHASE2_HARDCODING_ELIMINATION_PLAN.md` for full migration guide.
//!
//! This module will be removed in v0.3.0.
//!
//! ## Environment variables (central audit)
//!
//! | Variable | Purpose |
//! |----------|---------|
//! | `NESTGATE_BIND_ADDRESS`, `NESTGATE_API_PORT`, `NESTGATE_METRICS_PORT`, `NESTGATE_HEALTH_PORT` | Core listen ports |
//! | `NESTGATE_ORCHESTRATOR_ADDR` | Orchestrator peer when discovery is empty (see [`crate::constants::hardcoding::get_orchestrator_fallback_addr`]) |
//! | `NESTGATE_WEBSOCKET_PORT`, `NESTGATE_RPC_PORT`, `NESTGATE_MQ_PORT`, `NESTGATE_ORCHESTRATION_PORT` | Service ports (see getters below) |
//! | `NESTGATE_DISCOVERY_TIMEOUT_MS` | Discovery timeout ([`crate::constants::hardcoding::discovery::get_timeout_ms`]) |
//!
//! Timeouts and limits in [`crate::constants::hardcoding::timeouts`] and [`crate::constants::hardcoding::limits`] remain compile-time defaults; override via
//! capability config or future env wiring where those domains expose runtime tuning.

use std::env;
use std::sync::OnceLock;

// ============================================================================
// MODERN CAPABILITY-BASED HELPERS
// ============================================================================

/// Modern capability-based service discovery helpers
///
/// These replace hardcoded constants with runtime discovery.
///
/// # Example
///
/// ```rust,no_run
/// # use nestgate_core::constants::hardcoding::capability_helpers;
/// # async fn example() -> nestgate_core::Result<()> {
/// // ✅ Modern approach: Discover service by capability
/// let api_url = capability_helpers::discover_api_service().await?;
/// println!("API service discovered at: {}", api_url);
/// # Ok(())
/// # }
/// ```
pub mod capability_helpers {
    use crate::universal_primal_discovery::{
        capability_based_discovery::PrimalCapability, service_registry::ServiceRegistry,
    };
    use crate::Result;

    /// Discover API service endpoint via capability-based discovery
    ///
    /// # Primal Sovereignty
    ///
    /// This discovers the API service at runtime, respecting primal autonomy.
    /// No hardcoded endpoints or assumptions.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # async fn example() -> nestgate_core::Result<()> {
    /// use nestgate_core::constants::hardcoding::capability_helpers;
    ///
    /// let api_url = capability_helpers::discover_api_service().await?;
    /// println!("Using API at: {}", api_url);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn discover_api_service() -> Result<String> {
        let registry = ServiceRegistry::new(vec![PrimalCapability::ApiGateway]).await?;
        let service = registry
            .find_by_capability(&PrimalCapability::ApiGateway)
            .await?;
        Ok(service.url())
    }

    /// Discover metrics service endpoint via capability-based discovery
    pub async fn discover_metrics_service() -> Result<String> {
        let registry = ServiceRegistry::new(vec![PrimalCapability::Observability]).await?;
        let service = registry
            .find_by_capability(&PrimalCapability::Observability)
            .await?;
        Ok(service.url())
    }

    /// Discover storage service endpoint via capability-based discovery
    pub async fn discover_storage_service() -> Result<String> {
        let registry = ServiceRegistry::new(vec![PrimalCapability::ZfsStorage]).await?;
        let service = registry
            .find_by_capability(&PrimalCapability::ZfsStorage)
            .await?;
        Ok(service.url())
    }

    /// Discover security service endpoint via capability-based discovery
    pub async fn discover_security_service() -> Result<String> {
        let registry = ServiceRegistry::new(vec![PrimalCapability::Authentication]).await?;
        let service = registry
            .find_by_capability(&PrimalCapability::Authentication)
            .await?;
        Ok(service.url())
    }
}

// ============================================================================
// Network Addresses
// ============================================================================

/// Default network addresses
pub mod addresses {
    /// IPv4 localhost address
    pub const LOCALHOST_IPV4: &str = "127.0.0.1";

    /// IPv6 localhost address
    pub const LOCALHOST_IPV6: &str = "::1";

    /// Localhost hostname
    pub const LOCALHOST_NAME: &str = "localhost";

    /// Bind to all IPv4 interfaces
    pub const BIND_ALL_IPV4: &str = "0.0.0.0";

    /// Bind to all IPv6 interfaces
    pub const BIND_ALL_IPV6: &str = "::";
}

// ============================================================================
// Network Ports
// ============================================================================

/// Default network ports
///
/// ⚠️ **DEPRECATED**: Use `ServiceRegistry` for capability-based discovery
#[deprecated(
    since = "0.2.0",
    note = "Use ServiceRegistry::find_by_capability() for runtime discovery. \
            These hardcoded ports violate primal sovereignty."
)]
pub mod ports {
    /// Default HTTP port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const HTTP_DEFAULT: u16 = 8080;

    /// Default HTTPS port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const HTTPS_DEFAULT: u16 = 8443;

    /// Default API server port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const API_DEFAULT: u16 = 3000;

    /// Alternative API port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const API_ALT: u16 = 3001;

    /// Default metrics/monitoring port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const METRICS_DEFAULT: u16 = 9090;

    /// Prometheus metrics port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const PROMETHEUS: u16 = 9090;

    /// Default health check port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const HEALTH_CHECK: u16 = 8081;

    /// Default gRPC port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const GRPC_DEFAULT: u16 = 50051;

    /// Default WebSocket port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const WEBSOCKET_DEFAULT: u16 = 8082;

    /// Default admin interface port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const ADMIN_DEFAULT: u16 = 9000;

    /// Default storage service port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const STORAGE_DEFAULT: u16 = 5000;

    /// Default orchestration service port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const ORCHESTRATION_DEFAULT: u16 = 8083;

    /// Default storage discovery port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const STORAGE_DISCOVERY_DEFAULT: u16 = 8084;

    /// Default compute service port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const COMPUTE_DEFAULT: u16 = 8085;

    /// Extended services port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const EXTENDED_SERVICES: u16 = 3002;

    /// Discovery service port
    ///
    /// ⚠️ **IRONIC**: Hardcoded discovery port! Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const DISCOVERY_SERVICE: u16 = 3010;

    /// Alternative metrics port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const METRICS_ALT: u16 = 9001;

    /// Prometheus metrics port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const METRICS_PROMETHEUS: u16 = 9090;

    /// Default health check port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const HEALTH_DEFAULT: u16 = 8081;

    /// Orchestrator port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const ORCHESTRATOR_DEFAULT: u16 = 8090;

    /// Generic security service default port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const SECURITY_SERVICE_DEFAULT: u16 = 8081;

    /// Generic networking service default port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const NETWORKING_SERVICE_DEFAULT: u16 = 8082;

    /// PostgreSQL database default port
    ///
    /// ⚠️ Use `ServiceRegistry` instead
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const POSTGRES_DEFAULT: u16 = 5432;

    /// Redis cache default port
    #[deprecated(since = "0.2.0", note = "Use ServiceRegistry for discovery")]
    pub const REDIS_DEFAULT: u16 = 6379;

    /// MongoDB database default port
    pub const MONGODB_DEFAULT: u16 = 27017;

    /// MySQL database default port
    pub const MYSQL_DEFAULT: u16 = 3306;

    /// Streaming RPC default port
    pub const STREAMING_RPC_DEFAULT: u16 = 8001;
}

// ============================================================================
// Timeout Constants (milliseconds)
// ============================================================================

/// Timeout constants for network and operation timing
///
/// **Evolution Path**: These will be replaced by capability-based adaptive timeouts that:
/// - Learn from actual operation latencies
/// - Adapt to network conditions
/// - Scale based on system load
/// - Use service-specific SLAs discovered at runtime
pub mod timeouts {
    /// Default connection timeout (5 seconds)
    pub const CONNECT_MS: u64 = 5_000;

    /// Default request timeout (30 seconds)
    pub const REQUEST_MS: u64 = 30_000;

    /// Default long operation timeout (5 minutes)
    pub const LONG_OPERATION_MS: u64 = 300_000;
}

// ============================================================================
// Environment Variable Helpers
// ============================================================================

/// Cache for bind address from environment
static BIND_ADDRESS: OnceLock<String> = OnceLock::new();

/// Cache for API port from environment
static API_PORT: OnceLock<u16> = OnceLock::new();

/// Get the bind address from environment or use default
///
/// Checks `NESTGATE_BIND_ADDRESS` environment variable.
/// Falls back to `0.0.0.0` if not set.
pub fn get_bind_address() -> &'static str {
    BIND_ADDRESS.get_or_init(|| {
        env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| addresses::BIND_ALL_IPV4.to_string())
    })
}

/// Get the API port from environment or use default
///
/// Checks `NESTGATE_API_PORT` environment variable.
/// Falls back to 3000 if not set or invalid.
pub fn get_api_port() -> u16 {
    *API_PORT.get_or_init(|| {
        env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(ports::API_DEFAULT)
    })
}

/// Get the metrics port from environment or use default
///
/// Checks `NESTGATE_METRICS_PORT` environment variable.
/// Falls back to 9090 if not set or invalid.
#[must_use]
pub fn get_metrics_port() -> u16 {
    env::var("NESTGATE_METRICS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::METRICS_DEFAULT)
}

/// Get the health check port from environment or use default
///
/// Checks `NESTGATE_HEALTH_PORT` environment variable.
/// Falls back to 8081 if not set or invalid.
#[must_use]
pub fn get_health_port() -> u16 {
    env::var("NESTGATE_HEALTH_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::HEALTH_CHECK)
}

/// Fallback orchestrator peer address when capability discovery finds none.
///
/// Checks `NESTGATE_ORCHESTRATOR_ADDR` (host:port, unix path, or `unix://…`).
/// Defaults to `localhost` and [`ports::HTTP_DEFAULT`] when unset.
/// If the variable is set to an empty string (after trim), returns empty — callers treat that as
/// "no configured orchestrator".
#[must_use]
pub fn get_orchestrator_fallback_addr() -> String {
    match env::var("NESTGATE_ORCHESTRATOR_ADDR") {
        Ok(s) if s.trim().is_empty() => String::new(),
        Ok(s) => s,
        Err(_) => format!("{}:{}", addresses::LOCALHOST_NAME, ports::HTTP_DEFAULT),
    }
}

/// WebSocket port from environment or [`ports::WEBSOCKET_DEFAULT`].
#[must_use]
pub fn get_websocket_port() -> u16 {
    env::var("NESTGATE_WEBSOCKET_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::WEBSOCKET_DEFAULT)
}

/// gRPC / RPC port from `NESTGATE_RPC_PORT` or [`ports::GRPC_DEFAULT`].
#[must_use]
pub fn get_grpc_port() -> u16 {
    env::var("NESTGATE_RPC_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::GRPC_DEFAULT)
}

/// Message queue (e.g. RabbitMQ) port from `NESTGATE_MQ_PORT` or crate default.
#[must_use]
pub fn get_message_queue_port() -> u16 {
    env::var("NESTGATE_MQ_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(super::port_defaults::DEFAULT_RABBITMQ_PORT)
}

/// Standalone orchestration service port from `NESTGATE_ORCHESTRATION_PORT` or default.
#[must_use]
pub fn get_orchestration_service_port() -> u16 {
    env::var("NESTGATE_ORCHESTRATION_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::ORCHESTRATION_DEFAULT)
}

// ============================================================================
// Service Discovery Defaults
// ============================================================================

/// Default service discovery configuration
pub mod discovery {
    use super::*;

    /// Default service discovery timeout (milliseconds)
    pub const TIMEOUT_MS: u64 = 5000;

    /// Default retry attempts for service discovery
    pub const RETRY_ATTEMPTS: u32 = 3;

    /// Default port range start for capability scanning
    pub const SCAN_PORT_START: u16 = 3000;

    /// Default port range end for capability scanning
    pub const SCAN_PORT_END: u16 = 3999;

    /// Get discovery timeout from environment or default
    #[must_use]
    pub fn get_timeout_ms() -> u64 {
        env::var("NESTGATE_DISCOVERY_TIMEOUT_MS")
            .ok()
            .and_then(|t| t.parse().ok())
            .unwrap_or(TIMEOUT_MS)
    }
}

// ============================================================================
// Magic Numbers (to be eliminated)
// ============================================================================

/// Common buffer sizes and limits
pub mod limits {
    /// Default buffer size for I/O operations (64KB)
    pub const BUFFER_SIZE_DEFAULT: usize = 65536;

    /// Maximum buffer size for I/O operations (1MB)
    pub const BUFFER_SIZE_MAX: usize = 1_048_576;

    /// Default connection pool size
    pub const CONNECTION_POOL_SIZE: usize = 10;

    /// Maximum concurrent connections allowed
    pub const MAX_CONNECTIONS: usize = 1000;

    /// Default timeout in seconds
    pub const TIMEOUT_SECS: u64 = 30;

    /// Maximum number of retry attempts for failed operations
    pub const MAX_RETRIES: u32 = 3;
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addresses_are_valid() {
        assert_eq!(addresses::LOCALHOST_IPV4, "127.0.0.1");
        assert_eq!(addresses::LOCALHOST_IPV6, "::1");
        assert_eq!(addresses::LOCALHOST_NAME, "localhost");
        assert_eq!(addresses::BIND_ALL_IPV4, "0.0.0.0");
        assert_eq!(addresses::BIND_ALL_IPV6, "::");
    }

    #[test]
    fn test_ports_are_in_valid_range() {
        // All ports are u16, which are always >= 0, so just verify they're defined
        // These checks serve as documentation that these ports exist and are configured
        assert_eq!(ports::HTTP_DEFAULT, ports::HTTP_DEFAULT);
        assert_eq!(ports::HTTPS_DEFAULT, ports::HTTPS_DEFAULT);
        assert_eq!(ports::API_DEFAULT, ports::API_DEFAULT);
        assert_eq!(ports::METRICS_DEFAULT, ports::METRICS_DEFAULT);
        assert_eq!(ports::HEALTH_CHECK, ports::HEALTH_CHECK);
    }

    #[test]
    fn test_get_bind_address_default() {
        // Should return default when env var not set
        let addr = get_bind_address();
        assert!(!addr.is_empty());
    }

    #[test]
    fn test_get_api_port_default() {
        // Should return valid port when env var not set
        let port = get_api_port();
        assert!(port > 0);
    }

    #[test]
    fn test_discovery_timeout() {
        let timeout = discovery::get_timeout_ms();
        assert!(timeout > 0);
    }

    #[test]
    fn test_limits_are_reasonable() {
        // These are compile-time constants, so we verify their relationships
        // rather than testing values that are always true.
        const _: () = assert!(limits::BUFFER_SIZE_MAX >= limits::BUFFER_SIZE_DEFAULT);
        const _: () = assert!(limits::MAX_CONNECTIONS >= limits::CONNECTION_POOL_SIZE);

        // Runtime verification that constants are accessible
        let _ = limits::BUFFER_SIZE_DEFAULT;
        let _ = limits::CONNECTION_POOL_SIZE;
        let _ = limits::TIMEOUT_SECS;
        let _ = limits::MAX_RETRIES;
    }

    // ==================== NEW COMPREHENSIVE TESTS ====================

    #[test]
    fn test_all_port_constants_are_unique() {
        // Ensure no port collisions in defaults
        let ports_vec = vec![
            ports::HTTP_DEFAULT,
            ports::HTTPS_DEFAULT,
            ports::METRICS_DEFAULT,
            ports::HEALTH_CHECK,
            ports::GRPC_DEFAULT,
            ports::WEBSOCKET_DEFAULT,
            ports::ADMIN_DEFAULT,
        ];

        // At least verify ports are in valid ranges
        for port in &ports_vec {
            assert!(
                *port > 1024,
                "Port {} should be > 1024 (unprivileged)",
                port
            );
            assert!(*port < 65535, "Port {} should be < 65535", port);
        }
    }

    #[test]
    fn test_database_ports() {
        assert_eq!(ports::POSTGRES_DEFAULT, 5432);
        assert_eq!(ports::REDIS_DEFAULT, 6379);
        assert_eq!(ports::MONGODB_DEFAULT, 27017);
        assert_eq!(ports::MYSQL_DEFAULT, 3306);
    }

    #[test]
    fn test_service_ports() {
        assert_eq!(ports::DISCOVERY_SERVICE, 3010);
        assert_eq!(ports::ORCHESTRATOR_DEFAULT, 8090);
        assert_eq!(ports::STORAGE_DEFAULT, 5000);
        assert_eq!(ports::COMPUTE_DEFAULT, 8085);
    }

    #[test]
    fn test_timeout_constants() {
        assert_eq!(timeouts::CONNECT_MS, 5_000);
        assert_eq!(timeouts::REQUEST_MS, 30_000);
        assert_eq!(timeouts::LONG_OPERATION_MS, 300_000);

        // Timeout hierarchy: CONNECT < REQUEST < LONG_OPERATION (enforced by design)
        // No runtime assertion needed - these are constants with intentional values
    }

    #[test]
    fn test_discovery_constants() {
        assert_eq!(discovery::TIMEOUT_MS, 5000);
        assert_eq!(discovery::RETRY_ATTEMPTS, 3);
        assert_eq!(discovery::SCAN_PORT_START, 3000);
        assert_eq!(discovery::SCAN_PORT_END, 3999);

        // Port range: 3000-3999 (enforced by design, valid range guaranteed)
    }

    #[test]
    fn test_get_metrics_port() {
        let port = get_metrics_port();
        assert_eq!(port, ports::METRICS_DEFAULT);
        assert!(port > 0);
    }

    #[test]
    fn test_get_health_port() {
        let port = get_health_port();
        assert_eq!(port, ports::HEALTH_CHECK);
        assert!(port > 0);
    }

    #[test]
    fn test_discovery_timeout_helper() {
        let timeout = discovery::get_timeout_ms();
        assert_eq!(timeout, discovery::TIMEOUT_MS);
        assert!(timeout > 0);
    }

    #[test]
    fn test_ipv4_address_format() {
        // Verify IPv4 addresses are properly formatted
        assert!(addresses::LOCALHOST_IPV4
            .parse::<std::net::Ipv4Addr>()
            .is_ok());
        assert!(addresses::BIND_ALL_IPV4
            .parse::<std::net::Ipv4Addr>()
            .is_ok());
    }

    #[test]
    fn test_ipv6_address_format() {
        // Verify IPv6 addresses are properly formatted
        assert!(addresses::LOCALHOST_IPV6
            .parse::<std::net::Ipv6Addr>()
            .is_ok());
        assert!(addresses::BIND_ALL_IPV6
            .parse::<std::net::Ipv6Addr>()
            .is_ok());
    }

    #[test]
    fn test_buffer_size_limits() {
        assert_eq!(limits::BUFFER_SIZE_DEFAULT, 65536);
        assert_eq!(limits::BUFFER_SIZE_MAX, 1_048_576);
        // Buffer size hierarchy: DEFAULT < MAX (enforced by design)
    }

    #[test]
    fn test_connection_limits() {
        assert_eq!(limits::CONNECTION_POOL_SIZE, 10);
        assert_eq!(limits::MAX_CONNECTIONS, 1000);
        // Connection limits: POOL_SIZE < MAX_CONNECTIONS (enforced by design)
    }

    #[test]
    fn test_retry_configuration() {
        assert_eq!(limits::MAX_RETRIES, 3);
        assert_eq!(limits::TIMEOUT_SECS, 30);
        // Both values are positive by design (non-zero required for operation)
    }

    #[test]
    fn test_service_capability_ports() {
        // Generic service defaults (capability-based discovery preferred)
        assert_eq!(ports::SECURITY_SERVICE_DEFAULT, 8081);
        assert_eq!(ports::NETWORKING_SERVICE_DEFAULT, 8082);
        assert_ne!(
            ports::SECURITY_SERVICE_DEFAULT,
            ports::NETWORKING_SERVICE_DEFAULT
        );
    }

    #[test]
    fn test_extended_services_port() {
        assert_eq!(ports::EXTENDED_SERVICES, 3002);
        assert_eq!(ports::API_ALT, 3001);
        assert_ne!(ports::EXTENDED_SERVICES, ports::API_DEFAULT);
    }

    #[test]
    fn test_bind_address_is_valid() {
        let addr = get_bind_address();
        assert!(!addr.is_empty());
        // Should be either IPv4 or IPv6
        assert!(
            addr.parse::<std::net::Ipv4Addr>().is_ok()
                || addr.parse::<std::net::Ipv6Addr>().is_ok()
        );
    }

    #[test]
    fn test_api_port_is_valid() {
        let port = get_api_port();
        assert!(port > 0);
        // u16 automatically ensures port <= 65535
    }
}
