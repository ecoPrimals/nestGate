//! Streaming Client Demo - Comprehensive Client Implementation
//!
//! This example demonstrates how to build a client that connects to and consumes
//! all the streaming and bidirectional communication capabilities of NestGate:
//!
//! 1. SSE (Server-Sent Events) client for real-time event streaming
//! 2. WebSocket client for bidirectional real-time communication
//! 3. Streaming RPC client for type-safe bidirectional RPC
//! 4. MCP client for AI system integration
//! 5. Multi-protocol event aggregation and coordination
//!
//! This showcases how external systems and applications can integrate with
//! NestGate's comprehensive communication infrastructure.

use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime},
};

use futures_util::{stream::SplitSink, stream::SplitStream, SinkExt, StreamExt};
use reqwest::Client;
use serde_json::{json, Value};
use tokio::{
    sync::{broadcast, mpsc},
    time::{interval, sleep, timeout},
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::{debug, error, info, warn};

// Import streaming RPC client components
use nestgate_api::streaming_rpc::{
    ClientMessage, EventFilter, EventSubscription, ServerMessage, StorageOperation,
    StreamingRpcClient, SystemMetrics, ZfsOperation,
};

/// Comprehensive streaming client that connects to all NestGate communication layers
pub struct NestGateStreamingClient {
    /// HTTP client for REST API and SSE
    http_client: Client,
    /// Base URL for HTTP/SSE connections
    base_url: String,
    /// RPC client for bidirectional streaming RPC
    rpc_client: Option<StreamingRpcClient>,
    /// WebSocket connection
    websocket_tx:
        Option<SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Message>>,
    /// Event aggregator
    event_aggregator: EventAggregator,
}

/// Event aggregator that combines events from all communication layers
#[derive(Clone)]
pub struct EventAggregator {
    /// Broadcasting channel for aggregated events
    event_broadcaster: broadcast::Sender<AggregatedEvent>,
    /// Event statistics
    stats: Arc<tokio::sync::RwLock<AggregatorStats>>,
}

/// Aggregated event from any communication layer
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregatedEvent {
    /// Event identifier
    pub id: String,
    /// Source communication layer
    pub source_layer: CommunicationLayer,
    /// Event type
    pub event_type: String,
    /// Event data
    pub data: Value,
    /// Timestamp when received
    pub received_at: SystemTime,
    /// Original timestamp (if available)
    pub original_timestamp: Option<SystemTime>,
}

/// Communication layer enumeration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CommunicationLayer {
    SSE,
    WebSocket,
    StreamingRPC,
    MCP,
    HTTP,
}

/// Event aggregator statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregatorStats {
    pub total_events: u64,
    pub events_by_layer: HashMap<String, u64>,
    pub last_event_time: Option<SystemTime>,
    pub processing_errors: u64,
}

impl EventAggregator {
    /// Create a new event aggregator
    pub fn new() -> Self {
        let (event_broadcaster, _) = broadcast::channel(10000);

        Self {
            event_broadcaster,
            stats: Arc::new(tokio::sync::RwLock::new(AggregatorStats {
                total_events: 0,
                events_by_layer: HashMap::new(),
                last_event_time: None,
                processing_errors: 0,
            })),
        }
    }

    /// Aggregate an event from any communication layer
    pub async fn aggregate_event(&self, event: AggregatedEvent) {
        debug!(
            "Aggregating event from {:?}: {}",
            event.source_layer, event.event_type
        );

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_events += 1;
            stats.last_event_time = Some(SystemTime::now());

            let layer_key = format!("{:?}", event.source_layer);
            *stats.events_by_layer.entry(layer_key).or_insert(0) += 1;
        }

