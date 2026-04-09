// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Real-time WebSocket communication for streaming updates and bidirectional
// communication with connected clients.

//! Websocket module

use axum::{
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use tracing::{error, info, warn};

/// WebSocket event types for real-time communication
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Websocketevent
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
/// Manager for WebSocket operations
pub struct WebSocketManager {
    /// Broadcast channel for sending events to all connected clients
    event_sender: broadcast::Sender<WebSocketEvent>,
    /// Connected clients counter
    connected_clients: Arc<RwLock<usize>>,
}
impl WebSocketManager {
    /// Create a new WebSocket manager
    #[must_use]
    pub fn new() -> Self {
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
    pub fn broadcast_event(
        &self,
        event: WebSocketEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    pub fn handle_upgrade(&self, ws: WebSocketUpgrade) -> impl IntoResponse {
        let event_receiver = self.event_sender.subscribe();
        let client_counter = Arc::clone(&self.connected_clients);

        ws.on_upgrade(move |socket| {
            handle_websocket_connection(socket, event_receiver, client_counter)
        })
    }
}

impl Default for WebSocketManager {
    /// Returns the default instance
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

    if let Ok(welcome_json) = serde_json::to_string(&welcome_event)
        && let Err(e) = socket.send(Message::Text(welcome_json)).await
    {
        error!("Failed to send welcome message: {}", e);
        return;
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
                            content: format!("Echo: {text}"),
                            timestamp: chrono::Utc::now().to_rfc3339(),
                        };

                        if let Ok(response_json) = serde_json::to_string(&response)
                            && let Err(e) = socket.send(Message::Text(response_json)).await {
                                error!("Failed to send response: {}", e);
                                break;
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
                        if let Ok(event_json) = serde_json::to_string(&event)
                            && let Err(e) = socket.send(Message::Text(event_json)).await {
                                error!("Failed to send broadcast event: {}", e);
                                break;
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
    #[must_use]
    pub fn storage_update(operation: &str, status: &str, progress: Option<u8>) -> Self {
        Self::StorageUpdate {
            b_operation: operation.to_string(),
            status: status.to_string(),
            progress,
        }
    }
    /// Create a health update event
    #[must_use]
    pub fn health_update(service: &str, status: &str) -> Self {
        Self::HealthUpdate {
            service: service.to_string(),
            status: status.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a metrics update event
    #[must_use]
    pub fn metrics_update(metrics: HashMap<String, f64>) -> Self {
        Self::MetricsUpdate {
            metrics,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Create a simple message event
    #[must_use]
    pub fn message(content: &str) -> Self {
        Self::Message {
            content: content.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== WEBSOCKET MANAGER TESTS ====================

    #[tokio::test]
    async fn test_websocket_manager_creation() {
        let manager = WebSocketManager::new();
        assert_eq!(manager.connected_clients().await, 0);
    }

    #[tokio::test]
    async fn test_websocket_manager_default() {
        let manager = WebSocketManager::default();
        assert_eq!(manager.connected_clients().await, 0);
    }

    #[tokio::test]
    async fn test_websocket_manager_broadcast() {
        let manager = WebSocketManager::new();
        // Subscribe to receive events (prevents RecvError)
        let _receiver = manager.event_sender.subscribe();

        let event = WebSocketEvent::message("test broadcast");
        let result = manager.broadcast_event(event);
        // Broadcast succeeds with at least one receiver
        assert!(result.is_ok());
    }

    // ==================== WEBSOCKET EVENT TESTS ====================

    #[tokio::test]
    async fn test_websocket_event_storage_update() {
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
            _ => panic!("Expected StorageUpdate event"),
        }
    }

    #[tokio::test]
    async fn test_websocket_event_storage_update_no_progress() {
        let event = WebSocketEvent::storage_update("delete_dataset", "completed", None);

        match event {
            WebSocketEvent::StorageUpdate {
                b_operation,
                status,
                progress,
            } => {
                assert_eq!(b_operation, "delete_dataset");
                assert_eq!(status, "completed");
                assert_eq!(progress, None);
            }
            _ => panic!("Expected StorageUpdate event"),
        }
    }

    #[tokio::test]
    async fn test_websocket_event_health_update() {
        let event = WebSocketEvent::health_update("api-server", "healthy");

        match event {
            WebSocketEvent::HealthUpdate {
                service,
                status,
                timestamp,
            } => {
                assert_eq!(service, "api-server");
                assert_eq!(status, "healthy");
                assert!(!timestamp.is_empty());
            }
            _ => panic!("Expected HealthUpdate event"),
        }
    }

    #[tokio::test]
    async fn test_websocket_event_metrics_update() {
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_string(), 45.5);
        metrics.insert("memory_usage".to_string(), 70.2);

        let event = WebSocketEvent::metrics_update(metrics.clone());

        match event {
            WebSocketEvent::MetricsUpdate {
                metrics: event_metrics,
                timestamp,
            } => {
                assert_eq!(event_metrics.len(), 2);
                assert_eq!(event_metrics.get("cpu_usage"), Some(&45.5));
                assert_eq!(event_metrics.get("memory_usage"), Some(&70.2));
                assert!(!timestamp.is_empty());
            }
            _ => panic!("Expected MetricsUpdate event"),
        }
    }

    #[tokio::test]
    async fn test_websocket_event_message() {
        let event = WebSocketEvent::message("Hello, WebSocket!");

        match event {
            WebSocketEvent::Message { content, timestamp } => {
                assert_eq!(content, "Hello, WebSocket!");
                assert!(!timestamp.is_empty());
            }
            _ => panic!("Expected Message event"),
        }
    }

    // ==================== SERIALIZATION TESTS ====================

    #[tokio::test]
    async fn test_event_serialization_message() {
        let event = WebSocketEvent::message("test message");
        let serialized = serde_json::to_string(&event);
        assert!(serialized.is_ok());

        let deserialized: std::result::Result<WebSocketEvent, _> =
            serde_json::from_str(&serialized.expect("Test: serialization should succeed"));
        assert!(deserialized.is_ok());

        match deserialized.expect("Test: deserialization should succeed") {
            WebSocketEvent::Message { content, .. } => {
                assert_eq!(content, "test message");
            }
            _ => panic!("Expected Message event"),
        }
    }

    #[tokio::test]
    async fn test_event_serialization_storage_update() {
        let event = WebSocketEvent::storage_update("snapshot", "running", Some(75));
        let serialized = serde_json::to_string(&event);
        assert!(serialized.is_ok());

        let json = serialized.expect("Test: serialization should succeed");
        assert!(json.contains("StorageUpdate"));
        assert!(json.contains("snapshot"));
        assert!(json.contains("75"));
    }

    #[tokio::test]
    async fn test_event_serialization_health_update() {
        let event = WebSocketEvent::health_update("database", "degraded");
        let serialized = serde_json::to_string(&event);
        assert!(serialized.is_ok());

        let json = serialized.expect("Test: serialization should succeed");
        assert!(json.contains("HealthUpdate"));
        assert!(json.contains("database"));
        assert!(json.contains("degraded"));
    }

    #[tokio::test]
    async fn test_event_serialization_metrics_update() {
        let mut metrics = HashMap::new();
        metrics.insert("latency_ms".to_string(), 125.5);

        let event = WebSocketEvent::metrics_update(metrics);
        let serialized = serde_json::to_string(&event);
        assert!(serialized.is_ok());

        let json = serialized.expect("Test: serialization should succeed");
        assert!(json.contains("MetricsUpdate"));
        assert!(json.contains("latency_ms"));
    }

    // ==================== EDGE CASES ====================

    #[tokio::test]
    async fn test_storage_update_progress_boundaries() {
        // Progress at 0%
        let event_0 = WebSocketEvent::storage_update("op1", "started", Some(0));
        match event_0 {
            WebSocketEvent::StorageUpdate { progress, .. } => assert_eq!(progress, Some(0)),
            _ => panic!("Expected StorageUpdate"),
        }

        // Progress at 100%
        let event_100 = WebSocketEvent::storage_update("op2", "completed", Some(100));
        match event_100 {
            WebSocketEvent::StorageUpdate { progress, .. } => assert_eq!(progress, Some(100)),
            _ => panic!("Expected StorageUpdate"),
        }
    }

    #[tokio::test]
    async fn test_metrics_update_empty_metrics() {
        let metrics = HashMap::new();
        let event = WebSocketEvent::metrics_update(metrics);

        match event {
            WebSocketEvent::MetricsUpdate { metrics, .. } => {
                assert_eq!(metrics.len(), 0);
            }
            _ => panic!("Expected MetricsUpdate"),
        }
    }

    #[tokio::test]
    async fn test_metrics_update_many_metrics() {
        let mut metrics = HashMap::new();
        for i in 0..100 {
            metrics.insert(format!("metric_{i}"), f64::from(i));
        }

        let event = WebSocketEvent::metrics_update(metrics);

        match event {
            WebSocketEvent::MetricsUpdate { metrics, .. } => {
                assert_eq!(metrics.len(), 100);
            }
            _ => panic!("Expected MetricsUpdate"),
        }
    }

    #[tokio::test]
    async fn test_message_empty_content() {
        let event = WebSocketEvent::message("");

        match event {
            WebSocketEvent::Message { content, .. } => {
                assert_eq!(content, "");
            }
            _ => panic!("Expected Message"),
        }
    }

    #[tokio::test]
    async fn test_message_large_content() {
        let large_content = "X".repeat(10_000);
        let event = WebSocketEvent::message(&large_content);

        match event {
            WebSocketEvent::Message { content, .. } => {
                assert_eq!(content.len(), 10_000);
            }
            _ => panic!("Expected Message"),
        }
    }

    #[tokio::test]
    async fn test_message_special_characters() {
        let special_content = "Hello 🚀 World! \n\t Special: <>&\"'";
        let event = WebSocketEvent::message(special_content);

        match event {
            WebSocketEvent::Message { content, .. } => {
                assert_eq!(content, special_content);
            }
            _ => panic!("Expected Message"),
        }
    }

    // ==================== BROADCAST TESTS ====================

    #[tokio::test]
    async fn test_broadcast_multiple_events() {
        let manager = WebSocketManager::new();
        // Subscribe to receive events (prevents RecvError)
        let _receiver = manager.event_sender.subscribe();

        let event1 = WebSocketEvent::message("Event 1");
        let event2 = WebSocketEvent::message("Event 2");
        let event3 = WebSocketEvent::message("Event 3");

        assert!(manager.broadcast_event(event1).is_ok());
        assert!(manager.broadcast_event(event2).is_ok());
        assert!(manager.broadcast_event(event3).is_ok());
    }

    #[tokio::test]
    async fn test_connected_clients_count() {
        let manager = WebSocketManager::new();
        let initial_count = manager.connected_clients().await;
        assert_eq!(initial_count, 0);
    }

    // ==================== TIMESTAMP TESTS ====================

    #[tokio::test]
    async fn test_timestamp_format() {
        let event = WebSocketEvent::message("test");

        match event {
            WebSocketEvent::Message { timestamp, .. } => {
                // Verify timestamp is in RFC3339 format (contains T and Z)
                assert!(timestamp.contains('T'));
                assert!(timestamp.contains('Z') || timestamp.contains('+'));
            }
            _ => panic!("Expected Message"),
        }
    }

    #[tokio::test]
    async fn test_health_update_timestamp_format() {
        let event = WebSocketEvent::health_update("service", "status");

        match event {
            WebSocketEvent::HealthUpdate { timestamp, .. } => {
                assert!(timestamp.contains('T'));
                assert!(timestamp.contains('Z') || timestamp.contains('+'));
            }
            _ => panic!("Expected HealthUpdate"),
        }
    }
}
