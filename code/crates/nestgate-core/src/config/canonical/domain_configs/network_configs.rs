/// Network Configuration Domain
///
/// Replaces: NetworkConfig, ServerConfig, StreamConfig, ServiceMeshConfig,
/// NetworkFsConfig, and 8+ other network config structures
use super::CanonicalDomainConfig;
use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// **CANONICAL NETWORK CONFIGURATION**
/// Replaces: NetworkConfig, ServerConfig, StreamConfig, ServiceMeshConfig,
/// NetworkFsConfig, and 8+ other network config structures
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalNetworkConfig {
    /// Server settings
    pub server: NetworkServer,
    /// Client settings
    pub client: NetworkClient,
    /// Protocol settings
    pub protocols: NetworkProtocols,
    /// Security settings
    pub security: NetworkSecurity,
    /// Performance settings
    pub performance: NetworkPerformance,
    /// Service discovery settings
    pub discovery: NetworkDiscovery,
    /// Environment-specific overrides
    pub environment_overrides: HashMap<String, serde_json::Value>,
}
impl CanonicalDomainConfig for CanonicalNetworkConfig {
    fn domain() -> &'static str {
        "network"
    }

    fn validate(&self) -> Result<()> {
        if self.server.port == 0 {
            return Err(NestGateError::config_error(
                "server.port",
                "must be greater than 0",
            ));
        }
        Ok(())
    }

    fn merge(mut self, other: Self) -> Self {
        self.environment_overrides
            .extend(other.environment_overrides);
        self
    }

    fn from_environment() -> Result<Self> {
        Ok(Self::default())
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "server": {"type": "object", "description": "Server settings"},
                "protocols": {"type": "object", "description": "Protocol settings"}
            }
        })
    }
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkServer {
    pub bind_endpoint: String,
    pub port: u16,
    pub max_connections: u32,
    pub keep_alive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkClient {
    pub connection_timeout: Duration,
    pub read_timeout: Duration,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkProtocols {
    pub http: HttpProtocolConfig,
    pub https: HttpsProtocolConfig,
    pub grpc: GrpcProtocolConfig,
    pub websocket: WebSocketProtocolConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurity {
    pub enable_tls: bool,
    pub tls_version: TlsVersion,
    pub cipher_suites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPerformance {
    pub buffer_size: usize,
    pub compression_enabled: bool,
    pub keep_alive_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiscovery {
    pub enabled: bool,
    pub discovery_interval: Duration,
    pub endpoints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpProtocolConfig {
    pub version: HttpVersion,
    pub max_header_size: u32,
    pub max_body_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpsProtocolConfig {
    pub tls_version: TlsVersion,
    pub cipher_suites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcProtocolConfig {
    pub max_message_size: u32,
    pub keep_alive_interval: Duration,
    pub compression: GrpcCompression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketProtocolConfig {
    pub max_frame_size: u32,
    pub ping_interval: Duration,
    pub compression: bool,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpVersion {
    Http1_1,
    Http2,
    Http3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TlsVersion {
    Tls1_2,
    Tls1_3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrpcCompression {
    None,
    Gzip,
    Deflate,
}

// Default implementations
impl Default for NetworkServer {
    fn default() -> Self {
        Self {
            bind_endpoint: "0.0.0.0".to_string(),
            port: 8080,
            max_connections: 1000,
            keep_alive: true,
        }
    }
}

impl Default for NetworkClient {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(30),
            read_timeout: Duration::from_secs(60),
            retry_attempts: 3,
        }
    }
}

impl Default for NetworkSecurity {
    fn default() -> Self {
        Self {
            enable_tls: true,
            tls_version: TlsVersion::Tls1_3,
            cipher_suites: vec!["TLS_AES_256_GCM_SHA384".to_string()],
        }
    }
}

impl Default for NetworkPerformance {
    fn default() -> Self {
        Self {
            buffer_size: 8192,
            compression_enabled: true,
            keep_alive_timeout: Duration::from_secs(60),
        }
    }
}

impl Default for NetworkDiscovery {
    fn default() -> Self {
        Self {
            enabled: true,
            discovery_interval: Duration::from_secs(30),
            endpoints: Vec::new(),
        }
    }
}

impl Default for HttpProtocolConfig {
    fn default() -> Self {
        Self {
            version: HttpVersion::Http2,
            max_header_size: 8192,
            max_body_size: 1_048_576, // 1MB
        }
    }
}

impl Default for HttpsProtocolConfig {
    fn default() -> Self {
        Self {
            tls_version: TlsVersion::Tls1_3,
            cipher_suites: vec!["TLS_AES_256_GCM_SHA384".to_string()],
        }
    }
}

impl Default for GrpcProtocolConfig {
    fn default() -> Self {
        Self {
            max_message_size: 4_194_304, // 4MB
            keep_alive_interval: Duration::from_secs(30),
            compression: GrpcCompression::Gzip,
        }
    }
}

impl Default for WebSocketProtocolConfig {
    fn default() -> Self {
        Self {
            max_frame_size: 65536, // 64KB
            ping_interval: Duration::from_secs(30),
            compression: true,
        }
    }
}
