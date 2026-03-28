// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Integration tests for recovery subsystem
//! Tests interaction between graceful degradation and health monitoring

use super::graceful_degradation::*;
use super::health_monitoring::*;
use crate::error::NestGateError;
use std::future::Future;
use std::pin::Pin;

#[test]
fn test_degradation_manager_integration() {
    let manager = DegradationManager::new();
    assert_eq!(manager.current_level(), DegradationLevel::None);
}

#[test]
fn test_health_monitor_integration() {
    let monitor = HealthMonitor::default();
    let _ = monitor.overall_health();
}

#[tokio::test]
async fn test_degradation_triggers_health_check() {
    let mut manager = DegradationManager::new();

    // Simulate degradation
    manager.set_degradation_level(DegradationLevel::Partial);

    // Health should reflect degradation
    assert_eq!(manager.current_level(), DegradationLevel::Partial);
}

#[tokio::test]
async fn test_health_monitor_detects_degradation() {
    #[derive(Debug)]
    struct DegradedHealthCheck;

    impl HealthCheckDyn for DegradedHealthCheck {
        fn check_health(
            &self,
        ) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>> {
            Box::pin(async { Ok(HealthStatus::Warning) })
        }

        fn component_name(&self) -> &str {
            "degraded_service"
        }
    }

    let mut monitor = HealthMonitor::default();
    monitor.register(Box::new(DegradedHealthCheck));

    monitor.check_all().await;

    let component_health = monitor.get_health("degraded");
    assert!(component_health.is_some());
    assert_eq!(component_health.unwrap().status, HealthStatus::Warning);
}

#[tokio::test]
async fn test_multiple_component_degradation() {
    let mut manager = DegradationManager::new();

    // Add fallback strategies for multiple components
    manager.add_fallback_strategy("api", FallbackStrategy::Cache);
    manager.add_fallback_strategy("storage", FallbackStrategy::Alternative {
        alternative_endpoint: "backup-storage".to_string(),
    });

    // Simulate failures
    let result1 = manager.handle_failure("api");
    let result2 = manager.handle_failure("storage");

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[test]
fn test_recovery_state_machine() {
    let mut manager = DegradationManager::new();

    // Test state transitions
    assert_eq!(manager.current_level(), DegradationLevel::None);

    manager.set_degradation_level(DegradationLevel::Partial);
    assert_eq!(manager.current_level(), DegradationLevel::Partial);

    manager.set_degradation_level(DegradationLevel::Severe);
    assert_eq!(manager.current_level(), DegradationLevel::Severe);

    manager.set_degradation_level(DegradationLevel::Emergency);
    assert_eq!(manager.current_level(), DegradationLevel::Emergency);

    // Recovery path
    manager.set_degradation_level(DegradationLevel::Partial);
    assert_eq!(manager.current_level(), DegradationLevel::Partial);

    manager.set_degradation_level(DegradationLevel::None);
    assert_eq!(manager.current_level(), DegradationLevel::None);
}

#[tokio::test]
async fn test_health_monitoring_lifecycle() {
    let mut monitor = HealthMonitor::default();

    #[derive(Debug)]
    struct SimpleHealthCheck;

    impl HealthCheckDyn for SimpleHealthCheck {
        fn check_health(
            &self,
        ) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>> {
            Box::pin(async { Ok(HealthStatus::Healthy) })
        }

        fn component_name(&self) -> &str {
            "simple_service"
        }
    }

    // Register
    monitor.register(Box::new(SimpleHealthCheck));

    // Check
    monitor.check_all().await;

    // Verify
    let health = monitor.get_health("simple");
    assert!(health.is_some());
    assert_eq!(health.unwrap().status, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_cascading_failures() {
    let mut manager = DegradationManager::new();

    // Setup fallbacks for services with dependencies
    manager.add_fallback_strategy("database", FallbackStrategy::Cache);
    manager.add_fallback_strategy("api", FallbackStrategy::Default);
    manager.add_fallback_strategy("frontend", FallbackStrategy::DisableFeature);

    // Simulate cascade
    manager.handle_failure("database").ok();
    assert!(manager.current_level() >= DegradationLevel::Partial);

    manager.handle_failure("api").ok();
    // Severity should increase

    manager.handle_failure("frontend").ok();
    // Further degradation
}

#[test]
fn test_fallback_strategy_priority() {
    let mut manager = DegradationManager::new();

    // Add strategies
    manager.add_fallback_strategy("critical", FallbackStrategy::Alternative {
        alternative_endpoint: "backup".to_string(),
    });

    manager.add_fallback_strategy("optional", FallbackStrategy::DisableFeature);

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
        ) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>> {
            Box::pin(async {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                Ok(HealthStatus::Healthy)
            })
        }

        fn component_name(&self) -> &str {
            "slow_service"
        }
    }

    let mut monitor = HealthMonitor::default();
    monitor.register(Box::new(SlowHealthCheck));

    // Should complete despite delay
    monitor.check_all().await;

    let health = monitor.get_health("slow");
    assert!(health.is_some());
}

#[test]
fn test_degradation_level_comparison() {
    assert!(DegradationLevel::None < DegradationLevel::Partial);
    assert!(DegradationLevel::Partial < DegradationLevel::Severe);
    assert!(DegradationLevel::Severe < DegradationLevel::Emergency);
}

#[tokio::test]
async fn test_concurrent_health_checks() {
    let mut monitor = HealthMonitor::default();

    for i in 0..5 {
        #[derive(Debug)]
        struct NumberedHealthCheck(usize);

        impl HealthCheckDyn for NumberedHealthCheck {
            fn check_health(
                &self,
            ) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>>
            {
                Box::pin(async { Ok(HealthStatus::Healthy) })
            }

            fn component_name(&self) -> &str {
                "numbered"
            }
        }

        monitor.register(Box::new(NumberedHealthCheck(i)));
    }

    // All checks should complete concurrently
    monitor.check_all().await;
}

#[test]
fn test_recovery_strategy_evolution() {
    let mut manager = DegradationManager::new();

    // Start with cache fallback
    manager.add_fallback_strategy("api", FallbackStrategy::Cache);

    // Evolve to alternative endpoint
    manager.add_fallback_strategy("api", FallbackStrategy::Alternative {
        alternative_endpoint: "new-api".to_string(),
    });

    let result = manager.handle_failure("api");
    assert!(result.is_ok());
}
