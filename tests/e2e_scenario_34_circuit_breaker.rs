//! E2E Scenario 34: Circuit Breaker Pattern
//!
//! **Purpose**: Validate circuit breaker for fault tolerance
//! **Coverage**: Open/Closed/Half-Open states, failure thresholds

#[cfg(test)]
mod circuit_breaker_pattern {
    use std::time::{Duration, Instant};

    // Helper to wait for circuit breaker timeout (uses blocking sleep since CircuitBreaker uses Instant)
    fn wait_for_timeout(d: Duration) {
        std::thread::sleep(d);
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum CircuitState {
        Closed,   // Normal operation
        Open,     // Failing, reject requests
        HalfOpen, // Testing if service recovered
    }

    struct CircuitBreaker {
        state: CircuitState,
        failure_count: usize,
        failure_threshold: usize,
        last_failure_time: Option<Instant>,
        timeout: Duration,
    }

    impl CircuitBreaker {
        fn new(failure_threshold: usize, timeout: Duration) -> Self {
            Self {
                state: CircuitState::Closed,
                failure_count: 0,
                failure_threshold,
                last_failure_time: None,
                timeout,
            }
        }

        fn record_success(&mut self) {
            self.failure_count = 0;
            self.state = CircuitState::Closed;
        }

        fn record_failure(&mut self) {
            self.failure_count += 1;
            self.last_failure_time = Some(Instant::now());

            if self.failure_count >= self.failure_threshold {
                self.state = CircuitState::Open;
            }
        }

        fn can_attempt(&mut self) -> bool {
            match self.state {
                CircuitState::Closed => true,
                CircuitState::HalfOpen => true,
                CircuitState::Open => {
                    // Check if timeout expired
                    if let Some(last_failure) = self.last_failure_time {
                        if Instant::now().duration_since(last_failure) > self.timeout {
                            self.state = CircuitState::HalfOpen;
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
    }

    #[tokio::test]
    async fn test_circuit_breaker_closed_state() {
        let mut cb = CircuitBreaker::new(3, Duration::from_secs(60));

        assert_eq!(cb.state, CircuitState::Closed);
        assert!(cb.can_attempt());

        // Single failure shouldn't open circuit
        cb.record_failure();
        assert_eq!(cb.state, CircuitState::Closed);
        assert!(cb.can_attempt());
    }

    #[tokio::test]
    async fn test_circuit_breaker_opens_on_threshold() {
        let mut cb = CircuitBreaker::new(3, Duration::from_secs(60));

        // Record failures up to threshold
        cb.record_failure();
        cb.record_failure();
        assert_eq!(cb.state, CircuitState::Closed);

        cb.record_failure();
        assert_eq!(cb.state, CircuitState::Open);
        assert!(!cb.can_attempt());
    }

    #[tokio::test]
    async fn test_circuit_breaker_half_open_after_timeout() {
        let mut cb = CircuitBreaker::new(2, Duration::from_millis(100));

        // Open the circuit
        cb.record_failure();
        cb.record_failure();
        assert_eq!(cb.state, CircuitState::Open);

        // Wait for timeout
        wait_for_timeout(Duration::from_millis(150));

        // Should transition to half-open
        assert!(cb.can_attempt());
        assert_eq!(cb.state, CircuitState::HalfOpen);
    }

    #[tokio::test]
    async fn test_circuit_breaker_recovery() {
        let mut cb = CircuitBreaker::new(2, Duration::from_millis(50));

        // Open the circuit
        cb.record_failure();
        cb.record_failure();
        assert_eq!(cb.state, CircuitState::Open);

        // Wait for timeout and transition to half-open
        wait_for_timeout(Duration::from_millis(75));
        assert!(cb.can_attempt());

        // Successful request should close circuit
        cb.record_success();
        assert_eq!(cb.state, CircuitState::Closed);
        assert_eq!(cb.failure_count, 0);
    }
}
