//! # Rate Limiting
//! Rate Limiting functionality and utilities.
// High-performance rate limiting with sliding window algorithm

use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, SystemTime};

use crate::error::Result;
use super::config::RateLimitConfig;
use super::types::{RateLimitBucket, RateLimitStatistics};

/// **RATE LIMITER**
///
/// High-performance rate limiting with sliding window algorithm
pub struct RateLimiter {
    /// Rate limit buckets per IP
    buckets: HashMap<IpAddr, RateLimitBucket>,
    /// Configuration
    config: RateLimitConfig,
    /// Last cleanup time
    last_cleanup: SystemTime,
}
impl RateLimiter {
    /// Create new rate limiter
    #[must_use]
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            buckets: HashMap::new(),
            config,
            last_cleanup: SystemTime::now(),
        }
    }

    /// Check if IP is whitelisted
    pub fn is_whitelisted(&self, ip: IpAddr) -> bool {
        self.config.whitelist.contains(&ip)
    }

    /// Check rate limit for IP
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn check_rate_limit(&mut self, ip: IpAddr) -> Result<bool>  {
        let now = SystemTime::now();

        // Cleanup old buckets periodically
        if now.duration_since(self.last_cleanup).unwrap_or(Duration::ZERO) > Duration::from_secs(300) {
            self.cleanup_old_buckets(now);
            self.last_cleanup = now;
        }

        let bucket = self.buckets.entry(ip).or_insert_with(|| RateLimitBucket {
            requests: Vec::new(),
            tokens: self.config.burst_size,
            last_refill: now,
            blocked_until: None,
        );

        // Check if IP is currently blocked
        if let Some(blocked_until) = bucket.blocked_until {
            if now < blocked_until {
                return Ok(false); // Still blocked
            } else {
                bucket.blocked_until = None; // Unblock
            }
        }

        // Refill tokens based on time elapsed
        let elapsed = now.duration_since(bucket.last_refill).unwrap_or(Duration::ZERO);
        let tokens_to_add = (elapsed.as_secs() * self.config.requests_per_minute as u64 / 60) as u32;
        
        if tokens_to_add > 0 {
            bucket.tokens = (bucket.tokens + tokens_to_add).min(self.config.burst_size);
            bucket.last_refill = now;
        }

        // Check if request is allowed
        if bucket.tokens > 0 {
            bucket.tokens -= 1;
            bucket.requests.push(now);
            
            // Remove old requests (sliding window)
            let window_start = now - Duration::from_secs(60);
            bucket.requests.retain(|&time| time > window_start);
            
            Ok(true)
        } else {
            // Block IP for configured duration
            bucket.blocked_until = Some(now + self.config.block_duration);
            Ok(false)
        }
    }

    /// Cleanup old buckets
    fn cleanup_old_buckets(&mut self, now: SystemTime) {
        let cutoff = now - Duration::from_secs(3600); // 1 hour
        self.buckets.retain(|_, bucket| {
            bucket.last_refill > cutoff || bucket.blocked_until.map_or(false, |until| until > now)
        );
    }

    /// Get rate limiting statistics
    pub fn get_statistics(&self) -> RateLimitStatistics {
        let blocked_ips_count = self.buckets.values()
            .filter(|bucket| bucket.blocked_until.map_or(false, |until| until > SystemTime::now()))
            .count();

        RateLimitStatistics {
            active_buckets: self.buckets.len(),
            total_requests: 0, // Would track in production
            rate_limited_requests: 0, // Would track in production
            blocked_ips_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_rate_limiting() {
        let mut rate_limiter = RateLimiter::new(RateLimitConfig {
            requests_per_minute: 2,
            burst_size: 2,
            block_duration: Duration::from_secs(60),
            whitelist: Vec::new(),
        );

        let test_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));

        // First two requests should be allowed
        assert!(rate_limiter.check_rate_limit(test_ip).await.unwrap());
        assert!(rate_limiter.check_rate_limit(test_ip).await.unwrap());

        // Third request should be blocked
        assert!(!rate_limiter.check_rate_limit(test_ip).await.unwrap());
    }

    #[tokio::test]
    async fn test_whitelist() {
        let test_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let rate_limiter = RateLimiter::new(RateLimitConfig {
            requests_per_minute: 1,
            burst_size: 1,
            block_duration: Duration::from_secs(60),
            whitelist: vec![test_ip],
        );

        assert!(rate_limiter.is_whitelisted(test_ip));
    }
} 