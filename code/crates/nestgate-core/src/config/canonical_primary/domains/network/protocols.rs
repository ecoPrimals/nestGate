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
    /// Creates a development-optimized network protocol configuration
    ///
    /// Returns a `NetworkProtocolConfig` with HTTP/1.1, WebSocket enabled with large frames,
    /// and gRPC disabled for simplified local development.
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

    /// Creates a production-hardened network protocol configuration with HTTP/2
    ///
    /// Returns a `NetworkProtocolConfig` with HTTP/2, optimized WebSocket frames,
    /// and gRPC enabled for high-performance production deployments.
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

    /// Validates the network protocol configuration for correctness
    ///
    /// Currently performs basic validation. Can be extended to validate protocol-specific
    /// settings like frame sizes, message limits, and version compatibility.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails. Currently always succeeds but may be extended
    /// with additional validation rules in the future.
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Merges another protocol configuration into this one, overwriting with other's values
    ///
    /// Takes all protocol settings from `other` and replaces the current configuration.
    /// Useful for layering configurations (e.g., defaults + environment overrides).
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.http = other.http;
        self.websocket = other.websocket;
        self.grpc = other.grpc;
        self.protocol_settings = other.protocol_settings;
        self
    }
}
