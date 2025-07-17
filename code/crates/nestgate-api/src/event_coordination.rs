//! Event Coordination System
//!
//! This module provides reactive event coordination for the hybrid communication
//! system, enabling real-time coordination between WebSocket clients, internal
//! services, and MCP streams.

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};
use tracing::{debug, info};
use uuid::Uuid;

/// Event coordinator for managing reactive communication between components
pub struct EventCoordinator {
    /// Registered event handlers
    handlers: Arc<RwLock<HashMap<String, EventHandler>>>,
    /// Event broadcaster
    event_broadcaster: broadcast::Sender<CoordinatedEvent>,
    /// Event processing statistics
    stats: Arc<RwLock<EventStats>>,
}

/// Event processing statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStats {
    pub total_events: u64,
    pub events_processed: u64,
    pub active_handlers: u64,
    pub errors: u64,
}

/// Event handler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHandler {
    /// Handler identifier
    pub id: Uuid,
    /// Handler name
    pub name: String,
    /// Event patterns this handler responds to
    pub patterns: Vec<String>,
    /// Handler priority (higher = processed first)
    pub priority: Priority,
    /// Whether handler is active
    pub active: bool,
    /// Handler configuration
    pub config: serde_json::Value,
}

/// Handler priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    /// Lowest priority
    Low = 1,
    /// Normal priority
    Normal = 2,
    /// High priority
    High = 3,
    /// Critical priority (processed first)
    Critical = 4,
}

/// Coordinated event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatedEvent {
    /// Event identifier
    pub event_id: Uuid,
    /// Event type
    pub event_type: CoordinatedEventType,
    /// Event source
    pub source: String,
    /// Event data
    pub data: serde_json::Value,
    /// Event timestamp
    pub timestamp: std::time::SystemTime,
}

/// Types of coordinated events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinatedEventType {
    /// WebSocket event
    WebSocket,
    /// Internal service event
    InternalService,
    /// MCP streaming event
    McpStream,
    /// Storage operation event
    StorageOperation,
    /// Configuration change event
    ConfigurationChange,
    /// Health monitoring event
    HealthMonitoring,
}

/// Event processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventProcessingResult {
    pub event_id: Uuid,
    pub handler_id: String,
    pub success: bool,
    pub processing_time: std::time::Duration,
    pub error_message: Option<String>,
}

