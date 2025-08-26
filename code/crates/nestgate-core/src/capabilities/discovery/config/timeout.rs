use crate::NestGateError;
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

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== TIMEOUT DISCOVERY SETTINGS ====================

/// Timeout discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn new() -> Self {
        Self::default()
    }

    /// Create timeout settings with custom default timeout
    pub fn with_default_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Create timeout settings with custom cache TTL
    pub fn with_cache_ttl(mut self, ttl: Duration) -> Self {
        self.cache_ttl = ttl;
        self
    }

    /// Validate timeout settings
    pub fn validate(&self) -> crate::Result<()> {
        if self.min_timeout > self.max_timeout {
            return Err(crate::error::NestGateError::validation_error(
                "timeout_validation",
                "Minimum timeout cannot be greater than maximum timeout",
                None
            ));
        }

        if self.default_timeout > self.max_timeout || self.default_timeout < self.min_timeout {
            return Err(crate::error::NestGateError::validation_error(
                "timeout_validation",
                "Default timeout must be within min/max range",
                None
            ));
        }

        Ok(())
    }

    /// Calculate adaptive timeout based on historical performance
    pub fn calculate_adaptive_timeout(&self, base_timeout: Duration, success_rate: f64) -> Duration {
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