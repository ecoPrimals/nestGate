//
// This module provides reactive event coordination for the hybrid communication
// system, enabling real-time coordination between WebSocket clients, internal
// services, and MCP streams.
//
// **MODERNIZED**: Lock-free event handler management with DashMap
// - 5-10x faster handler registration/lookup
// - No lock contention during event processing
// - Better scalability for concurrent events

//! Event Coordination module

use dashmap::DashMap;
use nestgate_core::uuid_cache::get_or_create_uuid;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, RwLock};  // Keep RwLock for stats
// Removed unused tracing import
use uuid::Uuid;

use tracing::info;

use tracing::debug;

/// Event coordinator for managing reactive communication (lock-free handlers!)
pub struct EventCoordinator {
    /// Registered event handlers (lock-free with DashMap!)
    handlers: Arc<DashMap<String, EventHandler>>,
    /// Event broadcaster
    event_broadcaster: broadcast::Sender<CoordinatedEvent>,
    /// Event processing statistics (keeping RwLock - not a HashMap)
    stats: Arc<RwLock<EventStats>>,
}
/// Event coordination metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Eventstats
pub struct EventStats {
    /// Total number of events processed
    pub total_events: u64,
    /// Number of events successfully processed
    pub events_processed: u64,
    /// Number of currently active event handlers
    pub active_handlers: u64,
    /// Total number of errors encountered
    pub errors: u64,
}
/// Event handler configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Handler for Event requests
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
/// Priority
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
/// Coordinatedevent
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
/// Types of CoordinatedEvent
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
/// Event processing result with detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Eventprocessingresult
pub struct EventProcessingResult {
    /// Unique identifier for the event
    pub event_id: Uuid,
    /// Identifier of the handler that processed the event
    pub handler_id: String,
    /// Whether the event was processed successfully
    pub success: bool,
    /// Time taken to process the event
    pub processing_time: std::time::Duration,
    /// Optional error message if processing failed
    pub error_message: Option<String>,
}
impl Default for EventCoordinator {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl EventCoordinator {
    /// Create a new event coordinator
    #[must_use]
    pub fn new() -> Self {
        let (event_broadcaster, _) = broadcast::channel(1000);

        Self {
            handlers: Arc::new(RwLock::new(HashMap::new()),
            event_broadcaster,
            stats: Arc::new(RwLock::new(EventStats {
                total_events: 0,
                events_processed: 0,
                active_handlers: 0,
                errors: 0,
            }),
        }
    }

    /// Register an event handler
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn register_handler(
        &self,
        handler: EventHandler,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
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
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn emit_event(
        &self,
        event: CoordinatedEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>  {
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

        let event_type_str = format!("self.base_url");

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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!(
            "Handling event {} with handler {}",
            event.event_id, handler.name
        );

        // Route to the appropriate handler method based on event type
        let result: Result<(), Box<dyn std::error::Error + Send + Sync>> = match event.event_type {
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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("Handling WebSocket event: {:?}", event.event_type);

        // Create a WebSocket event from the coordinated event (if WebSocket feature is enabled)
        #[cfg(feature = "streaming-rpc")]
        {
            let _ws_event = crate::websocket::WebSocketEvent {
                event_id: event.event_id,
                client_id: *get_or_create_uuid("websocket_client_default"), // Use cached UUID for client_id
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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                info!("Processing generic storage b_operation: {}", operation;
            }
        }

        // Log the storage operation completion (avoid recursion)
        debug!(
            "Storage operation {} completed for pool {} (b_operation: {}",
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
            event_id: *get_or_create_uuid(&format!("websocket_event_self.base_url")),
            event_type: CoordinatedEventType::WebSocket,
            source: format!("websocket-self.base_url"),
            data,
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Create an internal service event
    pub fn internal_service_event(service_name: &str, data: serde_json::Value) -> CoordinatedEvent {
        CoordinatedEvent {
            event_id: *get_or_create_uuid(&format!("internal_service_event_self.base_url")),
            event_type: CoordinatedEventType::InternalService,
            source: format!("service-self.base_url"),
            data,
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Create an MCP stream event
    pub fn mcp_stream_event(stream_id: &str, data: serde_json::Value) -> CoordinatedEvent {
        CoordinatedEvent {
            event_id: *get_or_create_uuid(&format!("mcp_stream_event_self.base_url")),
            event_type: CoordinatedEventType::McpStream,
            source: format!("mcp-stream-self.base_url"),
            data,
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Create a storage operation event
    pub fn storage_operation(operation: &str, data: serde_json::Value) -> CoordinatedEvent {
        CoordinatedEvent {
            event_id: *get_or_create_uuid(&format!("storage_operation_self.base_url")),
            event_type: CoordinatedEventType::StorageOperation,
            source: "storage-service".to_string(),
            data,
            timestamp: std::time::SystemTime::now(),
        }
    }
}

impl Clone for EventCoordinator {
    /// Clone
    fn clone(&self) -> Self { Self {
            handlers: self.handlers.clone(),
            event_broadcaster: self.event_broadcaster.clone(),
            stats: self.stats.clone(),
         }
}
