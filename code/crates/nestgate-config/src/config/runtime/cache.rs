// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Cache configuration module
//!
//! Provides configuration for Redis and in-memory caching.

use nestgate_types::error::Result;
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
    ///
    /// # Errors
    ///
    /// Returns an error if `NESTGATE_REDIS_HOST` is not set. Redis is an external service
    /// and must be explicitly configured - no hardcoded localhost assumption.
    ///
    /// # Philosophy
    ///
    /// External services (Redis, databases) must be explicitly configured.
    /// Hardcoded "localhost" violates sovereignty principles and hides
    /// deployment configuration issues.
    ///
    /// # Migration
    ///
    /// **Before** (silently used localhost):
    /// ```ignore
    /// let config = CacheConfig::from_environment()?;
    /// ```
    ///
    /// **After** (requires explicit config):
    /// ```bash
    /// export NESTGATE_REDIS_HOST="redis.internal"
    /// export NESTGATE_REDIS_PORT="6379"
    /// ```
    pub fn from_environment() -> Result<Self> {
        let redis_host = env::var("NESTGATE_REDIS_HOST")
            .map_err(|_| nestgate_types::error::NestGateError::configuration_error(
                "redis_host",
                "NESTGATE_REDIS_HOST must be set explicitly. No hardcoded localhost for external services."
            ))?;

        Ok(Self {
            redis_host,
            redis_port: env::var("NESTGATE_REDIS_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(6379), // Port 6379 is Redis default (industry standard)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_config_default() {
        let config = CacheConfig::default();
        assert_eq!(config.redis_host, "localhost");
        assert_eq!(config.redis_port, 6379);
        assert_eq!(config.ttl_seconds, 3600);
        assert_eq!(config.max_size_mb, 100);
    }

    #[test]
    fn test_cache_config_redis_url() {
        let config = CacheConfig {
            redis_host: "redis.example.com".to_string(),
            redis_port: 6380,
            ttl_seconds: 7200,
            max_size_mb: 200,
        };
        assert_eq!(config.redis_url(), "redis://redis.example.com:6380");
    }

    #[test]
    fn test_cache_config_from_environment_requires_redis_host() {
        // Without NESTGATE_REDIS_HOST, from_environment returns error
        temp_env::with_vars([("NESTGATE_REDIS_HOST", None::<&str>)], || {
            let result = CacheConfig::from_environment();
            assert!(result.is_err());
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("NESTGATE_REDIS_HOST")
            );
        });
    }

    #[test]
    fn test_cache_config_from_environment_success() {
        temp_env::with_vars(
            [
                ("NESTGATE_REDIS_HOST", Some("redis.internal")),
                ("NESTGATE_REDIS_PORT", Some("6380")),
                ("NESTGATE_CACHE_TTL_SECONDS", Some("7200")),
                ("NESTGATE_CACHE_MAX_SIZE_MB", Some("256")),
            ],
            || {
                let config = CacheConfig::from_environment().unwrap();
                assert_eq!(config.redis_host, "redis.internal");
                assert_eq!(config.redis_port, 6380);
                assert_eq!(config.ttl_seconds, 7200);
                assert_eq!(config.max_size_mb, 256);
            },
        );
    }

    #[test]
    fn test_cache_config_serialization() {
        let config = CacheConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("localhost"));
        let parsed: CacheConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.redis_port, 6379);
    }
}
