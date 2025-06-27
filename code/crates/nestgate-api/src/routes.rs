//! API route definitions for NestGate

use axum::{
    routing::get,
    Router,
};

use crate::handlers;
use crate::handlers::auth::AuthService;

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

/// Create authentication router
pub fn create_auth_router() -> Router<AuthService> {
    use axum::routing::post;
    Router::new()
        .route("/api/v1/auth/authenticate", post(handlers::auth::authenticate))
        .route("/api/v1/auth/status", get(handlers::auth::auth_status))
        .route("/api/v1/auth/switch-mode", post(handlers::auth::switch_mode))
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

/// Create full-featured router with authentication and ZFS support
pub fn create_full_router() -> Router {
    use axum::routing::post;
    
    // Create authentication service in hybrid mode (fallback to standalone)
    let auth_service = AuthService::hybrid(
        nestgate_core::cert::BearDogConfig::default()
    );
    
    Router::new()
        // General endpoints (stateless)
        .route("/health", get(handlers::health::health_check))
        .route("/api/v1/status", get(handlers::status::get_status))
        // Authentication endpoints
        .route("/api/v1/auth/authenticate", post(handlers::auth::authenticate))
        .route("/api/v1/auth/status", get(handlers::auth::auth_status))
        .route("/api/v1/auth/switch-mode", post(handlers::auth::switch_mode))
        .with_state(auth_service)
} 