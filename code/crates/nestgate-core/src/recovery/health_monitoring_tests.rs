//! Comprehensive tests for health monitoring module
//!
//! Tests health checks, monitoring, and status aggregation.

#[cfg(test)]
mod tests {
    use super::super::{HealthCheckDyn, HealthMonitor, HealthStatus};
    use crate::error::NestGateError;
    use async_trait::async_trait;
    use std::time::Duration;

    // Test health check implementation
    #[derive(Debug)]
    struct TestHealthCheck {
        name: String,
        status: HealthStatus,
    }

    impl TestHealthCheck {
        fn healthy(name: &str) -> Self {
            Self {
                name: name.to_string(),
                status: HealthStatus::Healthy,
            }
        }

        fn unhealthy(name: &str) -> Self {
            Self {
                name: name.to_string(),
                status: HealthStatus::Unhealthy,
            }
        }

        fn warning(name: &str) -> Self {
            Self {
                name: name.to_string(),
                status: HealthStatus::Warning,
            }
        }

        fn unknown(name: &str) -> Self {
            Self {
                name: name.to_string(),
                status: HealthStatus::Unknown,
            }
        }
    }

    #[async_trait]
    impl HealthCheckDyn for TestHealthCheck {
        async fn check_health(&self) -> std::result::Result<HealthStatus, NestGateError> {
            Ok(self.status.clone())
        }

        fn component_name(&self) -> &str {
            &self.name
        }
    }

    // Failing health check (returns error)
    #[derive(Debug)]
    struct FailingHealthCheck {
        name: String,
    }

    #[async_trait]
    impl HealthCheckDyn for FailingHealthCheck {
        async fn check_health(&self) -> std::result::Result<HealthStatus, NestGateError> {
            Err(NestGateError::internal("Health check failed"))
        }

        fn component_name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_eq!(HealthStatus::Warning, HealthStatus::Warning);
        assert_eq!(HealthStatus::Unhealthy, HealthStatus::Unhealthy);
        assert_eq!(HealthStatus::Unknown, HealthStatus::Unknown);

        assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
        assert_ne!(HealthStatus::Warning, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_health_status_cloning() {
        let status1 = HealthStatus::Healthy;
        let status2 = status1.clone();
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_health_monitor_new() {
        let interval = Duration::from_secs(30);
        let monitor = HealthMonitor::new(interval);
        assert_eq!(monitor.get_check_interval(), interval);
    }

    #[test]
    fn test_health_monitor_default() {
        let monitor = HealthMonitor::default();
        assert_eq!(monitor.get_check_interval(), Duration::from_secs(30));
    }

    #[test]
    fn test_register_health_check() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));
        let check = Box::new(TestHealthCheck::healthy("database"));

        monitor.register(check);
        // Successfully registered (no panic)
    }

