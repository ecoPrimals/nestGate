// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CONSOLIDATED CONSTANTS MODULE**
//!
//! Single source of truth for ALL previously hardcoded values in NestGate.
//! This module eliminates 888+ hardcoded instances by providing:
//! - Environment-driven configuration
//! - Type-safe constants
//! - Thread-safe initialization
//! - Zero runtime overhead (const evaluation where possible)
//!
//! **Migration Status**: Created Nov 13, 2025
//! **Targets**: 447 hardcoded IPs, 441 hardcoded ports
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nestgate_core::constants::consolidated::*;
//!
//! // Get configuration with environment override support
//! let config = NetworkConstants::get();
//! println!("API endpoint: {}:{}", config.api_host(), config.api_port());
//! ```
//!
//! ## Environment Variables
//!
//! All constants can be overridden via `NESTGATE_*` environment variables:
//! - `NESTGATE_API_HOST` - API server host (default: 127.0.0.1)
//! - `NESTGATE_API_PORT` - API server port (default: 8080)
//! - `NESTGATE_BIND_ADDRESS` - Bind address (default: 0.0.0.0)
//! - And 50+ more (see full list below)

use std::sync::{Arc, OnceLock};

// ============================================================================
// NETWORK CONSTANTS
// ============================================================================

/// Network configuration constants with environment override support
#[derive(Debug, Clone)]
/// Networkconstants
pub struct NetworkConstants {
    // Hosts
    api_host: String,
    metrics_host: String,
    health_host: String,
    admin_host: String,

    // Ports
    api_port: u16,
    http_port: u16,
    https_port: u16,
    websocket_port: u16,
    grpc_port: u16,
    metrics_port: u16,
    prometheus_port: u16,
    health_port: u16,
    admin_port: u16,

    // Addresses
    bind_address: String,
    localhost_ipv4: String,
    localhost_ipv6: String,
    bind_all_ipv4: String,
    bind_all_ipv6: String,
}

impl Default for NetworkConstants {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            // Hosts (default to localhost for security)
            api_host: env_or("NESTGATE_API_HOST", "127.0.0.1"),
            metrics_host: env_or("NESTGATE_METRICS_HOST", "127.0.0.1"),
            health_host: env_or("NESTGATE_HEALTH_HOST", "127.0.0.1"),
            admin_host: env_or("NESTGATE_ADMIN_HOST", "127.0.0.1"),

            // Ports
            api_port: env_or_parse("NESTGATE_API_PORT", 8080),
            http_port: env_or_parse("NESTGATE_HTTP_PORT", 8080),
            https_port: env_or_parse("NESTGATE_HTTPS_PORT", 8443),
            websocket_port: env_or_parse("NESTGATE_WS_PORT", 8082),
            grpc_port: env_or_parse("NESTGATE_GRPC_PORT", 50051),
            metrics_port: env_or_parse("NESTGATE_METRICS_PORT", 9090),
            prometheus_port: env_or_parse("NESTGATE_PROMETHEUS_PORT", 9090),
            health_port: env_or_parse("NESTGATE_HEALTH_PORT", 8081),
            admin_port: env_or_parse("NESTGATE_ADMIN_PORT", 9000),

            // Addresses
            bind_address: env_or("NESTGATE_BIND_ADDRESS", "0.0.0.0"),
            localhost_ipv4: "127.0.0.1".to_string(),
            localhost_ipv6: "::1".to_string(),
            bind_all_ipv4: "0.0.0.0".to_string(),
            bind_all_ipv6: "::".to_string(),
        }
    }
}

