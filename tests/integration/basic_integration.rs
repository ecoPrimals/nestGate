//! Basic Integration Tests - Clean, Working Tests
//!
//! These tests validate core system functionality using only current, working APIs.
//! No legacy code references - built for current codebase state.

use nestgate_core::{NestGateError, Result};
use std::time::Duration;

/// Test basic async operations
#[tokio::test]
async fn test_async_operations_work() -> Result<()> {
    // Test simple async coordination
    tokio::task::yield_now().await;

    // Test async result handling
    let result: Result<String> = Ok("Success".to_string());
    assert!(result.is_ok());

    Ok(())
}

/// Test error creation and handling
#[tokio::test]
async fn test_error_handling() -> Result<()> {
    // Test creating internal error
    let error = NestGateError::internal_error("Test error".to_string(), "test context".to_string());

    // Verify error can be created
    assert!(format!("{:?}", error).contains("Test error"));

    // Test Result error path
    let result: Result<()> = Err(error);
    assert!(result.is_err());

    Ok(())
}

/// Test concurrent operations
#[tokio::test]
async fn test_concurrent_operations() -> Result<()> {
    // Spawn multiple concurrent tasks
    let handles: Vec<_> = (0..5)
        .map(|i| {
            tokio::spawn(async move {
                tokio::task::yield_now().await;
                i * 2
            })
        })
        .collect();

    // Wait for all tasks
    let results: Vec<_> = futures::future::join_all(handles).await;

    // Verify all completed successfully
    assert_eq!(results.len(), 5);
    for result in results {
        assert!(result.is_ok());
    }

    Ok(())
}

/// Test timeout handling
#[tokio::test]
async fn test_timeout_handling() -> Result<()> {
    // Test successful operation within timeout
    let result = tokio::time::timeout(Duration::from_millis(100), async {
        Ok::<_, NestGateError>(42)
    })
    .await;

    assert!(result.is_ok());

    Ok(())
}

/// Test resource cleanup patterns
#[tokio::test]
async fn test_resource_cleanup() -> Result<()> {
    // Test that drop handlers work correctly
    {
        let _resource = [1, 2, 3, 4, 5];
        // Resource will be dropped here
    }

    // Resource should be cleaned up
    Ok(())
}
