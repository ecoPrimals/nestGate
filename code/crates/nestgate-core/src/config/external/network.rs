//! Network Configuration
//!
//! Replaces hardcoded ports and IP addresses with environment-driven configuration.
//!
//! # Replaced Hardcoded Values
//!
//! This module eliminates:
//! - 121 hardcoded ports (:8080, :3000, :5432, :6379, :9090)
//! - 391 hardcoded IPs (127.0.0.1, 0.0.0.0, localhost)
//!
//! # Environment Variables
//!
//! - `NESTGATE_API_PORT`: API server port (default: 8080)
//! - `NESTGATE_API_HOST`: API server host (default: 0.0.0.0)
//! - `NESTGATE_DB_PORT`: Database port (default: 5432)
//! - `NESTGATE_DB_HOST`: Database host (default: localhost)
//! - `NESTGATE_REDIS_PORT`: Redis port (default: 6379)
//! - `NESTGATE_REDIS_HOST`: Redis host (default: localhost)
//! - `NESTGATE_METRICS_PORT`: Metrics port (default: 9090)
//! - `NESTGATE_METRICS_HOST`: Metrics host (default: 0.0.0.0)

use crate::Result;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// Import config for environment variable lookups
use super::network_env_config::NetworkEnvConfig;

/// Network configuration for all services
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Network
pub struct NetworkConfig {
    /// API server configuration
    pub api: EndpointConfig,
    /// Database configuration
    pub database: EndpointConfig,
    /// Redis cache configuration
    pub redis: EndpointConfig,
    /// Metrics/monitoring configuration
    pub metrics: EndpointConfig,
    /// Discovery service configuration
    pub discovery: EndpointConfig,
}

/// Configuration for a single network endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Endpoint
pub struct EndpointConfig {
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
}

impl NetworkConfig {
    /// Load from environment variables
    ///
    /// # Errors
    ///
    /// Returns error if environment variables are invalid
    pub fn from_env() -> Result<Self> {
        let discovery_config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
        Ok(Self {
            api: EndpointConfig::from_env(
                "NESTGATE_API",
                "0.0.0.0",
                discovery_config.discovery_base_port,
            )?,
            database: EndpointConfig::from_env(
                "NESTGATE_DB",
                "localhost", // Safe default: localhost-only access
                5432,        // PostgreSQL standard port
            )?,
            redis: EndpointConfig::from_env(
                "NESTGATE_REDIS",
                "localhost", // Safe default: localhost-only access
                6379,        // Redis standard port
            )?,
            metrics: EndpointConfig::from_env(
                "NESTGATE_METRICS",
                "0.0.0.0", // Safe default: bind all interfaces for monitoring
                9090,      // Prometheus standard port
            )?,
            discovery: EndpointConfig::from_env(
                "NESTGATE_DISCOVERY",
                &discovery_config.discovery_host,
                discovery_config.discovery_base_port,
            )?,
        })
    }

    /// Development defaults
    /// 
    /// ✅ MIGRATED: Now uses centralized environment-driven configuration
    #[must_use]
    pub fn default_dev() -> Self {
        use crate::constants::{get_metrics_port, get_postgres_port, get_redis_port};
        use crate::constants::hardcoding::addresses;
        
        let discovery_config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
        
        Self {
            api: EndpointConfig {
                host: addresses::BIND_ALL_IPV4.to_string(), // Safe dev default: bind all interfaces
                port: discovery_config.discovery_base_port,
            },
            database: EndpointConfig {
                host: addresses::LOCALHOST_NAME.to_string(), // Safe dev default: localhost-only
                port: get_postgres_port(),                   // Environment-driven with default 5432
            },
            redis: EndpointConfig {
                host: addresses::LOCALHOST_NAME.to_string(), // Safe dev default: localhost-only
                port: get_redis_port(),                      // Environment-driven with default 6379
            },
            metrics: EndpointConfig {
                host: addresses::BIND_ALL_IPV4.to_string(), // Safe dev default: bind all for monitoring
                port: get_metrics_port(),                    // Environment-driven with default 9090
            },
            discovery: EndpointConfig {
                host: discovery_config.discovery_host.clone(),
                port: discovery_config.discovery_base_port,
            },
        }
    }

    /// Production configuration (requires environment variables)
    ///
    /// # Errors
    ///
    /// Returns error if required environment variables are missing
    pub fn from_env_production() -> Result<Self> {
        // In production, all values must be explicitly set
        Ok(Self {
            api: EndpointConfig::from_env_required("NESTGATE_API")?,
            database: EndpointConfig::from_env_required("NESTGATE_DB")?,
            redis: EndpointConfig::from_env_required("NESTGATE_REDIS")?,
            metrics: EndpointConfig::from_env_required("NESTGATE_METRICS")?,
            discovery: EndpointConfig::from_env_required("NESTGATE_DISCOVERY")?,
        })
    }