        // Broadcast aggregated event
        if let Err(e) = self.event_broadcaster.send(event) {
            warn!("Failed to broadcast aggregated event: {}", e);

            let mut stats = self.stats.write().await;
            stats.processing_errors += 1;
        }
    }

    /// Subscribe to aggregated events
    pub fn subscribe(&self) -> broadcast::Receiver<AggregatedEvent> {
        self.event_broadcaster.subscribe()
    }

    /// Get aggregator statistics
    pub async fn get_stats(&self) -> AggregatorStats {
        self.stats.read().await.clone()
    }
}

impl NestGateStreamingClient {
    /// Create a new streaming client
    pub async fn new(base_url: String) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let http_client = Client::builder().timeout(Duration::from_secs(30)).build()?;

        let event_aggregator = EventAggregator::new();

        Ok(Self {
            http_client,
            base_url,
            rpc_client: None,
            websocket_tx: None,
            event_aggregator,
        })
    }

    /// Connect to all communication layers
    pub async fn connect_all(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Connecting to all NestGate communication layers...");

        // Connect to streaming RPC
        self.connect_streaming_rpc("127.0.0.1:8081").await?;

        // Connect to WebSocket
        self.connect_websocket().await?;

        // Start SSE connections
        self.start_sse_connections().await?;

        info!("Successfully connected to all communication layers");
        Ok(())
    }

    /// Connect to streaming RPC server
    async fn connect_streaming_rpc(
        &mut self,
        rpc_addr: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Connecting to streaming RPC server at {}", rpc_addr);

        let client = StreamingRpcClient::connect(rpc_addr.to_string()).await?;

        // Test connection with health check
        let health = client.health_check().await?;
        info!("RPC server health: {:?}", health);

        // Get capabilities
        let capabilities = client.get_capabilities().await?;
        info!("RPC server capabilities: {:?}", capabilities);

        self.rpc_client = Some(client);
        Ok(())
    }

    /// Connect to WebSocket
    async fn connect_websocket(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Connecting to WebSocket...");

        let ws_url = format!(
            "ws://{}/api/v1/communication/websocket",
            self.base_url.replace("http://", "")
        );
        let (ws_stream, _) = connect_async(&ws_url).await?;
        let (ws_tx, mut ws_rx) = ws_stream.split();

        self.websocket_tx = Some(ws_tx);

        // Start WebSocket message handler
        let aggregator = self.event_aggregator.clone();
        tokio::spawn(async move {
            while let Some(message) = ws_rx.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        if let Ok(data) = serde_json::from_str::<Value>(&text) {
                            let event = AggregatedEvent {
                                id: uuid::Uuid::new_v4().to_string(),
                                source_layer: CommunicationLayer::WebSocket,
                                event_type: "websocket_message".to_string(),
                                data,
                                received_at: SystemTime::now(),
                                original_timestamp: None,
                            };

                            aggregator.aggregate_event(event).await;
                        }
                    }
                    Ok(Message::Binary(data)) => {
                        debug!("Received WebSocket binary message: {} bytes", data.len());
                    }
                    Ok(Message::Close(_)) => {
                        info!("WebSocket connection closed");
                        break;
                    }
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// Start SSE connections for different event types
    async fn start_sse_connections(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Starting SSE connections...");

        // Start SSE connection for storage events
        self.start_sse_stream("storage", "/api/v1/sse/storage")
            .await?;

        // Start SSE connection for health events
        self.start_sse_stream("health", "/api/v1/sse/health")
            .await?;

        // Start SSE connection for metrics
        self.start_sse_stream("metrics", "/api/v1/sse/metrics")
            .await?;

        // Start SSE connection for all events
        self.start_sse_stream("all", "/api/v1/sse/events").await?;

        Ok(())
    }

    /// Start a specific SSE stream
    async fn start_sse_stream(
        &self,
        stream_name: &str,
        endpoint: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}", self.base_url, endpoint);
        let client = self.http_client.clone();
        let aggregator = self.event_aggregator.clone();
        let stream_name = stream_name.to_string();

        tokio::spawn(async move {
            loop {
                match client.get(&url).send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            let mut stream = response.bytes_stream();

                            while let Some(chunk) = stream.next().await {
                                match chunk {
                                    Ok(bytes) => {
                                        let text = String::from_utf8_lossy(&bytes);

                                        // Parse SSE format
                                        for line in text.lines() {
                                            if line.starts_with("data: ") {
                                                let data_part = &line[6..]; // Remove "data: " prefix

                                                if let Ok(data) =
                                                    serde_json::from_str::<Value>(data_part)
                                                {
                                                    let event = AggregatedEvent {
                                                        id: uuid::Uuid::new_v4().to_string(),
                                                        source_layer: CommunicationLayer::SSE,
                                                        event_type: format!("sse_{}", stream_name),
                                                        data,
                                                        received_at: SystemTime::now(),
                                                        original_timestamp: None,
                                                    };

                                                    aggregator.aggregate_event(event).await;
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("SSE stream error for {}: {}", stream_name, e);
                                        break;
                                    }
                                }
                            }
                        } else {
                            error!(
                                "SSE connection failed for {}: {}",
                                stream_name,
                                response.status()
                            );
                        }
                    }
                    Err(e) => {
                        error!("Failed to connect to SSE stream {}: {}", stream_name, e);
                    }
                }

                // Wait before reconnecting
                sleep(Duration::from_secs(5)).await;
                info!("Reconnecting to SSE stream: {}", stream_name);
            }
        });

        Ok(())
    }

    /// Subscribe to RPC event streams
    pub async fn subscribe_to_rpc_events(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref rpc_client) = self.rpc_client {
            info!("Subscribing to RPC event streams...");

            // Subscribe to storage events
            let storage_filter = EventFilter {
                event_types: vec!["storage".to_string(), "zfs".to_string()],
                source_filter: Some("nestgate".to_string()),
                priority_filter: None,
                since: Some(SystemTime::now()),
            };

            let mut storage_stream = rpc_client.stream_storage_events(storage_filter).await?;
            let aggregator = self.event_aggregator.clone();

            tokio::spawn(async move {
                while let Some(event) = storage_stream.next().await {
                    let aggregated_event = AggregatedEvent {
                        id: event.id.clone(),
                        source_layer: CommunicationLayer::StreamingRPC,
                        event_type: event.event_type.clone(),
                        data: event.data.clone(),
                        received_at: SystemTime::now(),
                        original_timestamp: Some(event.timestamp),
                    };

                    aggregator.aggregate_event(aggregated_event).await;
                }
            });

            // Subscribe to system metrics
            let mut metrics_stream = rpc_client
                .stream_system_metrics(Duration::from_secs(1))
                .await?;
            let aggregator = self.event_aggregator.clone();

            tokio::spawn(async move {
                while let Some(metrics) = metrics_stream.next().await {
                    let aggregated_event = AggregatedEvent {
                        id: uuid::Uuid::new_v4().to_string(),
                        source_layer: CommunicationLayer::StreamingRPC,
                        event_type: "system_metrics".to_string(),
                        data: json!({
                            "timestamp": metrics.timestamp.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                            "cpu_usage": metrics.cpu_usage,
                            "memory_usage": metrics.memory_usage,
                            "disk_usage": metrics.disk_usage,
                            "network_io": metrics.network_io,
                            "zfs_metrics": metrics.zfs_metrics
                        }),
                        received_at: SystemTime::now(),
                        original_timestamp: Some(metrics.timestamp),
                    };

                    aggregator.aggregate_event(aggregated_event).await;
                }
            });
        }

        Ok(())
    }

    /// Execute storage operations via RPC
    pub async fn execute_storage_operation(
        &self,
        operation: StorageOperation,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref rpc_client) = self.rpc_client {
            let result = rpc_client.execute_storage_operation(operation).await?;
            Ok(serde_json::to_value(result)?)
        } else {
            Err("RPC client not connected".into())
        }
    }

    /// Execute ZFS operations via RPC
    pub async fn execute_zfs_operation(
        &self,
        operation: ZfsOperation,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref rpc_client) = self.rpc_client {
            let result = rpc_client.execute_zfs_operation(operation).await?;
            Ok(serde_json::to_value(result)?)
        } else {
            Err("RPC client not connected".into())
        }
    }

    /// Send WebSocket message
    pub async fn send_websocket_message(
        &mut self,
        message: Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref mut ws_tx) = self.websocket_tx {
            let message_text = serde_json::to_string(&message)?;
            ws_tx.send(Message::Text(message_text)).await?;
            Ok(())
        } else {
            Err("WebSocket not connected".into())
        }
    }

    /// Get aggregated event stream
    pub fn get_event_stream(&self) -> broadcast::Receiver<AggregatedEvent> {
        self.event_aggregator.subscribe()
    }

    /// Get event aggregator statistics
    pub async fn get_aggregator_stats(&self) -> AggregatorStats {
        self.event_aggregator.get_stats().await
    }

    /// Get communication statistics from the server
    pub async fn get_server_stats(
        &self,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/v1/communication/stats", self.base_url);
        let response = self.http_client.get(&url).send().await?;
        let stats = response.json::<Value>().await?;
        Ok(stats)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_env_filter("info").init();

    info!("🚀 Starting NestGate Streaming Client Demo");

    // Create and connect streaming client
    let mut client = NestGateStreamingClient::new("http://127.0.0.1:8080".to_string()).await?;

    // Give the server time to start (if running the demo)
    sleep(Duration::from_secs(3)).await;

    // Connect to all communication layers
    if let Err(e) = client.connect_all().await {
        warn!(
            "Some connections failed: {}. Continuing with available connections...",
            e
        );
    }

    // Subscribe to RPC events
    if let Err(e) = client.subscribe_to_rpc_events().await {
        warn!("RPC event subscription failed: {}. Continuing...", e);
    }

    // Start event monitoring
    let mut event_stream = client.get_event_stream();
    let client_stats = Arc::new(tokio::sync::RwLock::new(ClientStats::default()));

    // Event processing task
    let stats_clone = client_stats.clone();
    let event_processor = tokio::spawn(async move {
        while let Ok(event) = event_stream.recv().await {
            info!(
                "📨 Received aggregated event from {:?}: {}",
                event.source_layer, event.event_type
            );
            debug!(
                "Event data: {}",
                serde_json::to_string_pretty(&event.data).unwrap_or_default()
            );

            // Update client statistics
            {
                let mut stats = stats_clone.write().await;
                stats.total_events_received += 1;
                stats.last_event_time = Some(SystemTime::now());

                let layer_key = format!("{:?}", event.source_layer);
                *stats.events_by_layer.entry(layer_key).or_insert(0) += 1;
            }
        }
    });

    // Demo storage operations
    info!("🗄️  Demonstrating storage operations...");
    demo_storage_operations(&mut client).await?;

    // Demo ZFS operations
    info!("💾 Demonstrating ZFS operations...");
    demo_zfs_operations(&mut client).await?;

    // Demo WebSocket communication
    info!("🌐 Demonstrating WebSocket communication...");
    demo_websocket_communication(&mut client).await?;

    // Monitor events for a while
    info!("📊 Monitoring events for 10 seconds...");
    sleep(Duration::from_secs(10)).await;

    // Display final statistics
    info!("📈 Final Statistics:");
    let client_stats = client_stats.read().await;
    info!("Client Stats: {:?}", *client_stats);

    let aggregator_stats = client.get_aggregator_stats().await;
    info!("Aggregator Stats: {:?}", aggregator_stats);

    if let Ok(server_stats) = client.get_server_stats().await {
        info!(
            "Server Stats: {}",
            serde_json::to_string_pretty(&server_stats)?
        );
    }

    // Cleanup
    event_processor.abort();

    info!("✅ Streaming Client Demo Complete!");
    Ok(())
}

