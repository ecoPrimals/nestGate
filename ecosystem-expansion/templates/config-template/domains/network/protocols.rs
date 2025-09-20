//! **NETWORK PROTOCOLS CONFIGURATION**

use serde::{Deserialize, Serialize};
use crate::Result;

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
    pub fn development_optimized() -> Self {
        Self {
            http: HttpConfig { enabled: true, version: "HTTP/1.1".to_string() },
            websocket: WebSocketConfig { enabled: true, max_frame_size: 1024 * 1024 },
            grpc: GrpcConfig { enabled: false, max_message_size: 4 * 1024 * 1024 },
        }
    }

    pub fn production_hardened() -> Self {
        Self {
            http: HttpConfig { enabled: true, version: "HTTP/2".to_string() },
            websocket: WebSocketConfig { enabled: true, max_frame_size: 64 * 1024 },
            grpc: GrpcConfig { enabled: true, max_message_size: 16 * 1024 * 1024 },
        }
    }

    pub fn validate(&self) -> Result<()> { Ok(()) }
    
    pub fn merge(mut self, other: Self) -> Self {
        self.http = other.http;
        self.websocket = other.websocket;
        self.grpc = other.grpc;
        self
    }
} 