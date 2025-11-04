use crate::error::NestGateError;
//
// Implements the Circuit Breaker pattern to prevent cascading failures by
// temporarily blocking calls to failing services.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    /// Closed: Normal operation, requests pass through
    Closed,
    /// Open: Failing service, requests are blocked
    Open,
    /// Half-Open: Testing if service has recovered
    HalfOpen,
}
/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Failure threshold to trip the circuit
    pub failure_threshold: u32,
    /// Success threshold to close the circuit from half-open
    pub success_threshold: u32,
    /// Timeout before transitioning from open to half-open
    pub timeout: Duration,
    /// Minimum number of requests before considering failure rate
    pub minimum_requests: u32,
    /// Failure rate threshold (0.0 to 1.0)
    pub failure_rate_threshold: f64,
    /// Time window for calculating failure rate
    pub time_window: Duration,
}
impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            minimum_requests: 10,
            failure_rate_threshold: 0.5,
            time_window: Duration::from_secs(60),
        }
    }
}

/// Circuit breaker implementation with idiomatic error handling
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    name: String,
    config: CircuitBreakerConfig,
    state: Arc<RwLock<CircuitBreakerData>>,
}
/// Circuit breaker error types
#[derive(Debug, thiserror::Error)]
pub enum CircuitBreakerError {
    #[error("Circuit breaker '{name}' is open - requests blocked")]
    CircuitOpen { name: String },
    #[error("Circuit breaker configuration error: {message}")]
    Configuration { message: String },

    #[error("Circuit breaker internal error: {message}")]
    Internal { message: String },
}

impl From<CircuitBreakerError> for NestGateError {
    fn from(err: CircuitBreakerError) -> Self {
        match err {
            CircuitBreakerError::CircuitOpen { name } => {
                NestGateError::ResilienceError {
                    message: format!("Circuit breaker '{}' is open - requests blocked", name),
                    source: None,
                }
            }
            CircuitBreakerError::Configuration { message } => {
                NestGateError::ConfigurationError {
                    message,
                    source: None,
                }
            }
            CircuitBreakerError::Internal { message } => {
                NestGateError::InternalError {
                    message,
                    source: None,
                }
            }
        }
    }
}

#[derive(Debug)]
struct CircuitBreakerData {
    state: CircuitBreakerState,
    failure_count: u32,
    success_count: u32,
    last_failure_time: Option<Instant>,
    request_history: Vec<RequestRecord>,
    metrics: CircuitBreakerMetrics,
}

#[derive(Debug, Clone)]
struct RequestRecord {
    timestamp: Instant,
    success: bool,
}

/// Circuit breaker metrics
#[derive(Debug, Clone)]
pub struct CircuitBreakerMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub trip_count: u64,
    pub current_state: CircuitBreakerState,
    pub last_trip_time: Option<Instant>,
}
impl CircuitBreaker {
    /// Create a new circuit breaker
    #[must_use]
    pub fn new(name: String, config: CircuitBreakerConfig) -> Self {
        let data = CircuitBreakerData {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
            request_history: Vec::new(),
            metrics: CircuitBreakerMetrics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                trip_count: 0,
                current_state: CircuitBreakerState::Closed,
                last_trip_time: None,
            },
        };

