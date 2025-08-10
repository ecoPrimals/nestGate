//! Streaming Configuration Settings
//!
//! Streaming and real-time configuration types for the NestGate API.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Streaming and real-time configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStreamingSettings {
    /// Enable MCP streaming
    pub enable_mcp_streaming: bool,
    /// Stream buffer size
    pub stream_buffer_size: usize,
    /// Maximum concurrent streams
    pub max_concurrent_streams: usize,
    /// Stream timeout
    pub stream_timeout: Duration,
    /// Enable stream compression
    pub enable_stream_compression: bool,
    /// Stream heartbeat interval
    pub stream_heartbeat_interval: Duration,
    /// Maximum stream message size
    pub max_stream_message_size: usize,
    /// Stream backpressure threshold
    pub stream_backpressure_threshold: usize,
}

/// Server-Sent Events (SSE) configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSseSettings {
    /// Enable SSE
    pub enable_sse: bool,
    /// SSE connection timeout
    pub sse_timeout: Duration,
    /// SSE keep-alive interval
    pub sse_keep_alive_interval: Duration,
    /// Maximum SSE connections per client
    pub max_sse_connections_per_client: usize,
    /// SSE message buffer size
    pub sse_message_buffer_size: usize,
    /// Enable SSE compression
    pub enable_sse_compression: bool,
}

impl Default for ApiStreamingSettings {
    fn default() -> Self {
        Self {
            enable_mcp_streaming: true,
            stream_buffer_size: 64 * 1024, // 64KB
            max_concurrent_streams: 100,
            stream_timeout: Duration::from_secs(300), // 5 minutes
            enable_stream_compression: true,
            stream_heartbeat_interval: Duration::from_secs(30),
            max_stream_message_size: 1024 * 1024, // 1MB
            stream_backpressure_threshold: 1000,
        }
    }
}

impl Default for ApiSseSettings {
    fn default() -> Self {
        Self {
            enable_sse: true,
            sse_timeout: Duration::from_secs(300), // 5 minutes
            sse_keep_alive_interval: Duration::from_secs(30),
            max_sse_connections_per_client: 10,
            sse_message_buffer_size: 1024, // 1KB
            enable_sse_compression: false,
        }
    }
} 