/// Client-side statistics
#[derive(Debug, Default, Clone)]
struct ClientStats {
    total_events_received: u64,
    events_by_layer: HashMap<String, u64>,
    last_event_time: Option<SystemTime>,
}

/// Demo storage operations
async fn demo_storage_operations(
    client: &mut NestGateStreamingClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create dataset
    let create_op = StorageOperation::CreateDataset {
        name: "demo/client-dataset".to_string(),
        properties: {
            let mut props = HashMap::new();
            props.insert("compression".to_string(), "lz4".to_string());
            props.insert("recordsize".to_string(), "128K".to_string());
            props
        },
    };

    if let Ok(result) = client.execute_storage_operation(create_op).await {
        info!(
            "Dataset creation result: {}",
            serde_json::to_string_pretty(&result)?
        );
    }

    // List datasets
    let list_op = StorageOperation::ListDatasets {
        pool: Some("demo".to_string()),
        recursive: true,
    };

    if let Ok(result) = client.execute_storage_operation(list_op).await {
        info!(
            "Dataset list result: {}",
            serde_json::to_string_pretty(&result)?
        );
    }

    // Create snapshot
    let snapshot_op = StorageOperation::CreateSnapshot {
        dataset: "demo/client-dataset".to_string(),
        snapshot_name: "client-snapshot-1".to_string(),
        recursive: false,
    };

    if let Ok(result) = client.execute_storage_operation(snapshot_op).await {
        info!(
            "Snapshot creation result: {}",
            serde_json::to_string_pretty(&result)?
        );
    }

    Ok(())
}

