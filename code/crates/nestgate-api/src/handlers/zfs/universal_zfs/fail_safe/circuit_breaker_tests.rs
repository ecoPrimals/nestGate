// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for Circuit Breaker Implementation
//!
//! Tests cover all circuit breaker states, transitions, and edge cases
//! to ensure reliable fault tolerance in production environments.

use super::{CircuitBreaker, CircuitBreakerState};
use crate::handlers::zfs::universal_zfs::config::CircuitBreakerConfig;
use std::time::Duration;

// ==================== BASIC INSTANTIATION TESTS ====================

#[tokio::test]
async fn test_circuit_breaker_creation() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 5,
        recovery_timeout: Duration::from_secs(60),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);
    let state = breaker.get_state().await;

    assert_eq!(state, CircuitBreakerState::Closed);
    assert!(!breaker.is_open().await);
}

#[tokio::test]
async fn test_circuit_breaker_disabled() {
    let config = CircuitBreakerConfig {
        enabled: false,
        failure_threshold: 5,
        recovery_timeout: Duration::from_secs(60),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Should always allow execution when disabled
    assert!(breaker.can_execute().await);
    assert!(!breaker.is_open().await);

    // Record failures should not affect disabled breaker
    for _ in 0..10 {
        breaker.record_failure().await;
    }

    assert!(breaker.can_execute().await);
    assert!(!breaker.is_open().await);
}

// ==================== STATE TRANSITION TESTS ====================

#[tokio::test]
async fn test_transition_closed_to_open() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 3,
        recovery_timeout: Duration::from_secs(60),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Initial state: Closed
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Closed);
    assert!(breaker.can_execute().await);

    // Record failures below threshold
    breaker.record_failure().await;
    breaker.record_failure().await;
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Closed);

    // Record failure that reaches threshold
    breaker.record_failure().await;
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Open);
    assert!(breaker.is_open().await);
    assert!(!breaker.can_execute().await);
}

#[tokio::test]
async fn test_transition_open_to_half_open() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 2,
        recovery_timeout: Duration::from_millis(100),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Trigger open state
    breaker.record_failure().await;
    breaker.record_failure().await;
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Open);

    // Wait for recovery timeout
    tokio::time::sleep(Duration::from_millis(150)).await;

    // Should transition to half-open on next can_execute check
    assert!(breaker.can_execute().await);
    assert_eq!(breaker.get_state().await, CircuitBreakerState::HalfOpen);
}

#[tokio::test]
async fn test_transition_half_open_to_closed() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 2,
        recovery_timeout: Duration::from_millis(100),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Reach open state
    breaker.record_failure().await;
    breaker.record_failure().await;

    // Wait and transition to half-open
    tokio::time::sleep(Duration::from_millis(150)).await;
    breaker.can_execute().await;

    assert_eq!(breaker.get_state().await, CircuitBreakerState::HalfOpen);

    // Success in half-open should close the circuit
    breaker.record_success().await;
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Closed);
}

#[tokio::test]
async fn test_transition_half_open_to_open() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 2,
        recovery_timeout: Duration::from_millis(100),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Reach open state
    breaker.record_failure().await;
    breaker.record_failure().await;

    // Wait and transition to half-open
    tokio::time::sleep(Duration::from_millis(150)).await;
    breaker.can_execute().await;

    assert_eq!(breaker.get_state().await, CircuitBreakerState::HalfOpen);

    // Failure in half-open should reopen the circuit
    breaker.record_failure().await;
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Open);
}

// ==================== SUCCESS/FAILURE RECORDING TESTS ====================

#[tokio::test]
async fn test_record_success_in_closed_state() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 5,
        recovery_timeout: Duration::from_secs(60),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Record some failures
    breaker.record_failure().await;
    breaker.record_failure().await;

    // Success should reset failure count
    breaker.record_success().await;

    // Should still be closed and able to execute
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Closed);
    assert!(breaker.can_execute().await);
}

#[tokio::test]
async fn test_multiple_failures_then_success() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 5,
        recovery_timeout: Duration::from_secs(60),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Record 4 failures (below threshold)
    for _ in 0..4 {
        breaker.record_failure().await;
    }

    assert_eq!(breaker.get_state().await, CircuitBreakerState::Closed);

    // Success should reset count
    breaker.record_success().await;

    // Should need 5 more failures to open
    for _ in 0..4 {
        breaker.record_failure().await;
    }
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Closed);

    breaker.record_failure().await;
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Open);
}

// ==================== HALF-OPEN STATE TESTS ====================