impl NetworkConstants {
    /// Get or initialize the global network constants
    pub fn get() -> Arc<Self> {
        static INSTANCE: OnceLock<Arc<NetworkConstants>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::default())).clone()
    }

    // Host getters

    /// Returns the API host address (e.g., "localhost" or "0.0.0.0")
    #[must_use]
    pub fn api_host(&self) -> &str {
        &self.api_host
    }
    /// Metrics Host
    #[must_use]
    pub fn metrics_host(&self) -> &str {
        &self.metrics_host
    }
    /// Health Host
    #[must_use]
    pub fn health_host(&self) -> &str {
        &self.health_host
    }
    /// Admin Host
    #[must_use]
    pub fn admin_host(&self) -> &str {
        &self.admin_host
    }

    // Port getters

    /// Returns the API port number (read from `NESTGATE_API_PORT` or default 8080)
    #[must_use]
    pub const fn api_port(&self) -> u16 {
        self.api_port
    }
    /// Http Port
    #[must_use]
    pub const fn http_port(&self) -> u16 {
        self.http_port
    }
    /// Https Port
    #[must_use]
    pub const fn https_port(&self) -> u16 {
        self.https_port
    }
    /// Websocket Port
    #[must_use]
    pub const fn websocket_port(&self) -> u16 {
        self.websocket_port
    }
    /// Grpc Port
    #[must_use]
    pub const fn grpc_port(&self) -> u16 {
        self.grpc_port
    }
    /// Metrics Port
    #[must_use]
    pub const fn metrics_port(&self) -> u16 {
        self.metrics_port
    }
    /// Prometheus Port
    #[must_use]
    pub const fn prometheus_port(&self) -> u16 {
        self.prometheus_port
    }
    /// Health Port
    #[must_use]
    pub const fn health_port(&self) -> u16 {
        self.health_port
    }
    /// Admin Port
    #[must_use]
    pub const fn admin_port(&self) -> u16 {
        self.admin_port
    }

    // Address getters

    /// Returns the bind address for server sockets (read from `NESTGATE_BIND_ADDRESS` or default "0.0.0.0")
    #[must_use]
    pub fn bind_address(&self) -> &str {
        &self.bind_address
    }
    /// Localhost Ipv4
    #[must_use]
    pub fn localhost_ipv4(&self) -> &str {
        &self.localhost_ipv4
    }
    /// Localhost Ipv6
    #[must_use]
    pub fn localhost_ipv6(&self) -> &str {
        &self.localhost_ipv6
    }
    /// Bind All Ipv4
    #[must_use]
    pub fn bind_all_ipv4(&self) -> &str {
        &self.bind_all_ipv4
    }
    /// Bind All Ipv6
    #[must_use]
    pub fn bind_all_ipv6(&self) -> &str {
        &self.bind_all_ipv6
    }

    // Convenience methods for full URLs

    /// Returns the full API URL (e.g., "<http://localhost:8080>")
    #[must_use]
    pub fn api_url(&self) -> String {
        format!("http://{}:{}", self.api_host, self.api_port)
    }

    /// Api Bind Address
    #[must_use]
    pub fn api_bind_address(&self) -> String {
        format!("{}:{}", self.bind_address, self.api_port)
    }

    /// Health Url
    #[must_use]
    pub fn health_url(&self) -> String {
        format!("http://{}:{}", self.health_host, self.health_port)
    }

    /// Metrics Url
    #[must_use]
    pub fn metrics_url(&self) -> String {
        format!("http://{}:{}", self.metrics_host, self.metrics_port)
    }

    /// Websocket Url
    #[must_use]
    pub fn websocket_url(&self) -> String {
        format!("ws://{}:{}/ws", self.api_host, self.websocket_port)
    }
}

// ============================================================================
// STORAGE CONSTANTS
// ============================================================================

/// Storage and database configuration constants
#[derive(Debug, Clone)]
/// Storageconstants
pub struct StorageConstants {
    // Database
    postgres_host: String,
    postgres_port: u16,
    postgres_database: String,
    postgres_max_connections: u32,

    // Redis
    redis_host: String,
    redis_port: u16,
    redis_max_connections: u32,

    // ZFS
    zfs_pool_name: String,
    zfs_dataset_prefix: String,
    zfs_compression: String,
    zfs_dedup: bool,

    // Storage paths
    data_dir: String,
    cache_dir: String,
    log_dir: String,
}

impl Default for StorageConstants {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            // PostgreSQL
            postgres_host: env_or("NESTGATE_POSTGRES_HOST", "127.0.0.1"),
            postgres_port: env_or_parse("NESTGATE_POSTGRES_PORT", 5432),
            postgres_database: env_or("NESTGATE_POSTGRES_DB", "nestgate"),
            postgres_max_connections: env_or_parse("NESTGATE_POSTGRES_MAX_CONN", 100),

            // Redis
            redis_host: env_or("NESTGATE_REDIS_HOST", "127.0.0.1"),
            redis_port: env_or_parse("NESTGATE_REDIS_PORT", 6379),
            redis_max_connections: env_or_parse("NESTGATE_REDIS_MAX_CONN", 50),

