//! Modern network configuration with smart defaults
//!
//! **Evolution from hardcoded constants to environment-driven configuration**
//!
//! # Philosophy
//!
//! Instead of hardcoded constants, we provide:
//! 1. **Type-safe defaults** - Using proper types (IpAddr, not strings)
//! 2. **Environment-driven** - Overridable via NESTGATE_* env vars
//! 3. **Capability-based** - Service discovery over hardcoded endpoints
//! 4. **Compile-time validation** - Invalid configurations caught early
//!
//! # Migration Pattern
//!
//! ```rust,ignore
//! // OLD: Hardcoded constant
//! use nestgate_core::constants::network_hardcoded::addresses::LOCALHOST_IPV4;
//! let host = LOCALHOST_IPV4; // Always "127.0.0.1"
//!
//! // NEW: Smart default with env override
//! use nestgate_core::constants::network_smart::default_host;
//! let host = default_host(); // Respects NESTGATE_HOST, falls back to 127.0.0.1
//! ```
//!
//! # Example Usage
//!
//! ```rust
//! use nestgate_core::constants::network_smart::{default_api_port, default_host};
//! use std::net::SocketAddr;
//!
//! // Type-safe configuration
//! let host = default_host();
//! let port = default_api_port();
//! let addr = SocketAddr::new(host, port.value());
//! ```

use std::env;
use std::net::{IpAddr, Ipv4Addr};

// ==================== TYPE-SAFE IP ADDRESSES ====================

/// Get the default host address (respects NESTGATE_HOST environment variable)
///
/// # Environment Variables
///
/// - `NESTGATE_HOST`: Override default host (e.g., "0.0.0.0", "192.168.1.100")
///
/// # Returns
///
/// Returns parsed IpAddr, falling back to IPv4 localhost (127.0.0.1) if:
/// - Environment variable not set
/// - Environment variable set but invalid IP format
///
/// # Examples
///
/// ```
/// use nestgate_core::constants::network_smart::default_host;
/// use std::net::{IpAddr, Ipv4Addr};
///
/// // Without env var (or in tests)
/// let host = default_host();
/// assert_eq!(host, IpAddr::V4(Ipv4Addr::LOCALHOST));
/// ```
pub fn default_host() -> IpAddr {
    env::var("NESTGATE_HOST")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST))
}

/// Get the default bind address for servers
///
/// # Environment Variables
///
/// - `NESTGATE_BIND_ADDRESS`: Override default (e.g., "0.0.0.0" for all interfaces)
/// - `NESTGATE_BIND_ALL`: Set to "true" to bind to all interfaces (0.0.0.0)
///
/// # Security Note
///
/// Default is localhost-only (127.0.0.1) for security. Production deployments
/// should explicitly set NESTGATE_BIND_ALL=true or NESTGATE_BIND_ADDRESS=0.0.0.0
/// to expose services on all interfaces.
///
/// # Returns
///
/// - 127.0.0.1 (localhost) - Default, secure for development
/// - 0.0.0.0 (all interfaces) - If NESTGATE_BIND_ALL=true
/// - Custom IpAddr - If NESTGATE_BIND_ADDRESS set with valid IP
pub fn default_bind_address() -> IpAddr {
    // Check explicit bind address first
    if let Ok(addr_str) = env::var("NESTGATE_BIND_ADDRESS") {
        if let Ok(addr) = addr_str.parse() {
            return addr;
        }
    }

    // Check BIND_ALL flag
    if env::var("NESTGATE_BIND_ALL")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(false)
    {
        return IpAddr::V4(Ipv4Addr::UNSPECIFIED); // 0.0.0.0
    }

    // Default to localhost for security
    IpAddr::V4(Ipv4Addr::LOCALHOST) // 127.0.0.1
}

/// Compile-time constants for when env lookup is not needed
///
/// Use these sparingly - prefer environment-driven functions above.
/// These are provided for:
/// - Compile-time configuration
/// - Const contexts where function calls aren't allowed
/// - Documentation and examples
pub mod compile_time {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    /// IPv4 localhost (127.0.0.1)
    pub const LOCALHOST_V4: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

    /// IPv6 localhost (::1)
    pub const LOCALHOST_V6: IpAddr = IpAddr::V6(Ipv6Addr::LOCALHOST);

    /// IPv4 all interfaces (0.0.0.0)
    pub const BIND_ALL_V4: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);

    /// IPv6 all interfaces (::)
    pub const BIND_ALL_V6: IpAddr = IpAddr::V6(Ipv6Addr::UNSPECIFIED);
}

// ==================== TYPE-SAFE PORTS ====================

/// Type-safe port number (validated at construction)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Port(u16);

impl Port {
    /// Create a new port, validating it's not 0
    ///
    /// # Errors
    ///
    /// Returns error if port is 0 (invalid)
    pub const fn new(port: u16) -> Result<Self, &'static str> {
        if port == 0 {
            Err("Port cannot be 0")
        } else {
            Ok(Port(port))
        }
    }

    /// Get the port value
    pub const fn value(self) -> u16 {
        self.0
    }

    /// Check if this is a privileged port (<1024)
    pub const fn is_privileged(self) -> bool {
        self.0 < 1024
    }
}

