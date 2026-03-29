// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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

#[cfg(test)]
mod round5_canonical_network_tests {
    use super::*;

    #[test]
    fn round5_endpoint_default_fields() {
        let e = Endpoint::default();
        assert_eq!(e.port, DEFAULT_API_PORT);
        assert_eq!(e.protocol, Protocol::Http);
    }

    #[test]
    fn round5_endpoint_serde_roundtrip() {
        let e = Endpoint {
            host: "10.0.0.1".to_string(),
            port: 443,
            protocol: Protocol::Https,
        };
        let json = serde_json::to_string(&e).unwrap();
        let back: Endpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(back.host, e.host);
        assert_eq!(back.port, e.port);
    }

    #[test]
    fn round5_connection_status_serde_roundtrip() {
        let s = ConnectionStatus::Timeout;
        let json = serde_json::to_string(&s).unwrap();
        let back: ConnectionStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }

    #[test]
    fn round5_protocol_serde_roundtrip() {
        let p = Protocol::WebSocket;
        let json = serde_json::to_string(&p).unwrap();
        let back: Protocol = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
    }
}