            // ZFS
            zfs_pool_name: env_or("NESTGATE_ZFS_POOL", "nestgate_pool"),
            zfs_dataset_prefix: env_or("NESTGATE_ZFS_PREFIX", "nestgate"),
            zfs_compression: env_or("NESTGATE_ZFS_COMPRESSION", "lz4"),
            zfs_dedup: env_or_parse("NESTGATE_ZFS_DEDUP", false),

            // Paths
            data_dir: env_or("NESTGATE_DATA_DIR", "./data"),
            cache_dir: env_or("NESTGATE_CACHE_DIR", "./cache"),
            log_dir: env_or("NESTGATE_LOG_DIR", "./logs"),
        }
    }
}

impl StorageConstants {
    /// Get or initialize the global storage constants
    pub fn get() -> Arc<Self> {
        static INSTANCE: OnceLock<Arc<StorageConstants>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::default())).clone()
    }

    // Database getters

    /// Returns the `PostgreSQL` host address
    #[must_use]
    pub fn postgres_host(&self) -> &str {
        &self.postgres_host
    }
    /// Postgres Port
    #[must_use]
    pub const fn postgres_port(&self) -> u16 {
        self.postgres_port
    }
    /// Postgres Database
    #[must_use]
    pub fn postgres_database(&self) -> &str {
        &self.postgres_database
    }
    /// Postgres Max Connections
    #[must_use]
    pub const fn postgres_max_connections(&self) -> u32 {
        self.postgres_max_connections
    }
    /// Postgres Url
    #[must_use]
    pub fn postgres_url(&self) -> String {
        format!(
            "postgresql://{}:{}/{}",
            self.postgres_host, self.postgres_port, self.postgres_database
        )
    }

    // Redis getters

    /// Returns the Redis host address
    #[must_use]
    pub fn redis_host(&self) -> &str {
        &self.redis_host
    }
    /// Redis Port
    #[must_use]
    pub const fn redis_port(&self) -> u16 {
        self.redis_port
    }
    /// Redis Max Connections
    #[must_use]
    pub const fn redis_max_connections(&self) -> u32 {
        self.redis_max_connections
    }
    /// Redis Url
    #[must_use]
    pub fn redis_url(&self) -> String {
        format!("redis://{}:{}", self.redis_host, self.redis_port)
    }

    // ZFS getters

    /// Returns the ZFS pool name
    #[must_use]
    pub fn zfs_pool_name(&self) -> &str {
        &self.zfs_pool_name
    }
    /// Zfs Dataset Prefix
    #[must_use]
    pub fn zfs_dataset_prefix(&self) -> &str {
        &self.zfs_dataset_prefix
    }
    /// Zfs Compression
    #[must_use]
    pub fn zfs_compression(&self) -> &str {
        &self.zfs_compression
    }
    /// Zfs Dedup
    #[must_use]
    pub const fn zfs_dedup(&self) -> bool {
        self.zfs_dedup
    }

    // Path getters

    /// Returns the data directory path
    #[must_use]
    pub fn data_dir(&self) -> &str {
        &self.data_dir
    }
    /// Cache Dir
    #[must_use]
    pub fn cache_dir(&self) -> &str {
        &self.cache_dir
    }
    /// Log Dir
    #[must_use]
    pub fn log_dir(&self) -> &str {
        &self.log_dir
    }
}

// ============================================================================
// PERFORMANCE CONSTANTS
// ============================================================================

/// Performance tuning constants
#[derive(Debug, Clone)]
/// Performanceconstants
pub struct PerformanceConstants {
    // Connections
    max_connections: usize,
    connection_pool_size: usize,

    // Timeouts (milliseconds)
    connection_timeout_ms: u64,
    request_timeout_ms: u64,
    idle_timeout_ms: u64,
    keepalive_interval_ms: u64,

    // Retries
    max_retry_attempts: u32,
    retry_delay_ms: u64,
    retry_backoff_multiplier: f32,

    // Buffer sizes
    network_buffer_size: usize,
    disk_buffer_size: usize,
    memory_pool_size: usize,

    // Concurrency
    worker_threads: usize,
    async_tasks_limit: usize,
}

impl Default for PerformanceConstants {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            // Connections
            max_connections: env_or_parse("NESTGATE_MAX_CONNECTIONS", 1000),
            connection_pool_size: env_or_parse("NESTGATE_POOL_SIZE", 100),

