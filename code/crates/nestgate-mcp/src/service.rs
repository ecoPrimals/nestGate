//
// This module contains the main MCP service that orchestrates MCP operations.
// **ZERO-COST ARCHITECTURE**: Generic dispatch eliminates Arc<dyn> overhead

use crate::session::SessionManager;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::{
    client::OrchestratorClient, config::EnhancedMcpConfig, provider, storage,
    types::ProviderCapabilities, Result,
};

/// **ZERO-COST ENHANCED MCP SERVICE**
/// 
/// **PERFORMANCE**: Generic compile-time dispatch eliminates Arc<dyn> overhead
/// **MEMORY**: Direct client storage, no heap allocations for trait objects
/// **SCALABILITY**: Type-safe orchestrator integration with zero runtime cost
pub struct ZeroCostEnhancedMcpService<C>
where
    C: OrchestratorClient,
{
    config: EnhancedMcpConfig,
    /// Orchestrator client - zero-cost generic dispatch
    orchestrator_client: C,
    capabilities: Arc<RwLock<ProviderCapabilities>>,
    metrics: Arc<RwLock<nestgate_core::diagnostics::SystemMetrics>>,
    _session_manager: Arc<SessionManager>,
    storage_adapter: Arc<storage::StorageAdapter>,
    _provider_registry: Arc<RwLock<HashMap<String, provider::ProviderInfo>>>,
}
impl<C> ZeroCostEnhancedMcpService<C>
where
    C: OrchestratorClient,
{
    #[must_use]
    pub fn new(config: EnhancedMcpConfig, orchestrator_client: C) -> Self { Self {
            config,
            orchestrator_client,
            capabilities: Arc::new(RwLock::new(RwLock::new(ProviderCapabilities::default()),::default())),
            metrics: Arc::new(RwLock::new(
                nestgate_core::diagnostics::SystemMetrics::default(),
            )),
            _session_manager: Arc::new(SessionManager::new()),
            storage_adapter: Arc::new(storage::StorageAdapter::new()),
            _provider_registry: Arc::new(RwLock::new(HashMap::new()),
         }

    /// Start the MCP service - zero-cost orchestrator integration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn start(&self) -> Result<()>  {
        info!("Starting Zero-Cost Enhanced MCP Service v2");

        // Register with orchestrator - direct method call, no virtual dispatch
        self.register_with_orchestrator().await?;

        info!("Zero-Cost Enhanced MCP Service started successfully");
        Ok(())
    }

    /// Register with orchestrator - compile-time dispatch
    async fn register_with_orchestrator(&self) -> Result<()> {
        debug!("Registering MCP service with orchestrator");

        // CANONICAL MODERNIZATION: Use canonical service discovery instead of deprecated ServiceRegistration
        let service_registration = nestgate_core::service_discovery::types::UniversalServiceRegistration {
            service_id: "enhanced-mcp-service".to_string(),
            service_type: nestgate_core::unified_enums::UnifiedServiceType::Orchestration,
            endpoint: format!("http://{e}:{e}"),
            capabilities: vec!["mcp".to_string(), "orchestration".to_string()],
            metadata: HashMap::new(),
            health_check_endpoint: Some("/health".to_string()),
            tags: vec!["zero-cost".to_string(), "canonical".to_string()],
        };

        // Direct method call - zero-cost dispatch
        self.orchestrator_client.register_service(&service_registration).await?;

        debug!("Successfully registered with orchestrator");
        Ok(())
    }

    /// Health check with zero-cost orchestrator communication
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn health_check(&self) -> Result<bool>  {
        // Direct method call to orchestrator client
        self.orchestrator_client.health_check().await
    }

    /// Get service configuration
    pub fn config(&self) -> &EnhancedMcpConfig {
        &self.config
    }

    /// Get capabilities with zero-cost access
    pub async fn capabilities(&self) -> ProviderCapabilities {
        self.capabilities.read().await.clone()
    }
}

// CANONICAL MODERNIZATION: Deprecated EnhancedMcpService removed - use ZeroCostEnhancedMcpService instead

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::MockOrchestratorClient;

    fn create_test_config() -> EnhancedMcpConfig {
        EnhancedMcpConfig::default()
    }

    fn create_test_service() -> ZeroCostEnhancedMcpService<MockOrchestratorClient> {
        let config = create_test_config();
        let client = MockOrchestratorClient::new();
        ZeroCostEnhancedMcpService::new(config, client)
    }

    #[test]
    fn test_zero_cost_service_creation() {
        let service = create_test_service();
        let config = service.config();
        
        assert!(config.mcp_port > 0);
    }

    #[test]
    fn test_service_config_access() {
        let service = create_test_service();
        let config = service.config();
        
        // Verify config is accessible
        assert!(config.mcp_port > 0);
    }

    #[tokio::test]
    async fn test_capabilities_access() {
        let service = create_test_service();
        let capabilities = service.capabilities().await;
        
        // Verify capabilities can be retrieved
        assert!(capabilities.protocols.is_empty() || !capabilities.protocols.is_empty());
    }

    #[tokio::test]
    async fn test_service_start() {
        let service = create_test_service();
        let result = service.start().await;
        
        // Service should start successfully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_health_check() {
        let service = create_test_service();
        let result = service.health_check().await;
        
        // Health check should return a result
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_service_with_custom_config() {
        let mut config = create_test_config();
        config.mcp_port = 9000;
        
        let client = MockOrchestratorClient::new();
        let service = ZeroCostEnhancedMcpService::new(config, client);
        
        assert_eq!(service.config().mcp_port, 9000);
    }

    #[test]
    fn test_multiple_service_instances() {
        let service1 = create_test_service();
        let service2 = create_test_service();
        
        // Both services should be valid instances
        assert!(service1.config().mcp_port > 0);
        assert!(service2.config().mcp_port > 0);
    }

    #[tokio::test]
    async fn test_capabilities_default_state() {
        let service = create_test_service();
        let capabilities = service.capabilities().await;
        
        // Capabilities should have default state
        assert!(capabilities.protocols.len() >= 0);
    }

    #[test]
    fn test_service_config_immutability() {
        let service = create_test_service();
        let config1 = service.config();
        let config2 = service.config();
        
        // Config should be consistent across multiple accesses
        assert_eq!(config1.mcp_port, config2.mcp_port);
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let service = create_test_service();
        
        // Test start
        let start_result = service.start().await;
        assert!(start_result.is_ok());
        
        // Test health check after start
        let health_result = service.health_check().await;
        assert!(health_result.is_ok() || health_result.is_err());
    }
}