        Self {
            name,
            config,
            state: Arc::new(RwLock::new(data)),
        }
    }

    /// Check if the circuit breaker allows execution
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn can_execute(&self) -> Result<bool, CircuitBreakerError>  {
        let mut data = self.state.write().await;

        match data.state {
            CircuitBreakerState::Closed => Ok(true),
            CircuitBreakerState::Open => {
                // Check if timeout has elapsed
                if let Some(last_failure) = data.last_failure_time {
                    if last_failure.elapsed() >= self.config.timeout {
                        // Transition to half-open
                        data.state = CircuitBreakerState::HalfOpen;
                        data.success_count = 0;
                        data.failure_count = 0;
                        tracing::info!(
                            "Circuit breaker '{}' transitioning to half-open",
                            self.name
                        );
                        Ok(true)
                    } else {
                        Err(CircuitBreakerError::CircuitOpen {
                            name: self.name.clone(),
                        })
                    }
                } else {
                    Err(CircuitBreakerError::CircuitOpen {
                        name: self.name.clone(),
                    })
                }
            }
            CircuitBreakerState::HalfOpen => Ok(true),
        }
    }

    /// Record a successful operation
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn record_success(&self) -> Result<(), CircuitBreakerError>  {
        let mut data = self.state.write().await;

        data.metrics.total_requests += 1;
        data.metrics.successful_requests += 1;

        // Add to request history
        data.request_history.push(RequestRecord {
            timestamp: Instant::now(),
            success: true,
        );

        // Clean old history
        self.clean_request_history(&mut data).await;

        match data.state {
            CircuitBreakerState::Closed => {
                // Reset failure count on success
                data.failure_count = 0;
            }
            CircuitBreakerState::HalfOpen => {
                data.success_count += 1;
                if data.success_count >= self.config.success_threshold {
                    // Transition to closed
                    data.state = CircuitBreakerState::Closed;
                    data.failure_count = 0;
                    data.success_count = 0;
                    data.metrics.current_state = CircuitBreakerState::Closed;
                    tracing::info!(
                        "Circuit breaker '{}' closed after successful recovery",
                        self.name
                    );
                }
            }
            CircuitBreakerState::Open => {
                // Should not happen if can_execute is called first
                tracing::warn!("Recording success on open circuit breaker '{}'", self.name);
            }
        }

        Ok(())
    }

    /// Record a failed operation
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn record_failure(&self) -> Result<(), CircuitBreakerError>  {
        let mut data = self.state.write().await;

        data.metrics.total_requests += 1;
        data.metrics.failed_requests += 1;
        data.last_failure_time = Some(Instant::now());

        // Add to request history
        data.request_history.push(RequestRecord {
            timestamp: Instant::now(),
            success: false,
        );

        // Clean old history
        self.clean_request_history(&mut data).await;

        match data.state {
            CircuitBreakerState::Closed => {
                data.failure_count += 1;

                // Check if we should trip the circuit
                if self.should_trip(&data).await {
                    data.state = CircuitBreakerState::Open;
                    data.metrics.current_state = CircuitBreakerState::Open;
                    data.metrics.trip_count += 1;
                    data.metrics.last_trip_time = Some(Instant::now());
                    tracing::warn!("Circuit breaker '{}' tripped due to failures", self.name);
                }
            }
            CircuitBreakerState::HalfOpen => {
                // Transition back to open on any failure in half-open state
                data.state = CircuitBreakerState::Open;
                data.metrics.current_state = CircuitBreakerState::Open;
                data.metrics.trip_count += 1;
                data.metrics.last_trip_time = Some(Instant::now());
                data.failure_count = 0;
                data.success_count = 0;
                tracing::warn!(
                    "Circuit breaker '{}' reopened after failure in half-open state",
                    self.name
                );
            }
            CircuitBreakerState::Open => {
                // Already open, just record the failure
            }
        }

        Ok(())
    }

    /// Get current circuit breaker state
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_state(&self) -> Result<CircuitBreakerState, CircuitBreakerError>  {
        let data = self.state.read().await;
        Ok(data.state.clone())
    }

    /// Get circuit breaker metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_metrics(&self) -> Result<CircuitBreakerMetrics, CircuitBreakerError>  {
        let data = self.state.read().await;
        Ok(data.metrics.clone())
    }

    /// Check if circuit should trip based on failure rate
    async fn should_trip(&self, data: &CircuitBreakerData) -> bool {
        // Check simple failure count threshold
        if data.failure_count >= self.config.failure_threshold {
            return true;
        }

        // Check failure rate threshold
        if data.request_history.len() >= self.config.minimum_requests as usize {
            let failures = data
                .request_history
                .iter()
                .filter(|record| !record.success)
                .count();

            let failure_rate = failures as f64 / data.(request_history.len() as f64);

            if failure_rate >= self.config.failure_rate_threshold {
                return true;
            }
        }

        false
    }

    /// Clean old request history outside the time window
    async fn clean_request_history(&self, data: &mut CircuitBreakerData) {
        let cutoff = Instant::now() - self.config.time_window;
        data.request_history
            .retain(|record| record.timestamp > cutoff);
    }

    /// Force circuit breaker to open (for testing/manual intervention)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn force_open(&self) -> Result<(), CircuitBreakerError>  {
        let mut data = self.state.write().await;
        data.state = CircuitBreakerState::Open;
        data.metrics.current_state = CircuitBreakerState::Open;
        data.metrics.trip_count += 1;
        data.metrics.last_trip_time = Some(Instant::now());
        data.last_failure_time = Some(Instant::now());
        tracing::warn!("Circuit breaker '{}' manually forced open", self.name);
        Ok(())
    }

    /// Force circuit breaker to close (for testing/manual intervention)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn force_close(&self) -> Result<(), CircuitBreakerError>  {
        let mut data = self.state.write().await;
        data.state = CircuitBreakerState::Closed;
        data.metrics.current_state = CircuitBreakerState::Closed;
        data.failure_count = 0;
        data.success_count = 0;
        tracing::info!("Circuit breaker '{}' manually forced closed", self.name);
        Ok(())
    }

    /// Get detailed circuit breaker status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_detailed_status(&self) -> Result<CircuitBreakerStatus, CircuitBreakerError>  {
        let data = self.state.read().await;

        let recent_failures = data
            .request_history
            .iter()
            .filter(|record| !record.success)
            .count();

        let failure_rate = if !data.request_history.is_empty() {
            recent_failures as f64 / data.(request_history.len() as f64)
        } else {
            0.0
        };

        Ok(CircuitBreakerStatus {
            name: self.name.clone(),
            state: data.state.clone(),
            failure_count: data.failure_count,
            success_count: data.success_count,
            recent_failure_rate: failure_rate,
            total_requests: data.request_history.len() as u64,
            last_failure_time: data.last_failure_time,
            metrics: data.metrics.clone(),
        })
    }
}