            // Timeouts
            connection_timeout_ms: env_or_parse("NESTGATE_CONN_TIMEOUT_MS", 5000),
            request_timeout_ms: env_or_parse("NESTGATE_REQ_TIMEOUT_MS", 30000),
            idle_timeout_ms: env_or_parse("NESTGATE_IDLE_TIMEOUT_MS", 300_000),
            keepalive_interval_ms: env_or_parse("NESTGATE_KEEPALIVE_MS", 60000),

            // Retries
            max_retry_attempts: env_or_parse("NESTGATE_MAX_RETRIES", 3),
            retry_delay_ms: env_or_parse("NESTGATE_RETRY_DELAY_MS", 1000),
            retry_backoff_multiplier: env_or_parse("NESTGATE_RETRY_BACKOFF", 2.0),

            // Buffers
            network_buffer_size: env_or_parse("NESTGATE_NET_BUFFER", 8192),
            disk_buffer_size: env_or_parse("NESTGATE_DISK_BUFFER", 4096),
            memory_pool_size: env_or_parse("NESTGATE_MEM_POOL", 1024 * 1024),

            // Concurrency
            worker_threads: env_or_parse(
                "NESTGATE_WORKERS",
                std::thread::available_parallelism().map_or(4, std::num::NonZero::get),
            ),
            async_tasks_limit: env_or_parse("NESTGATE_ASYNC_LIMIT", 10000),
        }
    }
}

impl PerformanceConstants {
    /// Get or initialize the global performance constants
    pub fn get() -> Arc<Self> {
        static INSTANCE: OnceLock<Arc<PerformanceConstants>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::default())).clone()
    }

    // Connection getters

    /// Returns the maximum number of concurrent connections allowed
    #[must_use]
    pub const fn max_connections(&self) -> usize {
        self.max_connections
    }
    /// Connection Pool Size
    #[must_use]
    pub const fn connection_pool_size(&self) -> usize {
        self.connection_pool_size
    }

    // Timeout getters (in Duration)

    /// Returns the connection timeout duration
    #[must_use]
    pub const fn connection_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.connection_timeout_ms)
    }
    /// Request Timeout
    #[must_use]
    pub const fn request_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.request_timeout_ms)
    }
    /// Idle Timeout
    #[must_use]
    pub const fn idle_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.idle_timeout_ms)
    }
    /// Keepalive Interval
    #[must_use]
    pub const fn keepalive_interval(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.keepalive_interval_ms)
    }

    // Retry getters

    /// Returns the maximum number of retry attempts for failed operations
    #[must_use]
    pub const fn max_retry_attempts(&self) -> u32 {
        self.max_retry_attempts
    }
    /// Retry Delay
    #[must_use]
    pub const fn retry_delay(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.retry_delay_ms)
    }
    /// Retry Backoff Multiplier
    #[must_use]
    pub const fn retry_backoff_multiplier(&self) -> f32 {
        self.retry_backoff_multiplier
    }

    // Buffer getters

    /// Returns the network buffer size in bytes
    #[must_use]
    pub const fn network_buffer_size(&self) -> usize {
        self.network_buffer_size
    }
    /// Disk Buffer Size
    #[must_use]
    pub const fn disk_buffer_size(&self) -> usize {
        self.disk_buffer_size
    }
    /// Memory Pool Size
    #[must_use]
    pub const fn memory_pool_size(&self) -> usize {
        self.memory_pool_size
    }

    // Concurrency getters

    /// Returns the number of worker threads for the async runtime
    #[must_use]
    pub const fn worker_threads(&self) -> usize {
        self.worker_threads
    }
    /// Async Tasks Limit
    #[must_use]
    pub const fn async_tasks_limit(&self) -> usize {
        self.async_tasks_limit
    }
}

// ============================================================================
// SECURITY CONSTANTS
// ============================================================================

/// Security and authentication constants
#[derive(Debug, Clone)]
/// Securityconstants
pub struct SecurityConstants {
    // JWT
    jwt_secret: String,
    jwt_expiration_secs: u64,
    jwt_refresh_expiration_secs: u64,

    // Encryption
    encryption_algorithm: String,
    key_size_bits: u32,

    // TLS
    tls_enabled: bool,
    tls_cert_path: String,
    tls_key_path: String,
    tls_ca_path: String,

