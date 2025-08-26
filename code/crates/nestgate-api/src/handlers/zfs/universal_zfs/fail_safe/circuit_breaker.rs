//
// Provides circuit breaker functionality for fail-safe operations.

use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
// Removed unused tracing import

use crate::handlers::zfs::universal_zfs::config::CircuitBreakerConfig;
use tracing::info;
use tracing::warn;

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
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
                if let Some(last_failure) = *self.last_failure_time.read().await {
                    if SystemTime::now()
                        .duration_since(last_failure)
                        .unwrap_or_default()
                        > self.config.recovery_timeout
                    {
                        drop(state);
                        self.transition_to_half_open().await;
                        return true;
                    }
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
    /// * Current circuit breaker state (Closed, Open, or HalfOpen)
    pub async fn get_state(&self) -> CircuitBreakerState {
        self.state.read().await.clone()
    }

    async fn transition_to_closed(&self) {
        info!("Circuit breaker transitioning to CLOSED");
        *self.state.write().await = CircuitBreakerState::Closed;
        *self.failure_count.write().await = 0;
        *self.half_open_calls.write().await = 0;
    }

    async fn transition_to_open(&self) {
        warn!("Circuit breaker transitioning to OPEN");
        *self.state.write().await = CircuitBreakerState::Open;
        *self.last_failure_time.write().await = Some(SystemTime::now());
    }

    async fn transition_to_half_open(&self) {
        info!("Circuit breaker transitioning to HALF-OPEN");
        *self.state.write().await = CircuitBreakerState::HalfOpen;
        *self.half_open_calls.write().await = 0;
    }
}
