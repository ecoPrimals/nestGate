//! Network API configuration module
//! Provides unified API configuration for network operations.

use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkApiConfig {
    /// API server bind address
    pub bind_address: IpAddr,

    /// API server port
    pub port: u16,

    /// Maximum concurrent connections
    pub max_connections: u32,

    /// Request timeout
    pub request_timeout: Duration,

    /// Connection timeout
    pub connection_timeout: Duration,

    /// Enable TLS
    pub tls_enabled: bool,

    /// TLS configuration
    pub tls: TlsConfig,

    /// Rate limiting configuration
    pub rate_limiting: RateLimitingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
    pub ca_path: Option<String>,
    pub verify_client: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    pub enabled: bool,
    pub requests_per_second: u32,
    pub burst_size: u32,
}

impl NetworkApiConfig {
    #[must_use]
    pub fn development_optimized() -> Self {
        use crate::constants::hardcoding::{addresses, ports};
        Self {
            bind_address: addresses::LOCALHOST_IPV4.parse().unwrap_or_else(|_| {
                // Fallback to safe default if parsing fails (should never happen)
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))
            }),
            port: ports::HTTP_DEFAULT,
            max_connections: 100,
            request_timeout: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(10),
            tls_enabled: false,
            tls: TlsConfig::default(),
            rate_limiting: RateLimitingConfig::development(),
        }
    }

    #[must_use]
    pub fn production_hardened() -> Self {
        use crate::constants::hardcoding::{addresses, ports};
        Self {
            bind_address: addresses::BIND_ALL_IPV4.parse().unwrap_or_else(|_| {
                // Fallback to safe default if parsing fails (should never happen)
                std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0))
            }),
            port: ports::HTTPS_DEFAULT,
            max_connections: 1000,
            request_timeout: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(30),
            tls_enabled: true,
            tls: TlsConfig::production(),
            rate_limiting: RateLimitingConfig::production(),
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn validate(&self) -> Result<()> {
        if self.port == 0 {
            return Err(NestGateError::validation_error("Port cannot be zero"));
        }
        if self.max_connections == 0 {
            return Err(NestGateError::validation_error(
                "Max connections cannot be zero",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.bind_address = other.bind_address;
        self.port = other.port;
        self.max_connections = other.max_connections;
        self.request_timeout = other.request_timeout;
        self.connection_timeout = other.connection_timeout;
        self.tls_enabled = other.tls_enabled;
        self.tls = other.tls;
        self.rate_limiting = other.rate_limiting;
        self
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            cert_path: "/etc/ssl/certs/nestgate.pem".to_string(),
            key_path: "/etc/ssl/private/nestgate.key".to_string(),
            ca_path: None,
            verify_client: false,
        }
    }
}

impl TlsConfig {
    #[must_use]
    pub fn production() -> Self {
        Self {
            cert_path: "/etc/ssl/certs/nestgate-prod.pem".to_string(),
            key_path: "/etc/ssl/private/nestgate-prod.key".to_string(),
            ca_path: Some("/etc/ssl/certs/ca-bundle.pem".to_string()),
            verify_client: true,
        }
    }
}

impl Default for RateLimitingConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl RateLimitingConfig {
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false,
            requests_per_second: 1000,
            burst_size: 2000,
        }
    }

    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            requests_per_second: 100,
            burst_size: 200,
        }
    }
}

impl Default for NetworkApiConfig {
    fn default() -> Self {
        Self::development_optimized()
    }
}
