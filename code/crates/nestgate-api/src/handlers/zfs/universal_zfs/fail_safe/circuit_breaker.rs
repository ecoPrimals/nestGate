// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Provides circuit breaker functionality for fail-safe operations.

//! Circuit Breaker module

use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
// Removed unused tracing import

use crate::handlers::zfs::universal_zfs::config::CircuitBreakerConfig;
use tracing::info;
use tracing::warn;

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq, Eq)]
/// Circuitbreakerstate
pub enum CircuitBreakerState {
    /// Circuit is closed, requests flow through normally
    Closed,
    /// Circuit is open, requests are blocked due to failures
    Open,
    /// Circuit is half-open, testing if service has recovered
    HalfOpen,
}
/// Circuit breaker implementation
#[derive(Debug)]
/// Circuitbreaker
pub struct CircuitBreaker {
    config: CircuitBreakerConfig,
    state: Arc<RwLock<CircuitBreakerState>>,
    failure_count: Arc<RwLock<u32>>,
    last_failure_time: Arc<RwLock<Option<SystemTime>>>,
    half_open_calls: Arc<RwLock<u32>>,
}
impl CircuitBreaker {
    /// Create a new circuit breaker with the specified configuration
    ///
    /// # Arguments
    /// * `config` - Configuration settings for the circuit breaker
    ///
    /// # Returns
    /// * New circuit breaker instance in the closed state
    #[must_use]
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(CircuitBreakerState::Closed)),
            failure_count: Arc::new(RwLock::new(0)),
            last_failure_time: Arc::new(RwLock::new(None)),
            half_open_calls: Arc::new(RwLock::new(0)),
        }
    }

    /// Check if the circuit breaker is currently in the open state
    ///
    /// # Returns
    /// * `true` if the circuit is open (blocking requests), `false` otherwise
    pub async fn is_open(&self) -> bool {
        if !self.config.enabled {
            return false;
        }

        let state = self.state.read().await;
        matches!(*state, CircuitBreakerState::Open)
    }

    /// Check if the circuit breaker allows execution of operations
    ///
    /// # Returns
    /// * `true` if operations can be executed, `false` if they should be blocked
    pub async fn can_execute(&self) -> bool {
        if !self.config.enabled {
            return true;
        }

        let state = self.state.read().await;
        match *state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                // Check if we should transition to half-open
                let last_failure_snapshot = *self.last_failure_time.read().await;
                if let Some(last_failure) = last_failure_snapshot
                    && SystemTime::now()
                        .duration_since(last_failure)
                        .unwrap_or_default()
                        > self.config.recovery_timeout
                {
                    drop(state);
                    self.transition_to_half_open().await;
                    return true;
                }
                false
            }
            CircuitBreakerState::HalfOpen => {
                let half_open_calls = *self.half_open_calls.read().await;
                half_open_calls < self.config.half_open_max_calls
            }
        }
    }

    /// Record a successful operation
    ///
    /// Updates the circuit breaker state based on a successful operation.
    /// In half-open state, this will transition back to closed.
    /// In closed state, this resets the failure count.
    pub async fn record_success(&self) {
        if !self.config.enabled {
            return;
        }

        let state = self.state.read().await;
        match *state {
            CircuitBreakerState::HalfOpen => {
                drop(state);
                self.transition_to_closed().await;
            }
            CircuitBreakerState::Closed => {
                // Reset failure count on success
                *self.failure_count.write().await = 0;
            }
            CircuitBreakerState::Open => {
                // Should not happen, but reset just in case
                *self.failure_count.write().await = 0;
            }
        }
    }

    /// Record a failed operation
    ///
    /// Updates the circuit breaker state based on a failed operation.
    /// Increments failure count and may trigger state transitions
    /// if failure threshold is exceeded.
    pub async fn record_failure(&self) {
        if !self.config.enabled {
            return;
        }

        let mut failure_count = self.failure_count.write().await;
        *failure_count += 1;
        *self.last_failure_time.write().await = Some(SystemTime::now());

        let state = self.state.read().await;
        match *state {
            CircuitBreakerState::Closed => {
                if *failure_count >= self.config.failure_threshold {
                    drop(state);
                    self.transition_to_open().await;
                }
            }
            CircuitBreakerState::HalfOpen => {
                drop(state);
                self.transition_to_open().await;
            }
            CircuitBreakerState::Open => {
                // Already open, just update failure count
            }
        }
    }

    /// Get the current state of the circuit breaker
    ///
    /// # Returns
    /// * Current circuit breaker state (Closed, Open, or `HalfOpen`)
    pub async fn get_state(&self) -> CircuitBreakerState {
        self.state.read().await.clone()
    }

    /// Transition To Closed
    async fn transition_to_closed(&self) {
        info!("Circuit breaker transitioning to CLOSED");
        *self.state.write().await = CircuitBreakerState::Closed;
        *self.failure_count.write().await = 0;
        *self.half_open_calls.write().await = 0;
    }

    /// Transition To Open
    async fn transition_to_open(&self) {
        warn!("Circuit breaker transitioning to OPEN");
        *self.state.write().await = CircuitBreakerState::Open;
        *self.last_failure_time.write().await = Some(SystemTime::now());
    }

    /// Transition To Half Open
    async fn transition_to_half_open(&self) {
        info!("Circuit breaker transitioning to HALF-OPEN");
        *self.state.write().await = CircuitBreakerState::HalfOpen;
        *self.half_open_calls.write().await = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::zfs::universal_zfs::config::CircuitBreakerConfig;
    use std::time::Duration;

    fn cb_enabled(threshold: u32) -> CircuitBreaker {
        CircuitBreaker::new(CircuitBreakerConfig {
            enabled: true,
            failure_threshold: threshold,
            recovery_timeout: Duration::from_secs(3600),
            half_open_max_calls: 3,
        })
    }

    #[tokio::test]
    async fn disabled_breaker_always_allows_and_skips_accounting() {
        let cb = CircuitBreaker::new(CircuitBreakerConfig {
            enabled: false,
            failure_threshold: 1,
            recovery_timeout: Duration::from_secs(1),
            half_open_max_calls: 1,
        });
        assert!(!cb.is_open().await);
        assert!(cb.can_execute().await);
        cb.record_failure().await;
        cb.record_success().await;
        assert!(!cb.is_open().await);
        assert_eq!(cb.get_state().await, CircuitBreakerState::Closed);
    }

    #[tokio::test]
    async fn failures_reach_threshold_opens_circuit() {
        let cb = cb_enabled(3);
        cb.record_failure().await;
        cb.record_failure().await;
        assert!(!cb.is_open().await);
        cb.record_failure().await;
        assert!(cb.is_open().await);
        assert_eq!(cb.get_state().await, CircuitBreakerState::Open);
    }

    #[tokio::test]
    async fn success_in_closed_resets_failure_streak() {
        let cb = cb_enabled(2);
        cb.record_failure().await;
        cb.record_success().await;
        cb.record_failure().await;
        assert!(!cb.is_open().await);
        cb.record_failure().await;
        assert!(cb.is_open().await);
    }

    #[tokio::test]
    async fn circuit_breaker_state_eq_derives() {
        assert_eq!(CircuitBreakerState::Closed, CircuitBreakerState::Closed);
        assert_ne!(CircuitBreakerState::Closed, CircuitBreakerState::Open);
    }
}
