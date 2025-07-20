//! Enhanced WebSocket + JSON Communication for External Clients
//!
//! This module provides real-time WebSocket communication optimized for:
//! - Web UIs and dashboards
//! - External service integration
//! - Real-time monitoring and notifications
//! - Event streaming to clients

use serde::{Deserialize, Serialize};

use nestgate_core::get_or_create_uuid;
use std::{collections::HashMap, sync::Arc, time::SystemTime};
use tokio::sync::{broadcast, RwLock};
use tracing::info;
use uuid::Uuid;

/// WebSocket event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketEventType {
    ConnectionEstablished,
    Message,
    Disconnection,
    Error,
    Ping,
    Pong,
}

/// WebSocket event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketEvent {
    pub event_id: Uuid,
    pub client_id: Uuid,
    pub event_type: WebSocketEventType,
    pub data: serde_json::Value,
    pub timestamp: SystemTime,
}

/// Connection parameters for WebSocket upgrade
#[derive(Debug, Deserialize)]
pub struct ConnectionParams {
    pub client_type: Option<String>,
    pub client_id: Option<String>,
}

/// WebSocket connection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketStats {
    pub total_connections: u64,
    pub active_connections: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_transferred: u64,
    pub errors: u64,
}

/// WebSocket client types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientType {
    WebUI,
    Monitor,
    Integration,
    Mobile,
    ApiClient,
}

/// WebSocket connection information
#[derive(Debug, Clone, Serialize)]
pub struct ConnectionInfo {
    pub client_id: Uuid,
    pub client_type: ClientType,
    pub connected_at: SystemTime,
    pub last_activity: SystemTime,
    pub subscriptions: Vec<String>,
}

/// WebSocket manager for handling connections
pub struct WebSocketManager {
    connections: Arc<RwLock<HashMap<Uuid, ConnectionInfo>>>,
    event_broadcaster: broadcast::Sender<WebSocketEvent>,
    stats: Arc<RwLock<WebSocketStats>>,
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketManager {
    pub fn new() -> Self {
        let (event_broadcaster, _) = broadcast::channel(1000);

        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            event_broadcaster,
            stats: Arc::new(RwLock::new(WebSocketStats {
                total_connections: 0,
                active_connections: 0,
                messages_sent: 0,
                messages_received: 0,
                bytes_transferred: 0,
                errors: 0,
            })),
        }
    }

    /// Get connection statistics
    pub async fn get_stats(&self) -> WebSocketStats {
        self.stats.read().await.clone()
    }

    /// Handle WebSocket upgrade
    pub async fn handle_websocket_upgrade(
        &self,
        ws: axum::extract::WebSocketUpgrade,
        params: ConnectionParams,
    ) -> axum::response::Response {
        let client_id = Uuid::new_v4();
        let client_type = match params.client_type.as_deref() {
            Some("WebUI") => ClientType::WebUI,
            Some("Monitor") => ClientType::Monitor,
            Some("Integration") => ClientType::Integration,
            Some("Mobile") => ClientType::Mobile,
            Some("ApiClient") | None => ClientType::ApiClient,
            _ => ClientType::ApiClient,
        };

        // Create a channel for this client
        let (_event_sender, _event_receiver) =
            tokio::sync::broadcast::channel::<WebSocketEvent>(100);

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
                event_id: *get_or_create_uuid(&format!("websocket_welcome_{client_id}")),
                client_id,
                event_type: WebSocketEventType::ConnectionEstablished,
                data: serde_json::json!({
                    "message": "Connection established",
                    "client_id": client_id,
                    "client_type": format!("{:?}", client_type)
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
                                "client_type": format!("{:?}", client_type)
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
    pub async fn broadcast_event(
        &self,
        event: WebSocketEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let connections = self.connections.read().await;

        // Pre-serialize event to avoid repeated serialization for each client (zero-copy optimization)
        let event_json = serde_json::to_string(&event)?;
        let event_size = event_json.len() as u64;

        // Create shared reference for zero-copy broadcasting
        let event_json_ref = std::sync::Arc::new(event_json);

        // Update statistics
        if let Ok(mut stats) = self.stats.try_write() {
            stats.messages_sent += 1; // Changed from events_broadcast to messages_sent
            stats.bytes_transferred += event_size * connections.len() as u64;
        }

        // In a real implementation, this would broadcast to all WebSocket connections
        info!(
            "Broadcasting event to {} clients: {}",
            connections.len(),
            event_json_ref
        );

        Ok(())
    }

    /// Get active connection count
    pub async fn get_connection_count(&self) -> usize {
        self.connections.read().await.len()
    }
}

impl Clone for WebSocketManager {
    fn clone(&self) -> Self {
        Self {
            connections: Arc::clone(&self.connections),
            event_broadcaster: self.event_broadcaster.clone(),
            stats: Arc::clone(&self.stats),
        }
    }
}