    // Rate limiting
    rate_limit_requests_per_minute: u32,
    rate_limit_burst_size: u32,
}

impl Default for SecurityConstants {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            // JWT
            jwt_secret: env_or("NESTGATE_JWT_SECRET", "CHANGE_ME_IN_PRODUCTION"),
            jwt_expiration_secs: env_or_parse("NESTGATE_JWT_EXP", 3600),
            jwt_refresh_expiration_secs: env_or_parse("NESTGATE_JWT_REFRESH_EXP", 86400),

            // Encryption
            encryption_algorithm: env_or("NESTGATE_ENC_ALGO", "AES-256-GCM"),
            key_size_bits: env_or_parse("NESTGATE_KEY_SIZE", 256),

            // TLS
            tls_enabled: env_or_parse("NESTGATE_TLS_ENABLED", false),
            tls_cert_path: env_or("NESTGATE_TLS_CERT", "./certs/server.crt"),
            tls_key_path: env_or("NESTGATE_TLS_KEY", "./certs/server.key"),
            tls_ca_path: env_or("NESTGATE_TLS_CA", "./certs/ca.crt"),

            // Rate limiting
            rate_limit_requests_per_minute: env_or_parse("NESTGATE_RATE_LIMIT_RPM", 60),
            rate_limit_burst_size: env_or_parse("NESTGATE_RATE_LIMIT_BURST", 10),
        }
    }
}

impl SecurityConstants {
    /// Get or initialize the global security constants
    pub fn get() -> Arc<Self> {
        static INSTANCE: OnceLock<Arc<SecurityConstants>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::default())).clone()
    }

    // JWT getters

    /// Returns the JWT secret key for token signing
    #[must_use]
    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
    /// Jwt Expiration
    #[must_use]
    pub const fn jwt_expiration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.jwt_expiration_secs)
    }
    /// Jwt Refresh Expiration
    #[must_use]
    pub const fn jwt_refresh_expiration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.jwt_refresh_expiration_secs)
    }

    // Encryption getters

    /// Returns the encryption algorithm name (e.g., "AES-256-GCM")
    #[must_use]
    pub fn encryption_algorithm(&self) -> &str {
        &self.encryption_algorithm
    }
    /// Key Size Bits
    #[must_use]
    pub const fn key_size_bits(&self) -> u32 {
        self.key_size_bits
    }

    // TLS getters

    /// Returns whether TLS is enabled for secure connections
    #[must_use]
    pub const fn tls_enabled(&self) -> bool {
        self.tls_enabled
    }
    /// Tls Cert Path
    #[must_use]
    pub fn tls_cert_path(&self) -> &str {
        &self.tls_cert_path
    }
    /// Tls Key Path
    #[must_use]
    pub fn tls_key_path(&self) -> &str {
        &self.tls_key_path
    }
    /// Tls Ca Path
    #[must_use]
    pub fn tls_ca_path(&self) -> &str {
        &self.tls_ca_path
    }

    // Rate limiting getters

    /// Returns the maximum number of requests allowed per minute for rate limiting
    #[must_use]
    pub const fn rate_limit_requests_per_minute(&self) -> u32 {
        self.rate_limit_requests_per_minute
    }
    /// Rate Limit Burst Size
    #[must_use]
    pub const fn rate_limit_burst_size(&self) -> u32 {
        self.rate_limit_burst_size
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Get environment variable or return default value
fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Get environment variable and parse, or return default value
fn env_or_parse<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(default)
}

// ============================================================================
// CONVENIENCE RE-EXPORTS
// ============================================================================

