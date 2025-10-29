//
// TCP, UDP, HTTP, WebSocket, and custom protocol configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

/// Network protocol configuration settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkProtocolSettings {
    /// TCP protocol settings
    pub tcp: TcpProtocolSettings,
    /// UDP protocol settings
    pub udp: UdpProtocolSettings,
    /// HTTP protocol settings
    pub http: HttpProtocolSettings,
    /// WebSocket protocol settings
    pub websocket: WebSocketProtocolSettings,
    /// Custom protocol settings
    pub custom_protocols: HashMap<String, CustomProtocolSettings>,
}
/// TCP protocol settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpProtocolSettings {
    /// Enable TCP keep-alive
    pub keep_alive: bool,
    /// Keep-alive timeout
    pub keep_alive_timeout: Duration,
    /// TCP no-delay option
    pub no_delay: bool,
    /// Socket buffer sizes
    pub buffer_sizes: SocketBufferSettings,
    /// Connection timeout
    pub connection_timeout: Duration,
}
/// UDP protocol settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UdpProtocolSettings {
    /// Enable UDP broadcast
    pub enable_broadcast: bool,
    /// Enable UDP multicast
    pub enable_multicast: bool,
    /// Multicast groups
    pub multicast_groups: Vec<IpAddr>,
    /// Socket buffer sizes
    pub buffer_sizes: SocketBufferSettings,
}
/// HTTP protocol settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpProtocolSettings {
    /// HTTP version (1.1, 2.0)
    pub version: String,
    /// Request timeout
    pub request_timeout: Duration,
    /// Maximum concurrent connections
    pub max_connections: u32,
    /// Connection pool settings
    pub connection_pool: ConnectionPoolSettings,
    /// Compression settings
    pub compression: CompressionSettings,
}
/// WebSocket protocol settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketProtocolSettings {
    /// WebSocket ping interval
    pub ping_interval: Duration,
    /// WebSocket pong timeout
    pub pong_timeout: Duration,
    /// Maximum message size
    pub max_message_size: usize,
    /// Maximum frame size
    pub max_frame_size: usize,
    /// Enable compression
    pub compression_enabled: bool,
}
/// Custom protocol settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProtocolSettings {
    /// Protocol name
    pub name: String,
    /// Protocol version
    pub version: String,
    /// Protocol configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Protocol enabled
    pub enabled: bool,
}
/// Socket buffer settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketBufferSettings {
    /// Send buffer size
    pub send_buffer_size: usize,
    /// Receive buffer size
    pub receive_buffer_size: usize,
}
/// Connection pool settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolSettings {
    /// Minimum connections
    pub min_connections: u32,
    /// Maximum connections
    pub max_connections: u32,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout
    pub idle_timeout: Duration,
}
/// Compression settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionSettings {
    /// Enable compression
    pub enabled: bool,
    /// Compression algorithm
    pub algorithm: String,
    /// Compression level
    pub level: u8,
    /// Minimum size for compression
    pub min_size: usize,
}

impl Default for TcpProtocolSettings {
    fn default() -> Self {
        Self {
            keep_alive: true,
            keep_alive_timeout: Duration::from_secs(60),
            no_delay: true,
            buffer_sizes: SocketBufferSettings::default(),
            connection_timeout: Duration::from_secs(30),
        }
    }
}

impl Default for HttpProtocolSettings {
    fn default() -> Self {
        Self {
            version: "1.1".to_string(),
            request_timeout: Duration::from_secs(30),
            max_connections: 100,
            connection_pool: ConnectionPoolSettings::default(),
            compression: CompressionSettings::default(),
        }
    }
}

impl Default for WebSocketProtocolSettings {
    fn default() -> Self {
        Self {
            ping_interval: Duration::from_secs(30),
            pong_timeout: Duration::from_secs(10),
            max_message_size: 1024 * 1024, // 1MB
            max_frame_size: 64 * 1024,     // 64KB
            compression_enabled: true,
        }
    }
}

impl Default for SocketBufferSettings {
    fn default() -> Self {
        Self {
            send_buffer_size: 64 * 1024,    // 64KB
            receive_buffer_size: 64 * 1024, // 64KB
        }
    }
}

impl Default for ConnectionPoolSettings {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 100,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
        }
    }
}

impl Default for CompressionSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: "gzip".to_string(),
            level: 6,
            min_size: 1024, // 1KB
        }
    }
}
