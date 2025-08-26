use crate::NestGateError;
use std::collections::HashMap;
//
// This module provides comprehensive rate limiting functionality
// to prevent abuse and ensure fair usage of API resources.

use crate::{Result, NestGateError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;

// **CANONICAL MODERNIZATION**: Type aliases to fix clippy complexity errors
/// Type alias for rate limit bucket storage
type BucketRegistry = Arc<RwLock<HashMap<String, RateLimitBucket>>>;

/// Rate limiting errors
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum RateLimitError {
    #[error(
        "Rate limit exceeded for {identifier}: {current} requests in {window:?}, limit is {limit}"
    )]
    LimitExceeded {
        identifier: String,
        current: u64,
        limit: u64,
        window: Duration,
        retry_after: Duration,
    },

    #[error("Invalid rate limit configuration: {message}")]
    InvalidConfiguration { message: String },

    #[error("Rate limiter internal error: {message}")]
    InternalError { message: String },
}

/// Rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum number of requests allowed
    pub max_requests: u64,
    /// Time window for the rate limit
    pub window: Duration,
    /// Burst allowance (additional requests allowed in short bursts)
    pub burst_allowance: u64,
    /// Cleanup interval for expired entries
    pub cleanup_interval: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window: Duration::from_secs(60), // 1 minute
            burst_allowance: 10,
            cleanup_interval: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Rate limit bucket for tracking requests
#[derive(Debug, Clone)]
struct RateLimitBucket {
    /// Request count in current window
    count: u64,
    /// Window start time
    window_start: Instant,
    /// Last request time
    last_request: Instant,
    /// Burst tokens available
    burst_tokens: u64,
}

impl RateLimitBucket {
    fn new(config: &RateLimitConfig) -> Self {
        let now = Instant::now();
        Self {
            count: 0,
            window_start: now,
            last_request: now,
            burst_tokens: config.burst_allowance,
        }
    }

    /// Check if the bucket has expired
    pub fn is_expired(&self, config: &RateLimitConfig) -> bool {
        self.window_start.elapsed() >= config.window
    }

    /// Reset the bucket if the window has expired
    pub fn reset_if_needed(&mut self, config: &RateLimitConfig, now: Instant) {
        if now.duration_since(self.window_start) >= config.window {
            self.count = 0;
            self.window_start = now;
            self.burst_tokens = config.burst_allowance;
        }
    }

    /// Consume a token
    pub fn consume(&mut self, now: Instant) {
        if self.burst_tokens > 0 {
            self.burst_tokens -= 1;
        }
        self.count += 1;
        self.last_request = now;
    }

    fn reset(&mut self, config: &RateLimitConfig) {
        self.count = 0;
        self.window_start = Instant::now();
        self.burst_tokens = config.burst_allowance;
    }
}

/// Rate limiter with multiple strategies
#[derive(Debug)]
pub struct RateLimiter {
    /// Rate limit configuration
    config: RateLimitConfig,
    /// Buckets for tracking requests by identifier
    buckets: BucketRegistry,
    /// Global request counter
    total_requests: Arc<AtomicU64>,
    /// Global blocked requests counter
    blocked_requests: Arc<AtomicU64>,
    /// Last cleanup time
    last_cleanup: Arc<RwLock<Instant>>,
}

impl RateLimiter {
    /// Create a new rate limiter with default configuration
    pub fn new() -> Self {
        Self::with_config(RateLimitConfig::default())
    }

    /// Create a new rate limiter with custom configuration
    pub fn with_config(config: RateLimitConfig) -> Self {
        Self {
            config,
            buckets: Arc::new(RwLock::new(HashMap::new())),
            total_requests: Arc::new(AtomicU64::new(0)),
            blocked_requests: Arc::new(AtomicU64::new(0)),
            last_cleanup: Arc::new(RwLock::new(Instant::now())),
        }
    }