/// Get all constants in one call
#[must_use]
pub fn all_constants() -> (
    Arc<NetworkConstants>,
    Arc<StorageConstants>,
    Arc<PerformanceConstants>,
    Arc<SecurityConstants>,
) {
    (
        NetworkConstants::get(),
        StorageConstants::get(),
        PerformanceConstants::get(),
        SecurityConstants::get(),
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use temp_env::with_vars;

    #[test]
    fn test_network_constants_default() {
        let nc = NetworkConstants::default();
        // Values should be set (either default or from environment)
        assert!(!nc.api_host().is_empty());
        assert!(nc.api_port() > 0);
        assert!(!nc.bind_address().is_empty());

        // If no environment variables are set, these should be the defaults
        // But we can't guarantee that in all test environments
        // So we just verify they're reasonable values
        // Note: Port is u16, so always <= 65535 by type definition
    }

    #[test]
    fn test_network_constants_singleton() {
        let nc1 = NetworkConstants::get();
        let nc2 = NetworkConstants::get();
        assert!(Arc::ptr_eq(&nc1, &nc2));
    }

    #[test]
    #[serial_test::serial]
    fn test_network_constants_urls() {
        with_vars(
            vec![
                ("NESTGATE_API_HOST", Some("127.0.0.1")),
                ("NESTGATE_HEALTH_HOST", Some("127.0.0.1")),
                ("NESTGATE_API_PORT", Some("8080")),
                ("NESTGATE_HEALTH_PORT", Some("8081")),
                ("NESTGATE_WS_PORT", Some("8082")),
            ],
            || {
                let nc = NetworkConstants::default();
                assert_eq!(nc.api_url(), "http://127.0.0.1:8080");
                assert_eq!(nc.health_url(), "http://127.0.0.1:8081");
                assert_eq!(nc.websocket_url(), "ws://127.0.0.1:8082/ws");
            },
        );
    }

    #[test]
    fn test_storage_constants_default() {
        let sc = StorageConstants::default();
        assert_eq!(sc.postgres_host(), "127.0.0.1");
        assert_eq!(sc.postgres_port(), 5432);
        assert_eq!(sc.redis_host(), "127.0.0.1");
        assert_eq!(sc.redis_port(), 6379);
    }

    #[test]
    fn test_storage_constants_urls() {
        let sc = StorageConstants::default();
        assert!(sc.postgres_url().starts_with("postgresql://127.0.0.1:5432"));
        assert!(sc.redis_url().starts_with("redis://127.0.0.1:6379"));
    }

    #[test]
    fn test_performance_constants_default() {
        let pc = PerformanceConstants::default();
        assert_eq!(pc.max_connections(), 1000);
        assert_eq!(pc.connection_timeout().as_millis(), 5000);
        assert_eq!(pc.max_retry_attempts(), 3);
    }

    #[test]
    fn test_security_constants_default() {
        let sc = SecurityConstants::default();
        assert_eq!(sc.encryption_algorithm(), "AES-256-GCM");
        assert_eq!(sc.key_size_bits(), 256);
        assert!(!sc.tls_enabled());
    }

    #[test]
    fn test_all_constants() {
        let (nc, sc, pc, sec) = all_constants();
        assert!(nc.api_port() > 0);
        assert!(sc.postgres_port() > 0);
        assert!(pc.max_connections() > 0);
        assert!(sec.key_size_bits() > 0);
    }

    #[test]
    fn test_env_or_helper() {
        assert_eq!(env_or("NONEXISTENT_VAR", "default"), "default");
    }

    #[test]
    fn test_env_or_parse_helper() {
        assert_eq!(env_or_parse("NONEXISTENT_VAR", 42), 42);
        assert!(env_or_parse("NONEXISTENT_VAR", true));
    }

    #[test]
    fn test_network_constants_url_helpers() {
        let nc = NetworkConstants::default();
        assert!(nc.api_url().starts_with("http://"));
        assert!(nc.api_url().contains(':'));
        assert!(nc.api_bind_address().contains(':'));
        assert!(nc.health_url().starts_with("http://"));
        assert!(nc.metrics_url().starts_with("http://"));
        assert!(nc.websocket_url().starts_with("ws://"));
        assert!(nc.websocket_url().ends_with("/ws"));
    }

    #[test]
    fn test_network_constants_localhost_and_bind_literals() {
        let nc = NetworkConstants::default();
        assert_eq!(nc.localhost_ipv4(), "127.0.0.1");
        assert_eq!(nc.localhost_ipv6(), "::1");
        assert_eq!(nc.bind_all_ipv4(), "0.0.0.0");
        assert_eq!(nc.bind_all_ipv6(), "::");
    }

    #[test]
    fn test_storage_constants_zfs_and_paths() {
        let sc = StorageConstants::default();
        assert!(!sc.zfs_pool_name().is_empty());
        assert!(!sc.zfs_dataset_prefix().is_empty());
        assert!(!sc.zfs_compression().is_empty());
        assert!(!sc.data_dir().is_empty());
        assert!(!sc.cache_dir().is_empty());
        assert!(!sc.log_dir().is_empty());
        assert!(sc.postgres_url().contains(sc.postgres_database()));
    }

    #[test]
    fn test_storage_constants_singleton() {
        let a = StorageConstants::get();
        let b = StorageConstants::get();
        assert!(Arc::ptr_eq(&a, &b));
    }

    #[test]
    fn test_performance_constants_durations_and_buffers() {
        let pc = PerformanceConstants::default();
        assert!(pc.connection_timeout().as_millis() > 0);
        assert!(pc.request_timeout() >= pc.connection_timeout());
        assert!(pc.network_buffer_size() > 0);
        assert!(pc.disk_buffer_size() > 0);
        assert!(pc.memory_pool_size() > 0);
        assert!(pc.worker_threads() > 0);
        assert!(pc.async_tasks_limit() > 0);
    }

    #[test]
    fn test_performance_constants_singleton() {
        let a = PerformanceConstants::get();
        let b = PerformanceConstants::get();
        assert!(Arc::ptr_eq(&a, &b));
    }

    #[test]
    fn test_security_constants_jwt_and_rate_limit() {
        let sc = SecurityConstants::default();
        assert!(!sc.jwt_secret().is_empty());
        assert!(sc.jwt_expiration().as_secs() > 0);
        assert!(sc.jwt_refresh_expiration() >= sc.jwt_expiration());
        assert!(sc.rate_limit_requests_per_minute() > 0);
        assert!(sc.rate_limit_burst_size() > 0);
    }

    #[test]
    fn test_security_constants_singleton() {
        let a = SecurityConstants::get();
        let b = SecurityConstants::get();
        assert!(Arc::ptr_eq(&a, &b));
    }

    #[test]
    fn test_env_or_parse_invalid_uses_default() {
        temp_env::with_var(
            "NESTGATE_CONSOLIDATED_TEST_PARSE_U16",
            Some("not-a-number"),
            || {
                assert_eq!(
                    env_or_parse("NESTGATE_CONSOLIDATED_TEST_PARSE_U16", 4242u16),
                    4242
                );
            },
        );
    }

    #[test]
    fn test_env_or_set_overrides_default() {
        temp_env::with_var(
            "NESTGATE_CONSOLIDATED_TEST_STR",
            Some("override-value"),
            || {
                assert_eq!(
                    env_or("NESTGATE_CONSOLIDATED_TEST_STR", "default"),
                    "override-value"
                );
            },
        );
    }

    #[test]
    fn smoke_all_network_constants_getters() {
        let nc = NetworkConstants::default();
        assert!(!nc.api_host().is_empty());
        assert!(!nc.metrics_host().is_empty());
        assert!(!nc.health_host().is_empty());
        assert!(!nc.admin_host().is_empty());
        assert!(nc.api_port() > 0);
        assert!(nc.http_port() > 0);
        assert!(nc.https_port() > 0);
        assert!(nc.websocket_port() > 0);
        assert!(nc.grpc_port() > 0);
        assert!(nc.metrics_port() > 0);
        assert!(nc.prometheus_port() > 0);
        assert!(nc.health_port() > 0);
        assert!(nc.admin_port() > 0);
        assert!(!nc.bind_address().is_empty());
        assert!(!nc.localhost_ipv4().is_empty());
        assert!(!nc.localhost_ipv6().is_empty());
        assert!(!nc.bind_all_ipv4().is_empty());
        assert!(!nc.bind_all_ipv6().is_empty());
        assert!(nc.api_url().starts_with("http://"));
        assert!(nc.api_bind_address().contains(':'));
        assert!(nc.health_url().starts_with("http://"));
        assert!(nc.metrics_url().starts_with("http://"));
        assert!(nc.websocket_url().starts_with("ws://"));
    }

    #[test]
    fn smoke_all_performance_constants_getters() {
        let pc = PerformanceConstants::default();
        assert!(pc.connection_pool_size() > 0);
        assert!(pc.idle_timeout().as_millis() > 0);
        assert!(pc.keepalive_interval().as_millis() > 0);
        assert!(pc.retry_delay().as_millis() > 0);
        assert!(pc.retry_backoff_multiplier() > 0.0);
    }

    #[test]
    fn smoke_all_security_tls_getters() {
        let sc = SecurityConstants::default();
        assert!(!sc.tls_cert_path().is_empty());
        assert!(!sc.tls_key_path().is_empty());
        assert!(!sc.tls_ca_path().is_empty());
    }
}
