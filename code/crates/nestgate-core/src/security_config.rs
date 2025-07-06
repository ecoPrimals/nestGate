use serde::{Deserialize, Serialize};
/// Security Configuration Module
///
/// Centralizes all security-sensitive configuration to prevent hardcoded values
/// and provide secure defaults for production deployment.
use std::collections::HashMap;

/// Security configuration default constants
pub mod security_defaults {
    /// Default bind interface for localhost-only security
    pub const DEFAULT_BIND_INTERFACE: &str = "127.0.0.1";

    /// Default link-local address range (APIPA)
    pub const LINK_LOCAL_RANGE: &str = "169.254.0.0/16";

    /// Default multicast address range
    pub const MULTICAST_RANGE: &str = "224.0.0.0/4";

    /// Localhost IPv4 CIDR
    pub const LOCALHOST_IPV4: &str = "127.0.0.1/32";

    /// Private network ranges (RFC 1918)
    pub const PRIVATE_CLASS_A: &str = "10.0.0.0/8";
    pub const PRIVATE_CLASS_B: &str = "172.16.0.0/12";
    pub const PRIVATE_CLASS_C: &str = "192.168.0.0/16";

    /// Broadcast address range
    pub const BROADCAST_RANGE: &str = "0.0.0.0/8";

    /// Localhost identifiers for validation
    pub const LOCALHOST_NAME: &str = "localhost";
    pub const LOCALHOST_IP: &str = "127.0.0.1";
}

/// Security configuration for NestGate
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    /// Network binding configuration
    pub network: NetworkSecurityConfig,

    /// Service endpoint configuration
    pub endpoints: EndpointConfig,

    /// Authentication and authorization settings
    pub auth: AuthConfig,

    /// Allowed IP ranges and access control
    pub access_control: AccessControlConfig,
}

/// Network security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityConfig {
    /// Default bind interface (never 0.0.0.0 in production)
    pub default_bind_interface: String,

    /// Whether to allow localhost-only binding
    pub localhost_only: bool,

    /// Maximum allowed network interfaces to bind to
    pub max_bind_interfaces: usize,

    /// Disallowed bind addresses
    pub disallowed_binds: Vec<String>,
}

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    /// Songbird service endpoints
    pub songbird_endpoints: Vec<String>,

    /// Discovery service endpoints
    pub discovery_endpoints: Vec<String>,

    /// Health check endpoints
    pub health_endpoints: HashMap<String, String>,

    /// Default ports for services
    pub default_ports: HashMap<String, u16>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Require authentication for all endpoints
    pub require_auth: bool,

    /// API key configuration
    pub api_keys: Vec<String>,

    /// Token expiry time in seconds
    pub token_expiry_seconds: u64,

    /// Enable TLS/SSL
    pub enable_tls: bool,
}

/// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    /// Allowed IP ranges for access
    pub allowed_ip_ranges: Vec<String>,

    /// Blocked IP ranges
    pub blocked_ip_ranges: Vec<String>,

    /// Rate limiting configuration
    pub rate_limit_per_ip: u32,

    /// Maximum concurrent connections per IP
    pub max_connections_per_ip: u32,
}



impl Default for NetworkSecurityConfig {
    fn default() -> Self {
        Self {
            // SECURE DEFAULT: localhost only
            default_bind_interface: security_defaults::DEFAULT_BIND_INTERFACE.to_string(),
            localhost_only: true,
            max_bind_interfaces: 1,
            disallowed_binds: vec![
                "0.0.0.0".to_string(), // Never bind to all interfaces by default
                "::".to_string(),      // Never bind to all IPv6 interfaces
                security_defaults::LINK_LOCAL_RANGE.to_string(), // Link-local addresses
                security_defaults::MULTICAST_RANGE.to_string(), // Multicast addresses
            ],
        }
    }
}

