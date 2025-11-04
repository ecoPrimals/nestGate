//! # Production Smart Service
//! Module definitions and exports.
// Real production implementation of the SmartService trait that replaces
//! the MockSmartService with genuine business logic and real service handling.

// Module declarations
pub mod config;
pub mod health;
pub mod metrics;
pub mod service;

// Re-export all public items for backward compatibility
pub use config::ProductionServiceConfig;
pub use health::{HealthMonitor, SystemResourceHealth};
pub use metrics::ProductionMetrics;
pub use service::ProductionSmartService;

// Tests module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::smart_abstractions::ServiceMetadata;

    #[tokio::test]
    async fn test_production_service_creation() {
        let metadata = ServiceMetadata {
            service_type: "test-service".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test production service".to_string()),
        };
        
        let service = ProductionSmartService::new(metadata);
        assert_eq!(service.metadata().service_type, "test-service");
    }

    #[tokio::test]
    async fn test_health_monitor() {
        let config = ProductionServiceConfig::default();
        let monitor = HealthMonitor::new(config);
        
        let health = monitor.perform_health_check().await.expect("Operation failed");
        // Health status should be available (may be healthy, degraded, or unhealthy)
        assert!(!health.message.is_empty());
    }

    #[test]
    fn test_production_metrics() {
        let mut metrics = ProductionMetrics::new();
        
        // Record some requests
        metrics.record_request(std::time::Duration::from_millis(100), true);
        metrics.record_request(std::time::Duration::from_millis(200), false);
        
        assert_eq!(metrics.total_requests, 2);
        assert_eq!(metrics.successful_requests, 1);
        assert_eq!(metrics.failed_requests, 1);
        assert_eq!(metrics.success_rate(), 50.0);
        assert_eq!(metrics.error_rate(), 50.0);
    }
} 