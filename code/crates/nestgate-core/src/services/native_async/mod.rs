pub mod development;
pub mod production;
/// Native Async Services Module - Split for File Size Compliance
/// This module was split from native_async_final_services.rs to maintain the 2000-line limit
/// while preserving all functionality and maintaining backward compatibility
// Sub-module declarations
pub mod traits;
pub mod types;

// Re-export all public types and traits for backward compatibility
pub use traits::{
    NativeAsyncAutomationService, NativeAsyncCommunicationProvider, NativeAsyncLoadBalancer,
    NativeAsyncMCPProtocolHandler, NativeAsyncMcpService, NativeAsyncSecurityService,
    NativeAsyncUniversalServiceProvider, NativeAsyncWorkflowService,
};

pub use types::{
    CommunicationMessage, ConnectionInfo, ConnectionStatus, ExecutionStatus, LoadBalancerStats,
    MCPError, MCPMessage, MCPResponse, MCPSessionInfo, MessagePriority, NetworkAddress,
    ServiceRequest, ServiceResponse, ServiceStats, WorkflowExecution,
};

pub use production::{ProductionCommunicationProvider, ProductionLoadBalancer};

pub use development::{DevelopmentLoadBalancer, DevelopmentServiceLoadBalancer};

// Tests module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::service_discovery::types::{
        CommunicationProtocol, ServiceCapability, ServiceCategory, ServiceEndpoint, ServiceInfo,
        ServiceMetadata,
    };
    use std::collections::HashMap;
    use std::time::SystemTime;
    use uuid::Uuid;

    /// Mock service info for testing
    #[derive(Debug, Clone)]
    struct MockServiceInfo {
        id: String,
        name: String,
        version: String,
        endpoints: Vec<String>,
    }

    impl Default for MockServiceInfo {
        fn default() -> Self {
            Self {
                id: "test-service-123".to_string(),
                name: "test-service".to_string(),
                version: "1.0.0".to_string(),
                endpoints: vec!["http://localhost:8080".to_string()],
            }
        }
    }

    #[tokio::test]
    async fn test_production_load_balancer() {
        let balancer = ProductionLoadBalancer::default();

        // Test service addition
        let service = ServiceInfo {
            service_id: uuid::Uuid::new_v4(),
            metadata: ServiceMetadata {
                name: "test-service".to_string(),
                category: ServiceCategory::Network,
                version: "1.0.0".to_string(),
                description: "Test service for load balancer".to_string(),
                health_endpoint: None,
                metrics_endpoint: None,
            },
            capabilities: vec![],
            endpoints: vec![ServiceEndpoint {
                url: "http://localhost:8080".to_string(),
                protocol: CommunicationProtocol::HTTP,
                health_check: None,
            }],
            last_seen: std::time::SystemTime::now(),
        };

        let add_result = balancer.add_service(service.clone()).await;
        assert!(add_result.is_ok());

        // Test service existence
        let exists = balancer.service_exists("test-service").await;
        assert!(exists.is_ok());
        assert!(exists.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
            .into());
        }));

        // Test service retrieval
        let retrieved = balancer.get_service("test-service").await;
        assert!(retrieved.is_ok());
        assert!(retrieved
            .unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into());
            })
            .is_some());

        // Test request routing
        let request = ServiceRequest {
            service_name: "test-service".to_string(),
            data: b"test data".to_vec(),
        };

        let response = balancer.route_request(request).await;
        assert!(response.is_ok());
        assert!(
            response
                .unwrap_or_else(|e| {
                    tracing::error!("Unwrap failed: {:?}", e);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Operation failed: {:?}", e),
                    )
                    .into());
                })
                .success
        );

        println!("✅ Production load balancer test passed!");
    }

    #[tokio::test]
    async fn test_development_load_balancer() {
        let mut balancer = DevelopmentLoadBalancer::new();

        // Add a service for testing
        let service = ServiceInfo {
            id: "test_service".to_string(),
            name: "test_service".to_string(),
            category: ServiceCategory::Storage,
            capabilities: vec![ServiceCapability::Storage],
            endpoint: ServiceEndpoint {
                protocol: CommunicationProtocol::Http,
                address: "localhost".to_string(),
                port: 8080,
                path: Some("/".to_string()),
            },
            metadata: ServiceMetadata {
                version: "1.0.0".to_string(),
                tags: HashMap::new(),
                created_at: SystemTime::now(),
                updated_at: SystemTime::now(),
            },
            health_status: true,
        };

        let add_result = balancer.add_service(service).await;
        assert!(add_result.is_ok());

        // Test stats retrieval
        let stats = balancer.get_stats().await;
        assert!(stats.is_ok());
        let stats_data = stats.expect("Failed to get stats");
        assert_eq!(stats_data.algorithm, "dev_mock");

        // Test health check
        let health = balancer.health_check_all().await;
        assert!(health.is_ok());
        let health_data = health.expect("Failed to get health check");
        assert!(!health_data.is_empty());

        println!("✅ Development load balancer test passed!");
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
}
