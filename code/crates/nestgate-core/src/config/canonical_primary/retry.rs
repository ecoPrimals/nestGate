// **CANONICAL RETRY CONFIGURATION**
//! Retry configuration functionality and utilities.
//! Consolidates all retry patterns across the system.
//! **PROBLEM SOLVED**: Eliminates duplicate retry logic across 7+ different implementations

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== RETRY CONFIGURATION ====================

/// **THE** canonical retry configuration - consolidates all retry patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Base delay between retries
    pub base_delay: Duration,
    /// Maximum delay between retries (for exponential backoff)
    pub max_delay: Duration,
    /// Backoff multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Whether to add random jitter to delays
    pub jitter_enabled: bool,
    /// Jitter factor (0.0 to 1.0)
    pub jitter_factor: f64,
    /// Whether to use exponential backoff
    pub exponential_backoff: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter_enabled: true,
            jitter_factor: 0.1,
            exponential_backoff: true,
        }
    }
}

impl RetryConfig {
    /// Get retry delay for a specific attempt number
    #[must_use]
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::from_millis(0);
        }

        let delay = if self.exponential_backoff {
            let base_delay_ms = self.base_delay.as_millis() as f64;
            let exponential_delay =
                base_delay_ms * self.backoff_multiplier.powi(attempt as i32 - 1);
            Duration::from_millis(exponential_delay as u64)
        } else {
            self.base_delay
        };

        // Cap at max delay
        let delay = std::cmp::min(delay, self.max_delay);

        // Add jitter if enabled
        if self.jitter_enabled {
            let jitter_ms = (delay.as_millis() as f64 * self.jitter_factor * 0.5) as u64;
            delay + Duration::from_millis(jitter_ms)
        } else {
            delay
        }
    }

    /// Create a fast retry configuration for low-latency operations
    #[must_use]
    pub fn fast() -> Self {
        Self {
            max_attempts: 5,
            base_delay: Duration::from_millis(50),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 1.5,
            jitter_enabled: true,
            jitter_factor: 0.1,
            exponential_backoff: true,
        }
    }

    /// Create a slow retry configuration for high-latency operations
    #[must_use]
    pub fn slow() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            backoff_multiplier: 3.0,
            jitter_enabled: true,
            jitter_factor: 0.15,
            exponential_backoff: true,
        }
    }

    /// Create a linear retry configuration (no exponential backoff)
    #[must_use]
    pub fn linear() -> Self {
        Self {
            max_attempts: 4,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 1.0,
            jitter_enabled: false,
            jitter_factor: 0.0,
            exponential_backoff: false,
        }
    }

    /// Create a high-frequency retry configuration for very frequent operations
    #[must_use]
    pub fn high_frequency() -> Self {
        Self {
            max_attempts: 10,
            base_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(500),
            backoff_multiplier: 1.2,
            jitter_enabled: true,
            jitter_factor: 0.05,
            exponential_backoff: true,
        }
    }

    /// Create a critical operations retry configuration with extended patience
    #[must_use]
    pub fn critical_operations() -> Self {
        Self {
            max_attempts: 8,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.5,
            jitter_enabled: true,
            jitter_factor: 0.2,
            exponential_backoff: true,
        }
    }
}

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for UnifiedRetryConfig
pub type UnifiedRetryConfig = RetryConfig;

