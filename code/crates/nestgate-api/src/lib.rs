//! NestGate API Crate
//!
//! This crate provides the HTTP REST API and advanced communication layer for NestGate
//! with support for:
//! - HTTP REST API endpoints
//! - WebSocket real-time communication
//! - Server-Sent Events (SSE) streaming
//! - Bidirectional RPC with tarpc
//! - Event coordination and streaming
//! - MCP protocol extensions

use axum::Router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::routes::create_router;

pub mod byob;
pub mod event_coordination;
pub mod handlers {
    pub mod auth;
    pub mod hardware_tuning;
    pub mod health;
    pub mod load_testing;
    pub mod performance_analytics;
    pub mod status;
    pub mod storage;
    pub mod workspace_management;
    pub mod zfs;
}
pub mod mcp_streaming;
pub mod models;
pub mod routes;
#[cfg(feature = "streaming-rpc")]
pub mod tarpc_service;
pub mod websocket;

// New modules for enhanced streaming and RPC
pub mod sse;
pub mod universal_primal;
pub mod universal_primal_config;

#[cfg(feature = "streaming-rpc")]
pub mod streaming_rpc;

/// Create the main API application with all communication layers
pub fn create_app() -> Router {
    info!("Creating NestGate API application with enhanced communication layers");

    // Create the router with built-in state management
    let router = create_router();

    // Add CORS middleware
    router.layer(CorsLayer::permissive())
}

/// Start the API server with all communication protocols
pub async fn start_server(addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!(
        "NestGate API server listening on {} with full communication suite",
        addr
    );

    axum::serve(listener, app).await?;
    Ok(())
}

/// Configuration for the NestGate API server
#[derive(Debug, Clone)]
pub struct Config {
    /// Address to bind the server to (e.g., "0.0.0.0:8080")
    pub bind_addr: String,
    /// Enable ZFS API endpoints
    pub enable_zfs_api: bool,
    /// Enable SSE streaming endpoints
    pub enable_sse: bool,
    /// Enable WebSocket endpoints
    pub enable_websockets: bool,
    /// Maximum request size in bytes
    pub max_request_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:8080".to_string(),
            enable_zfs_api: true,
            enable_sse: true,
            enable_websockets: true,
            max_request_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

/// Start the API server with ZFS integration
pub async fn serve_with_zfs(
    config: Config,
    _zfs_manager: Arc<nestgate_zfs::manager::ZfsManager>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting NestGate API server with ZFS integration");
    info!("Configuration: {:?}", config);

    // For now, we'll use the existing start_server function
    // In the future, this could be enhanced to pass the ZFS manager to routes
    // that need direct ZFS access
    start_server(&config.bind_addr).await
}

/// Start the streaming RPC server
#[cfg(feature = "streaming-rpc")]
pub async fn start_streaming_rpc_server(
    addr: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server = streaming_rpc::StreamingRpcServer::new();
    server.start(addr.to_string()).await?;
    Ok(())
}

/// Communication layer manager for coordinating all protocols
pub struct CommunicationManager {
    pub websocket_manager: websocket::WebSocketManager,
    pub sse_manager: sse::SseManager,
    #[cfg(feature = "streaming-rpc")]
    pub streaming_rpc_server: streaming_rpc::StreamingRpcServer,
    pub mcp_streaming_manager: mcp_streaming::McpStreamingManager,
    pub event_coordinator: event_coordination::EventCoordinator,
}

impl CommunicationManager {
    /// Create a new communication manager with all protocols
    pub fn new() -> Self {
        let websocket_manager = websocket::WebSocketManager::new();
        let sse_manager = sse::SseManager::new();
        #[cfg(feature = "streaming-rpc")]
        let streaming_rpc_server = streaming_rpc::StreamingRpcServer::new();
        let mcp_streaming_manager = mcp_streaming::McpStreamingManager::new();
        let event_coordinator = event_coordination::EventCoordinator::new();

        Self {
            websocket_manager,
            sse_manager,
            #[cfg(feature = "streaming-rpc")]
            streaming_rpc_server,
            mcp_streaming_manager,
            event_coordinator,
        }
    }

