// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for Retry Executor Implementation
//!
//! Tests cover retry logic, exponential backoff, max attempts, and edge cases
//! to ensure reliable operation recovery in production environments.

use super::RetryExecutor;
use crate::handlers::zfs::universal_zfs::config::RetryPolicy;
use crate::handlers::zfs::universal_zfs_types::UniversalZfsError;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

// ==================== BASIC INSTANTIATION TESTS ====================

#[test]
fn test_retry_executor_creation() {
    let policy = RetryPolicy {
        max_attempts: 3,
        initial_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    };

    let executor = RetryExecutor::new(policy);

    // Should create successfully
    assert!(format!("{executor:?}").contains("RetryExecutor"));
}

#[test]
fn test_retry_executor_clone() {
    let policy = RetryPolicy {
        max_attempts: 3,
        initial_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    };

    let executor1 = RetryExecutor::new(policy);
    let executor2 = executor1.clone();

    // Both should be valid
    assert!(format!("{executor1:?}").contains("RetryExecutor"));
    assert!(format!("{executor2:?}").contains("RetryExecutor"));
}

// ==================== SUCCESS ON FIRST TRY TESTS ====================

#[tokio::test]
async fn test_success_on_first_attempt() {
    let policy = RetryPolicy {
        max_attempts: 3,
        initial_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                count.fetch_add(1, Ordering::SeqCst);
                Ok::<String, UniversalZfsError>("success".to_string())
            })
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
    assert_eq!(attempt_count.load(Ordering::SeqCst), 1);
}

// ==================== RETRY AFTER FAILURES TESTS ====================

#[tokio::test]
async fn test_success_after_one_failure() {
    let policy = RetryPolicy {
        max_attempts: 3,
        initial_delay: Duration::from_millis(10),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                let attempts = count.fetch_add(1, Ordering::SeqCst) + 1;
                if attempts < 2 {
                    Err(UniversalZfsError::internal("temporary failure"))
                } else {
                    Ok("success".to_string())
                }
            })
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
    assert_eq!(attempt_count.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn test_success_after_two_failures() {
    let policy = RetryPolicy {
        max_attempts: 3,
        initial_delay: Duration::from_millis(10),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                let attempts = count.fetch_add(1, Ordering::SeqCst) + 1;
                if attempts < 3 {
                    Err(UniversalZfsError::internal("temporary failure"))
                } else {
                    Ok("success".to_string())
                }
            })
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
    assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
}

// ==================== MAX ATTEMPTS TESTS ====================

#[tokio::test]
async fn test_failure_after_max_attempts() {
    let policy = RetryPolicy {
        max_attempts: 3,
        initial_delay: Duration::from_millis(10),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                count.fetch_add(1, Ordering::SeqCst);
                Err::<String, UniversalZfsError>(UniversalZfsError::internal("permanent failure"))
            })
        })
        .await;

    assert!(result.is_err());
    assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_single_attempt_failure() {
    let policy = RetryPolicy {
        max_attempts: 1,
        initial_delay: Duration::from_millis(10),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                count.fetch_add(1, Ordering::SeqCst);
                Err::<String, UniversalZfsError>(UniversalZfsError::internal("failure"))
            })
        })
        .await;

    assert!(result.is_err());
    assert_eq!(attempt_count.load(Ordering::SeqCst), 1);
}

// ==================== BACKOFF TIMING TESTS ====================

#[tokio::test]
async fn test_exponential_backoff_timing() {
    let policy = RetryPolicy {
        max_attempts: 4,
        initial_delay: Duration::from_millis(50),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();
    let start_time = Instant::now();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                let attempts = count.fetch_add(1, Ordering::SeqCst) + 1;
                if attempts < 4 {
                    Err(UniversalZfsError::internal("retry"))
                } else {
                    Ok("success".to_string())
                }
            })
        })
        .await;

    let elapsed = start_time.elapsed();

    assert!(result.is_ok());
    // Expected delays: 50ms + 100ms + 200ms = 350ms minimum
    assert!(elapsed >= Duration::from_millis(300));
    assert_eq!(attempt_count.load(Ordering::SeqCst), 4);
}

