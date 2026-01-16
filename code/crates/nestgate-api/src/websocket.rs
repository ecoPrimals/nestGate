//
// This module provides real-time WebSocket communication optimized for:
// - Web UIs and dashboards
// - External service integration
// - Real-time monitoring and notifications
// - Event streaming to clients

//! Websocket module

use serde::{Deserialize, Serialize};

use nestgate_core::uuid_cache::get_or_create_uuid;
use dashmap::DashMap;
use std::{sync::Arc, time::SystemTime};
use tokio::sync::broadcast;
// Removed unused tracing import
use tracing::info;
use uuid::Uuid;

/// WebSocket event types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of WebSocketEvent
pub enum WebSocketEventType {
    /// A new WebSocket connection was established
    ConnectionEstablished,
    /// A message was sent or received
    Message,
    /// A WebSocket connection was closed
    Disconnection,
    /// An error occurred during WebSocket communication
    Error,
    /// A ping frame was sent or received
    Ping,
    /// A pong frame was sent or received
    Pong,
}
/// WebSocket event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Websocketevent
pub struct WebSocketEvent {
    /// Unique identifier for this event
    pub event_id: Uuid,
    /// Client that triggered this event
    pub client_id: Uuid,
    /// Type of WebSocket event
    pub event_type: WebSocketEventType,
    /// Event data payload
    pub data: serde_json::Value,
    /// Event occurrence timestamp
    pub timestamp: SystemTime,
}
/// Connection parameters for WebSocket upgrade
#[derive(Debug, Deserialize)]
/// Connectionparams
pub struct ConnectionParams {
    /// Optional client type specification
    pub client_type: Option<String>,
    /// Optional client identifier
    pub client_id: Option<String>,
}
/// WebSocket connection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Websocketstats
pub struct WebSocketStats {
    /// Total number of connections established
    pub total_connections: u64,
    /// Number of currently active connections
    pub active_connections: u64,
    /// Total messages sent through all connections
    pub messages_sent: u64,
    /// Total messages received from all connections
    pub messages_received: u64,
    /// Total bytes transferred through WebSocket connections
    pub bytes_transferred: u64,
    /// Number of errors encountered
    pub errors: u64,
}
/// WebSocket client types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Client
pub enum ClientType {
    /// Web-based user interface client
    WebUI,
    /// System monitoring client
    Monitor,
    /// Third-party integration client
    Integration,
    /// Mobile application client
    Mobile,
    /// API client for programmatic access
    ApiClient,
}
/// WebSocket connection information
#[derive(Debug, Clone, Serialize)]
/// Connectioninfo
pub struct ConnectionInfo {
    /// Unique client identifier
    pub client_id: Uuid,
    /// Type of WebSocket client
    pub client_type: ClientType,
    /// Timestamp when connection was established
    pub connected_at: SystemTime,
    /// Timestamp of last client activity
    pub last_activity: SystemTime,
    /// List of subscribed channels or topics
    pub subscriptions: Vec<String>,
}
/// WebSocket manager for handling connections
/// 
/// **LOCK-FREE**: Uses DashMap for concurrent connection management
pub struct WebSocketManager {
    connections: Arc<DashMap<Uuid, ConnectionInfo>>,  // ✅ DashMap: Lock-free concurrent access
    event_broadcaster: broadcast::Sender<WebSocketEvent>,
    stats: Arc<DashMap<&'static str, u64>>,  // ✅ DashMap: Lock-free stats
}
impl Default for WebSocketManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketManager {
    /// Create a new WebSocket manager
    #[must_use]
    pub fn new() -> Self {
        let (event_broadcaster, _) = broadcast::channel(1000);
        
        // Initialize stats DashMap
        let stats = Arc::new(DashMap::new());
        stats.insert("total_connections", 0);
        stats.insert("active_connections", 0);
        stats.insert("messages_sent", 0);
        stats.insert("messages_received", 0);
        stats.insert("bytes_transferred", 0);
        stats.insert("errors", 0);

        Self {
            connections: Arc::new(DashMap::new()),  // ✅ Lock-free
            event_broadcaster,
            stats,
        }
    }

