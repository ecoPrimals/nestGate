// Pool helpers for [`crate::config::canonical_primary::UnifiedConnectionPoolConfig`].

use std::time::Duration;

/// **MODERNIZED**: `UnifiedConnectionPoolConfig` is the canonical connection pool struct.
pub type PoolConfig = crate::config::canonical_primary::UnifiedConnectionPoolConfig;

impl crate::config::canonical_primary::UnifiedConnectionPoolConfig {
    /// Create a new pool configuration with custom parameters.
    pub fn new_pool_config(
        min_connections: usize,
        max_connections: usize,
        max_idle_time_seconds: u64,
        acquire_timeout_seconds: u64,
        health_check_interval_seconds: u64,
    ) -> Self {
        let mut config = Self::default();
        config.initial_size = min_connections as u32;
        config.max_size = max_connections as u32;
        config.min_idle = min_connections as u32;
        config.idle_timeout = Duration::from_secs(max_idle_time_seconds);
        config.connection_timeout = Duration::from_secs(acquire_timeout_seconds);
        config.health_check_interval = Duration::from_secs(health_check_interval_seconds);
        config
    }

    /// High-performance pool configuration preset.
    #[must_use]
    pub fn high_performance() -> Self {
        let mut config = Self::default();
        config.initial_size = 10;
        config.max_size = 100;
        config.idle_timeout = Duration::from_secs(600);
        config.connection_timeout = Duration::from_secs(10);
        config.health_check_interval = Duration::from_secs(30);
        config
    }

    /// Minimal pool configuration for development.
    #[must_use]
    pub fn minimal() -> Self {
        let mut config = Self::default();
        config.initial_size = 1;
        config.max_size = 5;
        config.idle_timeout = Duration::from_secs(300);
        config.connection_timeout = Duration::from_secs(60);
        config.health_check_interval = Duration::from_secs(120);
        config
    }

    /// Production-oriented defaults.
    #[must_use]
    pub fn production() -> Self {
        let mut config = Self::default();
        config.initial_size = 5;
        config.max_size = 50;
        config.idle_timeout = Duration::from_secs(900);
        config.connection_timeout = Duration::from_secs(30);
        config.health_check_interval = Duration::from_secs(60);
        config
    }
}
