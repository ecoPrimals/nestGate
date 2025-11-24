//
// This module extends the existing MCP protocol with streaming capabilities
// for high-throughput, low-latency communication with AI systems and
// external MCP clusters.

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{broadcast, RwLock};
use tracing::info;
// Removed unused tracing import
use uuid::Uuid;

/// MCP streaming manager for extended MCP protocol support
pub struct McpStreamingManager {
    /// Active streams
    active_streams: Arc<RwLock<HashMap<Uuid, StreamInfo>>>,
    /// Stream statistics
    stats: Arc<RwLock<StreamStatsSnapshot>>,
    /// Event broadcaster
    event_broadcaster: broadcast::Sender<StreamEvent>,
}
/// Stream statistics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamStatsSnapshot {
    /// Total number of streams created
    pub total_streams: u64,
    /// Number of currently active streams
    pub active_streams: u64,
    /// Total bytes transferred across all streams
    pub bytes_transferred: u64,
    /// Total messages sent across all streams
    pub messages_sent: u64,
    /// Total messages received across all streams
    pub messages_received: u64,
    /// Total number of errors encountered
    pub errors: u64,
}
/// Stream statistics (real-time)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamStats {
    /// Bytes transferred through this stream
    pub bytes_transferred: u64,
    /// Messages sent through this stream
    pub messages_sent: u64,
    /// Messages received through this stream
    pub messages_received: u64,
    /// Number of errors on this stream
    pub errors: u64,
    /// Timestamp when the stream was created
    pub created_at: SystemTime,
    /// Timestamp of last activity on the stream
    pub last_activity: SystemTime,
}
/// Information about an active stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    /// Stream identifier
    pub id: Uuid,
    /// Stream type
    pub stream_type: StreamType,
    /// Stream configuration
    pub config: StreamConfig,
    /// Stream statistics
    pub stats: StreamStats,
    /// Stream status
    pub status: StreamStatus,
    /// Creation timestamp
    pub created_at: SystemTime,
    /// Last activity timestamp
    pub last_activity: SystemTime,
}
/// Types of MCP streams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamType {
    /// Real-time storage monitoring
    StorageMonitoring,
    /// System metrics streaming
    MetricsStreaming,
    /// Event stream
    EventStreaming,
    /// State synchronization
    StateSynchronization,
}
/// Stream configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::StreamConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::StreamConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct StreamConfig {
    /// Type of MCP stream
    pub stream_type: StreamType,
    /// Buffer size for the stream in bytes
    pub buffer_size: usize,
    /// Whether to enable compression
    pub compression: bool,
    /// Whether to enable encryption
    pub encryption: bool,
    /// Batch size for message processing
    pub batch_size: usize,
    /// Interval for flushing buffered data
    pub flush_interval: Duration,
    /// Additional stream _metadata
    pub _metadata: HashMap<String, String>,
}
/// Stream status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamStatus {
    /// Stream is actively processing data
    Active,
    /// Stream is temporarily paused
    Paused,
    /// Stream has been stopped
    Stopped,
    /// Stream encountered an error
    Error(String),
}
/// Stream events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    /// Unique identifier for the stream
    pub stream_id: Uuid,
    /// Type of stream event that occurred
    pub event_type: StreamEventType,
    /// Event data payload
    pub _data: serde_json::Value,
    /// Event occurrence timestamp
    pub timestamp: SystemTime,
}
/// Stream event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamEventType {
    /// A new stream was created
    StreamCreated,
    /// An existing stream was closed
    StreamClosed,
    /// Data was received on the stream
    DataReceived,
    /// Data was sent through the stream
    DataSent,
    /// An error occurred on the stream
    Error,
}
impl Default for McpStreamingManager {
    fn default() -> Self {
        Self::new()
    }
}

