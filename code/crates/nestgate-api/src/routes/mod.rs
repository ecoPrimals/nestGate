// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! API Routes Module
//!
//! Defines all HTTP routes and endpoints for the NestGate REST API.
//!
//! # Architecture
//!
//! Routes are organized hierarchically:
//! - `/health` - Health check and system status
//! - `/api/v1/storage/*` - Storage management (pools, datasets, snapshots)
//! - `/api/v1/monitoring/*` - Metrics and performance analytics
//! - `/api/v1/workspaces/*` - Workspace management
//! - `/api/v1/load-testing/*` - Load testing and benchmarking
//!
//! # Handler Organization
//!
//! Handlers are grouped by domain:
//! - `storage`: ZFS pool/dataset operations
//! - `performance_analytics`: Metrics and recommendations
//! - `workspace_management`: Multi-tenant workspace isolation
//! - `load_testing`: Performance testing infrastructure
//!
//! # State Management
//!
//! The [`AppState`] struct contains shared resources:
//! - `zfs_manager`: ZFS operations manager
//! - `communication_counters`: live counters for WebSocket / SSE snapshot traffic (zeros until observed)
//! - `event_log`: optional operational events recorded by producers (empty until populated)
//! - Configuration and connection pools (as needed)
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_api::routes::create_router;
//!
//! #[tokio::main]
//! async fn main() {
//!     let router = create_router();
//!     let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
//!         .await
//!         .expect("bind listener");
//!     axum::serve(listener, router).await.expect("serve");
//! }
//! ```
//!
//! # Feature Flags
//!
//! - `dev-stubs`: Use stub implementations for development/testing
//! - `streaming-rpc`: Enable bidirectional RPC streaming (optional)

pub mod handlers;
pub mod register;
pub mod state;
pub mod streaming;

pub use register::{create_router, create_router_with_initialized_state, create_router_with_state};
pub use state::{AppState, CommunicationCounters, ZfsManager};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert!(state.get_zfs_manager().is_some());
    }

    #[test]
    fn test_app_state_new() {
        let state = AppState::new();
        assert!(state.get_zfs_manager().is_some());
    }

    #[test]
    fn test_app_state_without_streaming() {
        let state = AppState::without_streaming();
        assert!(state.get_zfs_manager().is_some());
    }

    #[test]
    fn test_app_state_with_zfs_manager() {
        let state = AppState::new().with_zfs_manager();
        assert!(state.get_zfs_manager().is_some());
    }

    #[test]
    fn test_create_router_returns_router() {
        let router = create_router();
        let _ = router;
    }

    #[test]
    fn test_create_router_with_state_returns_router() {
        let router = create_router_with_state();
        let _ = router;
    }

    #[cfg(feature = "streaming-rpc")]
    #[tokio::test]
    async fn websocket_ping_returns_pong() {
        let state = AppState::new();
        let msg = serde_json::json!({"type": "ping"});
        let out = streaming::handle_websocket_message(msg, &state).await;
        assert!(out.contains("pong"));
    }

    #[cfg(feature = "streaming-rpc")]
    #[tokio::test]
    async fn websocket_unknown_type_is_error() {
        let state = AppState::new();
        let msg = serde_json::json!({"type": "not_real_type_xyz"});
        let out = streaming::handle_websocket_message(msg, &state).await;
        assert!(out.contains("error"));
    }

    #[cfg(feature = "streaming-rpc")]
    #[tokio::test]
    async fn websocket_subscribe_includes_channel() {
        let state = AppState::new();
        let msg = serde_json::json!({"type": "subscribe", "channel": "metrics"});
        let out = streaming::handle_websocket_message(msg, &state).await;
        assert!(out.contains("metrics"));
    }
}
