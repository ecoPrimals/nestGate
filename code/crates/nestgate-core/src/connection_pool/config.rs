// 🚀 ECOSYSTEM UNIFICATION: Import unified types

/// **MODERNIZED**: UnifiedConfig now uses UnifiedConnectionPoolConfig directly
pub type PoolConfig = crate::config::canonical_primary::UnifiedConnectionPoolConfig;
// 🚀 MODERN CONVENIENCE METHODS: Extend UnifiedConnectionPoolConfig with pool-specific methods
impl crate::config::canonical_primary::UnifiedConnectionPoolConfig {
    /// Create a new pool configuration with custom parameters
    pub fn new_pool_config(
        min_connections: usize,
        max_connections: usize,
        max_idle_time_seconds: u64,
        acquire_timeout_seconds: u64,
        health_check_interval_seconds: u64,
    ) -> Self {
        let mut config = Self::default();
        config.min_connections = min_connections;
        config.max_connections = max_connections;
        config.max_idle_time_seconds = max_idle_time_seconds;
        config.acquire_timeout_seconds = acquire_timeout_seconds;
        config.health_check_interval_seconds = health_check_interval_seconds;
        config.enable_validation = true;
        config.retry_attempts = 3;
        config.retry_delay_seconds = 1;
        config.enable_metrics = true;
        config.pool_name = "default-pool".to_string();
        config
    }

    /// Create high-performance pool configuration
    #[must_use]
    pub fn high_performance() -> Self {
        let mut config = Self::default();
        config.min_connections = 10;
        config.max_connections = 100;
        config.max_idle_time_seconds = 600; // 10 minutes
        config.acquire_timeout_seconds = 10; // Shorter timeout for HP
        config.health_check_interval_seconds = 30; // More frequent checks
        config.enable_validation = true;
        config.retry_attempts = 5; // More retries for HP
        config.retry_delay_seconds = 1;
        config.enable_metrics = true;
        config.pool_name = "high-performance-pool".to_string();
        config
    }

    /// Create minimal pool configuration for development
    #[must_use]
    pub fn minimal() -> Self {
        let mut config = Self::default();
        config.min_connections = 1;
        config.max_connections = 5;
        config.max_idle_time_seconds = 300; // 5 minutes
        config.acquire_timeout_seconds = 60; // Longer timeout for dev
        config.health_check_interval_seconds = 120; // Less frequent checks
        config.enable_validation = false; // Skip validation in dev
        config.retry_attempts = 1;
        config.retry_delay_seconds = 2;
        config.enable_metrics = false; // Disable metrics in dev
        config.pool_name = "minimal-dev-pool".to_string();
        config
    }

    /// Create production-ready pool configuration
    #[must_use]
    pub fn production() -> Self {
        let mut config = Self::default();
        config.min_connections = 5;
        config.max_connections = 50;
        config.max_idle_time_seconds = 900; // 15 minutes
        config.acquire_timeout_seconds = 30;
        config.health_check_interval_seconds = 60;
        config.enable_validation = true;
        config.retry_attempts = 3;
        config.retry_delay_seconds = 1;
        config.enable_metrics = true;
        config.pool_name = "production-pool".to_string();
        config
    }
}