#[tokio::test]
async fn test_max_delay_cap() {
    let policy = RetryPolicy {
        max_attempts: 5,
        initial_delay: Duration::from_millis(100),
        max_delay: Duration::from_millis(150), // Cap at 150ms
        backoff_multiplier: 2.0,
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();
    let start_time = Instant::now();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                count.fetch_add(1, Ordering::SeqCst);
                Err::<String, UniversalZfsError>(UniversalZfsError::internal("fail"))
            })
        })
        .await;

    let elapsed = start_time.elapsed();

    assert!(result.is_err());
    // Expected: 100ms + 150ms (capped) + 150ms (capped) + 150ms (capped) = ~550ms
    assert!(elapsed >= Duration::from_millis(500));
    assert!(elapsed < Duration::from_millis(800));
}

// ==================== DIFFERENT BACKOFF MULTIPLIERS ====================

#[tokio::test]
async fn test_linear_backoff() {
    let policy = RetryPolicy {
        max_attempts: 3,
        initial_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 1.0, // Linear (no increase)
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();
    let start_time = Instant::now();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                count.fetch_add(1, Ordering::SeqCst);
                Err::<String, UniversalZfsError>(UniversalZfsError::internal("fail"))
            })
        })
        .await;

    let elapsed = start_time.elapsed();

    assert!(result.is_err());
    // Expected: 100ms + 100ms = 200ms (linear, no growth)
    assert!(elapsed >= Duration::from_millis(180));
    assert!(elapsed < Duration::from_millis(300));
}

#[tokio::test]
async fn test_aggressive_backoff() {
    let policy = RetryPolicy {
        max_attempts: 3,
        initial_delay: Duration::from_millis(10),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 5.0, // Aggressive growth
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                count.fetch_add(1, Ordering::SeqCst);
                Err::<String, UniversalZfsError>(UniversalZfsError::internal("fail"))
            })
        })
        .await;

    assert!(result.is_err());
    assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
}

// ==================== EDGE CASE TESTS ====================

#[tokio::test]
async fn test_zero_initial_delay() {
    let policy = RetryPolicy {
        max_attempts: 3,
        initial_delay: Duration::from_millis(0),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                let attempts = count.fetch_add(1, Ordering::SeqCst) + 1;
                if attempts < 3 {
                    Err(UniversalZfsError::internal("retry"))
                } else {
                    Ok("success".to_string())
                }
            })
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn test_very_high_max_attempts() {
    let policy = RetryPolicy {
        max_attempts: 100,
        initial_delay: Duration::from_millis(1),
        max_delay: Duration::from_millis(5),
        backoff_multiplier: 1.1,
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                let attempts = count.fetch_add(1, Ordering::SeqCst) + 1;
                if attempts < 50 {
                    Err(UniversalZfsError::internal("retry"))
                } else {
                    Ok("success".to_string())
                }
            })
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(attempt_count.load(Ordering::SeqCst), 50);
}

// ==================== DIFFERENT ERROR TYPES ====================

#[tokio::test]
async fn test_retry_with_different_errors() {
    let policy = RetryPolicy {
        max_attempts: 4,
        initial_delay: Duration::from_millis(10),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    };

    let executor = RetryExecutor::new(policy);
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = attempt_count.clone();

    let result = executor
        .execute(move || {
            let count = attempt_count_clone.clone();
            Box::pin(async move {
                let attempts = count.fetch_add(1, Ordering::SeqCst) + 1;
                match attempts {
                    1 => Err(UniversalZfsError::internal("error1")),
                    2 => Err(UniversalZfsError::internal("error2")),
                    3 => Err(UniversalZfsError::internal("error3")),
                    _ => Ok("finally_success".to_string()),
                }
            })
        })
        .await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "finally_success");
    assert_eq!(attempt_count.load(Ordering::SeqCst), 4);
}

// ==================== RETURN TYPE TESTS ====================

#[tokio::test]
async fn test_retry_with_different_return_types() {
    let policy = RetryPolicy {
        max_attempts: 3,
        initial_delay: Duration::from_millis(10),
        max_delay: Duration::from_secs(10),
        backoff_multiplier: 2.0,
    };

    let executor = RetryExecutor::new(policy);

    // Test with u64
    let result_u64 = executor
        .execute(|| Box::pin(async move { Ok::<u64, UniversalZfsError>(42) }))
        .await;
    assert_eq!(result_u64.unwrap(), 42);

    // Test with Vec
    let result_vec = executor
        .execute(|| {
            Box::pin(async move {
                Ok::<Vec<String>, UniversalZfsError>(vec!["a".to_string(), "b".to_string()])
            })
        })
        .await;
    assert_eq!(result_vec.unwrap().len(), 2);
}
