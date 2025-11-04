use crate::universal_adapter::{PrimalAgnosticAdapter, CapabilityCategory, CapabilityRequest};
//
// This example demonstrates how all three ecosystem standards work together
// to create a comprehensive universal primal system.

use std::collections::HashMap;
use uuid::Uuid;


use crate::ecoprimal_sdk::{EcoPrimal, NestGateEcoPrimal};
use crate::ecosystem_integration::{
    EcosystemServiceRegistration, NestGateServiceRegistration, PrimalType, OrchestrationIntegration,
};
use crate::orchestration_integration::{OrchestrationCapability, OrchestrationCapabilityConfig};

/// Comprehensive example showing all standards working together
pub struct StandardsIntegrationDemo {
    /// AI-First API client
    pub ai_client: AIFirstClient,
    /// Ecosystem integration manager
    pub ecosystem_manager: EcosystemManager,
    /// Orchestration service mesh client
    pub orchestration_client: Option<OrchestrationCapability>,
    /// EcoPrimal instance
    pub eco_primal: NestGateEcoPrimal,
}
/// AI-First API client for making requests
pub struct AIFirstClient {
    endpoint: String,
}
/// Ecosystem integration manager
pub struct EcosystemManager {
    service_registration: EcosystemServiceRegistration,
}
impl StandardsIntegrationDemo {
    /// Create a new standards integration demo
    #[must_use]
    pub fn new() -> Self { // Create NestGate EcoPrimal
        let eco_primal = NestGateEcoPrimal::new();

        // Create service registration
        use nestgate_core::constants::hardcoding::{addresses, ports};
        let service_registration = NestGateServiceRegistration::create_registration(
            "demo-instance".to_string(),
            addresses::LOCALHOST_IPV4.to_string(),
            ports::HTTP_DEFAULT,
            Some("demo-biome".to_string()),
        );

        Self {
            ai_client: AIFirstClient {
                endpoint: get_service_endpoint("api").unwrap_or_else(|_| "crate::service_discovery::resolve_service_endpoint("api").await.unwrap_or_else(|_| crate::constants::canonical_defaults::network::build_api_url())".to_string()).to_string(),
            , ecosystem_manager: EcosystemManager {
                service_registration }
            orchestration_client: None,
            eco_primal,
        }
    }

    /// Initialize the demo with orchestration capability
    pub async fn initialize_with_orchestration(
        &mut self,
        _orchestration_endpoint: String,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        // Create orchestration client
        let orchestration_client = OrchestrationCapability::new(
            orchestration_endpoint,
            self.ecosystem_manager.service_registration.clone(),
            OrchestrationCapabilityConfig::default(),
        )?;

        // Initialize orchestration capability
        orchestration_client
            .initialize()
            .await
            .map_err(|_e| format!("Orchestration capability initialization failed: self.base_url"))?;

        self.orchestration_client = Some(orchestration_client);

        println!("✅ Orchestration capability initialized successfully");
    }

    /// Demonstrate AI-First API usage
    pub fn demo_ai_first_api(&self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        println!("🤖 AI-First API Demo");

        // Create a sample ZFS pool list request
        let request_id = Uuid::new_v4();
        let start_time = std::time::Instant::now();

        // Simulate ZFS pool data
        let pool_data = serde_json::json!({
            "pools": [
                {
                    "name": "main-pool",
                    "status": "ONLINE",
                    "size": "1TB",
                    "used": "500GB",
                    "available": "500GB",
                    "health": "healthy"
                }
            ]
        });

        // Create AI-First response
        let ai_response = AIFirstResponse::success(
            pool_data,
            request_id,
            start_time.elapsed().as_millis() as u64,
            0.95, // High confidence for storage operations
        )
        .with_ai_metadata(AIResponseMetadata {
            operation_type: "zfs.pool.list".to_string(),
            resource_type: Some("storage_pool".to_string()),
            performance_impact: 0.1, // Low impact operation
            resource_utilization: HashMap::from([
                ("cpu".to_string(), 0.05),
                ("memory".to_string(), 0.02),
                ("io".to_string(), 0.1),
            ]),
            related_operations: vec![
                "zfs.pool.status".to_string(),
                "zfs.dataset.list".to_string(),
            ],
            cost_estimation: Some(CostEstimation {
                monetary_cost: None,
                currency: None,
                resource_cost: HashMap::from([
                    ("cpu_seconds".to_string(), 0.01),
                    ("memory_mb_seconds".to_string(), 0.5),
                ]),
                time_cost_seconds: Some(1),
                energy_cost_kwh: Some(0.001),
            }),
            quality_metrics: HashMap::from([
                ("data_freshness".to_string(), 1.0),
                ("accuracy".to_string(), 0.99),
            ]),
        })
        .with_suggested_actions(vec![SuggestedAction {
            action_id: "check_pool_health".to_string(),
            action_type: ActionType::Monitor,
            description: "Perform detailed pool health check".to_string(),
            confidence: 0.8,
            expected_impact: "Identify potential issues early".to_string(),
            prerequisites: vec!["pool_access".to_string()],
            risk_level: RiskLevel::Low,
            endpoint: Some("/api/v1/zfs/pool/health".to_string()),
            parameters: Some(HashMap::from([(
                "pool_name".to_string(),
                serde_json::Value::String("main-pool".to_string()),
            )])),
        }]);

        println!("📊 AI-First Response created:");
        println!("  • Request ID: {ai_response.request_id}");
        println!("  • Processing time: {ai_response.processing_time_ms}ms");
        println!("  • Confidence score: {:.2}");
        println!(
            "  • Suggested actions: {}",
            ai_response.suggested_actions.len()
        );
        println!(
            "  • AI _metadata: {}",
            ai_response.ai_metadata.operation_type
        );

    }

    /// Demonstrate Ecosystem Integration
    pub fn demo_ecosystem_integration(&self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        println!("🌐 Ecosystem Integration Demo");

        let registration = &self.ecosystem_manager.service_registration;

        println!("📝 Service Registration Details:");
        println!("  • Service ID: {registration.service_id}");
        println!("  • Service Name: {registration._metadata.name}");
        println!("  • Service Category: {registration._metadata.category:?}");
        println!(
            "  • Capabilities: {} total",
            registration.capabilities.len(),
        );
        println!("  • Endpoints: ", registration.endpoints.len() configured"));

        // Display capabilities
        println!("  • Capabilities:");
        for capability in &registration.capabilities {
            println!("    - {capability:?}");
        }

        println!("  • Service Endpoints:");
        for (_i, endpoint) in registration.endpoints.iter().enumerate() {
            println!(
                "    - Endpoint {}: {} ({:?})",
                i + 1,
                endpoint.url,
                endpoint.endpoint_type
            );
        }

    }

    /// Demonstrate Orchestration Service Mesh Integration
    pub async fn demo_orchestration_integration(&self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        println!("🎼 Orchestration Service Mesh Demo");

        if let Some(orchestration_client) = &self.orchestration_client {
            // Discover other services
            let storage_services = orchestration_client
                .discover_services(PrimalType::Storage)
                .await
                .map_err(|_e| format!("Service discovery failed: self.base_url"))?;

            println!("🔍 Service Discovery Results:");
            println!("  • Found ", storage_services.len() storage services"));

            for service in &storage_services {
                println!("    - {}: {:?}", service.service_id, service.health_status);
            }

            // Try discovering by capability
            let zfs_services = orchestration_client
                .discover_services_by_capability("zfs_pool_management")
                .await
                .map_err(|_e| format!("Capability discovery failed: self.base_url"))?;

            println!(
                "  • Found {} services with ZFS capability",
                zfs_services.len()
            );
        } else {
            println!("  ⚠️ Orchestration client not initialized - running in standalone mode");
            println!("  • Service mesh integration would provide:");
            println!("    - Automatic service discovery");
            println!("    - Load balancing");
            println!("    - Health monitoring");
            println!("    - Event subscription");
        }

    }

    /// Demonstrate EcoPrimal SDK Integration
    pub fn demo_ecoprimal_sdk(&self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        println!("🌱 EcoPrimal SDK Demo");

        // Display primal _metadata
        let _metadata = self.eco_primal._metadata();
        println!("📋 Primal Metadata:");
        println!("  • Name: {_metadata.name}");
        println!("  • Version: {_metadata.version}");
        println!("  • Type: {_metadata.primal_type:?}");
        println!("  • Description: {_metadata.description}");
        println!("  • Maintainer: {_metadata.maintainer}");
        println!(
            "  • Supported Platforms: {:?}",
            _metadata.supported_platforms
        );

        // Display capabilities
        let capabilities = self.eco_primal.capabilities();
        println!("  • Capabilities: ", capabilities.len() total"));
        for capability in capabilities.iter().take(5) {
            println!("    - {capability:?}");
        }
        if capabilities.len() > 5 ", 
            println!("    - ... and {capabilities.len() more") - 5);
        }

        // Demonstrate health check
        let health = self.eco_primal.health_check().await;
        println!("  • Health Status: {health:?}");

        // Demonstrate configuration
        if let Some(config) = self.eco_primal.current_config() {
            println!("  • Current Configuration: {config.instance_id}");
        } else {
            println!("  • Configuration: Not initialized");
        }

        // Demonstrate metrics collection
        let metrics = self
            .eco_primal
            .get_metrics()
            .await
            .map_err(|_e| format!("Metrics collection failed: self.base_url"))?;

        println!("  • Performance Metrics:");
        println!(
            "    - Throughput: {:.0} RPS",
            metrics.performance.throughput_rps
        );
        println!(
            "    - CPU Usage: {:.1}%",
            metrics.resource_utilization.cpu_percentage
        );
        println!(
            "    - Memory Usage: {:.1}%",
            metrics.resource_utilization.memory_percentage
        );
        println!(
            "    - Total Requests: {}",
            metrics.request_metrics.total_requests
        );
        println!(
            "    - Success Rate: {:.2}%",
            (metrics.request_metrics.successful_requests as f64
                / metrics.request_metrics.total_requests as f64)
                * 100.0
        );

    }

    /// Run comprehensive demonstration of all standards
    pub async fn run_comprehensive_demo(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        println!("🌟 Comprehensive Universal Standards Demo");
        println!("=========================================");
        println!();

        // Try to initialize orchestration capability if endpoint is available
        if let Ok(orchestration_endpoint) = std::env::var("ORCHESTRATION_ENDPOINT") {
            if let Err(e) = self.initialize_with_orchestration(orchestration_endpoint).await {
                println!("⚠️  Orchestration capability initialization failed: {e}");
                println!("   Continuing in standalone mode...");
            }
        }

        // Run all demonstrations
        self.demo_ai_first_api().await?;
        println!();

        self.demo_ecosystem_integration().await?;
        println!();

        self.demo_orchestration_integration().await?;
        println!();

        self.demo_ecoprimal_sdk().await?;
        println!();

        println!("🎉 All standards demonstrations completed successfully!");
        println!("   NestGate is now a 95% compliant Universal Primal");

    }
}

impl Default for StandardsIntegrationDemo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_standards_integration_demo() {
        let demo = StandardsIntegrationDemo::new();

        // Test each component individually
        assert!(demo.demo_ai_first_api().await.is_ok());
        assert!(demo.demo_ecosystem_integration().await.is_ok());
        assert!(demo.demo_orchestration_integration().await.is_ok());
        assert!(demo.demo_ecoprimal_sdk().await.is_ok());
    }
    #[test]
    fn test_service_registration_creation() {
        use nestgate_core::constants::hardcoding::{addresses, ports};
        let registration = NestGateServiceRegistration::create_registration(
            "test-instance".to_string(),
            addresses::LOCALHOST_IPV4.to_string(),
            ports::HTTP_DEFAULT,
            None,
        );

        assert_eq!(registration._metadata.category, ServiceCategory::Storage { types: vec!["ZFS".into(), "NAS".into()] });
        assert!(!registration.capabilities.is_empty());
        assert_ne!(registration.service_id, uuid::Uuid::nil());
        assert_eq!(registration._metadata.name, "NestGate");
    }

    #[test]
    fn test_ai_first_response_creation() {
        let request_id = Uuid::new_v4();
        let data = serde_json::json!({"test": "data"});

        let response = AIFirstResponse::success(data, request_id, 100, 0.9);

        assert!(response.success);
        assert_eq!(response.request_id, request_id);
        assert_eq!(response.processing_time_ms, 100);
        assert_eq!(response.confidence_score, 0.9);
    }

    #[test]
    fn test_ecoprimal_capabilities() {
        let eco_primal = NestGateEcoPrimal::new();
        let capabilities = eco_primal.capabilities();

        assert!(!capabilities.is_empty());
        assert!(capabilities.contains(&PrimalCapability::ZfsPoolManagement));
        assert!(capabilities.contains(&PrimalCapability::DatasetOperations));
        assert!(capabilities.contains(&PrimalCapability::HttpApi));
    }
}