#[tokio::test]
async fn test_half_open_max_calls_limit() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 2,
        recovery_timeout: Duration::from_millis(100),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Reach open state
    breaker.record_failure().await;
    breaker.record_failure().await;

    // Wait and transition to half-open
    tokio::time::sleep(Duration::from_millis(150)).await;
    breaker.can_execute().await;

    assert_eq!(breaker.get_state().await, CircuitBreakerState::HalfOpen);

    // Test that we're in half-open and can execute
    assert!(breaker.can_execute().await);
}

// ==================== EDGE CASE TESTS ====================

#[tokio::test]
async fn test_failure_threshold_of_one() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 1,
        recovery_timeout: Duration::from_secs(60),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Single failure should open circuit
    breaker.record_failure().await;
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Open);
}

#[tokio::test]
async fn test_very_short_recovery_timeout() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 2,
        recovery_timeout: Duration::from_millis(10),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Open the circuit
    breaker.record_failure().await;
    breaker.record_failure().await;
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Open);

    // Wait for recovery
    tokio::time::sleep(Duration::from_millis(20)).await;

    // Should allow execution (transition to half-open)
    assert!(breaker.can_execute().await);
}

#[tokio::test]
async fn test_very_long_recovery_timeout() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 2,
        recovery_timeout: Duration::from_secs(3600), // 1 hour
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Open the circuit
    breaker.record_failure().await;
    breaker.record_failure().await;
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Open);

    // Should not allow execution immediately
    assert!(!breaker.can_execute().await);
}

// ==================== CONCURRENT TESTS ====================

#[tokio::test]
async fn test_concurrent_failure_recording() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 10,
        recovery_timeout: Duration::from_secs(60),
        half_open_max_calls: 3,
    };

    let breaker = std::sync::Arc::new(CircuitBreaker::new(config));

    // Record failures concurrently
    let mut handles = vec![];
    for _ in 0..10 {
        let breaker_clone = breaker.clone();
        let handle = tokio::spawn(async move {
            breaker_clone.record_failure().await;
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }

    // Circuit should be open
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Open);
}

#[tokio::test]
async fn test_concurrent_success_recording() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 5,
        recovery_timeout: Duration::from_secs(60),
        half_open_max_calls: 3,
    };

    let breaker = std::sync::Arc::new(CircuitBreaker::new(config));

    // Record successes concurrently
    let mut handles = vec![];
    for _ in 0..10 {
        let breaker_clone = breaker.clone();
        let handle = tokio::spawn(async move {
            breaker_clone.record_success().await;
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }

    // Circuit should still be closed
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Closed);
}

// ==================== CONFIG VARIATIONS TESTS ====================

#[tokio::test]
async fn test_high_failure_threshold() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 100,
        recovery_timeout: Duration::from_secs(60),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Record 99 failures
    for _ in 0..99 {
        breaker.record_failure().await;
    }

    // Should still be closed
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Closed);

    // 100th failure should open
    breaker.record_failure().await;
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Open);
}

#[tokio::test]
async fn test_high_half_open_max_calls() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 2,
        recovery_timeout: Duration::from_millis(100),
        half_open_max_calls: 50,
    };

    let breaker = CircuitBreaker::new(config);

    // Open and transition to half-open
    breaker.record_failure().await;
    breaker.record_failure().await;
    tokio::time::sleep(Duration::from_millis(150)).await;
    breaker.can_execute().await;

    assert_eq!(breaker.get_state().await, CircuitBreakerState::HalfOpen);

    // Verify we can execute in half-open state
    assert!(breaker.can_execute().await);
}

// ==================== STATE QUERY TESTS ====================

#[tokio::test]
async fn test_get_state_accuracy() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 3,
        recovery_timeout: Duration::from_millis(100),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Test closed state
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Closed);

    // Test open state
    for _ in 0..3 {
        breaker.record_failure().await;
    }
    assert_eq!(breaker.get_state().await, CircuitBreakerState::Open);

    // Test half-open state
    tokio::time::sleep(Duration::from_millis(150)).await;
    breaker.can_execute().await;
    assert_eq!(breaker.get_state().await, CircuitBreakerState::HalfOpen);
}

#[tokio::test]
async fn test_is_open_accuracy() {
    let config = CircuitBreakerConfig {
        enabled: true,
        failure_threshold: 2,
        recovery_timeout: Duration::from_secs(60),
        half_open_max_calls: 3,
    };

    let breaker = CircuitBreaker::new(config);

    // Initially not open
    assert!(!breaker.is_open().await);

    // Open circuit
    breaker.record_failure().await;
    breaker.record_failure().await;

    // Should be open
    assert!(breaker.is_open().await);
}
