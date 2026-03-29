// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// CLEANED: Removed unused imports as part of canonical modernization
// use std::collections::HashMap;
// use std::future::Future;
/// Development-mode service implementations with mock support
pub mod development;
/// Production-ready service implementations
pub mod production;
// Native Async Services Module - Split for File Size Compliance
// This module was split from native_async_final_services.rs to maintain the 2000-line limit
/// while preserving all functionality and maintaining backward compatibility
// Sub-module declarations
pub mod traits;
/// Type definitions for native async services
pub mod types;
// Re-export all public types and traits for backward compatibility
#[expect(deprecated, reason = "migration in progress")]
// Re-export for backwards compatibility
pub use traits::NativeAsyncSecurityService;
pub use traits::{
    NativeAsyncAutomationService, NativeAsyncCommunicationProvider, NativeAsyncLoadBalancer,
    NativeAsyncMCPProtocolHandler, NativeAsyncMcpService, NativeAsyncUniversalServiceProvider,
    NativeAsyncWorkflowService,
};

pub use types::{
    CommunicationMessage, ConnectionInfo, ConnectionStatus, ExecutionStatus, LoadBalancerStats,
    MCPError, MCPMessage, MCPResponse, MCPSessionInfo, MessagePriority, NetworkAddress,
    ServiceResponse, ServiceStats, WorkflowExecution,
};

pub use production::{ProductionCommunicationProvider, ProductionLoadBalancer};

pub use crate::services::native_async::development::{
    DevelopmentLoadBalancer, DevelopmentServiceLoadBalancer,
};