/// Get the default API port (respects NESTGATE_API_PORT environment variable)
///
/// # Environment Variables
///
/// - `NESTGATE_API_PORT`: Override default API port
///
/// # Returns
///
/// Returns Port, falling back to 8080 if environment variable not set or invalid.
///
/// # Examples
///
/// ```
/// use nestgate_core::constants::network_smart::default_api_port;
///
/// let port = default_api_port();
/// assert_eq!(port.value(), 8080); // Default (unless overridden)
/// ```
pub fn default_api_port() -> Port {
    env::var("NESTGATE_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .and_then(|n| Port::new(n).ok())
        .unwrap_or_else(|| Port::new(8080).expect("8080 is valid"))
}

/// Get the default metrics port
///
/// # Environment Variables
///
/// - `NESTGATE_METRICS_PORT`: Override default metrics port
pub fn default_metrics_port() -> Port {
    env::var("NESTGATE_METRICS_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .and_then(|n| Port::new(n).ok())
        .unwrap_or_else(|| Port::new(9090).expect("9090 is valid"))
}

/// Get the default health check port
///
/// # Environment Variables
///
/// - `NESTGATE_HEALTH_PORT`: Override default health port
pub fn default_health_port() -> Port {
    env::var("NESTGATE_HEALTH_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .and_then(|n| Port::new(n).ok())
        .unwrap_or_else(|| Port::new(8081).expect("8081 is valid"))
}

/// Compile-time port defaults
///
/// Use function versions above for environment override support.
pub mod default_ports {
    /// API server default port
    pub const API: u16 = 8080;

    /// Metrics server default port
    pub const METRICS: u16 = 9090;

    /// Health check default port
    pub const HEALTH: u16 = 8081;

    /// WebSocket default port
    pub const WEBSOCKET: u16 = 8082;

    /// Storage service default port
    pub const STORAGE: u16 = 5000;

    /// Database default ports
    pub mod database {
        /// PostgreSQL default port
        pub const POSTGRES: u16 = 5432;

        /// Redis default port
        pub const REDIS: u16 = 6379;

        /// MongoDB default port
        pub const MONGODB: u16 = 27017;
    }
}

// ==================== CAPABILITY-BASED SERVICE DISCOVERY ====================

/// Service discovery functions - replaces hardcoded primal endpoints
///
/// These functions perform runtime capability discovery instead of
/// using hardcoded URLs or ports. This maintains primal sovereignty.
pub mod service_discovery {
    use super::Port;
    use std::net::{IpAddr, SocketAddr};

    /// Discover service endpoint by capability (not by primal name!)
    ///
    /// # Note
    ///
    /// This is a placeholder showing the pattern. Real implementation
    /// would use the ServiceRegistry for actual discovery.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // ❌ OLD: Hardcoded primal
    /// let security_url = "http://beardog:3000";
    ///
    /// // ✅ NEW: Capability discovery
    /// let security = discover_service_by_capability("authentication").await?;
    /// let url = security.endpoint();
    /// ```
    pub fn discover_service_by_capability(_capability: &str) -> Option<SocketAddr> {
        // This would integrate with the actual ServiceRegistry
        // For now, returns None to indicate discovery needed
        None
    }

    /// Build a service endpoint URL from discovered host and port
    ///
    /// # Arguments
    ///
    /// - `host`: Discovered service host
    /// - `port`: Discovered service port
    /// - `path`: Optional path component
    ///
    /// # Returns
    ///
    /// Formatted URL string
    pub fn build_service_url(host: IpAddr, port: Port, path: Option<&str>) -> String {
        let base = format!("http://{}:{}", host, port.value());
        if let Some(p) = path {
            format!("{}{}", base, p)
        } else {
            base
        }
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_default_host_returns_localhost() {
        // Without env var, should return localhost
        let host = default_host();
        assert_eq!(host, IpAddr::V4(Ipv4Addr::LOCALHOST));
    }

    #[test]
    fn test_default_bind_address_is_secure() {
        // Default should be localhost (secure)
        let bind = default_bind_address();
        assert_eq!(bind, IpAddr::V4(Ipv4Addr::LOCALHOST));
    }

    #[test]
    fn test_port_validation() {
        // Valid ports
        assert!(Port::new(8080).is_ok());
        assert!(Port::new(1).is_ok());
        assert!(Port::new(65535).is_ok());

        // Invalid port
        assert!(Port::new(0).is_err());
    }

    #[test]
    fn test_port_privileged_check() {
        let port_80 = Port::new(80).unwrap();
        let port_8080 = Port::new(8080).unwrap();

        assert!(port_80.is_privileged());
        assert!(!port_8080.is_privileged());
    }

    #[test]
    fn test_compile_time_constants() {
        assert_eq!(compile_time::LOCALHOST_V4, IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(compile_time::BIND_ALL_V4, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
    }

    #[test]
    fn test_default_ports() {
        let api_port = default_api_port();
        assert_eq!(api_port.value(), 8080);

        let metrics_port = default_metrics_port();
        assert_eq!(metrics_port.value(), 9090);
    }

    #[test]
    fn test_service_url_building() {
        let host = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let port = Port::new(8080).unwrap();

        let url = service_discovery::build_service_url(host, port, None);
        assert_eq!(url, "http://192.168.1.100:8080");

        let url_with_path = service_discovery::build_service_url(host, port, Some("/api/v1"));
        assert_eq!(url_with_path, "http://192.168.1.100:8080/api/v1");
    }
}
