//! Cache configuration module
//!
//! Provides configuration for Redis and in-memory caching.

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::env;

/// Cache configuration for Redis and in-memory caching.
///
/// # Environment Variables
///
/// - `NESTGATE_REDIS_HOST` - Redis host (default: "localhost")
/// - `NESTGATE_REDIS_PORT` - Redis port (default: 6379)
/// - `NESTGATE_CACHE_TTL_SECONDS` - Default TTL (default: 3600)
/// - `NESTGATE_CACHE_MAX_SIZE_MB` - Max cache size in MB (default: 100)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Redis host
    pub redis_host: String,

    /// Redis port
    pub redis_port: u16,

    /// Default TTL in seconds
    pub ttl_seconds: u64,

    /// Maximum cache size in MB
    pub max_size_mb: usize,
}

impl CacheConfig {
    /// Load cache configuration from environment variables.
    pub fn from_environment() -> Result<Self> {
        Ok(Self {
            redis_host: env::var("NESTGATE_REDIS_HOST").unwrap_or_else(|_| "localhost".to_string()),
            redis_port: env::var("NESTGATE_REDIS_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(6379),
            ttl_seconds: env::var("NESTGATE_CACHE_TTL_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600),
            max_size_mb: env::var("NESTGATE_CACHE_MAX_SIZE_MB")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
        })
    }

    /// Get Redis connection URL.
    #[must_use]
    pub fn redis_url(&self) -> String {
        format!("redis://{}:{}", self.redis_host, self.redis_port)
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            redis_host: "localhost".to_string(),
            redis_port: 6379,
            ttl_seconds: 3600,
            max_size_mb: 100,
        }
    }
}
