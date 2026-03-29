// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for health checks module
//! Added: November 21, 2025 - Observability Testing Sprint
//!
//! Target: Complete coverage of health check functionality

#[cfg(test)]
mod health_checks_tests {
    use super::super::health_checks::*;
    use nestgate_types::Result;
    use std::collections::HashMap;
    use std::time::SystemTime;

    // Mock health check provider for testing
    struct MockHealthProvider {
        name: String,
        status: HealthStatus,
        should_fail: bool,
    }

    impl HealthCheckProvider for MockHealthProvider {
        /// Check Health
        fn check_health(&self) -> Result<ComponentHealth> {
            if self.should_fail {
                return Err(nestgate_types::error::NestGateError::system(
                    "Mock health check failed",
                    "health_check",
                ));
            }

            Ok(ComponentHealth {
                status: self.status.clone(),
                message: format!(
                    "{} is {}",
                    self.name,
                    match self.status {
                        HealthStatus::Healthy => "healthy",
                        HealthStatus::Warning => "warning",
                        HealthStatus::Unhealthy => "unhealthy",
                        HealthStatus::Unknown => "unknown",
                    }
                ),
                last_success: Some(SystemTime::now()),
                last_failure: None,
                metadata: HashMap::new(),
            })
        }

        /// Component Name
        fn component_name(&self) -> &str {
            &self.name
        }
    }

    // ==================== Health Checker Creation Tests ====================

    #[tokio::test]
    async fn test_health_checker_creation() {
        let checker = HealthChecker::new();
        let health = checker.run_health_checks();
        assert!(health.is_ok());
    }

    #[tokio::test]
    async fn test_health_checker_no_providers() {
        let checker = HealthChecker::new();
        let health = checker.run_health_checks().unwrap();
        assert_eq!(health.overall_status, HealthStatus::Healthy);
        assert_eq!(health.components.len(), 0);
    }

    // ==================== Provider Registration Tests ====================

    #[tokio::test]
    async fn test_register_single_provider() {
        let checker = HealthChecker::new();
        let provider = Box::new(MockHealthProvider {
            name: "test_service".to_string(),
            status: HealthStatus::Healthy,
            should_fail: false,
        });

        checker.register_provider("test_service".to_string(), provider);

        let health = checker.run_health_checks().unwrap();
        assert!(health.components.contains_key("test_service"));
    }

    #[tokio::test]
    async fn test_register_multiple_providers() {
        let checker = HealthChecker::new();

        for i in 0..5 {
            let provider = Box::new(MockHealthProvider {
                name: format!("service_{i}"),
                status: HealthStatus::Healthy,
                should_fail: false,
            });
            checker.register_provider(format!("service_{i}"), provider);
        }

        let health = checker.run_health_checks().unwrap();
        assert_eq!(health.components.len(), 5);
    }

    #[tokio::test]
    async fn test_register_duplicate_provider_name() {
        let checker = HealthChecker::new();

        let provider1 = Box::new(MockHealthProvider {
            name: "duplicate".to_string(),
            status: HealthStatus::Healthy,
            should_fail: false,
        });

        let provider2 = Box::new(MockHealthProvider {
            name: "duplicate".to_string(),
            status: HealthStatus::Warning,
            should_fail: false,
        });

        checker.register_provider("duplicate".to_string(), provider1);
        checker.register_provider("duplicate".to_string(), provider2);

        let health = checker.run_health_checks().unwrap();
        // Should overwrite, so only one entry
        assert_eq!(health.components.len(), 1);
    }

    // ==================== Health Status Tests ====================

