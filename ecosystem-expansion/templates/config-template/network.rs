//! Network configuration structures

use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub api: ApiConfig,
    pub protocols: ProtocolConfig,
    pub service_endpoints: ServiceEndpoints,
    pub discovery: DiscoveryConfig,
    pub connection: ConnectionConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub bind_endpoint: IpAddr,
    pub enable_tls: bool,
    pub request_timeout: std::time::Duration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConfig {
    pub enabled: bool,
    pub http_version: String,
    pub websocket_enabled: bool,
    pub grpc_enabled: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    pub enabled: bool,
    pub api_endpoint: String,
    pub admin_endpoint: String,
    pub metrics_endpoint: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    pub enabled: bool,
    pub broadcast_interval_seconds: u32,
    pub discovery_timeout_seconds: u32,
    pub max_connections: u32,
    pub retry_attempts: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub max_connections: u32,
    pub connection_timeout_seconds: u32,
    pub keep_alive_seconds: u32,
    pub retry_attempts: u32,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            api: ApiConfig::default(),
            protocols: ProtocolConfig::default(),
            service_endpoints: ServiceEndpoints::default(),
            discovery: DiscoveryConfig::default(),
            connection: ConnectionConfig::default(),
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            bind_endpoint: std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            enable_tls: false,
            request_timeout: std::time::Duration::from_secs(30),
        }
    }
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            http_version: "HTTP/1.1".to_string(),
            websocket_enabled: true,
            grpc_enabled: false,
        }
    }
}

impl Default for ServiceEndpoints {
    fn default() -> Self {
        Self {
            enabled: true,
            api_endpoint: crate::constants::canonical_defaults::network::build_api_url(),
            admin_endpoint: "http://localhost:8081".to_string(),
            metrics_endpoint: "http://localhost:8082".to_string(),
        }
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            broadcast_interval_seconds: 30,
            discovery_timeout_seconds: 10,
            max_connections: 1000,
            retry_attempts: 3,
        }
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            max_connections: 1000,
            connection_timeout_seconds: 30,
            keep_alive_seconds: 300,
            retry_attempts: 3,
        }
    }
}
