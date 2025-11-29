// **NETWORK PROTOCOLS CONFIGURATION**

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for NetworkProtocol
pub struct NetworkProtocolConfig {
    /// Http
    pub http: HttpConfig,
    /// Websocket
    pub websocket: WebSocketConfig,
    /// Grpc
    pub grpc: GrpcConfig,
    /// Protocol-specific settings (key-value pairs for custom protocol configuration)
    pub protocol_settings: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for Http
pub struct HttpConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Version
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for WebSocket
pub struct WebSocketConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Size of max frame
    pub max_frame_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for Grpc
pub struct GrpcConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Size of max message
    pub max_message_size: u32,
}

impl NetworkProtocolConfig {
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            http: HttpConfig {
                enabled: true,
                version: "HTTP/1.1".to_string(),
            },
            websocket: WebSocketConfig {
                enabled: true,
                max_frame_size: 1024 * 1024,
            },
            grpc: GrpcConfig {
                enabled: false,
                max_message_size: 4 * 1024 * 1024,
            },
            protocol_settings: HashMap::new(),
        }
    }

    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            http: HttpConfig {
                enabled: true,
                version: "HTTP/2".to_string(),
            },
            websocket: WebSocketConfig {
                enabled: true,
                max_frame_size: 64 * 1024,
            },
            grpc: GrpcConfig {
                enabled: true,
                max_message_size: 16 * 1024 * 1024,
            },
            protocol_settings: HashMap::new(),
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.http = other.http;
        self.websocket = other.websocket;
        self.grpc = other.grpc;
        self.protocol_settings = other.protocol_settings;
        self
    }
}
