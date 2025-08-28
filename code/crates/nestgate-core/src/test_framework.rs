/// Test Framework - Core testing utilities and error handling
///
/// This module provides comprehensive testing utilities for the NestGate ecosystem,
/// including unified error handling using NestGateError variants, test configuration
/// management, and performance testing utilities.
///
/// ## Usage Examples
///
/// ```rust
/// use crate::{NestGateError, Result};
/// use crate::test_framework::*;
///
/// // Test setup failure
/// return Err(NestGateError::System {
///     message: "Test database connection failed".to_string(),
///     resource: SystemResource::Database,
///     recovery: RecoveryStrategy::Retry,
/// });
///
/// // Test assertion failure
/// return Err(NestGateError::Validation {
///     field: Some("test_result".to_string()),
///     issue: "Expected 'success' but got 'failure'".to_string(),
///     suggestion: "Check test logic and dependencies".to_string(),
/// });
///
/// // Test timeout
/// return Err(NestGateError::Timeout {
///     operation: "integration_test".to_string(),
///     duration: std::time::Duration::from_secs(45),
///     timeout: std::time::Duration::from_secs(30),
/// });
/// ```
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use crate::error::{RecoveryStrategy, SystemResource};
use crate::{NestGateError, Result};

/// Test result type using unified error system
/// All test errors now use NestGateError::Testing variant for consistency
pub type TestResult<T> = Result<T>;

// ==================== SECTION ====================

/// **SAFE TEST SETUP**
/// For initializing test environments, configs, managers
pub fn test_setup<F, T>(operation: &str, context: &str, f: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    f().map_err(|e| crate::error::NestGateError::System {
        message: format!("Test setup failed in {}: {}", operation, e),
        resource: crate::error::core::SystemResource::Memory,
        utilization: None,
        recovery: crate::error::core::RecoveryStrategy::ManualIntervention,
    })
    }

/// **SAFE ASYNC TEST SETUP**
/// For async initialization operations
pub async fn test_setup_async<F, Fut, T>(operation: &str, context: &str, f: F) -> Result<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    f().await.map_err(|e| crate::error::NestGateError::System {
        message: format!("Async test setup failed in {}: {}", operation, e),
        resource: crate::error::core::SystemResource::Memory,
        utilization: None,
        recovery: crate::error::core::RecoveryStrategy::ManualIntervention,
    })
    }

// ==================== SECTION ====================

/// **SAFE TEST OPERATION**
/// For test logic operations that might fail
pub fn test_operation<F, T>(operation: &str, context: &str, f: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    f().map_err(|e| crate::error::NestGateError::System {
        message: format!("Test operation failed in {}: {}", operation, e),
        resource: crate::error::core::SystemResource::Memory,
        utilization: None,
        recovery: crate::error::core::RecoveryStrategy::Retry,
    })
    }

/// **SAFE ASYNC TEST OPERATION**
/// For async test operations
pub async fn test_operation_async<F, Fut, T>(operation: &str, context: &str, f: F) -> Result<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    f().await.map_err(|e| crate::error::NestGateError::System {
        message: format!("Async test operation failed in {}: {}", operation, e),
        resource: crate::error::core::SystemResource::Memory,
        utilization: None,
        recovery: crate::error::core::RecoveryStrategy::Retry,
    })
    }

// ==================== SECTION ====================

/// **ENHANCED ASSERT_EQ** with rich error context
#[macro_export]
macro_rules! test_assert_eq {
    ($left:expr, $right:expr, $description:expr) => {
        if $left != $right {
            return Err($crate::test_framework::TestError::Assertion {
                description: $description.to_string(),
                expected: format!("{:?}", $right),
                current_value: format!("{:?}", $left),
                location: format!("{}:{}", file!(), line!()),
            });
    }
    };
    }

/// **ENHANCED ASSERT** with descriptive context
#[macro_export]
macro_rules! test_assert {
    ($condition:expr, $description:expr) => {
        if !$condition {
            return Err($crate::test_framework::TestError::Assertion {
                description: $description.to_string(),
                expected: "true".to_string(),
                current_value: "false".to_string(),
                location: format!("{}:{}", file!(), line!()),
            });
    }
    };
    }

