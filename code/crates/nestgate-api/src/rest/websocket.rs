//
// Real-time WebSocket communication for streaming updates and bidirectional
// communication with connected clients.

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{error, info, warn};

/// WebSocket event types for real-time communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketEvent {
    /// Storage operation updates
    StorageUpdate {
        /// Type of storage operation (create, delete, modify, etc.)
        b_operation: String,
        /// Current status of the operation (pending, running, completed, failed)
        status: String,
        /// Optional progress percentage (0-100)
        progress: Option<u8>,
    },
    /// System health status updates
    HealthUpdate {
        /// Name of the service reporting health status
        service: String,
        /// Current health status (healthy, degraded, unhealthy)
        status: String,
        /// ISO 8601 timestamp when the status was reported
        timestamp: String,
    },
    /// Performance metrics updates
    MetricsUpdate {
        /// Map of metric names to their current values
        metrics: HashMap<String, f64>,
        /// ISO 8601 timestamp when metrics were collected
        timestamp: String,
    },
    /// Generic message
    Message {
        /// Message content
        content: String,
        /// ISO 8601 timestamp when message was created
        timestamp: String,
    },
}
/// WebSocket connection manager
#[derive(Debug)]
pub struct WebSocketManager {
    /// Broadcast channel for sending events to all connected clients
    event_sender: broadcast::Sender<WebSocketEvent>,
    /// Connected clients counter
    connected_clients: Arc<RwLock<usize>>,
}
impl WebSocketManager {
    /// Create a new WebSocket manager
    pub const fn new() -> Self {
        let (event_sender, _) = broadcast::channel(1000);

        Self {
            event_sender,
            connected_clients: Arc::new(RwLock::new(0)),
        }
    }

    /// Broadcast an event to all connected clients
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn broadcast_event(
        &self,
        event: WebSocketEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
        match self.event_sender.send(event) {
            Ok(_) => Ok(()),
            Err(e) => {
                warn!("Failed to broadcast WebSocket event: {}", e);
                Err(Box::new(e))
            }
        }
    }

    /// Get the number of connected clients
    pub async fn connected_clients(&self) -> usize {
        *self.connected_clients.read().await
    }

    /// Handle WebSocket upgrade request
    pub const fn handle_upgrade(&self, ws: WebSocketUpgrade) -> impl IntoResponse {
        let event_receiver = self.event_sender.subscribe();
        let client_counter = Arc::clone(&self.connected_clients);

        ws.on_upgrade(move |socket| {
            handle_websocket_connection(socket, event_receiver, client_counter)
        })
    }
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle individual WebSocket connection
async fn handle_websocket_connection(
    mut socket: WebSocket,
    mut event_receiver: broadcast::Receiver<WebSocketEvent>,
    client_counter: Arc<RwLock<usize>>,
) {
    // Increment connected clients count
    {
        let mut count = client_counter.write().await;
        *count += 1;
        info!("WebSocket client connected. Total clients: {}", *count);
    }
    // Send welcome message
    let welcome_event = WebSocketEvent::Message {
        content: "Connected to NestGate WebSocket".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    if let Ok(welcome_json) = serde_json::to_string(&welcome_event) {
        if let Err(e) = socket.send(Message::Text(welcome_json)).await {
            error!("Failed to send welcome message: {}", e);
            return;
        }
    }

    // Handle incoming messages and broadcast events
    loop {
        tokio::select! {
            // Handle incoming WebSocket messages from client
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        info!("Received WebSocket message: {}", text);
                        // Echo back for now - in production this would handle commands
                        let response = WebSocketEvent::Message {
                            content: format!("Echo: {"actual_error_details"}"),
                            timestamp: chrono::Utc::now().to_rfc3339(),
                        };

                        if let Ok(response_json) = serde_json::to_string(&response) {
                            if let Err(e) = socket.send(Message::Text(response_json)).await {
                                error!("Failed to send response: {}", e);
                                break;
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) => {
                        info!("WebSocket client disconnected");
                        break;
                    }
                    Some(Err(e)) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                    None => {
                        info!("WebSocket connection closed");
                        break;
                    }
                    _ => {
                        // Handle other message types (binary, ping, pong) as needed
                    }
                }
            }
            // Handle broadcast events from the system
            event = event_receiver.recv() => {
                match event {
                    Ok(event) => {
                        if let Ok(event_json) = serde_json::to_string(&event) {
                            if let Err(e) = socket.send(Message::Text(event_json)).await {
                                error!("Failed to send broadcast event: {}", e);
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        info!("Broadcast channel closed");
                        break;
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        warn!("WebSocket client lagging behind broadcast events");
                        // Continue processing - client will catch up
                    }
                }
            }
        }
    }

    // Decrement connected clients count
    {
        let mut count = client_counter.write().await;
        *count = count.saturating_sub(1);
        info!("WebSocket client disconnected. Total clients: {}", *count);
    }
}

/// Helper function to create common WebSocket events
impl WebSocketEvent {
    /// Create a storage update event
    pub const fn storage_update(operation: &str, status: &str, progress: Option<u8>) -> Self {
        Self::StorageUpdate {
            b_operation: operation.to_string(),
            status: status.to_string(),
            progress,
        }
    }
    /// Create a health update event
    pub const fn health_update(service: &str, status: &str) -> Self {
        Self::HealthUpdate {
            service: service.to_string(),
            status: status.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a metrics update event
    pub const fn metrics_update(metrics: HashMap<String, f64>) -> Self {
        Self::MetricsUpdate {
            metrics,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a simple message event
    pub const fn message(content: &str) -> Self {
        Self::Message {
            content: content.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_manager_creation() {
        let manager = WebSocketManager::new();
        assert_eq!(manager.connected_clients().await, 0);
    }
    #[tokio::test]
    async fn test_websocket_event_creation() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let event = WebSocketEvent::storage_update("create_pool", "in_progress", Some(50));

        match event {
            WebSocketEvent::StorageUpdate {
                b_operation,
                status,
                progress,
            } => {
                assert_eq!(b_operation, "create_pool");
                assert_eq!(status, "in_progress");
                assert_eq!(progress, Some(50));
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Expected StorageUpdate event".to_string(),
                )
                .into())
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_event_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let event = WebSocketEvent::message("test message");
        let serialized = serde_json::to_string(&event).map_err(|e| {
            tracing::error!("JSON serialization failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("JSON serialization failed: {"actual_error_details"}"),
            )
        })?;
        let deserialized: WebSocketEvent = serde_json::from_str(&serialized).map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {"actual_error_details"}"),
            )
        })?;

        match deserialized {
            WebSocketEvent::Message { content, .. } => {
                assert_eq!(content, "test message");
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Expected Message event".to_string(),
                )
                .into())
            }
        }
        Ok(())
    }
}
