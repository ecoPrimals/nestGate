/// Unified Network Configuration Module
/// Consolidates all network-related configuration patterns
/// **PROBLEM SOLVED**: Eliminates duplicate network configuration structures
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

use super::retry_config::UnifiedRetryConfig;
use super::timeout_config::UnifiedTimeoutConfig;

/// Unified Network Configuration - consolidates all network settings
/// Replaces 25+ duplicate network configuration structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedNetworkConfig {
    /// Primary network bind address
    pub bind_address: IpAddr,
    /// Primary service port
    pub port: u16,
    /// API service port (legacy compatibility)
    pub api_port: u16,
    /// WebSocket service port (optional)
    pub websocket_port: Option<u16>,
    /// Service name for registration
    pub service_name: String,
    /// Number of worker threads for network operations
    pub worker_threads: u32,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// TCP keep-alive settings
    pub tcp_keepalive: bool,
    /// Keep-alive timeout
    pub keepalive_timeout: Duration,
    /// Connection timeout (legacy compatibility)
    pub connection_timeout: Duration,
    /// Keep alive flag (legacy compatibility)
    pub keep_alive: bool,
    /// Connection buffer sizes
    pub buffer_size: usize,
    /// Enable IPv6 support
    pub ipv6_enabled: bool,
    /// Network timeout configuration
    pub timeouts: UnifiedTimeoutConfig,
    /// Retry configuration for network operations
    pub retry_config: UnifiedRetryConfig,
    /// Retry configuration (legacy compatibility)
    pub retry: UnifiedRetryConfig,
    /// TLS/SSL configuration
    pub tls_config: Option<NetworkTlsConfig>,
    /// Proxy settings
    pub proxy_config: Option<NetworkProxyConfig>,
    /// Load balancing configuration
    pub load_balance_config: Option<NetworkLoadBalanceConfig>,
    /// Quality of Service settings
    pub qos_config: Option<NetworkQosConfig>,
    /// Network interface binding (optional)
    pub interface: Option<String>,
    /// Custom network headers
    pub custom_headers: HashMap<String, String>,
    /// Network compression settings
    pub compression_enabled: bool,
    /// Compression flag (legacy compatibility)
    pub compression: bool,
    /// Service discovery enabled
    pub discovery_enabled: bool,
    /// Service endpoints for discovery
    pub service_endpoints: HashMap<String, String>,
    /// Maximum request size (bytes)
    pub max_request_size: usize,
    /// Rate limiting configuration
    pub rate_limit_config: Option<NetworkRateLimitConfig>,
}

impl Default for UnifiedNetworkConfig {
    fn default() -> Self {
        Self {
            bind_address: crate::safe_operations::safe_parse_ip_with_fallback(
                "127.0.0.1",
                std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                "default_network_config",
            ),
            port: 8080,
            api_port: 8080,
            websocket_port: None,
            // SOVEREIGNTY FIX: Use dynamic service identification
            service_name: std::env::var("NESTGATE_SERVICE_NAME")
                .unwrap_or_else(|_| format!("nestgate-{}", uuid::Uuid::new_v4().simple())),
            worker_threads: 4, // Default worker threads
            max_connections: 1000_usize,
            tcp_keepalive: true,
            keepalive_timeout: Duration::from_secs(75),
            connection_timeout: Duration::from_secs(30),
            keep_alive: true,
            buffer_size: 8192,
            ipv6_enabled: false,
            timeouts: UnifiedTimeoutConfig::default(),
            retry_config: UnifiedRetryConfig::default(),
            retry: UnifiedRetryConfig::default(),
            tls_config: None,
            proxy_config: None,
            load_balance_config: None,
            qos_config: None,
            interface: None,
            custom_headers: HashMap::new(),
            compression_enabled: true,
            compression: true,
            discovery_enabled: true,
            service_endpoints: HashMap::new(),
            max_request_size: 1024 * 1024, // 1MB
            rate_limit_config: None,
        }
    }
}

/// TLS/SSL configuration for network connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTlsConfig {
    /// Enable TLS
    pub enabled: bool,
    /// Certificate file path
    pub cert_file: Option<String>,
    /// Private key file path
    pub key_file: Option<String>,
    /// CA certificate file path
    pub ca_file: Option<String>,
    /// TLS protocol version
    pub protocol_version: String,
    /// Cipher suites
    pub cipher_suites: Vec<String>,
    /// Enable client certificate verification
    pub verify_client: bool,
    /// Certificate validation mode
    pub validation_mode: String,
}

impl Default for NetworkTlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_file: None,
            key_file: None,
            ca_file: None,
            protocol_version: "TLSv1.3".to_string(),
            cipher_suites: vec![
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_AES_128_GCM_SHA256".to_string(),
            ],
            verify_client: false,
            validation_mode: "peer".to_string(),
        }
    }
}

