//! Server-Sent Events (SSE) Streaming for NestGate
//!
//! This module provides real-time Server-Sent Events streaming capabilities
//! for clients that prefer SSE over WebSocket connections. It integrates
//! with the existing event coordination system and provides:
//!
//! - Real-time storage operation streaming
//! - System health monitoring streams
//! - Performance metrics streaming
//! - Event-driven notifications
//! - Backpressure handling

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use axum::{
    extract::{Query, State},
    http::HeaderMap,
    response::sse::{Event, Sse},
};

// Stream processing
use futures::{stream, Stream, StreamExt};
use tokio::sync::{broadcast, RwLock};
use tokio_stream::wrappers::BroadcastStream;

use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::event_coordination::EventCoordinator;

/// SSE connection parameters
#[derive(Debug, Deserialize)]
pub struct SseParams {
    /// Stream type to subscribe to
    pub stream: Option<String>,
    /// Client identifier
    pub client_id: Option<String>,
    /// Authentication token
    pub token: Option<String>,
    /// Compression preference
    pub compress: Option<bool>,
    /// Buffer size preference
    pub buffer_size: Option<usize>,
}

/// SSE event types that can be streamed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SseEventType {
    /// Storage operation events
    StorageOperation,
    /// System health updates
    SystemHealth,
    /// Performance metrics
    PerformanceMetrics,
    /// Hardware tuning events
    HardwareTuning,
    /// ZFS events
    ZfsEvent,
    /// Authentication events
    AuthEvent,
    /// General system events
    SystemEvent,
}

/// SSE event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseEvent {
    /// Event identifier
    pub id: Uuid,
    /// Event type
    pub event_type: SseEventType,
    /// Event data
    pub data: serde_json::Value,
    /// Event timestamp
    pub timestamp: SystemTime,
    /// Event source
    pub source: String,
    /// Event priority
    pub priority: EventPriority,
}

/// Event priority for SSE streaming
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EventPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// SSE stream manager
pub struct SseManager {
    /// Active SSE connections
    connections: Arc<RwLock<HashMap<Uuid, SseConnection>>>,
    /// Event broadcaster
    event_broadcaster: broadcast::Sender<SseEvent>,
    /// Stream statistics
    stats: Arc<RwLock<SseStats>>,
    /// Event coordinator integration
    event_coordinator: Option<Arc<EventCoordinator>>,
}

/// SSE connection information
#[derive(Debug, Clone)]
pub struct SseConnection {
    /// Connection identifier
    pub id: Uuid,
    /// Client identifier
    pub client_id: Option<String>,
    /// Subscribed streams
    pub subscriptions: Vec<String>,
    /// Connection start time
    pub connected_at: SystemTime,
    /// Last activity time
    pub last_activity: SystemTime,
    /// Connection configuration
    pub config: SseConnectionConfig,
}

/// SSE connection configuration
#[derive(Debug, Clone)]
pub struct SseConnectionConfig {
    /// Buffer size for events
    pub buffer_size: usize,
    /// Enable compression
    pub compression: bool,
    /// Keep-alive interval
    pub keep_alive_interval: Duration,
    /// Maximum event size
    pub max_event_size: usize,
}

/// SSE streaming statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseStats {
    /// Total connections ever created
    pub total_connections: u64,
    /// Currently active connections
    pub active_connections: u64,
    /// Total events sent
    pub events_sent: u64,
    /// Total bytes transferred
    pub bytes_transferred: u64,
    /// Connection errors
    pub errors: u64,
    /// Last reset time
    pub last_reset: SystemTime,
}

