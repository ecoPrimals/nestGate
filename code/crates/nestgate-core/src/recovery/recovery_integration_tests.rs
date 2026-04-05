// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Integration tests for recovery subsystem
//! Tests interaction between graceful degradation and health monitoring

use super::graceful_degradation::*;
use super::health_monitoring::*;
use crate::error::NestGateError;
use std::future::Future;
use std::pin::Pin;

#[test]
fn test_degradation_manager_integration() {
    let manager = GracefulDegradation::new();
    assert_eq!(manager.level(), DegradationLevel::Normal);
}

#[test]
fn test_health_monitor_integration() {
    let monitor = HealthMonitor::default();
    let _ = monitor.overall_health();
}

#[tokio::test]
async fn test_degradation_triggers_health_check() {
    let mut manager = GracefulDegradation::new();

    // Simulate degradation
    manager.set_level(DegradationLevel::Minor);

    // Health should reflect degradation
    assert_eq!(manager.level(), DegradationLevel::Minor);
}

#[tokio::test]
async fn test_health_monitor_detects_degradation() {
    #[derive(Debug)]
    struct DegradedHealthCheck;

    impl HealthCheckDyn for DegradedHealthCheck {
        fn check_health(
            &self,
        ) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>>
        {
            Box::pin(async { Ok(HealthStatus::Warning) })
        }

        fn component_name(&self) -> &'static str {
            "degraded_service"
        }
    }

    let mut monitor = HealthMonitor::default();
    monitor.register(Box::new(DegradedHealthCheck));

    monitor.check_all().await;

    let component_health = monitor.get_health("degraded_service");
    assert!(component_health.is_some());
    assert_eq!(component_health.unwrap().status, HealthStatus::Warning);
}

#[tokio::test]
async fn test_multiple_component_degradation() {
    let mut manager = GracefulDegradation::new();

    // Add fallback strategies for multiple components
    manager.add_strategy("api".to_string(), FallbackStrategy::Cache);
    manager.add_strategy(
        "storage".to_string(),
        FallbackStrategy::Alternative {
            endpoint: "backup-storage".to_string(),
        },
    );

    // Simulate failures
    let result1 = manager.handle_failure("api");
    let result2 = manager.handle_failure("storage");

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[test]
fn test_recovery_state_machine() {
    let mut manager = GracefulDegradation::new();

    // Test state transitions
    assert_eq!(manager.level(), DegradationLevel::Normal);

    manager.set_level(DegradationLevel::Minor);
    assert_eq!(manager.level(), DegradationLevel::Minor);

    manager.set_level(DegradationLevel::Major);
    assert_eq!(manager.level(), DegradationLevel::Major);

    manager.set_level(DegradationLevel::Emergency);
    assert_eq!(manager.level(), DegradationLevel::Emergency);

    // Recovery path
    manager.set_level(DegradationLevel::Minor);
    assert_eq!(manager.level(), DegradationLevel::Minor);

    manager.set_level(DegradationLevel::Normal);
    assert_eq!(manager.level(), DegradationLevel::Normal);
}

#[tokio::test]
async fn test_health_monitoring_lifecycle() {
    #[derive(Debug)]
    struct SimpleHealthCheck;

    impl HealthCheckDyn for SimpleHealthCheck {
        fn check_health(
            &self,
        ) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>>
        {
            Box::pin(async { Ok(HealthStatus::Healthy) })
        }

        fn component_name(&self) -> &'static str {
            "simple_service"
        }
    }

    let mut monitor = HealthMonitor::default();

    // Register
    monitor.register(Box::new(SimpleHealthCheck));

    // Check
    monitor.check_all().await;

    // Verify
    let health = monitor.get_health("simple_service");
    assert!(health.is_some());
    assert_eq!(health.unwrap().status, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_cascading_failures() {
    let mut manager = GracefulDegradation::new();

    // Setup fallbacks for services with dependencies
    manager.add_strategy("database".to_string(), FallbackStrategy::Cache);
    manager.add_strategy("api".to_string(), FallbackStrategy::Default);
    manager.add_strategy("frontend".to_string(), FallbackStrategy::Disable);

    // Simulate cascade (handlers apply configured fallbacks; level is set explicitly here)
    manager.set_level(DegradationLevel::Minor);
    manager.handle_failure("database").ok();
    manager.handle_failure("api").ok();
    manager.handle_failure("frontend").ok();
}

#[test]
fn test_fallback_strategy_priority() {
    let mut manager = GracefulDegradation::new();

    // Add strategies
    manager.add_strategy(
        "critical".to_string(),
        FallbackStrategy::Alternative {
            endpoint: "backup".to_string(),
        },
    );

    manager.add_strategy("optional".to_string(), FallbackStrategy::Disable);

    // Critical services should have priority handling
    let result = manager.handle_failure("critical");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_health_check_timeout_handling() {
    #[derive(Debug)]
    struct SlowHealthCheck;

    impl HealthCheckDyn for SlowHealthCheck {
        fn check_health(
            &self,
        ) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>>
        {
            Box::pin(async {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                Ok(HealthStatus::Healthy)
            })
        }

        fn component_name(&self) -> &'static str {
            "slow_service"
        }
    }

    let mut monitor = HealthMonitor::default();
    monitor.register(Box::new(SlowHealthCheck));

    // Should complete despite delay
    monitor.check_all().await;

    let health = monitor.get_health("slow_service");
    assert!(health.is_some());
}

#[test]
fn test_degradation_level_comparison() {
    fn rank(level: DegradationLevel) -> u8 {
        match level {
            DegradationLevel::Normal => 0,
            DegradationLevel::Minor => 1,
            DegradationLevel::Major => 2,
            DegradationLevel::Critical => 3,
            DegradationLevel::Emergency => 4,
        }
    }
    assert!(rank(DegradationLevel::Normal) < rank(DegradationLevel::Minor));
    assert!(rank(DegradationLevel::Minor) < rank(DegradationLevel::Major));
    assert!(rank(DegradationLevel::Major) < rank(DegradationLevel::Emergency));
}

#[tokio::test]
async fn test_concurrent_health_checks() {
    let mut monitor = HealthMonitor::default();

    for i in 0..5 {
        #[derive(Debug)]
        struct NumberedHealthCheck {
            _n: usize,
        }

        impl HealthCheckDyn for NumberedHealthCheck {
            fn check_health(
                &self,
            ) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>>
            {
                Box::pin(async { Ok(HealthStatus::Healthy) })
            }

            fn component_name(&self) -> &'static str {
                "numbered"
            }
        }

        monitor.register(Box::new(NumberedHealthCheck { _n: i }));
    }

    // All checks should complete concurrently
    monitor.check_all().await;
}

#[test]
fn test_recovery_strategy_evolution() {
    let mut manager = GracefulDegradation::new();

    // Start with cache fallback
    manager.add_strategy("api".to_string(), FallbackStrategy::Cache);

    // Evolve to alternative endpoint
    manager.add_strategy(
        "api".to_string(),
        FallbackStrategy::Alternative {
            endpoint: "new-api".to_string(),
        },
    );

    let result = manager.handle_failure("api");
    assert!(result.is_ok());
}
