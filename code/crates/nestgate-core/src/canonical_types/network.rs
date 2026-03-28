//! **NETWORK TYPES** — Network communication and connectivity

use crate::canonical_modernization::canonical_constants::network::{DEFAULT_API_PORT, LOCALHOST};
use serde::{Deserialize, Serialize};

/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Status values for Connection
pub enum ConnectionStatus {
    /// Connection is established and active
    Connected,
    /// Connection is not established
    Disconnected,
    /// Connection is in progress
    Connecting,
    /// Connection attempt failed
    Failed,
    /// Connection attempt timed out
    Timeout,
}

/// Network protocol types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Protocol
pub enum Protocol {
    /// HTTP protocol
    Http,
    /// HTTPS (secure HTTP) protocol
    Https,
    /// TCP protocol
    Tcp,
    /// UDP protocol
    Udp,
    /// WebSocket protocol
    WebSocket,
    /// gRPC protocol
    Grpc,
    /// Custom protocol with specified name
    Custom(String),
}

/// Network endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Endpoint
pub struct Endpoint {
    /// Hostname or IP address
    pub host: String,
    /// Port number
    pub port: u16,
    /// Network protocol
    pub protocol: Protocol,
}

impl Default for Endpoint {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            host: LOCALHOST.to_string(),
            port: DEFAULT_API_PORT,
            protocol: Protocol::Http,
        }
    }
}
