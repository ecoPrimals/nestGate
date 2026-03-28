// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **UNIFIED RPC SYSTEM**
//!
//! Comprehensive Remote Procedure Call system providing unified communication
//! across all `NestGate` services with support for multiple protocols, load balancing,
//! health monitoring, and fail-safe mechanisms.

/// Bidirectional streaming RPC communication support
pub mod bidirectional_streams;
/// RPC configuration and settings management
pub mod config;
/// JSON-RPC service implementation for HTTP-based communication
pub mod json_rpc_service;
/// RPC connection management and orchestration
pub mod manager;
/// RPC routing and request dispatching
pub mod rpc_router;
/// tarpc service implementation for high-performance binary RPC
pub mod tarpc_service;
/// Core RPC types, errors, and data structures
pub mod types;

// Re-export all public items
pub use bidirectional_streams::BidirectionalStreamManager;
pub use config::*;
pub use json_rpc_service::JsonRpcService;
pub use manager::*;
pub use rpc_router::UnifiedRpcRouter;
pub use tarpc_service::TarpcRpcService;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rpc_manager_creation() {
        let manager = UnifiedRpcManager::new();
        // Test that manager was created successfully
        assert!(!manager.to_string().is_empty());
    }
    #[tokio::test]
    async fn test_default_config() {
        let config = NestGateRpcConfig::default();
        assert!(config.security.enable_tls);
        assert_eq!(config.load_balancing.strategy, "round_robin");
    }
}