impl Default for EventCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl EventCoordinator {
    /// Create a new event coordinator
    pub fn new() -> Self {
        let (event_broadcaster, _) = broadcast::channel(1000);

        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            event_broadcaster,
            stats: Arc::new(RwLock::new(EventStats {
                total_events: 0,
                events_processed: 0,
                active_handlers: 0,
                errors: 0,
            })),
        }
    }

    /// Register an event handler
    pub async fn register_handler(
        &self,
        handler: EventHandler,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let handler_id = handler.id.to_string();

        {
            let mut handlers = self.handlers.write().await;
            handlers.insert(handler_id.clone(), handler);
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.active_handlers += 1;
        }

        info!("Registered event handler: {}", handler_id);
        Ok(())
    }

    /// Emit an event to all registered handlers
    pub async fn emit_event(
        &self,
        event: CoordinatedEvent,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let handlers = self.handlers.read().await;

        for handler in handlers.values() {
            if handler.active && self.event_matches_handler(&event, handler) {
                let _ = self.handle_event_with_handler(event.clone(), handler).await;
            }
        }

        Ok(())
    }

    /// List all registered handlers
    pub async fn list_handlers(&self) -> Vec<EventHandler> {
        let handlers = self.handlers.read().await;
        handlers.values().cloned().collect()
    }

    /// Get handler count
    pub async fn get_handler_count(&self) -> u64 {
        let stats = self.stats.read().await;
        stats.active_handlers
    }

    /// Get event count
    pub async fn get_event_count(&self) -> u64 {
        let stats = self.stats.read().await;
        stats.events_processed
    }

    /// Get event processing statistics
    pub async fn get_stats(&self) -> EventStats {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Check if event matches handler pattern
    fn event_matches_handler(&self, event: &CoordinatedEvent, handler: &EventHandler) -> bool {
        if handler.patterns.is_empty() {
            return true; // Match all events
        }

        let event_type_str = format!("{:?}", event.event_type);

        for pattern in &handler.patterns {
            if pattern == "*" || pattern == "all" {
                return true;
            }
            if event_type_str
                .to_lowercase()
                .contains(&pattern.to_lowercase())
            {
                return true;
            }
        }

        false
    }

    /// Handle an event with a specific handler
    async fn handle_event_with_handler(
        &self,
        event: CoordinatedEvent,
        handler: &EventHandler,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!(
            "Handling event {} with handler {}",
            event.event_id, handler.name
        );

        // Route to the appropriate handler method based on event type
        let result: std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> =
            match event.event_type {
                CoordinatedEventType::WebSocket => self.handle_websocket_event(&event).await,
                CoordinatedEventType::InternalService => {
                    self.handle_internal_service_event(&event).await
                }
                CoordinatedEventType::McpStream => self.handle_mcp_stream_event(&event).await,
                CoordinatedEventType::StorageOperation => {
                    self.handle_storage_operation_event(&event).await
                }
                CoordinatedEventType::ConfigurationChange => {
                    debug!("Handling configuration change event: {}", event.event_id);
                    Ok(())
                }
                CoordinatedEventType::HealthMonitoring => {
                    debug!("Handling health monitoring event: {}", event.event_id);
                    Ok(())
                }
            };

        // Update stats
        let mut stats = self.stats.write().await;
        stats.total_events += 1;
        stats.events_processed += 1;

        // Check if the result is an error
        if result.is_err() {
            stats.errors += 1;
        }

        drop(stats);

        // Return the result
        result
    }

    /// Handle WebSocket events
    async fn handle_websocket_event(
        &self,
        event: &CoordinatedEvent,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("Handling WebSocket event: {:?}", event.event_type);

        // Create a WebSocket event from the coordinated event (if WebSocket feature is enabled)
        #[cfg(feature = "streaming-rpc")]
        {
            let _ws_event = crate::websocket::WebSocketEvent {
                event_id: event.event_id,
                client_id: uuid::Uuid::new_v4(), // Generate a new UUID for client_id
                event_type: crate::websocket::WebSocketEventType::Message,
                data: event.data.clone(),
                timestamp: std::time::SystemTime::now(),
            };

            // In a full implementation, this would be sent to the WebSocket manager
            tracing::debug!("WebSocket event created: {}", _ws_event.event_id);
        }

        #[cfg(not(feature = "streaming-rpc"))]
        {
            tracing::debug!("WebSocket support not enabled, skipping WebSocket event creation");
        }

        // Broadcast to WebSocket clients through the event broadcaster
        if let Err(e) = self.event_broadcaster.send(event.clone()) {
            tracing::warn!("Failed to broadcast WebSocket event: {}", e);
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.events_processed += 1;
        }

        info!("WebSocket event handled successfully: {}", event.event_id);
        Ok(())
    }

    /// Handle internal service events
    async fn handle_internal_service_event(
        &self,
        event: &CoordinatedEvent,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("Handling internal service event: {:?}", event.event_type);

        // Route to appropriate internal service based on event data
        match event.data.get("service_type").and_then(|v| v.as_str()) {
            Some("universal_primal") => {
                info!("Routing to Universal Primal service: {}", event.event_id);
                // In a full implementation, this would interface with UniversalPrimal
                // Log the routing action without emitting another event to avoid recursion
                debug!(
                    "Routed event {} to universal_primal service",
                    event.event_id
                );
            }
            Some("security") => {
                info!("Routing to Security service: {}", event.event_id);
                // Route to security manager
            }
            Some("performance") => {
                info!("Routing to Performance service: {}", event.event_id);
                // Route to performance monitor
            }
            _ => {
                info!("Generic internal service event: {}", event.event_id);
                // Handle generic internal service events
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.events_processed += 1;
        }

        Ok(())
    }

    /// Handle MCP stream events
    async fn handle_mcp_stream_event(
        &self,
        event: &CoordinatedEvent,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("Handling MCP stream event: {:?}", event.event_type);

        // Extract stream information from event data
        let stream_id = event
            .data
            .get("stream_id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let stream_type = event
            .data
            .get("stream_type")
            .and_then(|v| v.as_str())
            .unwrap_or("generic");

        // Handle different types of MCP stream events
        match stream_type {
            "model_request" => {
                info!("Processing MCP model request stream: {}", stream_id);
                // In a full implementation, this would coordinate with MCP streaming manager
                // to handle model requests, potentially routing to AI services
            }
            "data_stream" => {
                info!("Processing MCP data stream: {}", stream_id);
                // Handle data streaming events
            }
            "control_message" => {
                info!("Processing MCP control message: {}", stream_id);
                // Handle control messages that might affect stream state
            }
            _ => {
                info!("Processing generic MCP stream event: {}", stream_id);
            }
        }

        // Log the MCP stream processing completion (avoid recursion)
        debug!(
            "MCP stream event {} processed for stream {}",
            event.event_id, stream_id
        );

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.events_processed += 1;
        }

        Ok(())
    }

    /// Handle storage operation events
    async fn handle_storage_operation_event(
        &self,
        event: &CoordinatedEvent,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("Handling storage operation event: {:?}", event.event_type);

        // Extract storage operation information from event data
        let operation = event
            .data
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let pool_name = event
            .data
            .get("pool_name")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        let dataset_name = event.data.get("dataset_name").and_then(|v| v.as_str());

        // Handle different types of storage operations
        match operation {
            "pool_create" => {
                info!("Processing pool creation event for pool: {}", pool_name);
                // In a full implementation, this would coordinate with ZFS manager
                // to handle pool creation and emit status updates
            }
            "pool_destroy" => {
                info!("Processing pool destruction event for pool: {}", pool_name);
                // Handle pool destruction coordination
            }
            "dataset_create" => {
                if let Some(dataset) = dataset_name {
                    info!("Processing dataset creation event: {}", dataset);
                }
            }
            "dataset_destroy" => {
                if let Some(dataset) = dataset_name {
                    info!("Processing dataset destruction event: {}", dataset);
                }
            }
            "snapshot_create" => {
                info!("Processing snapshot creation event for pool: {}", pool_name);
                // Handle snapshot operations
            }
            "health_check" => {
                info!(
                    "Processing storage health check event for pool: {}",
                    pool_name
                );
                // Coordinate health monitoring
            }
            _ => {
                info!("Processing generic storage operation: {}", operation);
            }
        }

        // Log the storage operation completion (avoid recursion)
        debug!(
            "Storage operation {} completed for pool {} (operation: {})",
            event.event_id, pool_name, operation
        );

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.events_processed += 1;
        }

        Ok(())
    }

    /// Create a WebSocket event
    pub fn websocket_event(
        client_id: &str,
        _event_type: &str,
        data: serde_json::Value,
    ) -> CoordinatedEvent {
        CoordinatedEvent {
            event_id: Uuid::new_v4(),
            event_type: CoordinatedEventType::WebSocket,
            source: format!("websocket-{client_id}"),
            data,
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Create an internal service event
    pub fn internal_service_event(service_name: &str, data: serde_json::Value) -> CoordinatedEvent {
        CoordinatedEvent {
            event_id: Uuid::new_v4(),
            event_type: CoordinatedEventType::InternalService,
            source: format!("service-{service_name}"),
            data,
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Create an MCP stream event
    pub fn mcp_stream_event(stream_id: &str, data: serde_json::Value) -> CoordinatedEvent {
        CoordinatedEvent {
            event_id: Uuid::new_v4(),
            event_type: CoordinatedEventType::McpStream,
            source: format!("mcp-stream-{stream_id}"),
            data,
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Create a storage operation event
    pub fn storage_operation(_operation: &str, data: serde_json::Value) -> CoordinatedEvent {
        CoordinatedEvent {
            event_id: Uuid::new_v4(),
            event_type: CoordinatedEventType::StorageOperation,
            source: "storage-service".to_string(),
            data,
            timestamp: std::time::SystemTime::now(),
        }
    }
}

impl Clone for EventCoordinator {
    fn clone(&self) -> Self {
        Self {
            handlers: self.handlers.clone(),
            event_broadcaster: self.event_broadcaster.clone(),
            stats: self.stats.clone(),
        }
    }
}
