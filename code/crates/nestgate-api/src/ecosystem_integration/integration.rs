//! **INTEGRATION PREFERENCES**
//!
//! Integration preferences and configuration management.

use super::types::{IntegrationPreferences, RateLimitSpec, CircuitBreakerSpec};

impl Default for RateLimitSpec {
    fn default() -> Self {
        Self {
            rps: Some(100),
            burst: Some(200),
            window_seconds: Some(60),
        }
    }
}

impl Default for CircuitBreakerSpec {
    fn default() -> Self {
        Self {
            failure_threshold: Some(5),
            timeout_seconds: Some(30),
            recovery_seconds: Some(60),
        }
    }
}

/// Integration preferences manager
pub struct IntegrationManager {
    preferences: IntegrationPreferences,
}
impl IntegrationManager {
    /// Create new integration manager
    pub fn new() -> Self {
        Self {
            preferences: IntegrationPreferences::default(),
        }
    }

    /// Set rate limiting
    pub fn set_rate_limiting(&mut self, spec: RateLimitSpec) {
        self.preferences.rate_limiting = Some(spec);
    }

    /// Set circuit breaker
    pub fn set_circuit_breaker(&mut self, spec: CircuitBreakerSpec) {
        self.preferences.circuit_breaker = Some(spec);
    }

    /// Get preferences
    pub fn get_preferences(&self) -> &IntegrationPreferences {
        &self.preferences
    }

    /// Validate preferences
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if let Some(rate_limit) = &self.preferences.rate_limiting {
            if let Some(rps) = rate_limit.rps {
                if rps == 0 {
                    errors.push("Rate limit RPS cannot be zero".to_string());
                }
            }
        }

        if let Some(circuit_breaker) = &self.preferences.circuit_breaker {
            if let Some(threshold) = circuit_breaker.failure_threshold {
                if threshold == 0 {
                    errors.push("Circuit breaker failure threshold cannot be zero".to_string());
                }
            }
        }

        errors
    }
} 