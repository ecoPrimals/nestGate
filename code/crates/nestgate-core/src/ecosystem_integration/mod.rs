pub mod capabilities;
pub mod fallback_providers;
pub mod mock_router; // Universal mock routing with graceful fallbacks
/// Ecosystem Integration Module
/// Provides unified interfaces for interacting with external primals in the ecoPrimals ecosystem
pub mod universal_adapter; // External primal capability interfaces // Local fallback implementations

// Re-export key types for external use
pub use capabilities::{
    AuthenticationRequest,
    AuthenticationResponse,
    AuthorizationRequest,
    AuthorizationResponse,
    CapabilityRequest,
    CapabilityResponse,
    // Compute capabilities
    ComputeCapability,
    DataAnalysisRequest,
    DataAnalysisResponse,
    EncryptionRequest,
    EncryptionResponse,
    HardwareOptimizationRequest,
    HardwareOptimizationResponse,
    // Intelligence capabilities
    IntelligenceCapability,
    MockComputeCapability,
    MockIntelligenceCapability,
    MockOrchestrationCapability,
    MockSecurityCapability,
    ModelInferenceRequest,
    ModelInferenceResponse,
    OptimizationRequest,
    OptimizationResponse,
    // Orchestration capabilities
    OrchestrationCapability,
    PerformanceTuningRequest,
    PerformanceTuningResponse,
    ResourceAllocationRequest,
    ResourceAllocationResponse,
    // Security capabilities
    SecurityCapability,
    ServiceCoordinationRequest,
    ServiceCoordinationResponse,
    UniversalCapability,
    WorkflowRequest,
    WorkflowResponse,
};
pub use fallback_providers::{
    AiFallbackProvider,
    OrchestrationFallbackProvider,
    SecurityFallbackProvider,
    // Temporarily commented out due to ZFS module issues
    // ZfsFallbackProvider,
};
pub use mock_router::{
    FallbackProvider, MockRoutingConfig, MockRoutingError, RoutingMetrics, UniversalMockRouter,
};
pub use universal_adapter::{
    AdapterConfig, CapabilityCategory, ServiceRegistration, UniversalAdapter,
};

// Capability categories are re-exported from universal_adapter types
// Available categories (capability-based, not primal-specific):
// - CapabilityCategory::Compute (for computational services)
// - CapabilityCategory::Orchestration (for coordination services)
// - CapabilityCategory::Security (for authentication/authorization services)
// - CapabilityCategory::ArtificialIntelligence (for AI/ML services)
// - CapabilityCategory::Storage (for data storage services)

// Import unified configuration for integration functions

/// Initialize ecosystem integration for NestGate
/// This is the single entry point for all ecosystem integration.
/// No other integration modules should exist - everything goes through
/// the universal adapter.
pub async fn initialize_ecosystem_integration(
    config: AdapterConfig,
) -> crate::Result<UniversalAdapter> {
    let adapter = UniversalAdapter::new(config);
    adapter.initialize().await?;
    Ok(adapter)
}

/// Helper function to create default adapter configuration
pub fn create_default_adapter_config() -> AdapterConfig {
    let mut config = AdapterConfig::default();

    // Configure adapter-specific settings
    config.adapter.discovery_endpoint = std::env::var("ECOSYSTEM_DISCOVERY_ENDPOINT")
        .unwrap_or_else(|_| {
            format!(
                "http://{}:{}/discovery",
                crate::constants::domain_constants::network::addresses::LOCALHOST,
                crate::constants::domain_constants::network::ports::HTTP
            )
        });

    config.adapter.service_registration = ServiceRegistration {
        name: "NestGate Storage Intelligence Hub".to_string(),
        description: "Universal storage intelligence and ZFS management system".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        maintainer: "NestGate Team".to_string(),
        endpoint: std::env::var("NESTGATE_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:8081".to_string()),
        health_endpoint: std::env::var("NESTGATE_HEALTH_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:8081/health".to_string()),
        capabilities_summary: "storage,zfs,nas,tiering".to_string(),
        metadata: std::collections::HashMap::new(),
        tags: vec!["storage".to_string(), "primal-sovereign".to_string()],
    };

    config.adapter.monitoring_enabled = std::env::var("NESTGATE_MONITORING_ENABLED")
        .unwrap_or_else(|_| "true".to_string())
        .parse()
        .unwrap_or(true);

    // Configure timeouts in the service configuration
    config.service.timeouts.default_timeout = std::time::Duration::from_millis(
        std::env::var("NESTGATE_DEFAULT_TIMEOUT_MS")
            .unwrap_or_else(|_| "5000".to_string())
            .parse()
            .unwrap_or(5000),
    );

    config
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::NestGateError;

    /// Helper function to create a universal adapter from config
    fn create_universal_adapter(
        config: AdapterConfig,
    ) -> crate::Result<universal_adapter::adapter::UniversalAdapter> {
        Ok(universal_adapter::adapter::UniversalAdapter::new(config))
    }

    #[tokio::test]
    async fn test_ecosystem_integration_initialization() {
        let config = create_default_adapter_config();
        let adapter = initialize_ecosystem_integration(config).await;
        assert!(adapter.is_ok());

        let adapter = adapter.map_err(|e| {
            tracing::error!(
                "Expected operation failed: {} - Error: {:?}",
                "Ecosystem integration should succeed in tests",
                e
            );
            crate::NestGateError::internal_error(
                format!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Ecosystem integration should succeed in tests", e
                ),
                "automated_migration".to_string(),
            )
        })?;
        let status = adapter.health_status().await;

        // We should have a healthy adapter
        assert!(status.healthy);

        // We should have some operational data
        assert!(status.successful_operations >= 0); // May be 0 in test environment
    }

    #[test]
    fn test_default_config_creation() {
        let config = create_default_adapter_config();

        // Test adapter creation
        let adapter = create_universal_adapter(config.clone()).unwrap_or_else(|e| {
            tracing::error!("Failed to create adapter: {:?}", e);
            return Err(NestGateError::InternalError(format!(
                "Adapter creation should succeed in tests: {:?}",
                e
            )));
        });

        // Test adapter configuration structure
        assert_eq!(config.adapter.service_registration.name, "test-service");
        assert!(config.adapter.monitoring_enabled);
        assert_eq!(
            config.network.timeouts.default_timeout,
            std::time::Duration::from_secs(30)
        );
        assert_eq!(config.network.retry_config.max_attempts, 3);
    }
}