// ==================== SECTION ====================

/// **SAFE TEST TIMEOUT**
/// Operations with timeout tracking and context
pub async fn test_with_timeout<F, Fut, T>(operation: &str, timeout: Duration, f: F) -> Result<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let start = std::time::Instant::now();

    match tokio::time::timeout(timeout, f()).await {
        Ok(result) => result,
        Err(_) => Err(crate::error::NestGateError::System {
            message: format!(
                "Test operation '{}' timed out after {:?}",
                operation,
                start.elapsed()
            ),
            resource: crate::error::core::SystemResource::Memory,
            utilization: None,
            recovery: crate::error::core::RecoveryStrategy::ManualIntervention,
        }),
    }
    }

// ==================== SECTION ====================

/// Test error type for framework
#[derive(Debug, Clone)]
pub struct TestError {
    pub message: String,
    pub context: Option<String>,
    }

impl From<NestGateError> for TestError {
    fn from(error: NestGateError) -> Self {
        TestError {
            message: error.to_string(),
            context: None,
    }
    }
    }

/// Convert NestGateError to TestError with context
pub fn system_error(error: NestGateError, test_context: &str) -> TestError {
    TestError {
        message: error.to_string(),
        context: Some(test_context.to_string()),
    }
    }

// ==================== SECTION ====================

/// **QUICK SETUP MACRO** for common patterns
#[macro_export]
macro_rules! setup {
    ($expr:expr, $context:expr) => {
        $crate::test_framework::test_setup(stringify!($expr), $context, || Ok($expr))?
    };
    }

/// **QUICK ASYNC SETUP MACRO**
#[macro_export]
macro_rules! setup_async {
    ($expr:expr, $context:expr) => {
        $crate::test_framework::test_setup_async(stringify!($expr), $context, || async {
            Ok($expr)
        })
        .await?
    };
    }

// ==================== SECTION ====================

/// Test performance tracking
#[derive(Debug)]
pub struct TestTimer {
    start: std::time::Instant,
    operation: String,
    }

impl TestTimer {
    pub fn new(operation: &str) -> Self {
        Self {
            start: std::time::Instant::now(),
            operation: operation.to_string(),
    }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn check_performance(&self, max_duration: Duration) -> Result<()> {
        let elapsed = self.elapsed();
        if elapsed > max_duration {
            Err(crate::error::NestGateError::System {
                message: format!(
                    "Test operation '{}' exceeded maximum duration: {:?} > {:?}",
                    self.operation, elapsed, max_duration
                ),
                resource: crate::error::core::SystemResource::Memory,
                utilization: Some(
                    ((elapsed.as_millis() as f64 / max_duration.as_millis() as f64) * 100.0) as u8
                        as f64,
                ),
                recovery: crate::error::core::RecoveryStrategy::Retry,
            })
        } else {
    }
    }
    }

/// Create rich test context from multiple sources
pub fn create_test_context(
    test_name: &str,
    test_phase: &str,
    additional_info: &[(&str, &str)],
) -> String {
    let mut context = format!("Test: {test_name} | Phase: {test_phase}");

    for (key, value) in additional_info {
        context.push_str(&format!(" | {key}: {value}"));
    }

    context
    }

// Re-export test-specific safe operations
pub mod test_safe_operations;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_framework_setup_success() -> Result<()> {
        let value = test_setup("create test value", "framework test", || {
            Ok::<i32, crate::error::NestGateError>(42)
        })?;

        test_assert_eq!(value, 42, "test setup should return correct value");
    }

    #[tokio::test]
    async fn test_framework_timeout() -> Result<()> {
        let result = test_with_timeout("slow operation", Duration::from_millis(10), || async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok::<(), TestError>(())
        })
        .await;

        // Should timeout
        assert!(result.is_err());
    }
    }