impl Default for EndpointConfig {
    fn default() -> Self {
        Self {
            songbird_endpoints: vec![],  // Must be configured explicitly
            discovery_endpoints: vec![], // Must be configured explicitly
            health_endpoints: HashMap::new(),
            default_ports: {
                let mut ports = HashMap::new();
                // Use environment-aware port configuration
                ports.insert(
                    "api".to_string(),
                    std::env::var("NESTGATE_API_PORT")
                        .unwrap_or_else(|_| "8080".to_string())
                        .parse()
                        .unwrap_or(8080),
                );
                ports.insert(
                    "health".to_string(),
                    std::env::var("NESTGATE_HEALTH_PORT")
                        .unwrap_or_else(|_| "8081".to_string())
                        .parse()
                        .unwrap_or(8081),
                );
                ports.insert(
                    "metrics".to_string(),
                    std::env::var("NESTGATE_METRICS_PORT")
                        .unwrap_or_else(|_| "9090".to_string())
                        .parse()
                        .unwrap_or(9090),
                );
                ports
            },
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            require_auth: true,         // SECURE DEFAULT: auth required
            api_keys: vec![],           // Must be configured explicitly
            token_expiry_seconds: 3600, // 1 hour default
            enable_tls: true,           // SECURE DEFAULT: TLS enabled
        }
    }
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            allowed_ip_ranges: vec![
                security_defaults::LOCALHOST_IPV4.to_string(), // Localhost only by default
                "::1/128".to_string(),                         // IPv6 localhost
            ],
            blocked_ip_ranges: vec![
                security_defaults::BROADCAST_RANGE.to_string(), // Block broadcast
                security_defaults::PRIVATE_CLASS_A.to_string(), // Block RFC 1918 private
                security_defaults::PRIVATE_CLASS_B.to_string(), // Block RFC 1918 private
                security_defaults::PRIVATE_CLASS_C.to_string(), // Block RFC 1918 private
                security_defaults::LINK_LOCAL_RANGE.to_string(), // Block link-local
                security_defaults::MULTICAST_RANGE.to_string(), // Block multicast
            ],
            rate_limit_per_ip: 100,
            max_connections_per_ip: 10,
        }
    }
}