    #[tokio::test]
    async fn test_all_healthy_components() {
        let checker = HealthChecker::new();

        for i in 0..3 {
            let provider = Box::new(MockHealthProvider {
                name: format!("healthy_{i}"),
                status: HealthStatus::Healthy,
                should_fail: false,
            });
            checker.register_provider(format!("healthy_{i}"), provider);
        }

        let health = checker.run_health_checks().unwrap();
        assert_eq!(health.overall_status, HealthStatus::Healthy);
        assert!((health.health_score - 1.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_mixed_health_statuses() {
        let checker = HealthChecker::new();

        checker.register_provider(
            "healthy".to_string(),
            Box::new(MockHealthProvider {
                name: "healthy".to_string(),
                status: HealthStatus::Healthy,
                should_fail: false,
            }),
        );

        checker.register_provider(
            "warning".to_string(),
            Box::new(MockHealthProvider {
                name: "warning".to_string(),
                status: HealthStatus::Warning,
                should_fail: false,
            }),
        );

        let health = checker.run_health_checks().unwrap();
        assert_eq!(health.overall_status, HealthStatus::Warning);
        assert!(health.health_score < 1.0);
        assert!(health.health_score > 0.0);
    }

    #[tokio::test]
    async fn test_unhealthy_component() {
        let checker = HealthChecker::new();

        checker.register_provider(
            "unhealthy".to_string(),
            Box::new(MockHealthProvider {
                name: "unhealthy".to_string(),
                status: HealthStatus::Unhealthy,
                should_fail: false,
            }),
        );

        let health = checker.run_health_checks().unwrap();
        assert_eq!(health.overall_status, HealthStatus::Unhealthy);
    }

    #[tokio::test]
    async fn test_unknown_status_component() {
        let checker = HealthChecker::new();

        checker.register_provider(
            "unknown".to_string(),
            Box::new(MockHealthProvider {
                name: "unknown".to_string(),
                status: HealthStatus::Unknown,
                should_fail: false,
            }),
        );

        let health = checker.run_health_checks().unwrap();
        assert!(health.components.get("unknown").unwrap().status == HealthStatus::Unknown);
    }

    // ==================== Health Score Calculation Tests ====================

    #[tokio::test]
    async fn test_health_score_all_healthy() {
        let checker = HealthChecker::new();

        for i in 0..10 {
            checker.register_provider(
                format!("service_{i}"),
                Box::new(MockHealthProvider {
                    name: format!("service_{i}"),
                    status: HealthStatus::Healthy,
                    should_fail: false,
                }),
            );
        }

        let health = checker.run_health_checks().unwrap();
        assert!((health.health_score - 1.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_health_score_all_unhealthy() {
        let checker = HealthChecker::new();

        for i in 0..10 {
            checker.register_provider(
                format!("service_{i}"),
                Box::new(MockHealthProvider {
                    name: format!("service_{i}"),
                    status: HealthStatus::Unhealthy,
                    should_fail: false,
                }),
            );
        }

        let health = checker.run_health_checks().unwrap();
        assert!(health.health_score < 0.1);
    }

    // ==================== Component Health Tests ====================

    #[tokio::test]
    async fn test_component_health_structure() {
        let health = ComponentHealth {
            status: HealthStatus::Healthy,
            message: "All systems operational".to_string(),
            last_success: Some(SystemTime::now()),
            last_failure: None,
            metadata: HashMap::new(),
        };

        assert_eq!(health.status, HealthStatus::Healthy);
        assert_eq!(health.message, "All systems operational");
        assert!(health.last_success.is_some());
        assert!(health.last_failure.is_none());
    }

    #[tokio::test]
    async fn test_component_health_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("uptime".to_string(), "3600s".to_string());

        let health = ComponentHealth {
            status: HealthStatus::Healthy,
            message: "Service running".to_string(),
            last_success: Some(SystemTime::now()),
            last_failure: None,
            metadata: metadata.clone(),
        };

        assert_eq!(health.metadata.len(), 2);
        assert_eq!(health.metadata.get("version"), Some(&"1.0.0".to_string()));
    }

    // ==================== Error Handling Tests ====================

    #[tokio::test]
    async fn test_provider_failure_handling() {
        let checker = HealthChecker::new();

        checker.register_provider(
            "failing".to_string(),
            Box::new(MockHealthProvider {
                name: "failing".to_string(),
                status: HealthStatus::Healthy,
                should_fail: true,
            }),
        );

        let health = checker.run_health_checks().unwrap();
        // Failing provider should result in Unhealthy status
        if let Some(component) = health.components.get("failing") {
            assert_eq!(component.status, HealthStatus::Unhealthy);
            assert!(component.message.contains("Health check failed"));
            assert!(component.last_failure.is_some());
        } else {
            panic!("Expected failing component to be present");
        }
    }

    // ==================== Concurrent Health Checks ====================

    #[tokio::test]
    async fn test_concurrent_health_checks() {
        let checker = std::sync::Arc::new(HealthChecker::new());

        for i in 0..5 {
            checker.register_provider(
                format!("concurrent_{i}"),
                Box::new(MockHealthProvider {
                    name: format!("concurrent_{i}"),
                    status: HealthStatus::Healthy,
                    should_fail: false,
                }),
            );
        }

        let mut handles = vec![];
        for _ in 0..10 {
            let checker_clone = checker.clone();
            let handle = tokio::spawn(async move { checker_clone.run_health_checks() });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }
    }

    // ==================== System Health Tests ====================

    #[tokio::test]
    async fn test_system_health_timestamp() {
        let checker = HealthChecker::new();
        let before = SystemTime::now();
        let health = checker.run_health_checks().unwrap();
        let after = SystemTime::now();

        assert!(health.timestamp >= before);
        assert!(health.timestamp <= after);
    }

    #[tokio::test]
    async fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_eq!(HealthStatus::Warning, HealthStatus::Warning);
        assert_eq!(HealthStatus::Unhealthy, HealthStatus::Unhealthy);
        assert_eq!(HealthStatus::Unknown, HealthStatus::Unknown);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Warning);
    }
}
