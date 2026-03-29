// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CIRCUIT BREAKER PATTERN**
//!
//! Circuit breaker implementation for preventing cascade failures.

use crate::error::NestGateError;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq, Eq)]
/// Circuitstate
pub enum CircuitState {
    /// Circuit is closed - requests pass through
    Closed,
    /// Circuit is open - requests fail fast
    Open,
    /// Circuit is half-open - testing if service recovered
    HalfOpen,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
/// Configuration for CircuitBreaker
pub struct CircuitBreakerConfig {
    /// Failure threshold to open circuit
    pub failure_threshold: u32,
    /// Success threshold to close circuit from half-open
    pub success_threshold: u32,
    /// Timeout before trying half-open
    pub timeout: Duration,
    /// Window size for failure counting
    pub window_size: Duration,
}

impl Default for CircuitBreakerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            window_size: Duration::from_secs(60),
        }
    }
}

/// Circuit breaker implementation
#[derive(Debug)]
/// Circuitbreaker
pub struct CircuitBreaker {
    /// Circuit breaker configuration
    config: CircuitBreakerConfig,
    /// Current state
    state: Arc<RwLock<CircuitState>>,
    /// Failure count in current window
    failure_count: Arc<RwLock<u32>>,
    /// Success count (for half-open state)
    success_count: Arc<RwLock<u32>>,
    /// Last failure time
    last_failure_time: Arc<RwLock<Option<Instant>>>,
    /// Window start time
    window_start: Arc<RwLock<Instant>>,
    /// Service name for logging
    service_name: String,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    #[must_use]
    pub fn new(service_name: String, config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(CircuitState::Closed)),
            failure_count: Arc::new(RwLock::new(0)),
            success_count: Arc::new(RwLock::new(0)),
            last_failure_time: Arc::new(RwLock::new(None)),
            window_start: Arc::new(RwLock::new(Instant::now())),
            service_name,
        }
    }

    /// Execute a function with circuit breaker protection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn execute<F, Fut, T>(&self, operation: F) -> Result<T, NestGateError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, NestGateError>>,
    {
        // Check if circuit allows request
        if !self.can_execute().await {
            return Err(NestGateError::Internal(Box::new(
                crate::error::variants::core_errors::InternalErrorDetails {
                    message: format!("Circuit breaker open for service: {}", self.service_name),
                    component: "circuit_breaker".to_string(),
                    location: Some(format!("{}:{}", file!(), line!())),
                    is_bug: false,
                    context: None,
                },
            )));
        }

        // Execute the operation
        match operation().await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(error) => {
                self.on_failure().await;
                Err(error)
            }
        }
    }

    /// Check if circuit allows execution
    async fn can_execute(&self) -> bool {
        let state = self.state.read().await;
        match *state {
            CircuitState::Closed | CircuitState::HalfOpen => true,
            CircuitState::Open => {
                // Check if timeout has elapsed (avoid holding `RwLockReadGuard` across `if let` body)
                let last_failure_opt = {
                    let lf = self.last_failure_time.read().await;
                    *lf
                };
                if let Some(last_failure) = last_failure_opt {
                    if last_failure.elapsed() >= self.config.timeout {
                        drop(state);
                        self.transition_to_half_open().await;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }
    }

    /// Handle successful operation
    async fn on_success(&self) {
        let state = self.state.read().await;
        match *state {
            CircuitState::HalfOpen => {
                let mut success_count = self.success_count.write().await;
                *success_count += 1;

                if *success_count >= self.config.success_threshold {
                    drop(state);
                    drop(success_count);
                    self.transition_to_closed().await;
                }
            }
            CircuitState::Closed => {
                // Reset failure count on success
                self.reset_failure_count().await;
            }
            CircuitState::Open => {
                // Should not happen, but handle gracefully
                debug!(
                    "Success received while circuit open for {}",
                    self.service_name
                );
            }
        }
    }

    /// Handle failed operation
    async fn on_failure(&self) {
        let state = self.state.read().await;
        match *state {
            CircuitState::Closed => {
                self.increment_failure_count().await;
                let failure_count = *self.failure_count.read().await;

                if failure_count >= self.config.failure_threshold {
                    drop(state);
                    self.transition_to_open().await;
                }
            }
            CircuitState::HalfOpen => {
                drop(state);
                self.transition_to_open().await;
            }
            CircuitState::Open => {
                // Update last failure time
                let mut last_failure = self.last_failure_time.write().await;
                *last_failure = Some(Instant::now());
            }
        }
    }

    /// Transition to closed state
    async fn transition_to_closed(&self) {
        let mut state = self.state.write().await;
        *state = CircuitState::Closed;

        // Reset counters
        let mut failure_count = self.failure_count.write().await;
        *failure_count = 0;
        let mut success_count = self.success_count.write().await;
        *success_count = 0;

        info!("Circuit breaker closed for service: {}", self.service_name);
    }

    /// Transition to open state
    async fn transition_to_open(&self) {
        let mut state = self.state.write().await;
        *state = CircuitState::Open;

        let mut last_failure = self.last_failure_time.write().await;
        *last_failure = Some(Instant::now());

        warn!("Circuit breaker opened for service: {}", self.service_name);
    }

    /// Transition to half-open state
    async fn transition_to_half_open(&self) {
        let mut state = self.state.write().await;
        *state = CircuitState::HalfOpen;

        let mut success_count = self.success_count.write().await;
        *success_count = 0;

        info!(
            "Circuit breaker half-open for service: {}",
            self.service_name
        );
    }

    /// Increment failure count
    async fn increment_failure_count(&self) {
        self.reset_window_if_needed().await;

        let mut failure_count = self.failure_count.write().await;
        *failure_count += 1;

        debug!(
            "Failure count for {}: {}",
            self.service_name, *failure_count
        );
    }

    /// Reset failure count
    async fn reset_failure_count(&self) {
        let mut failure_count = self.failure_count.write().await;
        if *failure_count > 0 {
            *failure_count = 0;
            debug!("Reset failure count for {}", self.service_name);
        }
    }

    /// Reset window if needed
    async fn reset_window_if_needed(&self) {
        let window_start = self.window_start.read().await;
        if window_start.elapsed() >= self.config.window_size {
            drop(window_start);

            let mut window_start = self.window_start.write().await;
            *window_start = Instant::now();

            let mut failure_count = self.failure_count.write().await;
            *failure_count = 0;

            debug!("Reset failure window for {}", self.service_name);
        }
    }

    /// Get current circuit state
    pub async fn state(&self) -> CircuitState {
        self.state.read().await.clone()
    }

    /// Get current failure count
    pub async fn failure_count(&self) -> u32 {
        *self.failure_count.read().await
    }

    /// Get circuit breaker metrics
    pub async fn metrics(&self) -> CircuitBreakerMetrics {
        CircuitBreakerMetrics {
            service_name: self.service_name.clone(),
            state: self.state().await,
            failure_count: self.failure_count().await,
            success_count: *self.success_count.read().await,
            last_failure_time: *self.last_failure_time.read().await,
        }
    }
}

/// Circuit breaker metrics
#[derive(Debug, Clone)]
/// Circuitbreakermetrics
pub struct CircuitBreakerMetrics {
    /// Service name
    pub service_name: String,
    /// State
    pub state: CircuitState,
    /// Count of failure
    pub failure_count: u32,
    /// Count of success
    pub success_count: u32,
    /// Last Failure Time
    pub last_failure_time: Option<Instant>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_circuit_breaker_closed_to_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 1,
            timeout: Duration::from_millis(100),
            window_size: Duration::from_secs(60),
        };

        let cb = CircuitBreaker::new("test_service".to_string(), config);

        // Initially closed
        assert_eq!(cb.state().await, CircuitState::Closed);

        // First failure
        let result: Result<(), _> = cb
            .execute(|| async {
                Err(NestGateError::Internal(Box::new(
                    crate::error::variants::core_errors::InternalErrorDetails {
                        message: "Test error".to_string(),
                        component: "test".to_string(),
                        location: None,
                        is_bug: false,
                        context: None,
                    },
                )))
            })
            .await;
        assert!(result.is_err());
        assert_eq!(cb.state().await, CircuitState::Closed);

        // Second failure - should open circuit
        let result: Result<(), _> = cb
            .execute(|| async {
                Err(NestGateError::Internal(Box::new(
                    crate::error::variants::core_errors::InternalErrorDetails {
                        message: "Test error".to_string(),
                        component: "test".to_string(),
                        location: None,
                        is_bug: false,
                        context: None,
                    },
                )))
            })
            .await;
        assert!(result.is_err());
        assert_eq!(cb.state().await, CircuitState::Open);

        // Next request should fail fast
        let result = cb.execute(|| async { Ok("success") }).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Circuit breaker open")
        );
    }

    #[tokio::test]
    async fn test_circuit_breaker_recovery() {
        let config = CircuitBreakerConfig {
            failure_threshold: 1,
            success_threshold: 1,
            timeout: Duration::from_millis(50),
            window_size: Duration::from_secs(60),
        };

        let cb = CircuitBreaker::new("test_service".to_string(), config);

        // Fail to open circuit
        let result: Result<(), _> = cb
            .execute(|| async {
                Err(NestGateError::Internal(Box::new(
                    crate::error::variants::core_errors::InternalErrorDetails {
                        message: "Test error".to_string(),
                        component: "test".to_string(),
                        location: None,
                        is_bug: false,
                        context: None,
                    },
                )))
            })
            .await;
        assert!(result.is_err());
        assert_eq!(cb.state().await, CircuitState::Open);

        // Wait for timeout
        sleep(Duration::from_millis(60)).await;

        // Should succeed and close circuit
        let result = cb.execute(|| async { Ok("success") }).await;
        assert!(result.is_ok());
        assert_eq!(result.expect("Operation failed"), "success");
        assert_eq!(cb.state().await, CircuitState::Closed);
    }
}
