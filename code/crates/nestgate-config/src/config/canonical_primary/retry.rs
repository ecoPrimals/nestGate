// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **CANONICAL RETRY CONFIGURATION**
//! Retry configuration functionality and utilities.
//! Consolidates all retry patterns across the system.
//! **PROBLEM SOLVED**: Eliminates duplicate retry logic across 7+ different implementations

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== RETRY CONFIGURATION ====================

/// **THE** canonical retry configuration - consolidates all retry patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Retry
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
    /// Returns the default instance
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
    #[expect(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap
    )]
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
    pub const fn fast() -> Self {
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
    pub const fn slow() -> Self {
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
    pub const fn linear() -> Self {
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
    pub const fn high_frequency() -> Self {
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
    pub const fn critical_operations() -> Self {
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

/// Backward compatibility alias for `UnifiedRetryConfig`
pub type UnifiedRetryConfig = RetryConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_values() {
        let cfg = RetryConfig::default();
        assert_eq!(cfg.max_attempts, 3);
        assert_eq!(cfg.base_delay, Duration::from_millis(100));
        assert_eq!(cfg.max_delay, Duration::from_secs(30));
        assert!(cfg.jitter_enabled);
        assert!(cfg.exponential_backoff);
    }

    #[test]
    fn attempt_zero_returns_zero_delay() {
        let cfg = RetryConfig::default();
        assert_eq!(cfg.delay_for_attempt(0), Duration::from_millis(0));
    }

    #[test]
    fn exponential_backoff_increases_delay() {
        let cfg = RetryConfig {
            jitter_enabled: false,
            ..RetryConfig::default()
        };
        let d1 = cfg.delay_for_attempt(1);
        let d2 = cfg.delay_for_attempt(2);
        let d3 = cfg.delay_for_attempt(3);
        assert!(d2 > d1, "attempt 2 should be longer than attempt 1");
        assert!(d3 > d2, "attempt 3 should be longer than attempt 2");
    }

    #[test]
    fn delay_capped_at_max() {
        let cfg = RetryConfig {
            max_delay: Duration::from_millis(200),
            jitter_enabled: false,
            ..RetryConfig::default()
        };
        let d = cfg.delay_for_attempt(100);
        assert!(d <= Duration::from_millis(200));
    }

    #[test]
    fn linear_mode_constant_delay() {
        let cfg = RetryConfig::linear();
        let d1 = cfg.delay_for_attempt(1);
        let d2 = cfg.delay_for_attempt(2);
        let d3 = cfg.delay_for_attempt(3);
        assert_eq!(d1, d2);
        assert_eq!(d2, d3);
    }

    #[test]
    fn jitter_adds_positive_offset() {
        let no_jitter = RetryConfig {
            jitter_enabled: false,
            ..RetryConfig::default()
        };
        let with_jitter = RetryConfig::default();
        let base = no_jitter.delay_for_attempt(1);
        let jittered = with_jitter.delay_for_attempt(1);
        assert!(jittered >= base, "jitter should not reduce delay");
    }

    #[test]
    fn preset_constructors_have_distinct_settings() {
        let fast = RetryConfig::fast();
        let slow = RetryConfig::slow();
        let hf = RetryConfig::high_frequency();
        let crit = RetryConfig::critical_operations();

        assert!(fast.base_delay < slow.base_delay);
        assert!(hf.max_attempts > fast.max_attempts);
        assert!(crit.max_attempts > slow.max_attempts);
    }

    #[test]
    fn serde_roundtrip() {
        let cfg = RetryConfig::fast();
        let json = serde_json::to_string(&cfg).expect("serialize");
        let decoded: RetryConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded.max_attempts, cfg.max_attempts);
        assert_eq!(decoded.base_delay, cfg.base_delay);
    }
}
