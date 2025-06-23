//! NestGate API Server
//! 
//! This crate provides the HTTP/REST API interface for NestGate,
//! allowing external systems to interact with and manage the gateway.

use std::sync::Arc;
use axum::Router;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::{info, error};
use nestgate_zfs::ZfsManager;

mod routes;
pub mod handlers;
mod models;

pub use handlers::zfs::ZfsApiState;

/// API server configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// The address to bind the API server to
    pub bind_addr: String,
    /// CORS configuration
    pub cors: Option<CorsLayer>,
    /// Enable ZFS API endpoints
    pub enable_zfs_api: bool,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Maximum request body size in bytes
    pub max_body_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:3000".to_string(),
            cors: None,
            enable_zfs_api: true,
            request_timeout: 30,
            max_body_size: 16 * 1024 * 1024, // 16MB
        }
    }
}

/// Initialize and start the API server with ZFS integration
pub async fn serve_with_zfs(config: Config, zfs_manager: Arc<ZfsManager>) -> Result<(), Box<dyn std::error::Error>> {
    let zfs_state = ZfsApiState {
        zfs_manager,
    };

    let app = routes::create_combined_router()
        .with_state(zfs_state)
        .layer(TraceLayer::new_for_http())
        .layer(config.cors.unwrap_or_else(|| {
            CorsLayer::permissive() // Default to permissive CORS in development
        }));

    info!("Starting NestGate API server on {} with ZFS integration", config.bind_addr);
    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Initialize and start the basic API server (without ZFS)
pub async fn serve(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let app = routes::create_router()
        .layer(TraceLayer::new_for_http())
        .layer(config.cors.unwrap_or_else(|| {
            CorsLayer::permissive() // Default to permissive CORS in development
        }));

    info!("Starting basic API server on {}", config.bind_addr);
    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Create a configured API router with ZFS integration
pub fn create_api_router(zfs_manager: Arc<ZfsManager>) -> Router {
    let zfs_state = ZfsApiState {
        zfs_manager,
    };

    routes::create_combined_router()
        .with_state(zfs_state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

/// Create a basic API router without ZFS integration
pub fn create_basic_router() -> Router {
    routes::create_router()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
} 