/// Detailed circuit breaker status
#[derive(Debug, Clone)]
pub struct CircuitBreakerStatus {
    pub name: String,
    pub state: CircuitBreakerState,
    pub failure_count: u32,
    pub success_count: u32,
    pub recent_failure_rate: f64,
    pub total_requests: u64,
    pub last_failure_time: Option<Instant>,
    pub metrics: CircuitBreakerMetrics,
}
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_circuit_breaker_closed_to_open() {
        let config = CircuitBreakerConfig {
            failure_threshold: 3,
            ..Default::default()
        };

        let cb = CircuitBreaker::new("test".to_string(), config);

        // Should start closed
        assert_eq!(cb.get_state().await.expect("Operation failed"), CircuitBreakerState::Closed);
        assert!(cb.can_execute().await.expect("Operation failed"));

        // Record failures
        cb.record_failure().await.expect("Operation failed");
        cb.record_failure().await.expect("Operation failed");
        cb.record_failure().await.expect("Operation failed");

        // Should now be open
        assert_eq!(cb.get_state().await.expect("Operation failed"), CircuitBreakerState::Open);
        assert!(!cb.can_execute().await.expect("Operation failed"));
    }

    #[tokio::test]
    async fn test_circuit_breaker_recovery() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            success_threshold: 2,
            timeout: Duration::from_millis(100),
            ..Default::default()
        };

        let cb = CircuitBreaker::new("test".to_string(), config);

        // Trip the circuit
        cb.record_failure().await.expect("Operation failed");
        cb.record_failure().await.expect("Operation failed");
        assert_eq!(cb.get_state().await.expect("Operation failed"), CircuitBreakerState::Open);

        // Wait for timeout
        sleep(Duration::from_millis(150)).await;

        // Should transition to half-open
        assert!(cb.can_execute().await.expect("Operation failed"));
        assert_eq!(cb.get_state().await.expect("Operation failed"), CircuitBreakerState::HalfOpen);

        // Record successes
        cb.record_success().await.expect("Operation failed");
        cb.record_success().await.expect("Operation failed");

        // Should now be closed
        assert_eq!(cb.get_state().await.expect("Operation failed"), CircuitBreakerState::Closed);
    }
}
