//! API route definitions for NestGate

use axum::{
    routing::get,
    Router,
};

use crate::handlers;

/// Create the main API router with health and status endpoints
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/api/v1/status", get(handlers::status::get_status))
        // Add more routes as needed
}

/// Create the ZFS API router (requires ZFS state)
pub fn create_zfs_router() -> Router<handlers::zfs::ZfsApiState> {
    handlers::zfs::create_zfs_routes()
}

/// Create a combined router with both general and ZFS endpoints
pub fn create_combined_router() -> Router<handlers::zfs::ZfsApiState> {
    Router::new()
        // General endpoints (stateless)
        .route("/health", get(handlers::health::health_check))
        .route("/api/v1/status", get(handlers::status::get_status))
        // ZFS endpoints (with ZFS state)
        .nest("/api/v1/zfs", handlers::zfs::create_zfs_routes())
} 