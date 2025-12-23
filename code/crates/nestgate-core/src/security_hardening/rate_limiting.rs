//! Rate Limiting
//!
//! Advanced rate limiting with token bucket algorithm

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// **RATE LIMITER**
///
/// Advanced rate limiting with multiple strategies
pub struct RateLimiter {
    buckets: Mutex<HashMap<String, TokenBucket>>,
    global_limit: Option<RateLimit>,
    per_ip_limit: Option<RateLimit>,
    per_user_limit: Option<RateLimit>,
    stats: RateLimitStats,
}

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimit {
    /// Requests allowed per window
    pub requests_per_window: u64,
    /// Time window duration
    pub window_duration: Duration,
}

#[derive(Debug)]
struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
    capacity: f64,
    refill_rate: f64, // tokens per second
}

#[derive(Debug, Default)]
struct RateLimitStats {
    requests_processed: AtomicU64,
    requests_blocked: AtomicU64,
    buckets_created: AtomicU64,
}

impl RateLimiter {
    /// Create a new rate limiter
    #[must_use]
    pub fn new() -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
            global_limit: None,
            per_ip_limit: None,
            per_user_limit: None,
            stats: RateLimitStats::default(),
        }
    }
    
    /// Set global rate limit
    #[must_use]
    pub fn with_global_limit(mut self, limit: RateLimit) -> Self {
        self.global_limit = Some(limit);
        self
    }
    
    /// Set per-IP rate limit
    #[must_use]
    pub fn with_per_ip_limit(mut self, limit: RateLimit) -> Self {
        self.per_ip_limit = Some(limit);
        self
    }
    
    /// Set per-user rate limit
    #[must_use]
    pub fn with_per_user_limit(mut self, limit: RateLimit) -> Self {
        self.per_user_limit = Some(limit);
        self
    }
    
    /// Check if request is allowed
    pub fn is_allowed(&self, ip: &str, user_id: Option<&str>) -> RateLimitResult {
        self.stats.requests_processed.fetch_add(1, Ordering::Relaxed);
        
        // Check global limit
        if let Some(ref limit) = self.global_limit {
            if !self.check_bucket("global", limit) {
                self.stats.requests_blocked.fetch_add(1, Ordering::Relaxed);
                return RateLimitResult::Blocked("Global rate limit exceeded".to_string());
            }
        }
        
        // Check per-IP limit
        if let Some(ref limit) = self.per_ip_limit {
            let key = format!("ip:{ip}");
            if !self.check_bucket(&key, limit) {
                self.stats.requests_blocked.fetch_add(1, Ordering::Relaxed);
                return RateLimitResult::Blocked("IP rate limit exceeded".to_string());
            }
        }
        
        // Check per-user limit
        if let (Some(user_id), Some(ref limit)) = (user_id, &self.per_user_limit) {
            let key = format!("user:{user_id}");
            if !self.check_bucket(&key, limit) {
                self.stats.requests_blocked.fetch_add(1, Ordering::Relaxed);
                return RateLimitResult::Blocked("User rate limit exceeded".to_string());
            }
        }
        
        RateLimitResult::Allowed
    }
    
    fn check_bucket(&self, key: &str, limit: &RateLimit) -> bool {
        // SAFE: Mutex lock - handle poisoned mutex gracefully
        let mut buckets = self.buckets.lock().unwrap_or_else(|poisoned| {
            poisoned.into_inner()
        });
        
        let bucket = buckets.entry(key.to_string()).or_insert_with(|| {
            self.stats.buckets_created.fetch_add(1, Ordering::Relaxed);
            TokenBucket {
                tokens: limit.requests_per_window as f64,
                last_refill: Instant::now(),
                capacity: limit.requests_per_window as f64,
                refill_rate: limit.requests_per_window as f64 / limit.window_duration.as_secs_f64(),
            }
        });
        
        // Refill tokens
        let now = Instant::now();
        let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
        bucket.tokens = (bucket.tokens + elapsed * bucket.refill_rate).min(bucket.capacity);
        bucket.last_refill = now;
        
        // Check if request is allowed
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }
    
    /// Get rate limiting statistics
    pub fn stats(&self) -> (u64, u64, u64, f64) {
        let processed = self.stats.requests_processed.load(Ordering::Relaxed);
        let blocked = self.stats.requests_blocked.load(Ordering::Relaxed);
        let buckets = self.stats.buckets_created.load(Ordering::Relaxed);
        let block_rate = if processed > 0 { 
            blocked as f64 / processed as f64 
        } else { 
            0.0 
        };
        (processed, blocked, buckets, block_rate)
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limit result
#[derive(Debug)]
pub enum RateLimitResult {
    /// Request is allowed
    Allowed,
    /// Request is blocked with reason
    Blocked(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new()
            .with_per_ip_limit(RateLimit {
                requests_per_window: 10,
                window_duration: Duration::from_secs(60),
            });
        
        // Should allow first request
        match limiter.is_allowed("192.168.1.1", None) {
            RateLimitResult::Allowed => {},
            RateLimitResult::Blocked(_) => panic!("Should allow request"),
        }
    }
    
    #[test]
    fn test_rate_limit_blocking() {
        let limiter = RateLimiter::new()
            .with_per_ip_limit(RateLimit {
                requests_per_window: 2,
                window_duration: Duration::from_secs(3600),
            });
        
        // First two should succeed
        assert!(matches!(limiter.is_allowed("test_ip", None), RateLimitResult::Allowed));
        assert!(matches!(limiter.is_allowed("test_ip", None), RateLimitResult::Allowed));
        
        // Third should be blocked
        assert!(matches!(limiter.is_allowed("test_ip", None), RateLimitResult::Blocked(_)));
    }
    
    #[test]
    fn test_different_ips_independent() {
        let limiter = RateLimiter::new()
            .with_per_ip_limit(RateLimit {
                requests_per_window: 1,
                window_duration: Duration::from_secs(3600),
            });
        
        // Different IPs should be independent
        assert!(matches!(limiter.is_allowed("ip1", None), RateLimitResult::Allowed));
        assert!(matches!(limiter.is_allowed("ip2", None), RateLimitResult::Allowed));
    }
}