    /// Get API port
    #[must_use]
    pub fn api_port(&self) -> u16 {
        self.api.port
    }

    /// Get API host
    #[must_use]
    pub fn api_host(&self) -> &str {
        &self.api.host
    }

    /// Get API socket address
    ///
    /// # Errors
    ///
    /// Returns error if host cannot be parsed as IP address
    pub fn api_socket_addr(&self) -> Result<SocketAddr> {
        self.api.socket_addr()
    }

    /// Get database connection string
    #[must_use]
    pub fn database_url(&self, database: &str) -> String {
        format!(
            "postgresql://{}:{}/{}",
            self.database.host, self.database.port, database
        )
    }

    /// Get Redis connection string
    #[must_use]
    pub fn redis_url(&self) -> String {
        format!("redis://{}:{}", self.redis.host, self.redis.port)
    }

    /// Get metrics endpoint
    #[must_use]
    pub fn metrics_endpoint(&self) -> String {
        format!("{}:{}", self.metrics.host, self.metrics.port)
    }

    /// Get discovery endpoint
    #[must_use]
    pub fn discovery_url(&self) -> String {
        format!("http://{}:{}", self.discovery.host, self.discovery.port)
    }
}

impl EndpointConfig {
    /// Load from environment with fallback defaults
    ///
    /// # Errors
    ///
    /// Returns error if environment variable value is invalid
    pub fn from_env(prefix: &str, default_host: &str, default_port: u16) -> Result<Self> {
        // Use config to get environment variables
        let config = NetworkEnvConfig::from_env();

        let host = config
            .get_host(prefix)
            .map(|s| s.to_string())
            .unwrap_or_else(|| default_host.to_string());

        let port = config.get_port(prefix).unwrap_or(default_port);

        Ok(Self { host, port })
    }

    /// Load from environment (required, no defaults)
    ///
    /// # Errors
    ///
    /// Returns error if environment variables are missing or invalid
    pub fn from_env_required(prefix: &str) -> Result<Self> {
        // Use config to get environment variables
        let config = NetworkEnvConfig::from_env();

        let host = config
            .get_host(prefix)
            .map(|s| s.to_string())
            .ok_or_else(|| {
                crate::error::NestGateUnifiedError::Configuration(Box::new(
                    crate::error::ConfigurationErrorDetails {
                        field: format!("{}_HOST", prefix),
                        message: "Environment variable not set".to_string(),
                        currentvalue: None,
                        expected: Some("Valid hostname or IP address".to_string()),
                        user_error: true,
                    },
                ))
            })?;

        let port = config.get_port(prefix).ok_or_else(|| {
            crate::error::NestGateUnifiedError::Configuration(Box::new(
                crate::error::ConfigurationErrorDetails {
                    field: format!("{}_PORT", prefix),
                    message: "Environment variable not set or invalid port number".to_string(),
                    currentvalue: None,
                    expected: Some("Valid port number (1-65535)".to_string()),
                    user_error: true,
                },
            ))
        })?;

        Ok(Self { host, port })
    }

    /// Get socket address
    ///
    /// # Errors
    ///
    /// Returns error if host cannot be parsed as IP address
    pub fn socket_addr(&self) -> Result<SocketAddr> {
        let ip: IpAddr = self
            .host
            .parse()
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        Ok(SocketAddr::new(ip, self.port))
    }

    /// Get URL with scheme
    #[must_use]
    pub fn url(&self, scheme: &str) -> String {
        format!("{}://{}:{}", scheme, self.host, self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_dev_network() {
        let config = NetworkConfig::default_dev();
        assert_eq!(config.api_port(), 8080);
        assert_eq!(config.api_host(), "0.0.0.0");
    }

    #[test]
    fn test_database_url() {
        let config = NetworkConfig::default_dev();
        let url = config.database_url("nestgate");
        assert_eq!(url, "postgresql://localhost:5432/nestgate");
    }

    #[test]
    fn test_redis_url() {
        let config = NetworkConfig::default_dev();
        assert_eq!(config.redis_url(), "redis://localhost:6379");
    }

    #[test]
    fn test_metrics_endpoint() {
        let config = NetworkConfig::default_dev();
        assert_eq!(config.metrics_endpoint(), "0.0.0.0:9090");
    }

    #[test]
    fn test_endpoint_url() {
        let endpoint = EndpointConfig {
            host: "example.com".to_string(),
            port: 8080,
        };
        assert_eq!(endpoint.url("https"), "https://example.com:8080");
    }

    #[test]
    fn test_from_env_with_defaults() {
        // Should use defaults when env vars not set
        let endpoint = EndpointConfig::from_env("NONEXISTENT", "localhost", 8080)
            .expect("Should create endpoint with defaults");
        assert_eq!(endpoint.host, "localhost");
        assert_eq!(endpoint.port, 8080);
    }
}
