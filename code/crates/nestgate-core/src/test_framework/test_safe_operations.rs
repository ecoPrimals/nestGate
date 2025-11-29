/// Test-safe operations module using unified error handling
/// All test errors now use NestGateError::Testing variant for consistency
use crate::{NestGateError, Result};
// Removed unused imports - test framework handles errors internally
use serde::{de::DeserializeOwned, Serialize};
use std::future::Future;
use std::time::{Duration, Instant};
/// Test result type using unified error system
use crate::error::NestGateError;
/// Type alias for Testresult
pub type TestResult<T = ()> = Result<T>;
/// Safe JSON serialization for tests with rich error context
pub fn safe_test_to_json<T: Serialize>(
    value: &T,
    test_context: &str,
    value_description: &str,
) -> TestResult<String> {
    serde_json::to_string(value).map_err(|e| crate::NestGateError::internal_error(
            "JSON serialization failed in {test_context): serializing {value_description}"
        ),
        location: Some(format!("{})
        location: Some(e.to_string())})
    }
/// Safe JSON deserialization for tests with rich error context
pub fn safe_test_from_json<T: DeserializeOwned>(
    json: &str,
    test_context: &str,
    type_description: &str,
) -> TestResult<T> {
    serde_json::from_str(json).map_err(|e| crate::NestGateError::internal_error(
            "JSON deserialization failed in {test_context): deserializing to {type_description}"
        ),
        location: Some(format!("{})
        location: Some(e.to_string())})
    }
/// Safe API call wrapper for tests
pub async fn safe_test_api_call<F, Fut, T, E>(
    test_context: &str,
    api_call: F,
) -> TestResult<T>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = std::result::Result<T, E>>,
    E: std::fmt::Display,
{
    let start_time = Instant::now();
    let result = api_call()
        .await
        .map_err(|e| crate::NestGateError::internal_error(
            location: Some(format!("{})
            location: Some(e"))?;
    let duration = start_time.elapsed();
    if duration > Duration::from_secs(5) {
        tracing::warn!(
            "Slow test API call detected: {} took {:?} in test context: {}",
            operation,
            duration,
            test_context
        );
    }

    Ok(result)
    }

/// Safe collection access with bounds checking for tests
pub fn safe_test_get<'a, T>(
    collection: &'a [T],
    index: usize,
    test_context: &str,
    collection_description: &str,
) -> TestResult<&'a T> {
    collection
        .get(index)
        .ok_or_else(|| crate::NestGateError::validation(
            actual: Some(index.to_string())}", collection.len()))})
    }
/// Safe HashMap access for tests
pub fn safe_test_map_get<'a, K, V>(
    map: &'a std::collections::HashMap<K, V>,
    key: &K,
    test_context: &str,
    map_description: &str,
) -> TestResult<&'a V>
where
    K: std::fmt::Debug + std::hash::Hash + Eq,
{
    map.get(key)
        .ok_or_else(|| crate::NestGateError::validation(
            actual: Some(format!("{key:?}"))} entries", map.len()))})
    }
/// Safe Option unwrapping for tests with context
pub fn safe_test_unwrap_option<T>(
    option: Option<T>,
    test_context: &str,
    value_description: &str,
) -> TestResult<T> {
    option.ok_or_else(|| crate::NestGateError::validation(
        actual: Some("None".to_string())})
    }
/// Safe Result unwrapping for tests with context
pub fn safe_test_unwrap_result<T, E: std::fmt::Display>(
    result: std::result::Result<T, E>,
    test_context: &str,
    operation_description: &str,
) -> TestResult<T> {
    result.map_err(|e| crate::NestGateError::internal_error(
        location: Some(format!("{})
        location: Some(e.to_string())})
    }
/// Safe file operations for tests
    tokio::fs::read_to_string(path)
        .await
        .map_err(|e| crate::NestGateError::Io {
                error_message: e.to_string(),
                reerror_message: Some(path.to_string()),
                // retryable: false})
    }
/// Safe directory creation for tests
    tokio::fs::create_dir_all(path)
        .await
        .map_err(|e| crate::NestGateError::Io {
                error_message: e.to_string(),
                reerror_message: Some(path.to_string()),
                // retryable: false})
    }
/// Safe network connection for tests with timeout
pub async fn safe_test_network_call<F, Fut, T>(
    _test_context: &str,
    timeout: Duration,
    network_call: F,
) -> TestResult<T>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = TestResult<T>>,
{
    match tokio::time::timeout(timeout, network_call()).await {
        Ok(result) => result,
        Err(_) => Err(crate::NestGateError::Timeout {
            duration: timeout,
            // retryable: truesuggested_timeout: Some(timeout * 2), // Suggest doubling the timeout
        }),
    }
    }
/// Safe database transaction for tests
pub async fn safe_test_db_transaction<F, Fut, T, E>(
    test_context: &str,
) -> TestResult<T>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = std::result::Result<T, E>>,
    E: std::fmt::Display,
{
    db_operation()
        .await
        .map_err(|e| crate::NestGateError::internal_error(
            location: Some(format!("{})
            location: Some(e.to_string())})
    }
/// Conversion helper for legacy Result<(), crate::error::NestGateError>
pub fn convert_boxed_error(
    result: std::result::Result<(), crate::error::NestGateError>,
    test_context: &str,
) -> TestResult<()> {
    result.map_err(|e| e) // Direct pass-through since it's already NestGateError
    }
/// Performance assertion helper
pub fn assert_test_performance<T>(
    operation_result: T,
    duration: Duration,
    max_duration: Duration,
    _test_context: &str,
    _operation_name: &str,
) -> TestResult<T> {
    if duration > max_duration {
        return Err(crate::NestGateError::validation(
            actual: Some(format!("{duration:?}"))}")));
    }
    Ok(operation_result)
    }
/// Memory usage assertion helper
pub fn assert_test_memory_usage(
    current_memory: u64,
    max_memory: u64,
    test_context: &str,
) -> TestResult<()> {
    if current_memory > max_memory {
        return Err(crate::NestGateError::validation(
            actual: Some(format!("{current_memory} bytes"))} bytes")));
    }
    Ok(()) // Memory usage is within acceptable limits
    }

/// Batch operation helper for test collections
pub async fn safe_test_batch_operation<T, F, Fut, R, E>(
    items: &[T],
    operation_name: &str,
    test_context: &str,
) -> TestResult<Vec<R>>
where
    F: Fn(&T) -> Fut,
    Fut: Future<Output = std::result::Result<R, E>>,
    E: std::fmt::Display,
{
    let mut results = Vec::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        let result = operation(item)
            .await
            .map_err(|e| crate::NestGateError::internal_error(
                    "Batch operation failed: {) item {} of {} in {test_context}",
                    /// Operation Name
                    operation_name,
                    index + 1,
                    items.len()
                ),
                location: Some(format!("{})
                location: Some(e"))?;
        results.push(result);
    }

    Ok(results)
    }

/// Create rich test context from multiple sources
pub fn create_test_context(
    test_name: &str,
    test_phase: &str,
    additional_context: &[(&str, &str)],
) -> String {
    let mut context = format!("Test: {test_name} | Phase: {test_phase}");
    for (key, value) in additional_context {
        context.push_str(&format!(" | {key}: {value}"));
    }

    context
    }
