//
// Performance tuning and resilience configuration types for the NestGate API.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Performance configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Apiperformancesettings
pub struct ApiPerformanceSettings {
    /// Enable request caching
    pub enable_request_caching: bool,
    /// Cache TTL
    pub cache_ttl: Duration,
    /// Maximum cache size (entries)
    pub max_cache_size: usize,
    /// Enable response compression
    pub enable_response_compression: bool,
    /// Response compression threshold (bytes)
    pub compression_threshold: usize,
    /// Enable connection pooling
    pub enable_connection_pooling: bool,
    /// Connection pool size
    pub connection_pool_size: usize,
    /// Connection pool timeout
    pub connection_pool_timeout: Duration,
}
/// Circuit breaker configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Apicircuitbreakersettings
pub struct ApiCircuitBreakerSettings {
    /// Enable circuit breaker
    pub enable_circuit_breaker: bool,
    /// Failure threshold
    pub failure_threshold: u32,
    /// Timeout duration
    pub timeout_duration: Duration,
    /// Reset timeout
    pub reset_timeout: Duration,
    /// Success threshold for half-open state
    pub success_threshold: u32,
    /// Enable exponential backoff
    pub enable_exponential_backoff: bool,
    /// Initial backoff duration
    pub initial_backoff: Duration,
    /// Maximum backoff duration
    pub max_backoff: Duration,
}
/// Retry policy configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Apiretrypolicysettings
pub struct ApiRetryPolicySettings {
    /// Enable retries
    pub enable_retries: bool,
    /// Maximum retry attempts
    pub max_retry_attempts: u32,
    /// Initial retry delay
    pub initial_retry_delay: Duration,
    /// Maximum retry delay
    pub max_retry_delay: Duration,
    /// Retry multiplier for exponential backoff
    pub retry_multiplier: f64,
    /// Enable jitter
    pub enable_jitter: bool,
    /// Jitter range (0.0 to 1.0)
    pub jitter_range: f64,
}
/// Connection pool configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Apiconnectionpoolsettings
pub struct ApiConnectionPoolSettings {
    /// Minimum pool size
    pub min_pool_size: usize,
    /// Maximum pool size
    pub max_pool_size: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout
    pub idle_timeout: Duration,
    /// Maximum connection lifetime
    pub max_connection_lifetime: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Enable connection validation
    pub enable_connection_validation: bool,
}
impl Default for ApiPerformanceSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enable_request_caching: true,
            cache_ttl: Duration::from_secs(300), // 5 minutes
            max_cache_size: 10_000,
            enable_response_compression: true,
            compression_threshold: 1024, // 1KB
            enable_connection_pooling: true,
            connection_pool_size: 100,
            connection_pool_timeout: Duration::from_secs(30),
         }
}

impl Default for ApiCircuitBreakerSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enable_circuit_breaker: true,
            failure_threshold: 5,
            timeout_duration: Duration::from_secs(60),
            reset_timeout: Duration::from_secs(300), // 5 minutes
            success_threshold: 3,
            enable_exponential_backoff: true,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(30),
         }
}

impl Default for ApiRetryPolicySettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enable_retries: true,
            max_retry_attempts: 3,
            initial_retry_delay: Duration::from_millis(100),
            max_retry_delay: Duration::from_secs(10),
            retry_multiplier: 2.0,
            enable_jitter: true,
            jitter_range: 0.1,
         }
}

impl Default for ApiConnectionPoolSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            min_pool_size: 5,
            max_pool_size: 100,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600), // 10 minutes
            max_connection_lifetime: Duration::from_secs(3600), // 1 hour
            health_check_interval: Duration::from_secs(60),
            enable_connection_validation: true,
         }
} 