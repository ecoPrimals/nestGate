// **NETWORK DOMAIN CONSTANTS**
//! Network functionality and utilities.
// Network-related constants extracted from the consolidated constants system.
// This module consolidates all network-related constants:
//! - Port numbers, addresses, timeouts
//! - Connection limits, buffer sizes
//! - Protocol-specific settings

use std::time::Duration;

// ==================== NETWORK DOMAIN CONSTANTS ====================

/// **NETWORK DOMAIN CONSTANTS**
///
/// Consolidates all network-related constants:
/// - Port numbers, addresses, timeouts
/// - Connection limits, buffer sizes
/// - Protocol-specific settings
#[derive(Debug, Clone)]
pub struct NetworkDomainConstants {
    /// Port configuration
    pub ports: NetworkPorts,

    /// Address configuration
    pub addresses: NetworkAddresses,

    /// Timeout configuration
    pub timeouts: NetworkTimeouts,

    /// Connection limits
    pub limits: NetworkLimits,

    /// Buffer sizes
    pub buffers: NetworkBuffers,

    /// Protocol settings
    pub protocols: NetworkProtocols,
}
#[derive(Debug, Clone)]
pub struct NetworkPorts {
    /// Primary API server port
    pub api: u16,

    /// Internal service communication port
    pub internal: u16,

    /// Health check endpoint port
    pub health: u16,

    /// Metrics collection port
    pub metrics: u16,

    /// WebSocket connection port
    pub websocket: u16,

    /// gRPC service port
    pub grpc: u16,

    /// Development/testing port
    pub dev: u16,

    /// MCP protocol port
    pub mcp: u16,
}

#[derive(Debug, Clone)]
pub struct NetworkAddresses {
    /// Default bind address
    pub bind: &'static str,

    /// Localhost address
    pub localhost: &'static str,

    /// Any address (0.0.0.0)
    pub any: &'static str,

    /// Loopback address
    pub loopback: &'static str,

    /// Default API base URL
    pub api_base: &'static str,

    /// Default discovery endpoint
    pub discovery: &'static str,
}

#[derive(Debug, Clone)]
pub struct NetworkTimeouts {
    /// Connection establishment timeout
    pub connection: Duration,

    /// Request processing timeout
    pub request: Duration,

    /// Response timeout
    pub response: Duration,

    /// Health check timeout
    pub health_check: Duration,

    /// Keep-alive timeout
    pub keep_alive: Duration,

    /// Circuit breaker timeout
    pub circuit_breaker: Duration,

    /// Load balancer timeout
    pub load_balancer: Duration,
}

#[derive(Debug, Clone)]
pub struct NetworkLimits {
    /// Maximum concurrent connections
    pub max_connections: usize,

    /// Maximum request size in bytes
    pub max_request_size: usize,

    /// Maximum response size in bytes
    pub max_response_size: usize,

    /// Rate limit requests per minute
    pub rate_limit_rpm: u32,

    /// Rate limit burst size
    pub rate_limit_burst: u32,

    /// Circuit breaker failure threshold
    pub circuit_breaker_threshold: u32,
}

#[derive(Debug, Clone)]
pub struct NetworkBuffers {
    /// Default network buffer size
    pub default: usize,

    /// Receive buffer size
    pub receive: usize,

    /// Send buffer size
    pub send: usize,

    /// Maximum packet size
    pub max_packet: usize,

    /// TCP buffer size
    pub tcp: usize,

    /// UDP buffer size
    pub udp: usize,
}

#[derive(Debug, Clone)]
pub struct NetworkProtocols {
    /// HTTP protocol version
    pub http_version: &'static str,

    /// Default content type
    pub content_type: &'static str,

    /// User agent string
    pub user_agent: &'static str,

    /// Default load balancer algorithm
    pub lb_algorithm: &'static str,

    /// Discovery protocol
    pub discovery_protocol: &'static str,
}

impl Default for NetworkDomainConstants {
    fn default() -> Self {
        Self {
            ports: NetworkPorts {
                api: 8080,
                internal: 8081,
                health: 8082,
                metrics: 8083,
                websocket: 8084,
                grpc: 8085,
                dev: 18080,
                mcp: 8086,
            },
            addresses: NetworkAddresses {
                bind: "127.0.0.1",
                localhost: "127.0.0.1",
                any: "0.0.0.0",
                loopback: "127.0.0.1",
                api_base: "http://localhost:8080/api/v1",
                discovery: "http://localhost:8083/discovery",
            },
            timeouts: NetworkTimeouts {
                connection: Duration::from_secs(30),
                request: Duration::from_secs(60),
                response: Duration::from_secs(30),
                health_check: Duration::from_secs(10),
                keep_alive: Duration::from_secs(75),
                circuit_breaker: Duration::from_secs(60),
                load_balancer: Duration::from_secs(30),
            },
            limits: NetworkLimits {
                max_connections: 1000,
                max_request_size: 10 * 1024 * 1024,  // 10MB
                max_response_size: 10 * 1024 * 1024, // 10MB
                rate_limit_rpm: 1000,
                rate_limit_burst: 100,
                circuit_breaker_threshold: 5,
            },
            buffers: NetworkBuffers {
                default: 65536,
                receive: 131_072,
                send: 131_072,
                max_packet: 65536,
                tcp: 262_144,
                udp: 65536,
            },
            protocols: NetworkProtocols {
                http_version: "HTTP/1.1",
                content_type: "application/json",
                user_agent: "NestGate/1.0",
                lb_algorithm: "round_robin",
                discovery_protocol: "http",
            },
        }
    }
}

// ==================== CONVENIENCE EXPORTS ====================

/// Convenience module for easy access to network constants
pub mod network_defaults {
    use super::*;
    /// Get default network domain constants
    #[must_use]
    pub fn constants() -> NetworkDomainConstants {
        NetworkDomainConstants::default()
    }

    /// Get default API port
    pub const DEFAULT_API_PORT: u16 = 8080;

    /// Get default bind address
    pub const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1";

    /// Get localhost address
    pub const LOCALHOST: &str = "127.0.0.1";
}
