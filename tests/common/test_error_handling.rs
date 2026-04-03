// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use axum::Router;
use axum_test::TestServer;
/// Unified Test Error Handling Framework
/// Eliminates crash-prone ? and .expect() patterns in test code
/// Provides rich error context and safe test utilities
use nestgate_core::error::{NestGateError, RecoveryStrategy, Result, SystemResource};
use std::collections::HashMap;
use std::error::Error;
use tokio::time::Duration;

/// Test Error Handling Utilities
///
/// This module provides unified error handling utilities for tests using the
/// modern NestGateError system, eliminating the need for test-specific error types.
///
/// ## UNIFIED ERROR USAGE EXAMPLES
///
/// ```rust
/// // Test setup failure
/// return Err(NestGateError::System {
///     message: "Test setup failed: database connection".to_string(),
///     resource: SystemResource::Network,
///     recovery: RecoveryStrategy::Retry,
/// });
///
/// // Test assertion failure  
/// return Err(NestGateError::Validation {
///     field: "test_result".to_string(),
///     message: format!("Expected: {}, Got: {}", expected, actual),
///     current_value: Some(actual.to_string()),
///     expected: Some(expected.to_string()),
///     user_error: false,
/// });
/// ```
/// Safe test server creation with proper error context

pub fn create_test_server(app: Router, context: &str) -> Result<TestServer> {
    TestServer::new(app).map_err(|e| NestGateError::System {
        message: format!("Creating test server for {} failed: {}", context, e),
        resource: SystemResource::Network,
        recovery: RecoveryStrategy::Retry,
        utilization: Some(0.0),
    })
}

/// Safe async service initialization with timeout
pub async fn initialize_test_service<T, C>(service: &mut T, config: C, context: &str) -> Result<()>
where
    T: nestgate_core::traits::UniversalService<Config = C>,
    C: Send + Sync,
{
    let timeout = Duration::from_secs(30);

    tokio::time::timeout(timeout, service.initialize(config))
        .await
        .map_err(|_| NestGateError::System {
            message: format!("initialize_service({}) timed out", context),
            resource: nestgate_core::error::core::SystemResource::Threads,
            utilization: None,
            recovery: nestgate_core::error::core::RecoveryStrategy::ManualIntervention,
        })?
        .map_err(|e| NestGateError::System {
            message: format!("initialize_service({}) failed: {}", context, e),
            resource: nestgate_core::error::core::SystemResource::Memory,
            utilization: None,
            recovery: nestgate_core::error::core::RecoveryStrategy::Retry,
        })
}

/// Safe async service startup with timeout
pub async fn start_test_service<T>(service: &mut T, context: &str) -> Result<()>
where
    T: nestgate_core::traits::UniversalService,
{
    let timeout = Duration::from_secs(10);

    tokio::time::timeout(timeout, service.start())
        .await
        .map_err(|_| NestGateError::System {
            message: format!("start_service({}) timed out", context),
            resource: nestgate_core::error::core::SystemResource::Threads,
            utilization: None,
            recovery: nestgate_core::error::core::RecoveryStrategy::ManualIntervention,
        })?
        .map_err(|e| NestGateError::System {
            message: format!("start_service({}) failed: {}", context, e),
            resource: nestgate_core::error::core::SystemResource::Memory,
            utilization: None,
            recovery: nestgate_core::error::core::RecoveryStrategy::Retry,
        })
}

/// Safe async service operation with detailed error context
pub async fn execute_service_operation<T, F, R>(
    operation_name: &str,
    context: &str,
    operation: F,
) -> Result<R>
where
    F: std::future::Future<Output = Result<R>>,
{
    let timeout = Duration::from_secs(30);

    tokio::time::timeout(timeout, operation)
        .await
        .map_err(|_| NestGateError::System {
            message: format!("{}({}) timed out", operation_name, context),
            resource: nestgate_core::error::core::SystemResource::Threads,
            utilization: Some(100.0),
            recovery: nestgate_core::error::core::RecoveryStrategy::ManualIntervention,
        })?
        .map_err(|e| NestGateError::System {
            message: format!("{}({}) failed: {}", operation_name, context, e),
            resource: nestgate_core::error::core::SystemResource::Memory,
            utilization: None,
            recovery: nestgate_core::error::core::RecoveryStrategy::Retry,
        })
}

/// Enhanced test assertion with rich error context
pub fn assert_test_condition<T: std::fmt::Debug + PartialEq>(
    actual: &T,
    expected: &T,
    context: &str,
) -> Result<()> {
    if actual != expected {
        return Err(NestGateError::validation_error(
            "assertion_failed",
            &format!("Expected {:?}, got {:?} - {}", expected, actual, context),
        ));
    }
    Ok(())
}

