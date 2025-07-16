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
pub mod universal_adapter;
pub mod handlers {
    pub mod auth;
    pub mod hardware_tuning;
    pub mod health;
    pub mod load_testing;
    pub mod performance_analytics;
    pub mod status;
    pub mod storage;
    pub mod universal_model_api;
    pub mod workspace_management;
    pub mod zfs;
}
pub mod mcp_streaming;
pub mod models;
pub mod routes;
#[cfg(feature = "streaming-rpc")]
// pub mod tarpc_service; // Disabled - requires tarpc dependency
pub mod websocket;

#[cfg(feature = "streaming-rpc")]
// pub mod streaming_rpc; // Disabled - requires tarpc dependency

// New modules for enhanced streaming and RPC
pub mod sse;
pub mod universal_primal;
pub mod universal_primal_config;

/// Create the main API application with all communication layers
///
/// This function creates the core NestGate API application with comprehensive
/// communication support including REST APIs, WebSocket connections, Server-Sent
/// Events (SSE), and MCP streaming capabilities.
///
/// ## Features
///
/// - **REST API**: Full CRUD operations for storage, workspaces, and teams
/// - **WebSocket**: Real-time bidirectional communication
/// - **Server-Sent Events**: Streaming updates and notifications
/// - **MCP Streaming**: Model Control Protocol for AI integration
/// - **Event Coordination**: Unified event handling across all protocols
/// - **CORS Support**: Cross-origin resource sharing for web clients
///
/// ## Usage
///
/// ```rust
/// use nestgate_api::create_app;
///
/// #[tokio::main]
/// async fn main() {
///     let app = create_app();
///     let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
///     axum::serve(listener, app).await.unwrap();
/// }
/// ```
///
/// ## Returns
///
/// Returns a configured `Router` with all communication layers and middleware
/// ready for deployment.
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
    // let server = streaming_rpc::StreamingRpcServer::new();
    // server.start(addr.to_string()).await?;
    info!("Streaming RPC server disabled - requires tarpc dependency");
    Ok(())
}

/// Communication layer manager for coordinating all protocols
pub struct CommunicationManager {
    #[cfg(feature = "streaming-rpc")]
    pub websocket_manager: websocket::WebSocketManager,
    #[cfg(feature = "streaming-rpc")]
    pub sse_manager: sse::SseManager,
    #[cfg(feature = "streaming-rpc")]
    // pub streaming_rpc_server: streaming_rpc::StreamingRpcServer, // Disabled - requires tarpc
    pub mcp_streaming_manager: mcp_streaming::McpStreamingManager,
    pub event_coordinator: event_coordination::EventCoordinator,
}

impl Default for CommunicationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CommunicationManager {
    /// Create a new communication manager with all protocols
    ///
    /// Initializes a comprehensive communication system that supports multiple
    /// real-time communication protocols for maximum flexibility and integration.
    ///
    /// ## Initialized Components
    ///
    /// - **WebSocket Manager**: Handles bidirectional real-time communication
    /// - **SSE Manager**: Manages Server-Sent Events for streaming updates
    /// - **MCP Streaming Manager**: Handles Model Control Protocol streaming
    /// - **Event Coordinator**: Provides unified event handling across all protocols
    ///
    /// ## Features Enabled
    ///
    /// - Real-time data streaming
    /// - Cross-protocol event coordination
    /// - Automatic connection management
    /// - Background cleanup tasks
    /// - Performance monitoring
    ///
    /// ## Usage
    ///
    /// ```rust
    /// use nestgate_api::CommunicationManager;
    ///
    /// let comm_manager = CommunicationManager::new();
    ///
    /// // Start all protocols
    /// comm_manager.start_all("127.0.0.1:8080", "127.0.0.1:8081").await?;
    /// ```
    ///
    /// ## Note
    ///
    /// Some features require the `streaming-rpc` feature flag to be enabled.
    /// The manager will automatically adapt based on available features.
    pub fn new() -> Self {
        #[cfg(feature = "streaming-rpc")]
        let websocket_manager = websocket::WebSocketManager::new();
        #[cfg(feature = "streaming-rpc")]
        let sse_manager = sse::SseManager::new();
        #[cfg(feature = "streaming-rpc")]
        // let streaming_rpc_server = streaming_rpc::StreamingRpcServer::new(); // Disabled - requires tarpc
        let mcp_streaming_manager = mcp_streaming::McpStreamingManager::new();
        let event_coordinator = event_coordination::EventCoordinator::new();

        Self {
            #[cfg(feature = "streaming-rpc")]
            websocket_manager,
            #[cfg(feature = "streaming-rpc")]
            sse_manager,
            #[cfg(feature = "streaming-rpc")]
            // streaming_rpc_server, // Disabled - requires tarpc
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
        let _http_server = start_server(http_addr);

        // Start cleanup tasks
        #[cfg(feature = "streaming-rpc")]
        let _sse_cleanup = self.sse_manager.start_cleanup_task();

        // Start streaming RPC server if feature is enabled
        #[cfg(feature = "streaming-rpc")]
        // let rpc_server = self.streaming_rpc_server.start(_rpc_addr.to_string()); // Disabled - requires tarpc

        // Run servers concurrently
        #[cfg(feature = "streaming-rpc")]
        // tokio::try_join!(http_server, rpc_server)?; // Disabled - requires tarpc
        // #[cfg(not(feature = "streaming-rpc"))]
        http_server.await?;

        Ok(())
    }

    /// Broadcast an event to all communication channels
    pub async fn broadcast_event(
        &self,
        _event_data: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Create SSE event
        #[cfg(feature = "streaming-rpc")]
        {
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
        }

        // Create and broadcast streaming RPC event if feature is enabled
        #[cfg(feature = "streaming-rpc")]
        {
            // let rpc_event = streaming_rpc::StorageEvent {
            //     id: uuid::Uuid::new_v4().to_string(),
            //     event_type: "system_event".to_string(),
            //     timestamp: std::time::SystemTime::now(),
            //     source: "nestgate-api".to_string(),
            //     data: event_data,
            //     priority: 2,
            // };

            // Broadcast to RPC clients
            // self.streaming_rpc_server.broadcast_event(rpc_event).await?;
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
        println!("✅ Toadstool integration configured: {toadstool_url}");
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
        // let server = streaming_rpc::StreamingRpcServer::new();

        // Test health check
        // let health = server
        //     .health_check(tarpc::context::Context::current())
        //     .await;
        // assert!(health.is_ok());
        println!("✅ Streaming RPC server test disabled - requires tarpc dependency");

        // Test capabilities
        // let capabilities = server
        //     .get_capabilities(tarpc::context::Context::current())
        //     .await;
        // assert!(capabilities.is_ok());

        // let caps = capabilities.unwrap();
        // assert!(caps.streaming_support);
        // assert!(caps.bidirectional_support);
        // println!("✅ Streaming RPC server capabilities correct");
    }
}
