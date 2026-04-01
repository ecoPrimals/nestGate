// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module provides timeout discovery functionality,
// extracted from the monolithic unified_dynamic_config.rs file.
//
// **PROVIDES**:
// - Dynamic timeout discovery settings
// - Timeout cache management
// - Adaptive timeout strategies
// - Integration with universal adapter patterns
//
// **EXTRACTED FROM**: unified_dynamic_config.rs timeout-related sections

//! Timeout module

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== SECTION ====================

/// Timeout discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Timeoutdiscoverysettings
pub struct TimeoutDiscoverySettings {
    /// Enable dynamic timeout discovery
    pub enable_dynamic_timeouts: bool,
    /// Timeout cache TTL
    pub cache_ttl: Duration,
    /// Default timeout for operations
    pub default_timeout: Duration,
    /// Maximum allowed timeout
    pub max_timeout: Duration,
    /// Minimum allowed timeout
    pub min_timeout: Duration,
    /// Adaptive timeout adjustment factor
    pub adjustment_factor: f64,
}
impl Default for TimeoutDiscoverySettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_dynamic_timeouts: true,
            cache_ttl: Duration::from_secs(300), // 5 minutes
            default_timeout: Duration::from_secs(30),
            max_timeout: Duration::from_secs(300),
            min_timeout: Duration::from_secs(1),
            adjustment_factor: 1.2,
        }
    }
}

impl TimeoutDiscoverySettings {
    /// Create new timeout discovery settings with defaults
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create timeout settings with custom default timeout
    #[must_use]
    pub fn with_default_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Create timeout settings with custom cache TTL
    #[must_use]
    pub fn with_cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }

    /// Validate timeout settings
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        if self.min_timeout > self.max_timeout {
            return Err(nestgate_types::error::NestGateError::validation_error(
                "Invalid argument",
            ));
        }

        if self.default_timeout > self.max_timeout || self.default_timeout < self.min_timeout {
            return Err(nestgate_types::error::NestGateError::validation_error(
                "Invalid argument",
            ));
        }

        Ok(())
    }

    /// Calculate adaptive timeout based on historical performance
    #[must_use]
    pub fn calculate_adaptive_timeout(
        &self,
        base_timeout: Duration,
        success_rate: f64,
    ) -> Duration {
        if !self.enable_dynamic_timeouts {
            return self.default_timeout;
        }

        let adjustment = if success_rate > 0.9 {
            1.0 / self.adjustment_factor // Reduce timeout for high success rate
        } else if success_rate < 0.7 {
            self.adjustment_factor // Increase timeout for low success rate
        } else {
            1.0 // No adjustment for moderate success rate
        };

        let adjusted = Duration::from_secs_f64(base_timeout.as_secs_f64() * adjustment);

        // Clamp to min/max bounds
        if adjusted > self.max_timeout {
            self.max_timeout
        } else if adjusted < self.min_timeout {
            self.min_timeout
        } else {
            adjusted
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::time::Duration;

    #[test]
    fn default_settings_validate() -> Result<()> {
        let s = TimeoutDiscoverySettings::default();
        s.validate()?;
        Ok(())
    }

    #[test]
    fn validate_rejects_min_greater_than_max() -> Result<()> {
        let mut s = TimeoutDiscoverySettings::default();
        s.min_timeout = Duration::from_secs(10);
        s.max_timeout = Duration::from_secs(5);
        assert!(s.validate().is_err());
        Ok(())
    }

    #[test]
    fn validate_rejects_default_outside_bounds() -> Result<()> {
        let mut s = TimeoutDiscoverySettings::default();
        s.default_timeout = Duration::from_secs(500);
        assert!(s.validate().is_err());
        let mut s2 = TimeoutDiscoverySettings::default();
        s2.default_timeout = Duration::from_millis(0);
        assert!(s2.validate().is_err());
        Ok(())
    }

    #[test]
    fn adaptive_timeout_disabled_returns_default() -> Result<()> {
        let mut s = TimeoutDiscoverySettings::default();
        s.enable_dynamic_timeouts = false;
        let out = s.calculate_adaptive_timeout(Duration::from_secs(10), 0.99);
        assert_eq!(out, s.default_timeout);
        Ok(())
    }

    #[test]
    fn adaptive_timeout_clamps_to_bounds() -> Result<()> {
        let mut s = TimeoutDiscoverySettings::default();
        s.enable_dynamic_timeouts = true;
        s.adjustment_factor = 10.0;
        s.min_timeout = Duration::from_secs(2);
        s.max_timeout = Duration::from_secs(20);
        // Low success rate increases timeout; very large base should clamp to max
        let hi = s.calculate_adaptive_timeout(Duration::from_secs(100), 0.5);
        assert_eq!(hi, s.max_timeout);
        // High success rate decreases timeout; small base should clamp to min
        let lo = s.calculate_adaptive_timeout(Duration::from_secs(1), 0.95);
        assert_eq!(lo, s.min_timeout);
        Ok(())
    }
}