    /// Start all communication protocols
    pub async fn start_all(
        &self,
        http_addr: &str,
        _rpc_addr: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("Starting all communication protocols");

        // Start HTTP/WebSocket/SSE server
        let http_server = start_server(http_addr);

        // Start cleanup tasks
        let _sse_cleanup = self.sse_manager.start_cleanup_task();

        // Start streaming RPC server if feature is enabled
        #[cfg(feature = "streaming-rpc")]
        let rpc_server = self.streaming_rpc_server.start(_rpc_addr.to_string());

        // Run servers concurrently
        #[cfg(feature = "streaming-rpc")]
        tokio::try_join!(http_server, rpc_server)?;
        #[cfg(not(feature = "streaming-rpc"))]
        http_server.await?;

        Ok(())
    }

    /// Broadcast an event to all communication channels
    pub async fn broadcast_event(
        &self,
        event_data: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Create SSE event
        let sse_event = sse::SseEvent {
            id: uuid::Uuid::new_v4(),
            event_type: sse::SseEventType::SystemEvent,
            data: event_data.clone(),
            timestamp: std::time::SystemTime::now(),
            source: "nestgate-api".to_string(),
            priority: sse::EventPriority::Normal,
        };

        // Broadcast to SSE clients
        self.sse_manager.broadcast_event(sse_event).await?;

        // Create WebSocket event
        let ws_event = websocket::WebSocketEvent {
            event_id: uuid::Uuid::new_v4(),
            client_id: uuid::Uuid::new_v4(), // Broadcast to all
            event_type: websocket::WebSocketEventType::Message,
            data: event_data.clone(),
            timestamp: std::time::SystemTime::now(),
        };

        // Broadcast to WebSocket clients
        self.websocket_manager.broadcast_event(ws_event).await?;

        // Create and broadcast streaming RPC event if feature is enabled
        #[cfg(feature = "streaming-rpc")]
        {
            let rpc_event = streaming_rpc::StorageEvent {
                id: uuid::Uuid::new_v4().to_string(),
                event_type: "system_event".to_string(),
                timestamp: std::time::SystemTime::now(),
                source: "nestgate-api".to_string(),
                data: event_data,
                priority: 2,
            };

            // Broadcast to RPC clients
            self.streaming_rpc_server.broadcast_event(rpc_event).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toadstool_integration() {
        // Test that Toadstool integration is properly initialized
        let toadstool_url = std::env::var("NESTGATE_TOADSTOOL_COMPUTE_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());

        assert!(!toadstool_url.is_empty());
        println!("✅ Toadstool integration configured: {}", toadstool_url);
    }

    #[tokio::test]
    async fn test_communication_manager() {
        let manager = CommunicationManager::new();

        // Test event broadcasting
        let test_event = serde_json::json!({
            "type": "test_event",
            "message": "Hello from NestGate!",
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        // This should not fail even if no clients are connected
        assert!(manager.broadcast_event(test_event).await.is_ok());
        println!("✅ Communication manager event broadcasting works");
    }

    #[cfg(feature = "streaming-rpc")]
    #[tokio::test]
    async fn test_streaming_rpc_server() {
        let server = streaming_rpc::StreamingRpcServer::new();

        // Test health check
        let health = server
            .health_check(tarpc::context::Context::current())
            .await;
        assert!(health.is_ok());
        println!("✅ Streaming RPC server health check works");

        // Test capabilities
        let capabilities = server
            .get_capabilities(tarpc::context::Context::current())
            .await;
        assert!(capabilities.is_ok());

        let caps = capabilities.unwrap();
        assert!(caps.streaming_support);
        assert!(caps.bidirectional_support);
        println!("✅ Streaming RPC server capabilities correct");
    }
}