impl SseManager {
    /// Create a new SSE manager
    pub fn new() -> Self {
        let (event_broadcaster, _) = broadcast::channel(10000); // Large buffer for SSE

        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            event_broadcaster,
            stats: Arc::new(RwLock::new(SseStats {
                total_connections: 0,
                active_connections: 0,
                events_sent: 0,
                bytes_transferred: 0,
                errors: 0,
                last_reset: SystemTime::now(),
            })),
            event_coordinator: None,
        }
    }

    /// Set event coordinator for integration
    pub fn with_event_coordinator(mut self, coordinator: Arc<EventCoordinator>) -> Self {
        self.event_coordinator = Some(coordinator);
        self
    }

    /// Create a new SSE stream for a client
    pub async fn create_stream(
        &self,
        params: SseParams,
        _headers: HeaderMap,
    ) -> impl Stream<Item = Result<Event, Infallible>> {
        let connection_id = Uuid::new_v4();
        let client_id = params.client_id.clone();

        // Create connection configuration
        let config = SseConnectionConfig {
            buffer_size: params.buffer_size.unwrap_or(1000),
            compression: params.compress.unwrap_or(false),
            keep_alive_interval: Duration::from_secs(30),
            max_event_size: 64 * 1024, // 64KB max event size
        };

        // Register connection
        let connection = SseConnection {
            id: connection_id,
            client_id: client_id.clone(),
            subscriptions: vec![params.stream.unwrap_or_else(|| "all".to_string())],
            connected_at: SystemTime::now(),
            last_activity: SystemTime::now(),
            config: config.clone(),
        };

        {
            let mut connections = self.connections.write().await;
            connections.insert(connection_id, connection);
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_connections += 1;
            stats.active_connections += 1;
        }

        info!("Created SSE stream for client: {:?}", client_id);

        // Create event stream
        let event_receiver = self.event_broadcaster.subscribe();
        let connections_clone = self.connections.clone();
        let stats_clone = self.stats.clone();

        // Convert broadcast receiver to stream
        let event_stream = BroadcastStream::new(event_receiver).filter_map(move |result| {
            let connections = connections_clone.clone();
            let stats = stats_clone.clone();
            async move {
                match result {
                    Ok(sse_event) => {
                        // Update connection activity
                        if let Ok(mut connections) = connections.try_write() {
                            if let Some(conn) = connections.get_mut(&connection_id) {
                                conn.last_activity = SystemTime::now();
                            }
                        }

                        // Update statistics
                        if let Ok(mut stats) = stats.try_write() {
                            stats.events_sent += 1;
                            stats.bytes_transferred += serde_json::to_string(&sse_event)
                                .map(|s| s.len() as u64)
                                .unwrap_or(0);
                        }

                        // Convert to SSE event
                        let event_data = serde_json::to_string(&sse_event).ok()?;
                        let event = Event::default()
                            .id(sse_event.id.to_string())
                            .event(format!("{:?}", sse_event.event_type))
                            .data(event_data);

                        Some(Ok(event))
                    }
                    Err(_) => {
                        // Broadcast error (receiver lagged)
                        warn!("SSE client {} lagged behind", connection_id);
                        None
                    }
                }
            }
        });

        // Add keep-alive events
        let keep_alive_stream = stream::unfold((), move |_| async move {
            tokio::time::sleep(config.keep_alive_interval).await;
            let keep_alive_event = Event::default().event("keep-alive").data("ping");
            Some((Ok(keep_alive_event), ()))
        });

        // Merge event stream with keep-alive stream
        futures::stream::select(event_stream, keep_alive_stream)
    }

    /// Broadcast an event to all SSE clients
    pub async fn broadcast_event(
        &self,
        event: SseEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("Broadcasting SSE event: {:?}", event.event_type);

        match self.event_broadcaster.send(event) {
            Ok(_) => Ok(()),
            Err(e) => {
                warn!("Failed to broadcast SSE event: {}", e);
                Err(e.into())
            }
        }
    }

    /// Get SSE statistics
    pub async fn get_stats(&self) -> SseStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Clean up inactive connections
    pub async fn cleanup_connections(&self) {
        let mut connections = self.connections.write().await;
        let now = SystemTime::now();
        let timeout = Duration::from_secs(300); // 5 minutes timeout

        let mut to_remove = Vec::new();
        for (id, connection) in connections.iter() {
            if let Ok(elapsed) = now.duration_since(connection.last_activity) {
                if elapsed > timeout {
                    to_remove.push(*id);
                }
            }
        }

        for id in to_remove {
            connections.remove(&id);

            // Update statistics
            if let Ok(mut stats) = self.stats.try_write() {
                stats.active_connections = stats.active_connections.saturating_sub(1);
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
                manager.cleanup_connections().await;
            }
        })
    }

    /// Create storage operations stream
    pub async fn create_storage_stream(&self) -> impl Stream<Item = Result<Event, Infallible>> {
        let params = SseParams {
            stream: Some("storage".to_string()),
            client_id: Some("storage-client".to_string()),
            token: None,
            compress: Some(false),
            buffer_size: Some(500),
        };

        self.create_stream(params, HeaderMap::new()).await
    }

    /// Create system health stream
    pub async fn create_health_stream(&self) -> impl Stream<Item = Result<Event, Infallible>> {
        let params = SseParams {
            stream: Some("health".to_string()),
            client_id: Some("health-client".to_string()),
            token: None,
            compress: Some(false),
            buffer_size: Some(100),
        };

        self.create_stream(params, HeaderMap::new()).await
    }

    /// Create performance metrics stream
    pub async fn create_metrics_stream(&self) -> impl Stream<Item = Result<Event, Infallible>> {
        let params = SseParams {
            stream: Some("metrics".to_string()),
            client_id: Some("metrics-client".to_string()),
            token: None,
            compress: Some(false),
            buffer_size: Some(1000),
        };

        self.create_stream(params, HeaderMap::new()).await
    }
}