/// Proxy configuration for network connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProxyConfig {
    /// Enable proxy
    pub enabled: bool,
    /// Proxy server address
    pub server: String,
    /// Proxy server port
    pub port: u16,
    /// Proxy authentication
    pub username: Option<String>,
    /// Proxy password
    pub password: Option<String>,
    /// Proxy type (http, socks5)
    pub proxy_type: String,
    /// Bypass proxy for these hosts
    pub bypass_hosts: Vec<String>,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkLoadBalanceConfig {
    /// Load balancing algorithm
    pub algorithm: String,
    /// Backend servers
    pub backends: Vec<String>,
    /// Health check configuration
    pub health_check: LoadBalanceHealthCheck,
    /// Sticky sessions configuration
    pub sticky_sessions: bool,
    /// Session timeout
    pub session_timeout: Duration,
}

/// Health check configuration for load balancing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalanceHealthCheck {
    /// Enable health checks
    pub enabled: bool,
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Health check endpoint
    pub endpoint: String,
    /// Expected status code
    pub expected_status: u16,
}

/// Quality of Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkQosConfig {
    /// Enable QoS
    pub enabled: bool,
    /// Traffic shaping
    pub traffic_shaping: bool,
    /// Bandwidth limit (bytes per second)
    pub bandwidth_limit: Option<u64>,
    /// Priority levels
    pub priority_levels: HashMap<String, u8>,
    /// Queue management
    pub queue_management: String,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per second limit
    pub requests_per_second: u32,
    /// Burst capacity
    pub burst_capacity: u32,
    /// Rate limiting window (seconds)
    pub window_seconds: u64,
    /// Rate limiting strategy
    pub strategy: String,
    /// Exclude these IPs from rate limiting
    pub exclude_ips: Vec<String>,
}

impl UnifiedNetworkConfig {
    /// Create a production-optimized network configuration
    pub fn production() -> Self {
        Self {
            bind_address: crate::safe_operations::safe_parse_ip_with_fallback(
                "0.0.0.0",
                std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED),
                "production_config",
            ),
            port: 443,
            worker_threads: 8, // Production worker threads
            max_connections: 10000,
            tcp_keepalive: true,
            keepalive_timeout: Duration::from_secs(300),
            buffer_size: 16384,
            ipv6_enabled: true,
            timeouts: UnifiedTimeoutConfig::production(),
            retry_config: UnifiedRetryConfig::slow(),
            tls_config: Some(NetworkTlsConfig {
                enabled: true,
                ..Default::default()
            }),
            compression_enabled: true,
            max_request_size: 10 * 1024 * 1024, // 10MB
            rate_limit_config: Some(NetworkRateLimitConfig {
                enabled: true,
                requests_per_second: 1000,
                burst_capacity: 2000,
                window_seconds: 60,
                strategy: "sliding_window".to_string(),
                exclude_ips: vec!["127.0.0.1".to_string()],
            }),
            ..Default::default()
        }
    }

    /// Create a development-optimized network configuration
    pub fn development() -> Self {
        Self {
            bind_address: crate::safe_operations::safe_parse_ip_with_fallback(
                "127.0.0.1",
                std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                "development_network_config",
            ),
            port: 8080,
            worker_threads: 2,
            max_connections: 100,
            timeouts: UnifiedTimeoutConfig::development(),
            retry_config: UnifiedRetryConfig::fast(),
            tls_config: None,
            rate_limit_config: None,
            ..Default::default()
        }
    }

    /// Validate network configuration
    pub fn validate(&self) -> crate::Result<()> {
        if self.port == 0 {
            return Err(crate::NestGateError::Configuration {
                field: "port".to_string(),
                message: "Network port cannot be zero".to_string(),
                current_value: Some("0".to_string()),
                expected: Some("positive port number (1-65535)".to_string()),
                user_error: true,
            });
        }

        if self.max_connections == 0 {
            return Err(crate::NestGateError::Configuration {
                field: "max_connections".to_string(),
                message: "Max connections cannot be zero".to_string(),
                current_value: Some("0".to_string()),
                expected: Some("positive integer (e.g., 1000)".to_string()),
                user_error: true,
            });
        }

        if self.worker_threads == 0 {
            return Err(crate::NestGateError::Configuration {
                field: "worker_threads".to_string(),
                message: "Worker threads cannot be zero".to_string(),
                current_value: Some("0".to_string()),
                expected: Some("positive integer (e.g., 4)".to_string()),
                user_error: true,
            });
        }
        Ok(())
    }
}
