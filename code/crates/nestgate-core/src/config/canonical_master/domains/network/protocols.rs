// **NETWORK PROTOCOLS CONFIGURATION**

use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkProtocolConfig {
    pub http: HttpConfig,
    pub websocket: WebSocketConfig,
    pub grpc: GrpcConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HttpConfig {
    pub enabled: bool,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebSocketConfig {
    pub enabled: bool,
    pub max_frame_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrpcConfig {
    pub enabled: bool,
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
        self
    }
}