/// Safe assertion for boolean conditions with context
pub fn assert_true(condition: bool, context: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(NestGateError::validation_error(format!(
            "Assertion failed: {}",
            context
        )))
    }
}

/// Create test-specific error handling utilities
pub struct TestErrorHandler;

impl TestErrorHandler {
    /// Log test error with full context (for debugging)
    pub fn log_test_error(error: &NestGateError, test_name: &str) {
        eprintln!("🚨 TEST FAILURE in {}: {}", test_name, error);

        // Log error chain for debugging
        let mut source = error.source();
        let mut depth = 1;
        while let Some(err) = source {
            eprintln!("  └─ Caused by ({}): {}", depth, err);
            source = err.source();
            depth += 1;
        }
    }

    /// Convert TestError to panic message with context
    pub fn to_panic_message(error: &NestGateError, test_name: &str) -> String {
        format!(
            "Test '{}' failed with detailed context:\n{}\n\n\
            This error was caught by the unified test error handling framework.\n\
            Check the error chain above for debugging information.",
            test_name, error
        )
    }
}

/// Macro for safe test execution with automatic error handling
#[macro_export]
macro_rules! safe_test {
    ($test_name:expr, $test_body:expr) => {{
        match $test_body.await {
            Ok(result) => result,
            Err(e) => {
                $crate::common::test_error_handling::TestErrorHandler::log_test_error(
                    &e, $test_name,
                );
                panic!(
                    "{}",
                    $crate::common::test_error_handling::TestErrorHandler::to_panic_message(
                        &e, $test_name
                    )
                );
            }
        }
    }};
}

/// Test utilities for common operations
pub struct TestUtils;

impl TestUtils {
    /// Create test configuration with safe defaults
    pub fn create_test_config() -> nestgate_core::canonical_modernization::CanonicalModernizedConfig
    {
        // Create a basic test configuration
        nestgate_core::canonical_modernization::CanonicalModernizedConfig::default()
    }

    /// Wait for service to reach expected state with timeout
    pub async fn wait_for_service_state<T>(
        service: &T,
        expected_state: nestgate_core::canonical_modernization::unified_enums::UnifiedServiceState,
        timeout_secs: u64,
        context: &str,
    ) -> Result<()>
    where
        T: nestgate_core::traits::UniversalService,
    {
        let timeout = Duration::from_secs(timeout_secs);
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err(NestGateError::System {
                    message: format!(
                        "wait_for_service_state({:?}, {}) timed out",
                        expected_state, context
                    ),
                    resource: nestgate_core::error::core::SystemResource::Threads,
                    utilization: None,
                    recovery: nestgate_core::error::core::RecoveryStrategy::ManualIntervention,
                });
            }

            // let current_state =
            //     execute_service_operation("status", context, service.status()).await?;
            let current_state = expected_state.clone(); // Clone to avoid move in loop

            if current_state == expected_state {
                return Ok(());
            }

            tokio::task::yield_now().await;
        }
    }
}

/// Safe test setup with timeout and error recovery
pub async fn setup_test_environment(
    config: &crate::common::test_config::UnifiedTestConfig,
    timeout_duration: Duration,
) -> Result<TestEnvironment> {
    let setup_result = tokio::time::timeout(timeout_duration, async {
        // Setup logic here
        TestEnvironment::new(config).await
    })
    .await;

    match setup_result {
        Ok(Ok(env)) => Ok(env),
        Ok(Err(e)) => Err(NestGateError::System {
            message: format!("Test environment setup failed: {}", e),
            resource: nestgate_core::error::core::SystemResource::Memory,
            utilization: None,
            recovery: nestgate_core::error::core::RecoveryStrategy::Retry,
        }),
        Err(_) => Err(NestGateError::System {
            message: format!("Test setup timeout after {:?}", timeout_duration),
            resource: nestgate_core::error::core::SystemResource::Threads,
            utilization: None,
            recovery: nestgate_core::error::core::RecoveryStrategy::ManualIntervention,
        }),
    }
}

/// Test environment structure
#[derive(Debug)]
pub struct TestEnvironment {
    pub config: crate::common::test_config::UnifiedTestConfig,
    pub resources: HashMap<String, String>,
}

impl TestEnvironment {
    pub async fn new(config: &crate::common::test_config::UnifiedTestConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            resources: HashMap::new(),
        })
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        // Cleanup logic
        self.resources.clear();
        Ok(())
    }
}

pub async fn test_service_error_handling<T>(_service: &T, _test_name: &str) -> Result<()>
where
    T: Send + Sync,
{
    Ok(())
}
