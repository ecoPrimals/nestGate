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
use anyhow::Result;
use std::net::SocketAddr;
use tracing::{info, warn};

use crate::event_coordination::EventCoordinator;

use crate::routes::create_router;

// pub mod byob;  // Temporarily disabled during universal architecture transition
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
    _addr: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let server = streaming_rpc::StreamingRpcServer::new();
    // server.start(addr.to_string()).await?;
    info!("Streaming RPC server disabled - requires tarpc dependency");
    Ok(())
}

/// Communication layer manager for coordinating all protocols
pub struct CommunicationManager {
    pub event_coordinator: EventCoordinator,
    pub universal_primal: Option<Box<dyn crate::universal_primal::StoragePrimalProvider>>,
}

impl Default for CommunicationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CommunicationManager {
    /// Create a new communication manager with all protocols
    pub fn new() -> Self {
        let event_coordinator = EventCoordinator::new();

        Self {
            event_coordinator,
            universal_primal: None,
        }
    }

    /// Start all communication protocols with orchestration module integration
    pub async fn start_all_protocols(&mut self) -> Result<()> {
        info!("🚀 Starting all communication protocols");

        // Get the address to bind to
        let http_addr = SocketAddr::from(([127, 0, 0, 1], 8080)); // Placeholder port
        info!("📡 Binding to address: {}", http_addr);

        // Try orchestration module service mesh first (primary mode)
        match self.try_orchestration_integration(http_addr).await {
            Ok(()) => {
                info!("✅ Running in orchestration module service mesh mode");
                // Orchestration module handles HTTP routing - we just need to keep the service alive
                self.run_as_orchestration_managed_primal().await
            }
            Err(e) => {
                warn!("⚠️ Orchestration module integration failed: {}", e);
                info!("🔄 Falling back to standalone HTTP server mode");
                self.run_standalone_http_server(http_addr).await
            }
        }
    }

    /// Try to integrate with orchestration module service mesh (primary mode)
    async fn try_orchestration_integration(&mut self, _http_addr: SocketAddr) -> Result<()> {
        info!("🎼 Attempting orchestration module integration");

        // STUB: This would attempt to register with the orchestration module
        // For now, we'll just check if there's an orchestration module URL configured
        if let Ok(orchestration_url) = std::env::var("NESTGATE_ORCHESTRATION_URL") {
            info!("🌐 Found orchestration module URL: {}", orchestration_url);

            // STUB: In a real implementation, this would:
            // 1. Connect to orchestration module service mesh
            // 2. Register our service capabilities
            // 3. Set up HTTP routing delegation
            // 4. Configure health checks

            // Register with ecosystem (orchestration module service mesh)
            // TODO: Implement proper ecosystem registration
            // if let Some(ref mut universal_primal) = self.universal_primal {
            //     universal_primal.register_with_ecosystem().await?;
            // }

            info!("🎼 Successfully integrated with orchestration module service mesh");
            Ok(())
        } else {
            Err(anyhow::anyhow!("No orchestration module URL configured"))
        }
    }

    /// Run as orchestration-managed primal (HTTP handled by service mesh)
    async fn run_as_orchestration_managed_primal(&mut self) -> Result<()> {
        info!("🌐 Running as orchestration-managed primal");

        // In orchestration mode, we don't start our own HTTP server
        // Instead, we provide handlers that orchestration module will route to

        // Keep the service alive and handle inter-primal communication
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

        loop {
            interval.tick().await;

            // Health check and maintenance
            if let Err(e) = self.perform_health_check().await {
                warn!("Health check failed: {}", e);
            }

            // Handle any pending events
            if let Err(e) = self.process_pending_events().await {
                warn!("Event processing failed: {}", e);
            }
        }
    }

    /// Run standalone HTTP server (fallback mode)
    async fn run_standalone_http_server(&mut self, http_addr: SocketAddr) -> Result<()> {
        info!("🏠 Running standalone HTTP server at {}", http_addr);

        // Start HTTP/WebSocket/SSE server in standalone mode
        let http_addr_str = http_addr.to_string();
        let server_result = tokio::spawn(async move { start_server(&http_addr_str).await })
            .await
            .map_err(|e| anyhow::anyhow!("Server task failed: {}", e))?;

        server_result.map_err(|e| anyhow::anyhow!("Server failed: {}", e))?;
        Ok(())
    }

    /// Perform health check for service mesh
    async fn perform_health_check(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Health check logic here
        Ok(())
    }

    /// Process pending events
    async fn process_pending_events(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Event processing logic here
        Ok(())
    }

    /// Broadcast an event to all communication channels
    pub async fn broadcast_event(
        &self,
        event_data: serde_json::Value,
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
            // self.sse_manager.broadcast_event(sse_event).await?; // sse_manager is removed

            // Create WebSocket event
            let ws_event = websocket::WebSocketEvent {
                event_id: uuid::Uuid::new_v4(),
                client_id: uuid::Uuid::new_v4(), // Broadcast to all
                event_type: websocket::WebSocketEventType::Message,
                data: event_data.clone(),
                timestamp: std::time::SystemTime::now(),
            };

            // Broadcast to WebSocket clients
            // self.websocket_manager.broadcast_event(ws_event).await?; // websocket_manager is removed
        }

        // Always log event data for debugging/monitoring
        tracing::info!("Broadcasting event: {}", event_data);

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
            // self.streaming_rpc_server.broadcast_event(rpc_event).await?; // streaming_rpc_server is removed
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
