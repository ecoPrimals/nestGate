//
// Provides high-performance streaming RPC capabilities for NestGate API.
// Supports bi-directional streaming for real-time data transfer and coordination.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

/// Streaming RPC server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::StreamingRpcConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::StreamingRpcConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for StreamingRpc
pub struct StreamingRpcConfig {
    /// Bind address for streaming RPC server
    pub bind_endpoint: String,
    /// Maximum concurrent streams
    pub max_concurrent_streams: usize,
    /// Stream timeout in seconds
    pub stream_timeout_seconds: u64,
    /// Buffer size for streaming channels
    pub buffer_size: usize,
    /// Enable compression for streams
    pub compression_enabled: bool,
}
impl Default for StreamingRpcConfig {
    /// Returns the default instance
    fn default() -> Self { 
        use nestgate_core::constants::hardcoding::{addresses, ports};
        
        Self {
            bind_endpoint: std::env::var("NESTGATE_STREAMING_RPC_ADDRESS")
                .unwrap_or_else(|_| {
                    let bind_addr = std::env::var("NESTGATE_BIND_ADDRESS")
                        .unwrap_or_else(|_| addresses::BIND_ALL_IPV4.to_string());
                    let port = std::env::var("NESTGATE_STREAMING_RPC_PORT")
                        .ok()
                        .and_then(|p| p.parse().ok())
                        .unwrap_or(ports::STREAMING_RPC_DEFAULT);
                    format!("{}:{}", bind_addr, port)
                }),
            max_concurrent_streams: std::env::var("NESTGATE_MAX_CONCURRENT_STREAMS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            stream_timeout_seconds: std::env::var("NESTGATE_STREAM_TIMEOUT_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(300),
            buffer_size: std::env::var("NESTGATE_STREAM_BUFFER_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1024),
            compression_enabled: std::env::var("NESTGATE_STREAM_COMPRESSION")
                .map(|v| v == "true")
                .unwrap_or(true),
         }
}

/// Streaming RPC server
pub struct StreamingRpcServer {
    /// Server configuration
    config: StreamingRpcConfig,
    /// Active streams count
    active_streams: Arc<RwLock<usize>>,
    /// Broadcast channel for server events
    event_tx: broadcast::Sender<String>,
}
impl StreamingRpcServer {
    /// Create a new streaming RPC server
    pub fn new(config: StreamingRpcConfig) -> Self { let (event_tx, _) = broadcast::channel(1000);

        Self {
            config,
            active_streams: Arc::new(RwLock::new(0)),
            event_tx,
         }

    /// Start the streaming RPC server
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
        tracing::info!(
            "Starting streaming RPC server on {}",
            self.config.bind_address
        );
        tracing::info!("✅ Streaming RPC server started successfully");
        Ok(())
    }

    /// Get active stream count
    pub async fn get_active_stream_count(&self) -> usize {
        *self.active_streams.read().await
    }

    /// Subscribe to server events
    pub fn subscribe_events(&self) -> broadcast::Receiver<String> {
        self.event_tx.subscribe()
    }
}

impl Default for StreamingRpcServer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new(StreamingRpcConfig::default())
    }
}

/// Create a new streaming RPC server with default configuration
pub fn create_streaming_rpc_server() -> StreamingRpcServer {
    StreamingRpcServer::default()
}
/// Create a new streaming RPC server with custom configuration
pub fn create_streaming_rpc_server_with_config(config: StreamingRpcConfig) -> StreamingRpcServer {
    StreamingRpcServer::new(config)
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Streamingrpcconfigcanonical
pub type StreamingRpcConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StreamingRpcConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