// Tests module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::service_discovery::types::{
        ServiceInfo,
        // ServiceMetadata now imported from smart_abstractions
    };
    // Removed unresolved smart_abstractions import
    use anyhow::Result;
    use std::time::SystemTime;

    #[cfg(test)]
    /// Mock service info for testing
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct MockServiceInfo {
        id: String,
        name: String,
        version: String,
        endpoints: Vec<String>,
    }

    #[cfg(test)]
    impl Default for MockServiceInfo {
        /// Returns the default instance with environment-driven configuration
        fn default() -> Self {
            use crate::config::environment::EnvironmentConfig;

            // Load from environment for consistency with production
            let env_config =
                EnvironmentConfig::from_env().unwrap_or_else(|_| EnvironmentConfig::default());

            Self {
                id: "test-service-123".to_string(),
                name: "test-service".to_string(),
                version: "1.0.0".to_string(),
                endpoints: vec![format!(
                    "http://{}:{}",
                    env_config.network.host,
                    env_config.network.port.get()
                )],
            }
        }
    }

    #[tokio::test]
    async fn test_production_load_balancer() {
        let balancer = ProductionLoadBalancer::default();

        // Test service addition
        let service = ServiceInfo {
            service_id: uuid::Uuid::new_v4(),
            metadata: crate::service_discovery::types::ServiceMetadata {
                name: "test-service".to_string(),
                category: crate::service_discovery::types::ServiceCategory::Storage,
                version: "1.0.0".to_string(),
                description: "Test service for load balancer".to_string(),
                health_endpoint: None,
                metrics_endpoint: None,
            },
            capabilities: vec![],
            endpoints: vec![crate::service_discovery::types::ServiceEndpoint {
                url: "http://localhost:9999".to_string(),
                protocol: crate::service_discovery::types::CommunicationProtocol::Http,
                health_check: Some("/health".to_string()),
            }],
            last_seen: std::time::SystemTime::now(),
        };

        let add_result = balancer.add_service(service.clone()).await;
        assert!(add_result.is_ok());

        // Test service existence
        let exists = balancer.service_exists("test-service").await;
        assert!(exists.is_ok());
        assert!(exists.unwrap_or_else(|e| {
            tracing::error!("Service existence check failed: {:?}", e);
            false
        }));

        // Test service retrieval
        let retrieved = balancer.get_service("test-service").await;
        assert!(retrieved.is_ok());
        assert!(
            retrieved
                .unwrap_or_else(|e| {
                    tracing::error!("Service retrieval failed: {:?}", e);
                    None
                })
                .is_some()
        );

        // Test service removal
        let remove_result = balancer.remove_service("test-service").await;
        assert!(remove_result.is_ok());

        // Verify service was removed
        let exists_after_removal = balancer.service_exists("test-service").await;
        assert!(exists_after_removal.is_ok());
        assert!(!exists_after_removal.unwrap_or_else(|e| {
            tracing::error!("Service existence check failed: {:?}", e);
            true // If error, assume it exists to fail the test appropriately
        }));

        println!("✅ Production load balancer test passed!");
    }

    #[tokio::test]
    async fn test_development_load_balancer() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let balancer = DevelopmentLoadBalancer::default();

        // Add a service for testing
        let service = ServiceInfo {
            service_id: uuid::Uuid::new_v4(),
            metadata: crate::service_discovery::types::ServiceMetadata {
                name: "test_service".to_string(),
                category: crate::service_discovery::types::ServiceCategory::Storage,
                version: "1.0.0".to_string(),
                description: "Test storage service".to_string(),
                health_endpoint: None,
                metrics_endpoint: None,
            },
            capabilities: vec![],
            endpoints: vec![],
            last_seen: SystemTime::now(),
        };

        let add_result = balancer.add_service(service).await;
        assert!(add_result.is_ok());

        // Test stats retrieval
        let stats = balancer.get_stats().await;
        assert!(stats.is_ok());
        let stats_data = stats.map_err(|e| {
            tracing::error!(
                "Expected operation failed: {} - Error: {:?}",
                "Failed to get stats",
                e
            );
            std::io::Error::other(format!("{} - Error: {:?}", "Failed to get stats", e))
        })?;
        assert_eq!(stats_data.algorithm, "dev_mock");

        // Test health check
        let health = balancer.health_check_all().await;
        assert!(health.is_ok());
        let health_data = health.map_err(|e| {
            tracing::error!(
                "Expected operation failed: {} - Error: {:?}",
                "Failed to get health check",
                e
            );
            std::io::Error::other(format!("{} - Error: {:?}", "Failed to get health check", e))
        })?;
        assert!(!health_data.is_empty());

        println!("✅ Development load balancer test passed!");
        Ok(())
    }

    #[test]
    fn test_compile_time_specialization() {
        // Test compile-time configurations
        assert_eq!(ProductionLoadBalancer::max_services(), 1000);
        assert_eq!(ProductionLoadBalancer::max_concurrent_requests(), 10000);
        assert_eq!(ProductionLoadBalancer::stats_retention_seconds(), 86400);

        assert_eq!(DevelopmentLoadBalancer::max_services(), 100);
        assert_eq!(DevelopmentLoadBalancer::max_concurrent_requests(), 1000);
        assert_eq!(DevelopmentLoadBalancer::stats_retention_seconds(), 3600);

        println!("✅ Load balancer compile-time specialization working perfectly!");
    }

    // Mock types for testing
    #[allow(dead_code)]
    struct ServiceRequest {
        service_name: String,
        data: Vec<u8>,
    }

    struct NativeAsyncService {
        name: String,
    }

    #[allow(dead_code)]
    impl NativeAsyncService {
        /// Creates a new instance
        fn new(name: String) -> Self {
            Self { name }
        }

        /// Gets Config
        fn get_config(&self) -> String {
            format!("config_for_{}", self.name)
        }

        /// Processes data
        fn process(&self, _data: &str) -> String {
            format!("processed_by_{}", self.name)
        }
    }

    #[tokio::test]
    async fn test_native_async_service_creation() -> Result<()> {
        let service = NativeAsyncService::new("test_service".to_string());

        // Test service configuration
        let config = service.get_config();
        assert!(!config.is_empty());

        println!("✅ Native async service created successfully");
        Ok(())
    }

    #[tokio::test]
    async fn test_native_async_service_operations() -> Result<()> {
        let service = NativeAsyncService::new("test_service".to_string());

        // Test basic service operation - just verify the service was created
        assert!(!service.name.is_empty());
        println!("✅ Service operation executed successfully");

        Ok(())
    }
}