/// Demo ZFS operations
async fn demo_zfs_operations(
    client: &mut NestGateStreamingClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // List pools
    let list_pools_op = ZfsOperation::ListPools {
        include_status: true,
    };

    if let Ok(result) = client.execute_zfs_operation(list_pools_op).await {
        info!(
            "Pool list result: {}",
            serde_json::to_string_pretty(&result)?
        );
    }

    // Get pool status
    let pool_status_op = ZfsOperation::GetPoolStatus {
        name: "rpool".to_string(),
    };

    if let Ok(result) = client.execute_zfs_operation(pool_status_op).await {
        info!(
            "Pool status result: {}",
            serde_json::to_string_pretty(&result)?
        );
    }

    Ok(())
}

/// Demo WebSocket communication
async fn demo_websocket_communication(
    client: &mut NestGateStreamingClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Send various WebSocket messages
    let messages = vec![
        json!({
            "type": "client_hello",
            "message": "Hello from streaming client!",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }),
        json!({
            "type": "status_request",
            "request_id": "req-001",
            "requested_data": ["system_health", "storage_status"]
        }),
        json!({
            "type": "subscription_request",
            "subscription": {
                "events": ["storage_operations", "performance_metrics"],
                "priority": "high"
            }
        }),
    ];

    for message in messages {
        if let Err(e) = client.send_websocket_message(message.clone()).await {
            warn!("Failed to send WebSocket message: {}", e);
        } else {
            info!(
                "Sent WebSocket message: {}",
                message.get("type").unwrap_or(&json!("unknown"))
            );
        }

        sleep(Duration::from_millis(500)).await;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_aggregator() {
        let aggregator = EventAggregator::new();

        let test_event = AggregatedEvent {
            id: "test-1".to_string(),
            source_layer: CommunicationLayer::SSE,
            event_type: "test_event".to_string(),
            data: json!({"test": true}),
            received_at: SystemTime::now(),
            original_timestamp: None,
        };

        aggregator.aggregate_event(test_event).await;

        let stats = aggregator.get_stats().await;
        assert_eq!(stats.total_events, 1);
        assert!(stats.events_by_layer.contains_key("SSE"));
    }

    #[tokio::test]
    async fn test_client_creation() {
        let client = NestGateStreamingClient::new("http://localhost:8080".to_string()).await;
        assert!(client.is_ok());
    }
}