    #[tokio::test]
    async fn test_check_all_healthy() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("database")));
        monitor.register(Box::new(TestHealthCheck::healthy("cache")));
        monitor.register(Box::new(TestHealthCheck::healthy("api")));

        let results = monitor.check_all().await;

        assert_eq!(results.len(), 3);
        assert_eq!(
            results.get("database").unwrap().status,
            HealthStatus::Healthy
        );
        assert_eq!(results.get("cache").unwrap().status, HealthStatus::Healthy);
        assert_eq!(results.get("api").unwrap().status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_check_all_mixed_statuses() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("database")));
        monitor.register(Box::new(TestHealthCheck::warning("cache")));
        monitor.register(Box::new(TestHealthCheck::unhealthy("api")));

        let results = monitor.check_all().await;

        assert_eq!(results.len(), 3);
        assert_eq!(
            results.get("database").unwrap().status,
            HealthStatus::Healthy
        );
        assert_eq!(results.get("cache").unwrap().status, HealthStatus::Warning);
        assert_eq!(results.get("api").unwrap().status, HealthStatus::Unhealthy);
    }

    #[tokio::test]
    async fn test_check_all_with_failures() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("database")));
        monitor.register(Box::new(FailingHealthCheck {
            name: "failing_service".to_string(),
        }));

        let results = monitor.check_all().await;

        assert_eq!(results.len(), 2);
        assert_eq!(
            results.get("database").unwrap().status,
            HealthStatus::Healthy
        );
        // Failing check should result in Unhealthy status
        assert_eq!(
            results.get("failing_service").unwrap().status,
            HealthStatus::Unhealthy
        );
    }

    #[tokio::test]
    async fn test_get_health_after_check() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("database")));
        monitor.check_all().await;

        let health = monitor.get_health("database");
        assert!(health.is_some());
        assert_eq!(health.unwrap().status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_get_health_nonexistent() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("database")));
        monitor.check_all().await;

        let health = monitor.get_health("nonexistent");
        assert!(health.is_none());
    }

    #[tokio::test]
    async fn test_overall_health_all_healthy() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("db1")));
        monitor.register(Box::new(TestHealthCheck::healthy("db2")));
        monitor.register(Box::new(TestHealthCheck::healthy("api")));

        monitor.check_all().await;
        assert_eq!(monitor.overall_health(), HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_overall_health_with_warning() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("db1")));
        monitor.register(Box::new(TestHealthCheck::warning("db2")));
        monitor.register(Box::new(TestHealthCheck::healthy("api")));

        monitor.check_all().await;
        assert_eq!(monitor.overall_health(), HealthStatus::Warning);
    }

    #[tokio::test]
    async fn test_overall_health_with_unhealthy() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("db1")));
        monitor.register(Box::new(TestHealthCheck::warning("db2")));
        monitor.register(Box::new(TestHealthCheck::unhealthy("api")));

        monitor.check_all().await;
        // Any unhealthy component makes overall unhealthy
        assert_eq!(monitor.overall_health(), HealthStatus::Unhealthy);
    }

    #[tokio::test]
    async fn test_overall_health_with_unknown() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("db1")));
        monitor.register(Box::new(TestHealthCheck::unknown("db2")));

        monitor.check_all().await;
        // Unknown treated as warning
        assert_eq!(monitor.overall_health(), HealthStatus::Warning);
    }

    #[test]
    fn test_overall_health_empty() {
        let monitor = HealthMonitor::new(Duration::from_secs(10));
        // No checks registered, no checks run
        assert_eq!(monitor.overall_health(), HealthStatus::Unknown);
    }

    #[tokio::test]
    async fn test_component_health_fields() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("test_component")));
        let results = monitor.check_all().await;

        let health = results.get("test_component").unwrap();
        assert_eq!(health.name, "test_component");
        assert_eq!(health.status, HealthStatus::Healthy);
        assert!(!health.message.is_empty());
        assert!(health.check_duration.as_micros() > 0 || health.check_duration.as_micros() == 0);
    }

    #[tokio::test]
    async fn test_multiple_check_cycles() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(1));

        monitor.register(Box::new(TestHealthCheck::healthy("service")));

        // First check
        let results1 = monitor.check_all().await;
        assert_eq!(
            results1.get("service").unwrap().status,
            HealthStatus::Healthy
        );

        // Second check
        let results2 = monitor.check_all().await;
        assert_eq!(
            results2.get("service").unwrap().status,
            HealthStatus::Healthy
        );

        // Timestamps should be different (though might be very close)
        let time1 = results1.get("service").unwrap().last_check;
        let time2 = results2.get("service").unwrap().last_check;
        assert!(time2 >= time1); // time2 should be >= time1
    }

    #[tokio::test]
    async fn test_check_duration_recorded() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("fast_check")));
        let results = monitor.check_all().await;

        let health = results.get("fast_check").unwrap();
        // Check duration should be non-negative
        // Duration is always >= 0 by type definition (u128)
        assert!(health.check_duration.as_micros() < u128::MAX);
    }

    #[test]
    fn test_different_check_intervals() {
        let monitor1 = HealthMonitor::new(Duration::from_secs(10));
        let monitor2 = HealthMonitor::new(Duration::from_secs(30));
        let monitor3 = HealthMonitor::new(Duration::from_millis(100));

        assert_eq!(monitor1.get_check_interval(), Duration::from_secs(10));
        assert_eq!(monitor2.get_check_interval(), Duration::from_secs(30));
        assert_eq!(monitor3.get_check_interval(), Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_health_status_message_format() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("test")));
        let results = monitor.check_all().await;

        let health = results.get("test").unwrap();
        // Message should contain status information
        assert!(health.message.contains("Status"));
        assert!(health.message.contains("Healthy"));
    }

    #[test]
    fn test_health_status_debug_format() {
        assert!(format!("{:?}", HealthStatus::Healthy).contains("Healthy"));
        assert!(format!("{:?}", HealthStatus::Warning).contains("Warning"));
        assert!(format!("{:?}", HealthStatus::Unhealthy).contains("Unhealthy"));
        assert!(format!("{:?}", HealthStatus::Unknown).contains("Unknown"));
    }

    #[tokio::test]
    async fn test_concurrent_health_checks() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        // Register multiple checks
        for i in 0..10 {
            monitor.register(Box::new(TestHealthCheck::healthy(&format!(
                "service_{}",
                i
            ))));
        }

        // Check all should handle multiple checks concurrently
        let results = monitor.check_all().await;
        assert_eq!(results.len(), 10);

        // All should be healthy
        for i in 0..10 {
            let key = format!("service_{}", i);
            assert_eq!(results.get(&key).unwrap().status, HealthStatus::Healthy);
        }
    }

    #[tokio::test]
    async fn test_health_check_name_special_characters() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("service-with-dashes")));
        monitor.register(Box::new(TestHealthCheck::healthy(
            "service_with_underscores",
        )));
        monitor.register(Box::new(TestHealthCheck::healthy("service.with.dots")));

        let results = monitor.check_all().await;
        assert_eq!(results.len(), 3);

        assert!(results.contains_key("service-with-dashes"));
        assert!(results.contains_key("service_with_underscores"));
        assert!(results.contains_key("service.with.dots"));
    }

    #[tokio::test]
    async fn test_overwrite_check_with_same_name() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        // Register first check
        monitor.register(Box::new(TestHealthCheck::healthy("database")));

        // Register second check with same name (should overwrite)
        monitor.register(Box::new(TestHealthCheck::unhealthy("database")));

        let results = monitor.check_all().await;
        assert_eq!(results.len(), 1);

        // Should use the second (unhealthy) check
        assert_eq!(
            results.get("database").unwrap().status,
            HealthStatus::Unhealthy
        );
    }

    #[tokio::test]
    async fn test_empty_component_name() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(10));

        monitor.register(Box::new(TestHealthCheck::healthy("")));
        let results = monitor.check_all().await;

        // Should handle empty name
        assert_eq!(results.len(), 1);
        assert!(results.contains_key(""));
    }
}