impl Clone for SseManager {
    fn clone(&self) -> Self {
        Self {
            connections: self.connections.clone(),
            event_broadcaster: self.event_broadcaster.clone(),
            stats: self.stats.clone(),
            event_coordinator: self.event_coordinator.clone(),
        }
    }
}

/// SSE endpoint handlers

/// Generic SSE endpoint for all event types
pub async fn sse_events(
    Query(params): Query<SseParams>,
    State(app_state): State<crate::routes::AppState>,
    _headers: HeaderMap,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = app_state
        .sse_manager
        .create_stream(params, HeaderMap::new())
        .await;

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive"),
    )
}

/// Storage operations SSE endpoint
pub async fn sse_storage(
    State(app_state): State<crate::routes::AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = app_state.sse_manager.create_storage_stream().await;

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("ping"),
    )
}

/// System health SSE endpoint
pub async fn sse_health(
    State(app_state): State<crate::routes::AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = app_state.sse_manager.create_health_stream().await;

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("ping"),
    )
}

/// Performance metrics SSE endpoint
pub async fn sse_metrics(
    State(app_state): State<crate::routes::AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = app_state.sse_manager.create_metrics_stream().await;

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("ping"),
    )
}

/// Utility functions for creating SSE events

/// Create a storage operation SSE event
pub fn create_storage_event(operation: &str, path: &str, data: serde_json::Value) -> SseEvent {
    SseEvent {
        id: Uuid::new_v4(),
        event_type: SseEventType::StorageOperation,
        data: serde_json::json!({
            "operation": operation,
            "path": path,
            "data": data,
        }),
        timestamp: SystemTime::now(),
        source: "nestgate-storage".to_string(),
        priority: EventPriority::Normal,
    }
}

/// Create a system health SSE event
pub fn create_health_event(component: &str, status: &str, metrics: serde_json::Value) -> SseEvent {
    SseEvent {
        id: Uuid::new_v4(),
        event_type: SseEventType::SystemHealth,
        data: serde_json::json!({
            "component": component,
            "status": status,
            "metrics": metrics,
        }),
        timestamp: SystemTime::now(),
        source: "nestgate-health".to_string(),
        priority: if status == "error" {
            EventPriority::Critical
        } else {
            EventPriority::Normal
        },
    }
}

/// Create a performance metrics SSE event
pub fn create_metrics_event(metrics: HashMap<String, f64>) -> SseEvent {
    SseEvent {
        id: Uuid::new_v4(),
        event_type: SseEventType::PerformanceMetrics,
        data: serde_json::json!(metrics),
        timestamp: SystemTime::now(),
        source: "nestgate-metrics".to_string(),
        priority: EventPriority::Normal,
    }
}
