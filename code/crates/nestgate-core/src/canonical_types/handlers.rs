// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **HANDLER CONFIGURATION TYPES** — Unified handler configuration system

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Universal handler configuration pattern
/// Replaces all scattered handler-specific config structs
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for UniversalHandler
pub struct UniversalHandlerConfig<T = ()> {
    /// Handler identification
    pub handler_id: String,
    /// Enable/disable handler
    pub enabled: bool,
    /// Request timeout
    pub timeout: Duration,
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
    /// Rate limiting configuration
    pub rate_limit: RateLimitConfig,
    /// Retry configuration
    pub retry: RetryConfig,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
    /// Handler-specific configuration
    pub specific: T,
}
/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for RateLimit
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst size
    pub burst_size: u32,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Retry
pub struct RetryConfig {
    /// Enable retries
    pub enabled: bool,
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Base delay between retries
    pub base_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Use exponential backoff
    pub exponential_backoff: bool,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Monitoring
pub struct MonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Enable performance monitoring
    pub performance_monitoring: bool,
}

// Default implementations
impl<T: Default> Default for UniversalHandlerConfig<T> {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            handler_id: String::new(),
            enabled: true,
            timeout: Duration::from_secs(
                crate::canonical_modernization::canonical_constants::handlers::DEFAULT_HANDLER_TIMEOUT_SECS,
            ),
            max_concurrent_requests: crate::canonical_modernization::canonical_constants::handlers::MAX_CONCURRENT_REQUESTS,
            rate_limit: RateLimitConfig::default(),
            retry: RetryConfig::default(),
            monitoring: MonitoringConfig::default(),
            specific: T::default(),
        }
    }
}

impl Default for RateLimitConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            requests_per_minute: crate::canonical_modernization::canonical_constants::handlers::DEFAULT_RATE_LIMIT_RPM,
            burst_size: crate::canonical_modernization::canonical_constants::handlers::DEFAULT_RATE_LIMIT_BURST,
        }
    }
}

impl Default for RetryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: crate::canonical_modernization::canonical_constants::handlers::DEFAULT_RETRY_ATTEMPTS,
            base_delay: Duration::from_millis(
                crate::canonical_modernization::canonical_constants::handlers::DEFAULT_RETRY_DELAY_MS,
            ),
            max_delay: Duration::from_secs(30),
            exponential_backoff: true,
        }
    }
}

impl Default for MonitoringConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval: Duration::from_secs(
                crate::canonical_modernization::canonical_constants::handlers::METRICS_COLLECTION_INTERVAL_SECS,
            ),
            health_check_interval: Duration::from_secs(
                crate::canonical_modernization::canonical_constants::handlers::HEALTH_CHECK_INTERVAL_SECS,
            ),
            performance_monitoring: true,
        }
    }
}