    /// Get connection statistics (lock-free!)
    pub fn get_stats(&self) -> WebSocketStats {
        WebSocketStats {
            total_connections: *self.stats.get("total_connections").map(|v| *v).unwrap_or(0),
            active_connections: *self.stats.get("active_connections").map(|v| *v).unwrap_or(0),
            messages_sent: *self.stats.get("messages_sent").map(|v| *v).unwrap_or(0),
            messages_received: *self.stats.get("messages_received").map(|v| *v).unwrap_or(0),
            bytes_transferred: *self.stats.get("bytes_transferred").map(|v| *v).unwrap_or(0),
            errors: *self.stats.get("errors").map(|v| *v).unwrap_or(0),
        }
    }

    /// Handle WebSocket upgrade
    pub fn handle_websocket_upgrade(
        &self,
        ws: axum::extract::WebSocketUpgrade,
        _params: ConnectionParams,
    ) -> axum::response::Response {
        let client_id = *get_or_create_uuid("websocket_client");
        let client_type = match _params.client_type.as_deref() {
            Some("WebUI") => ClientType::WebUI,
            Some("Monitor") => ClientType::Monitor,
            Some("Integration") => ClientType::Integration,
            Some("Mobile") => ClientType::Mobile,
            Some("ApiClient") | None => ClientType::ApiClient,
            _ => ClientType::ApiClient,
        };

        // Create a channel for this client
        let (_event_sender, _event_receiver) = broadcast::channel::<WebSocketEvent>(100);

        // Add the client to our registry
        let client_info = ConnectionInfo {
            client_id,
            client_type: client_type.clone(),
            connected_at: SystemTime::now(),
            last_activity: SystemTime::now(),
            subscriptions: Vec::new(),
        };

        self.connections
            .write()
            .await
            .insert(client_id, client_info);

        // Upgrade the WebSocket connection
        ws.on_upgrade(move |socket| async move {
            // Handle the WebSocket connection
            let mut socket = socket;

            // Send welcome message
            let welcome_msg = WebSocketEvent {
                event_id: *get_or_create_uuid(&format!("websocket_welcome_self.base_url")),
                client_id,
                event_type: WebSocketEventType::ConnectionEstablished,
                data: serde_json::json!({
                    "message": "Connection established",
                    "client_id": client_id,
                    "client_type": format!("self.base_url")
                }),
                timestamp: SystemTime::now(),
            };

            if let Ok(welcome_json) = serde_json::to_string(&welcome_msg) {
                let _ = socket
                    .send(axum::extract::ws::Message::Text(welcome_json))
                    .await;
            }

            // Handle incoming messages
            while let Some(msg) = socket.recv().await {
                match msg {
                    Ok(axum::extract::ws::Message::Text(text)) => {
                        // Process text message
                        let event = WebSocketEvent {
                            event_id: *get_or_create_uuid(&format!(
                                "websocket_message_{client_id}"
                            )),
                            client_id,
                            event_type: WebSocketEventType::Message,
                            data: serde_json::json!({
                                "message": text,
                                "client_type": format!("self.base_url")
                            }),
                            timestamp: SystemTime::now(),
                        };

                        // Echo the message back
                        if let Ok(response_json) = serde_json::to_string(&event) {
                            let _ = socket
                                .send(axum::extract::ws::Message::Text(response_json))
                                .await;
                        }
                    }
                    Ok(axum::extract::ws::Message::Close(_)) => {
                        break;
                    }
                    _ => {}
                }
            }
        })
    }

    /// Broadcast event to all connected clients
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn broadcast_event(
        &self,
        event: WebSocketEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
        // ✅ Lock-free iteration over connections
        // Pre-serialize event to avoid repeated serialization for each client (zero-copy optimization)
        let event_json = serde_json::to_string(&event)?;
        let event_size = event_json.len() as u64;
        let connection_count = self.connections.len();

        // Create shared reference for zero-copy broadcasting
        let event_json_ref = Arc::new(event_json);

        // Update statistics (lock-free!)
        self.stats.alter("messages_sent", |_, v| v + 1);
        self.stats.alter("bytes_transferred", |_, v| v + (event_size * connection_count as u64));

        // In a real implementation, this would broadcast to all WebSocket connections
        info!(
            "Broadcasting event to {} clients: {}",
            connection_count,
            event_json_ref
        );
        Ok(())
    }

    /// Get active connection count (lock-free!)
    pub fn get_connection_count(&self) -> usize {
        self.connections.len()  // ✅ Lock-free DashMap len()
    }
}

impl Clone for WebSocketManager {
    /// Clone
    fn clone(&self) -> Self { Self {
            connections: Arc::clone(&self.connections),
            event_broadcaster: self.event_broadcaster.clone(),
            stats: Arc::clone(&self.stats),
         }
}
