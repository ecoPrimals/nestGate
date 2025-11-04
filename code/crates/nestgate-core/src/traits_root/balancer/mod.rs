//! # Load Balancer Traits and Implementations
//! Module definitions and exports.
// Comprehensive load balancing system with multiple algorithms and health awareness.
// Split from the original load_balancer.rs for better maintainability.

// Module declarations
pub mod algorithms;
pub mod health_aware;
pub mod traits;
pub mod weighted;

// Re-export all public items for backward compatibility
pub use algorithms::{LeastConnectionsLoadBalancer, RandomLoadBalancer, RoundRobinLoadBalancer};
pub use health_aware::HealthAwareLoadBalancer;
pub use traits::{LoadBalancer, LoadBalancerStats, LoadBalancingAlgorithm, ServiceStats};
pub use weighted::{WeightedRandomLoadBalancer, WeightedRoundRobinLoadBalancer};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universal_traits::{ServiceInfo, ServiceRequest};

    fn create_test_services() -> Vec<ServiceInfo> {
        vec![
            ServiceInfo {
                id: "service1".to_string(),
                name: "service1".to_string(),
                version: "1.0.0".to_string(),
                capabilities: vec![],
                status: crate::universal_traits::orchestration::ServiceStatus::Healthy,
                last_seen: std::time::SystemTime::now(),
            },
            ServiceInfo {
                id: "service2".to_string(),
                name: "service2".to_string(),
                version: "1.0.0".to_string(),
                capabilities: vec![],
                status: crate::universal_traits::orchestration::ServiceStatus::Healthy,
                last_seen: std::time::SystemTime::now(),
            },
        ]
    }

    fn create_test_request() -> ServiceRequest {
        ServiceRequest {
            service_id: "test-service".to_string(),
            action: "GET".to_string(),
            parameters: std::collections::HashMap::new(),
            timeout_seconds: Some(30),
        }
    }

    #[tokio::test]
    async fn test_round_robin_load_balancer() {
        let lb = RoundRobinLoadBalancer::new();
        let services = create_test_services();
        let request = create_test_request();

        // Test service selection
        let service1 = lb
            .select_service(&services, &request)
            .await
            .expect("Operation failed");
        let service2 = lb
            .select_service(&services, &request)
            .await
            .expect("Operation failed");

        // Should rotate between services
        assert_ne!(service1.name, service2.name);
        assert_eq!(lb.algorithm(), "round_robin");
    }

    #[tokio::test]
    async fn test_random_load_balancer() {
        let lb = RandomLoadBalancer::new();
        let services = create_test_services();
        let request = create_test_request();

        // Test service selection
        let selected = lb
            .select_service(&services, &request)
            .await
            .expect("Operation failed");
        assert!(services.iter().any(|s| s.name == selected.name));
        assert_eq!(lb.algorithm(), "random");
    }

    #[tokio::test]
    async fn test_least_connections_load_balancer() {
        let lb = LeastConnectionsLoadBalancer::new();
        let services = create_test_services();
        let request = create_test_request();

        // Test service selection
        let selected = lb
            .select_service(&services, &request)
            .await
            .expect("Operation failed");
        assert!(services.iter().any(|s| s.name == selected.name));
        assert_eq!(lb.algorithm(), "least_connections");
    }

    #[tokio::test]
    async fn test_weighted_round_robin_load_balancer() {
        let lb = WeightedRoundRobinLoadBalancer::new();
        let services = create_test_services();
        let request = create_test_request();

        // Set weights
        let mut weights = std::collections::HashMap::new();
        weights.insert("service1".to_string(), 2.0);
        weights.insert("service2".to_string(), 1.0);
        lb.update_weights(weights).await.expect("Operation failed");

        // Test service selection
        let selected = lb
            .select_service(&services, &request)
            .await
            .expect("Operation failed");
        assert!(services.iter().any(|s| s.name == selected.name));
        assert_eq!(lb.algorithm(), "weighted_round_robin");
    }

    #[tokio::test]
    async fn test_weighted_random_load_balancer() {
        let lb = WeightedRandomLoadBalancer::new();
        let services = create_test_services();
        let request = create_test_request();

        // Set weights
        let mut weights = std::collections::HashMap::new();
        weights.insert("service1".to_string(), 2.0);
        weights.insert("service2".to_string(), 1.0);
        lb.update_weights(weights).await.expect("Operation failed");

        // Test service selection
        let selected = lb
            .select_service(&services, &request)
            .await
            .expect("Operation failed");
        assert!(services.iter().any(|s| s.name == selected.name));
        assert_eq!(lb.algorithm(), "weighted_random");
    }

    #[tokio::test]
    async fn test_health_aware_load_balancer() {
        let inner = RoundRobinLoadBalancer::new();
        let lb = HealthAwareLoadBalancer::new(inner);
        let services = create_test_services();
        let request = create_test_request();

        // Test service selection
        let selected = lb
            .select_service(&services, &request)
            .await
            .expect("Operation failed");
        assert!(services.iter().any(|s| s.name == selected.name));
        assert_eq!(lb.algorithm(), "round_robin"); // Returns inner algorithm
    }

    #[tokio::test]
    async fn test_empty_services() {
        let lb = RoundRobinLoadBalancer::new();
        let services = vec![];
        let request = create_test_request();

        // Should return error for empty services
        let result = lb.select_service(&services, &request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_balancer_stats() {
        let lb = RoundRobinLoadBalancer::new();
        let services = create_test_services();
        let request = create_test_request();

        // Initial stats should be empty
        let stats = lb.get_stats().await.expect("Operation failed");
        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.algorithm, "round_robin");

        // Select a service to update stats
        let selected = lb
            .select_service(&services, &request)
            .await
            .expect("Operation failed");
        let response = crate::universal_traits::ServiceResponse {
            success: true,
            data: None,
            error_message: None,
            execution_time_ms: 100,
        };

        // Record response
        lb.record_response(&selected, &response)
            .await
            .expect("Operation failed");

        // Stats should be updated
        let stats = lb.get_stats().await.expect("Operation failed");
        assert_eq!(stats.total_requests, 1);
    }
}