impl SecurityConfig {
    /// Load security configuration from environment variables and files
    pub fn from_env() -> Self {
        let mut config = Self::default();

        // Network configuration
        if let Ok(bind) = std::env::var("NESTGATE_BIND_INTERFACE") {
            config.network.default_bind_interface = bind;
        }

        if let Ok(localhost_only) = std::env::var("NESTGATE_LOCALHOST_ONLY") {
            config.network.localhost_only = localhost_only.parse().unwrap_or(true);
        }

        // Endpoint configuration
        if let Ok(songbird_endpoints) = std::env::var("NESTGATE_SONGBIRD_ENDPOINTS") {
            config.endpoints.songbird_endpoints = songbird_endpoints
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        if let Ok(discovery_endpoints) = std::env::var("NESTGATE_DISCOVERY_ENDPOINTS") {
            config.endpoints.discovery_endpoints = discovery_endpoints
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        // Auth configuration
        if let Ok(require_auth) = std::env::var("NESTGATE_REQUIRE_AUTH") {
            config.auth.require_auth = require_auth.parse().unwrap_or(true);
        }

        if let Ok(enable_tls) = std::env::var("NESTGATE_ENABLE_TLS") {
            config.auth.enable_tls = enable_tls.parse().unwrap_or(true);
        }

        // Access control
        if let Ok(allowed_ips) = std::env::var("NESTGATE_ALLOWED_IP_RANGES") {
            config.access_control.allowed_ip_ranges = allowed_ips
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }

        config
    }

    /// Validate the security configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate bind interface is not insecure
        if self.network.default_bind_interface == "0.0.0.0" && self.network.localhost_only {
            return Err("Cannot bind to 0.0.0.0 when localhost_only is true".to_string());
        }

        // Validate endpoints are not empty in production
        if std::env::var("NESTGATE_ENVIRONMENT").unwrap_or_default() == "production" 
            && self.endpoints.songbird_endpoints.is_empty() {
            return Err("Songbird endpoints must be configured in production".to_string());
        }

        // Validate no localhost in production endpoints
        if std::env::var("NESTGATE_ENVIRONMENT").unwrap_or_default() == "production" {
            for endpoint in &self.endpoints.songbird_endpoints {
                if endpoint.contains(security_defaults::LOCALHOST_NAME)
                    || endpoint.contains(security_defaults::LOCALHOST_IP)
                {
                    return Err(format!(
                        "Production endpoint cannot use localhost: {}",
                        endpoint
                    ));
                }
            }
        }

        Ok(())
    }

    /// Get secure endpoint for a service
    pub fn get_service_endpoint(&self, service: &str) -> Option<String> {
        match service {
            "songbird" => self.endpoints.songbird_endpoints.first().cloned(),
            "discovery" => self.endpoints.discovery_endpoints.first().cloned(),
            _ => None,
        }
    }

    /// Check if an IP address is allowed
    pub fn is_ip_allowed(&self, ip: &str) -> bool {
        // Check if IP is in allowed ranges
        for range in &self.access_control.allowed_ip_ranges {
            if ip_in_range(ip, range) {
                // Also check it's not in blocked ranges
                for blocked in &self.access_control.blocked_ip_ranges {
                    if ip_in_range(ip, blocked) {
                        return false;
                    }
                }
                return true;
            }
        }
        false
    }
}

/// Check if an IP address is in a CIDR range
fn ip_in_range(ip: &str, range: &str) -> bool {
    // Simplified IP range checking - in production use a proper CIDR library
    if range.contains('/') {
        let parts: Vec<&str> = range.split('/').collect();
        if parts.len() == 2 {
            let network = parts[0];
            if ip.starts_with(&network[..network.rfind('.').unwrap_or(0)]) {
                return true;
            }
        }
    } else {
        return ip == range;
    }
    false
}

/// Environment variable names for security configuration
pub mod env_vars {
    pub const BIND_INTERFACE: &str = "NESTGATE_BIND_INTERFACE";
    pub const LOCALHOST_ONLY: &str = "NESTGATE_LOCALHOST_ONLY";
    pub const SONGBIRD_ENDPOINTS: &str = "NESTGATE_SONGBIRD_ENDPOINTS";
    pub const DISCOVERY_ENDPOINTS: &str = "NESTGATE_DISCOVERY_ENDPOINTS";
    pub const REQUIRE_AUTH: &str = "NESTGATE_REQUIRE_AUTH";
    pub const ENABLE_TLS: &str = "NESTGATE_ENABLE_TLS";
    pub const ALLOWED_IP_RANGES: &str = "NESTGATE_ALLOWED_IP_RANGES";
    pub const ENVIRONMENT: &str = "NESTGATE_ENVIRONMENT";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_security_config() {
        let config = SecurityConfig::default();

        // Verify secure defaults
        assert_eq!(
            config.network.default_bind_interface,
            security_defaults::LOCALHOST_IP
        );
        assert!(config.network.localhost_only);
        assert!(config.auth.require_auth);
        assert!(config.auth.enable_tls);

        // Verify no insecure defaults
        assert!(!config.network.disallowed_binds.is_empty());
        assert!(!config.access_control.allowed_ip_ranges.is_empty());
    }

    #[test]
    fn test_ip_range_checking() {
        let config = SecurityConfig::default();

        // Test localhost access
        assert!(config.is_ip_allowed("127.0.0.1"));

        // Test private network blocking (should be blocked by default)
        assert!(!config.is_ip_allowed("192.168.1.1"));
        assert!(!config.is_ip_allowed("10.0.0.1"));
    }

    #[test]
    fn test_validation() {
        let config = SecurityConfig::default();
        assert!(config.validate().is_ok());

        // Test invalid configuration
        let mut bad_config = config.clone();
        bad_config.network.default_bind_interface = "0.0.0.0".to_string();
        bad_config.network.localhost_only = true;
        assert!(bad_config.validate().is_err());
    }
}