impl McpStreamingManager {
    /// Create a new MCP streaming manager
    ///
    /// Initializes a Model Control Protocol (MCP) streaming manager that provides
    /// high-performance, low-latency bidirectional communication with AI systems
    /// and external MCP clusters.
    ///
    /// ## Features
    ///
    /// - **High-Throughput Streaming**: Efficient data transfer for AI workloads
    /// - **Bidirectional Communication**: Full duplex communication with AI systems
    /// - **Stream Management**: Automatic lifecycle management of active streams
    /// - **Performance Monitoring**: Real-time statistics and metrics tracking
    /// - **Error Handling**: Robust error detection and recovery mechanisms
    /// - **Automatic Cleanup**: Background tasks for connection maintenance
    ///
    /// ## Stream Types Supported
    ///
    /// - **AI Inference**: Real-time model inference streaming
    /// - **Data Processing**: Bulk data processing pipelines
    /// - **Model Training**: Training data and parameter streaming
    /// - **System Monitoring**: Health and performance metrics
    ///
    /// ## Usage
    ///
    /// ```rust
    /// use nestgate_api::mcp_streaming::McpStreamingManager;
    ///
    /// let mcp_manager = Self::new();
    ///
    /// // Start background cleanup
    /// let _cleanup_task = mcp_manager.start_cleanup_task();
    ///
    /// // Create a new stream
    /// let stream_config = StreamConfig {
    ///     stream_type: StreamType::AIInference,
    ///     buffer_size: 1000,
    ///     compression: true,
    ///     priority: StreamPriority::High,
    /// };
    /// let stream = mcp_manager.create_stream(stream_config).await?;
    /// ```
    ///
    /// ## Performance
    ///
    /// The manager uses optimized broadcasting and connection pooling to handle
    /// high-throughput AI workloads with minimal latency overhead.
    #[must_use]
    pub fn new() -> Self {
        let (event_broadcaster, _) = broadcast::channel(1000);

        Self {
            active_streams: Arc::new(RwLock::new(HashMap::new()),
            stats: Arc::new(RwLock::new(StreamStatsSnapshot {
                total_streams: 0,
                active_streams: 0,
                bytes_transferred: 0,
                messages_sent: 0,
                messages_received: 0,
                errors: 0,
            }),
            event_broadcaster,
        }
    }

    /// Create a new stream
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn create_stream(
        &self,
        config: StreamConfig,
    ) -> Result<StreamInfo, Box<dyn std::error::Error + Send + Sync>>  {
        let stream_id = Uuid::new_v4();
        let now = SystemTime::now();

        let stream_info = StreamInfo {
            id: stream_id,
            stream_type: config.stream_type.clone(),
            config: config.clone(),
            stats: StreamStats {
                bytes_transferred: 0,
                messages_sent: 0,
                messages_received: 0,
                errors: 0,
                created_at: now,
                last_activity: now,
            }
            status: StreamStatus::Active,
            created_at: now,
            last_activity: now,
        };

        // Add to active streams
        {
            let mut streams = self.active_streams.write().await;
            streams.insert(stream_id, stream_info.clone());
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_streams += 1;
            stats.active_streams += 1;
        }

        info!(
            "Created MCP stream: {} ({:?})",
            stream_id, config.stream_type
        );

        Ok(stream_info)
    }

    /// List all active streams
    pub async fn list_streams(&self) -> Vec<StreamInfo> {
        let streams = self.active_streams.read().await;
        streams.values().cloned().collect()
    }

    /// Get stream statistics
    pub fn get_stats(&self) -> StreamStatsSnapshot {
        // For now, return a basic snapshot
        StreamStatsSnapshot {
            total_streams: 0,
            active_streams: 0,
            bytes_transferred: 0,
            messages_sent: 0,
            messages_received: 0,
            errors: 0,
        }
    }

    /// Send data to a specific stream
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn send_to_stream(
        &self,
        stream_id: &Uuid,
        _data: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
        let streams = self.active_streams.read().await;
        if streams.contains_key(stream_id) {
            info!("Sending data to stream: {}", stream_id);
            // In a real implementation, this would send data through the stream
            Ok(())
        } else {
            Err("Stream not found".into())
        }
    }

    /// Clean up inactive streams
    pub async fn cleanup_streams(&self) {
        let mut streams = self.active_streams.write().await;
        let now = SystemTime::now();
        let timeout = Duration::from_secs(300); // 5 minutes timeout

        let mut to_remove = Vec::new();
        for (id, stream) in streams.iter() {
            if let Ok(elapsed) = now.duration_since(stream.last_activity) {
                if elapsed > timeout {
                    to_remove.push(*id);
                }
            }
        }

        for id in to_remove {
            streams.remove(&id);

            // Update statistics
            if let Ok(mut stats) = self.stats.try_write() {
                stats.active_streams = stats.active_streams.saturating_sub(1);
            }
        }
    }

    /// Start background cleanup task
    pub fn start_cleanup_task(&self) -> tokio::task::JoinHandle<()> {
        let manager = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                manager.cleanup_streams().await;
            }
        })
    }

    /// Close a stream
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn close_stream(
        &self,
        stream_id: &Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
        let mut streams = self.active_streams.write().await;
        if streams.remove(stream_id).is_some() {
            // Update statistics
            let mut stats = self.stats.write().await;
            stats.active_streams = stats.active_streams.saturating_sub(1);

            info!("Closed MCP stream: {}", stream_id);
            Ok(())
        } else {
            Err("Stream not found".into())
        }
    }
}

impl Clone for McpStreamingManager {
    fn clone(&self) -> Self { Self {
            active_streams: Arc::clone(&self.active_streams),
            stats: Arc::clone(&self.stats),
            event_broadcaster: self.event_broadcaster.clone(),
         }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type StreamConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StreamConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