    /// Check if a request should be rate limited
    pub async fn check_rate_limit(&self, identifier: &str) -> Result<()> {
        let mut buckets = self.buckets.write().await;

        let bucket = buckets
            .entry(identifier.to_string())
            .or_insert_with(|| RateLimitBucket::new(&self.config));

        if bucket.is_expired(&self.config) {
            bucket.reset_if_needed(&self.config, Instant::now());
        }

        bucket.consume(Instant::now());

        if bucket.count > self.config.max_requests {
            return Err(NestGateError::Security(Box::new(
                crate::error::SecurityErrorData {
                    message: format!(
                        "Rate limit exceeded for {}: {} requests in {:?} window",
                        identifier, bucket.count, self.config.window
                    ),
                    operation: "rate_limit_check".to_string(),
                    resource: Some(identifier.to_string()),
                    principal: None,
                    context: None,
                },
            )));
        }

        // Update global counters
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// Get the current status for a rate limit bucket
    pub async fn get_status(&self, identifier: &str) -> Result<RateLimitStatus> {
        let buckets = self.buckets.read().await;

        if let Some(bucket) = buckets.get(identifier) {
            Ok(RateLimitStatus {
                allowed: bucket.count <= self.config.max_requests,
                current_count: bucket.count,
                limit: self.config.max_requests,
                window: self.config.window,
                time_until_reset: bucket.window_start + self.config.window - Instant::now(),
                burst_tokens_remaining: bucket.burst_tokens,
            })
        } else {
            Ok(RateLimitStatus {
                allowed: true,
                current_count: 0,
                limit: self.config.max_requests,
                window: self.config.window,
                time_until_reset: self.config.window,
                burst_tokens_remaining: self.config.burst_allowance,
            })
        }
    }

    /// Reset rate limit for a specific identifier
    pub async fn reset_rate_limit(&self, identifier: &str) -> Result<()> {
        let mut buckets = self.buckets.write().await;

        if let Some(bucket) = buckets.get_mut(identifier) {
            bucket.reset(&self.config);
        }
        Ok(())
    }

    /// Get rate limiter statistics
    pub async fn get_statistics(&self) -> Result<RateLimiterStatistics> {
        let buckets = self.buckets.read().await;

        Ok(RateLimiterStatistics {
            total_requests: self.total_requests.load(Ordering::Relaxed),
            blocked_requests: self.blocked_requests.load(Ordering::Relaxed),
            active_buckets: buckets.len(),
            config: self.config.clone(),
        })
    }

    /// Clean up expired rate limit buckets
    pub async fn cleanup_expired(&self) -> Result<usize> {
        let mut buckets = self.buckets.write().await;

        let now = Instant::now();
        let initial_count = buckets.len();

        // Update last cleanup time
        if let Ok(mut last_cleanup) = self.last_cleanup.try_write() {
            *last_cleanup = now;
        }

        // Remove expired buckets
        buckets.retain(|_, bucket| !bucket.is_expired(&self.config));

        let removed_count = initial_count - buckets.len();
        Ok(removed_count)
    }
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            buckets: Arc::clone(&self.buckets),
            total_requests: Arc::clone(&self.total_requests),
            blocked_requests: Arc::clone(&self.blocked_requests),
            last_cleanup: Arc::clone(&self.last_cleanup),
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limit status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStatus {
    /// Whether the request would be allowed
    pub allowed: bool,
    /// Current request count in window
    pub current_count: u64,
    /// Maximum requests allowed
    pub limit: u64,
    /// Rate limit window duration
    pub window: Duration,
    /// Time until rate limit resets
    pub time_until_reset: Duration,
    /// Remaining burst tokens
    pub burst_tokens_remaining: u64,
}

/// Rate limiter statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimiterStatistics {
    /// Total requests processed
    pub total_requests: u64,
    /// Total requests blocked
    pub blocked_requests: u64,
    /// Number of active rate limit buckets
    pub active_buckets: usize,
    /// Current configuration
    pub config: RateLimitConfig,
}

impl RateLimiterStatistics {
    /// Calculate block rate as percentage
    pub fn block_rate(&self) -> f64 {
        if self.total_requests > 0 {
            (self.blocked_requests as f64 / self.total_requests as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Calculate success rate as percentage
    pub fn success_rate(&self) -> f64 {
        100.0 - self.block_rate()
    }
}

/// Pre-configured rate limiters for common use cases
pub struct RateLimitPresets;

impl RateLimitPresets {
    /// Strict rate limiter for sensitive operations
    pub fn strict() -> RateLimiter {
        RateLimiter::with_config(RateLimitConfig {
            max_requests: 10,
            window: Duration::from_secs(60),
            burst_allowance: 2,
            cleanup_interval: Duration::from_secs(300),
        })
    }

    /// Moderate rate limiter for general API usage
    pub fn moderate() -> RateLimiter {
        RateLimiter::with_config(RateLimitConfig {
            max_requests: 100,
            window: Duration::from_secs(60),
            burst_allowance: 10,
            cleanup_interval: Duration::from_secs(300),
        })
    }

    /// Permissive rate limiter for high-throughput operations
    pub fn permissive() -> RateLimiter {
        RateLimiter::with_config(RateLimitConfig {
            max_requests: 1000,
            window: Duration::from_secs(60),
            burst_allowance: 100,
            cleanup_interval: Duration::from_secs(300),
        })
    }

    /// Rate limiter for authentication attempts
    pub fn auth_attempts() -> RateLimiter {
        RateLimiter::with_config(RateLimitConfig {
            max_requests: 5,
            window: Duration::from_secs(300), // 5 minutes
            burst_allowance: 0,               // No burst for auth
            cleanup_interval: Duration::from_secs(600),
        })
    }
}

// Global rate limiter instances
lazy_static::lazy_static! {
    /// Global API rate limiter
    pub static ref GLOBAL_API_RATE_LIMITER: RateLimiter = RateLimitPresets::moderate();

    /// Global authentication rate limiter
    pub static ref GLOBAL_AUTH_RATE_LIMITER: RateLimiter = RateLimitPresets::auth_attempts();
}

/// Check API rate limit (convenience function)
pub async fn check_api_rate_limit(identifier: &str) -> Result<()> {
    let limiter = RateLimiter::with_config(RateLimitConfig::default());
    limiter.check_rate_limit(identifier).await
}

/// Check authentication rate limit (convenience function)  
pub async fn check_auth_rate_limit(identifier: &str) -> Result<()> {
    let config = RateLimitConfig {
        max_requests: 5,
        window: Duration::from_secs(300), // 5 minutes
        burst_allowance: 0,
        cleanup_interval: Duration::from_secs(600),
    };
    let limiter = RateLimiter::with_config(config);
    limiter.check_rate_limit(identifier).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let limiter = RateLimiter::with_config(RateLimitConfig {
            max_requests: 3,
            window: Duration::from_secs(60),
            burst_allowance: 1,
            cleanup_interval: Duration::from_secs(300),
        });

        // First 3 requests should succeed
        assert!(limiter.check_rate_limit("user1").await.is_ok());
        assert!(limiter.check_rate_limit("user1").await.is_ok());
        assert!(limiter.check_rate_limit("user1").await.is_ok());

        // 4th request should use burst allowance
        assert!(limiter.check_rate_limit("user1").await.is_ok());

        // 5th request should fail
        assert!(limiter.check_rate_limit("user1").await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limiter_different_identifiers() {
        let limiter = RateLimiter::with_config(RateLimitConfig {
            max_requests: 2,
            window: Duration::from_secs(60),
            burst_allowance: 0,
            cleanup_interval: Duration::from_secs(300),
        });

        // Different identifiers should have separate limits
        assert!(limiter.check_rate_limit("user1").await.is_ok());
        assert!(limiter.check_rate_limit("user2").await.is_ok());
        assert!(limiter.check_rate_limit("user1").await.is_ok());
        assert!(limiter.check_rate_limit("user2").await.is_ok());

        // Both should be at limit now
        assert!(limiter.check_rate_limit("user1").await.is_err());
        assert!(limiter.check_rate_limit("user2").await.is_err());
    }

    #[tokio::test]
    async fn test_rate_limit_status() -> crate::Result<()> {
        let limiter = RateLimiter::with_config(RateLimitConfig {
            max_requests: 5,
            window: Duration::from_secs(60),
            burst_allowance: 2,
            cleanup_interval: Duration::from_secs(300),
        });

        let status = limiter.get_status("user1").await?;
        assert!(status.allowed);
        assert_eq!(status.current_count, 0);
        assert_eq!(status.limit, 5);

        // Make a request
        limiter.check_rate_limit("user1").await?;

        let status = limiter.get_status("user1").await?;
        assert!(status.allowed);
        assert_eq!(status.current_count, 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_rate_limiter_statistics() -> crate::Result<()> {
        let limiter = RateLimiter::new();

        // Make some requests
        limiter.check_rate_limit("user1").await?;
        limiter.check_rate_limit("user1").await?;

        // Get statistics
        let stats = limiter.get_statistics().await?;
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.blocked_requests, 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_rate_limit_reset() -> crate::Result<()> {
        let limiter = RateLimiter::new();

        // Exhaust the rate limit
        limiter.check_rate_limit("user1").await?;
        assert!(limiter.check_rate_limit("user1").await.is_err());

        // Reset and try again
        limiter.reset_rate_limit("user1").await?;

        // Should work now
        assert!(limiter.check_rate_limit("user1").await.is_ok());

        Ok(())
    }
}
