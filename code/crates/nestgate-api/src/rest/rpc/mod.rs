//
// Real-time bidirectional communication layer for NestGate Data API.
// Integrates both tarpc (binary RPC) and JSON RPC systems for seamless
// communication with beardog, songbird, and other primals.

pub mod bidirectional_streams;
pub mod config;
pub mod json_rpc_service;
pub mod manager;
pub mod rpc_router;
pub mod tarpc_service;
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
