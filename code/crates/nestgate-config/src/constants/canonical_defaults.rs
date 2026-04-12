// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CANONICAL DEFAULTS**
//!
//! Default values for canonical configuration and network settings.

use nestgate_types::{EnvSource, ProcessEnv, env_var_or_default};

pub mod network {
    //! Network-related canonical defaults

    use super::{EnvSource, ProcessEnv, env_var_or_default};

    /// Hostname used when building default URLs when `NESTGATE_DEV_HOST` and
    /// `NESTGATE_DISCOVERY_FALLBACK_HOST` are unset (last resort: `localhost` with a warning).
    ///
    /// Like [`discovery_default_host_from_env_source`], but resolves once per process and reads
    /// from [`ProcessEnv`].
    fn discovery_default_host() -> String {
        use std::sync::OnceLock;
        static HOST: OnceLock<String> = OnceLock::new();
        HOST.get_or_init(|| discovery_default_host_from_env_source(&ProcessEnv))
            .clone()
    }

    /// Hostname for default URLs from an injectable [`EnvSource`].
    #[must_use]
    pub fn discovery_default_host_from_env_source(env: &(impl EnvSource + ?Sized)) -> String {
        env.get("NESTGATE_DEV_HOST")
            .or_else(|| env.get("NESTGATE_DISCOVERY_FALLBACK_HOST"))
            .unwrap_or_else(|| {
                tracing::warn!(
                    "Canonical default URL: NESTGATE_DEV_HOST and NESTGATE_DISCOVERY_FALLBACK_HOST unset; \
                     using host `localhost`. Set explicit capability endpoints or discovery env vars for production."
                );
                "localhost".to_string()
            })
    }

    /// Default API base URL (environment-driven)
    ///
    /// Uses `NESTGATE_API_PORT` for the port when set, otherwise defaults to 8080.
    /// Hostname comes from `NESTGATE_DEV_HOST` / `NESTGATE_DISCOVERY_FALLBACK_HOST` before
    /// falling back to `localhost` (with warning).
    #[must_use]
    pub fn default_api_base_url() -> String {
        let port = env_var_or_default(&ProcessEnv, "NESTGATE_API_PORT", "8080");
        format!("http://{}:{}", discovery_default_host(), port)
    }

    /// Like [`default_api_base_url`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn default_api_base_url_from_env_source(env: &(impl EnvSource + ?Sized)) -> String {
        let port = env_var_or_default(env, "NESTGATE_API_PORT", "8080");
        format!(
            "http://{}:{}",
            discovery_default_host_from_env_source(env),
            port
        )
    }

    /// Default WebSocket URL (environment-driven)
    ///
    /// Uses `NESTGATE_WEBSOCKET_PORT` when set. Hostname follows [`default_api_base_url`].
    #[must_use]
    pub fn default_websocket_url() -> String {
        let port = env_var_or_default(&ProcessEnv, "NESTGATE_WEBSOCKET_PORT", "8080");
        format!("ws://{}:{}/ws", discovery_default_host(), port)
    }

    /// Like [`default_websocket_url`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn default_websocket_url_from_env_source(env: &(impl EnvSource + ?Sized)) -> String {
        let port = env_var_or_default(env, "NESTGATE_WEBSOCKET_PORT", "8080");
        format!(
            "ws://{}:{}/ws",
            discovery_default_host_from_env_source(env),
            port
        )
    }

    /// Default metrics URL (environment-driven)
    ///
    /// Uses `NESTGATE_METRICS_PORT` when set. Hostname follows [`default_api_base_url`].
    #[must_use]
    pub fn default_metrics_url() -> String {
        let port = env_var_or_default(&ProcessEnv, "NESTGATE_METRICS_PORT", "9090");
        format!("http://{}:{}", discovery_default_host(), port)
    }

    /// Like [`default_metrics_url`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn default_metrics_url_from_env_source(env: &(impl EnvSource + ?Sized)) -> String {
        let port = env_var_or_default(env, "NESTGATE_METRICS_PORT", "9090");
        format!(
            "http://{}:{}",
            discovery_default_host_from_env_source(env),
            port
        )
    }

    /// Default web UI URL (environment-driven)
    #[must_use]
    pub fn default_web_ui_url() -> String {
        let port = env_var_or_default(&ProcessEnv, "NESTGATE_WEB_UI_PORT", "3000");
        format!("http://{}:{}", discovery_default_host(), port)
    }

    /// Like [`default_web_ui_url`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn default_web_ui_url_from_env_source(env: &(impl EnvSource + ?Sized)) -> String {
        let port = env_var_or_default(env, "NESTGATE_WEB_UI_PORT", "3000");
        format!(
            "http://{}:{}",
            discovery_default_host_from_env_source(env),
            port
        )
    }

    /// Localhost constant
    pub const LOCALHOST: &str = "127.0.0.1";

    /// Default bind address
    pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";

    // Port constants moved to port_defaults.rs (single source of truth)
    // Re-exported here for backward compatibility
    pub use super::super::port_defaults::{
        DEFAULT_ADMIN_PORT as DEFAULT_INTERNAL_PORT, DEFAULT_API_PORT, DEFAULT_METRICS_PORT,
    };

    /// Build API URL from environment or runtime discovery.
    #[must_use]
    pub fn build_api_url() -> String {
        ProcessEnv
            .get("NESTGATE_API_URL")
            .unwrap_or_else(default_api_base_url)
    }

    /// Like [`build_api_url`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn build_api_url_from_env_source(env: &(impl EnvSource + ?Sized)) -> String {
        env.get("NESTGATE_API_URL")
            .unwrap_or_else(|| default_api_base_url_from_env_source(env))
    }

    /// Build WebSocket URL from environment or runtime discovery.
    #[must_use]
    pub fn build_websocket_url() -> String {
        ProcessEnv
            .get("NESTGATE_WS_URL")
            .unwrap_or_else(default_websocket_url)
    }

    /// Like [`build_websocket_url`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn build_websocket_url_from_env_source(env: &(impl EnvSource + ?Sized)) -> String {
        env.get("NESTGATE_WS_URL")
            .unwrap_or_else(|| default_websocket_url_from_env_source(env))
    }

    /// Build metrics URL from environment or runtime discovery.
    #[must_use]
    pub fn build_metrics_url() -> String {
        ProcessEnv
            .get("NESTGATE_METRICS_URL")
            .unwrap_or_else(default_metrics_url)
    }

    /// Like [`build_metrics_url`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn build_metrics_url_from_env_source(env: &(impl EnvSource + ?Sized)) -> String {
        env.get("NESTGATE_METRICS_URL")
            .unwrap_or_else(|| default_metrics_url_from_env_source(env))
    }

    /// Build generic endpoint from environment or runtime discovery.
    #[must_use]
    pub fn build_endpoint() -> String {
        build_api_url()
    }

    pub mod limits {
        //! Network limits and constraints

        /// Maximum concurrent requests the system will handle simultaneously
        ///
        /// **Philosophy**: Balanced for typical server capacity without overwhelming system resources.
        /// This prevents resource exhaustion attacks while allowing good throughput.
        ///
        /// **Override**: Set `NESTGATE_MAX_CONCURRENT_REQUESTS` environment variable for custom limits.
        /// Consider your available memory (each request ~1-10MB) and CPU cores.
        ///
        /// **Evolution**: This will become capability-based discovery in future versions.
        pub const MAX_CONCURRENT_REQUESTS: usize = 1000;

        /// Maximum size of a single HTTP request body in bytes
        ///
        /// **Philosophy**: 1MB strikes a balance between:
        /// - Accepting reasonable payloads (JSON configs, small uploads)
        /// - Preventing memory exhaustion from malicious large requests
        ///
        /// **Override**: Set `NESTGATE_MAX_REQUEST_SIZE` for your use case.
        /// For file uploads, consider streaming instead of buffering entire request.
        ///
        /// **Evolution**: Will move to capability-based negotiation based on available memory.
        pub const MAX_REQUEST_SIZE: usize = 1024 * 1024; // 1MB

        /// Connection establishment timeout in milliseconds
        ///
        /// **Philosophy**: 5 seconds allows for:
        /// - Typical network conditions (local: <100ms, remote: <3s)
        /// - DNS resolution time
        /// - TLS handshake completion
        ///
        /// **When to adjust**: Increase for satellite/poor connections, decrease for local networks.
        ///
        /// **Evolution**: Will become dynamic based on connection history and network quality detection.
        pub const CONNECTION_TIMEOUT_MS: u64 = 5000;
    }
}

pub mod performance {
    //! Performance-related defaults
    //!
    //! # Buffer Size Guidelines
    //!
    //! **IMPORTANT**: Do NOT consolidate these buffer sizes - they are intentionally different!
    //!
    //! ## Network Buffer (8192 bytes)
    //! Optimized for network I/O operations:
    //! - Balances memory usage vs system call overhead
    //! - Smaller than `canonical::performance::NETWORK_BUFFER_SIZE` (64KB) for memory efficiency
    //! - Good for moderate throughput scenarios
    //!
    //! ## Default Buffer (4096 bytes)
    //! Optimized for disk/general I/O:
    //! - Matches typical filesystem page size
    //! - Optimal for disk operations
    //! - Fits in L1/L2 cache for fast access

    /// Network buffer size (8KB) - optimized for network I/O
    ///
    /// **Use for**: Network operations, socket I/O, moderate throughput
    /// **Performance**: Balances memory vs system call overhead
    pub const NETWORK_BUFFER_SIZE: usize = 8192;

    /// Default buffer size (4KB) - optimized for disk I/O
    ///
    /// **Use for**: File operations, disk I/O, general buffering
    /// **Performance**: Matches page size, optimal for filesystem operations
    pub const DEFAULT_BUFFER_SIZE: usize = 4096;

    /// Maximum number of persistent connections to maintain in the connection pool
    ///
    /// **Philosophy**: Connection pooling amortizes TCP/TLS handshake costs.
    /// 1000 connections balances:
    /// - Memory per connection (~4-8KB)
    /// - File descriptor limits (typical: ulimit 1024-4096)
    /// - Concurrent client capacity
    ///
    /// **Trade-offs**:
    /// - More connections = higher memory but lower latency for new requests
    /// - Fewer connections = lower memory but may need to establish connections
    ///
    /// **Evolution**: Will become adaptive based on:
    /// - Available system resources (memory, file descriptors)
    /// - Actual traffic patterns
    /// - Connection reuse rates
    pub const MAX_CONNECTIONS: usize = 1000;
}

pub mod concurrency {
    //! Concurrency and threading defaults
    //!
    //! **Philosophy**: Balance parallelism with resource consumption.
    //! Too many threads waste memory, too few underutilize CPUs.

    /// Default thread pool size
    ///
    /// **Rationale**: 8 threads balances:
    /// - Typical multi-core systems (4-16 cores)
    /// - Context switching overhead
    /// - Memory per thread (~2-8MB stack)
    ///
    /// **Tuning**: Set to `logical_cpu_count * 2` for I/O-bound work,
    /// `logical_cpu_count` (from `std::thread::available_parallelism`) for CPU-bound work.
    ///
    /// **Evolution**: Will become auto-tuned based on:
    /// - Detected CPU count
    /// - Workload characteristics (I/O vs CPU-bound)
    /// - Available memory
    pub const DEFAULT_THREAD_POOL_SIZE: usize = 8;

    /// Default maximum connections
    ///
    /// **Rationale**: 1000 concurrent connections handles:
    /// - Small to medium deployments
    /// - Typical connection duration (100ms-10s)
    /// - Memory per connection (~10-50KB)
    ///
    /// **Trade-offs**:
    /// - More connections = Higher memory but better throughput
    /// - Fewer connections = Lower memory but potential queuing
    ///
    /// **Evolution**: Will adapt to available memory and connection patterns.
    pub const DEFAULT_MAX_CONNECTIONS: usize = 1000;

    /// Default worker count for background tasks
    ///
    /// **Rationale**: 4 workers provides:
    /// - Parallel background processing
    /// - Without overwhelming system
    /// - Good for periodic tasks (cleanup, monitoring, etc.)
    ///
    /// **Use cases**: Maintenance tasks, cache warming, metrics collection
    ///
    /// **Evolution**: Will scale with system resources and task queue depth.
    pub const DEFAULT_WORKER_COUNT: usize = 4;

    /// Default task queue size
    ///
    /// **Rationale**: 10,000 tasks allows:
    /// - Burst traffic absorption
    /// - Smooth out processing spikes
    /// - ~10MB memory (1KB per task)
    ///
    /// **Back-pressure**: When queue fills, apply back-pressure to producers.
    ///
    /// **Evolution**: Will become adaptive based on:
    /// - Producer rate
    /// - Consumer rate
    /// - Available memory
    pub const DEFAULT_QUEUE_SIZE: usize = 10000;

    /// Default maximum parallel tests
    ///
    /// **Rationale**: 4 parallel tests balances:
    /// - Test execution speed
    /// - Resource isolation (ports, files, etc.)
    /// - Deterministic test results
    ///
    /// **Why not more?**: Tests may conflict on shared resources.
    /// **Why not fewer?**: Underutilizes test infrastructure.
    ///
    /// **Override**: `RUST_TEST_THREADS=N` environment variable
    pub const DEFAULT_MAX_PARALLEL_TESTS: usize = 4;
}

pub mod sizes {
    //! Size and capacity defaults
    //! **CONSOLIDATED**: Buffer sizes now reference `hardcoding::limits`

    /// Default buffer size - **CONSOLIDATED** to `hardcoding::limits`
    pub const DEFAULT_BUFFER_SIZE: usize =
        crate::constants::hardcoding::limits::BUFFER_SIZE_DEFAULT;

    /// Default cache size (128MB in bytes)
    pub const DEFAULT_CACHE_SIZE: u64 = 128 * 1024 * 1024;

    /// Default page size (4KB) - **CONSOLIDATED** to `hardcoding::limits`
    pub const DEFAULT_PAGE_SIZE: usize = crate::constants::hardcoding::limits::BUFFER_SIZE_DEFAULT;

    /// Default record size
    pub const DEFAULT_RECORD_SIZE: usize = 256;

    /// Default file size limit (100MB)
    pub const DEFAULT_FILE_SIZE_LIMIT: u64 = 100 * 1024 * 1024;

    /// Default memory limit (1GB)
    pub const DEFAULT_MEMORY_LIMIT: u64 = 1024 * 1024 * 1024;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_default_api_base_url() {
        let url = network::default_api_base_url();
        assert!(url.starts_with("http://"));
        assert!(url.contains("localhost"));
        assert!(url.split(':').next_back().unwrap().parse::<u16>().is_ok());
    }

    #[test]
    fn test_network_default_websocket_url() {
        let url = network::default_websocket_url();
        assert!(url.starts_with("ws://"));
        assert!(url.contains("/ws"));
    }

    #[test]
    fn test_network_default_metrics_url() {
        let url = network::default_metrics_url();
        assert!(url.starts_with("http://"));
        assert!(url.contains("localhost"));
    }

    #[test]
    fn test_network_default_web_ui_url() {
        let url = network::default_web_ui_url();
        assert!(url.starts_with("http://"));
        assert!(url.contains("localhost"));
    }

    #[test]
    fn test_network_constants() {
        assert_eq!(network::LOCALHOST, "127.0.0.1");
        assert_eq!(network::DEFAULT_BIND_ADDRESS, "0.0.0.0");
    }

    #[test]
    fn test_network_limits_constants() {
        assert!(network::limits::MAX_CONCURRENT_REQUESTS > 0);
        assert!(network::limits::MAX_REQUEST_SIZE > 0);
        assert!(network::limits::CONNECTION_TIMEOUT_MS > 0);
    }

    #[test]
    fn test_performance_constants() {
        assert!(performance::NETWORK_BUFFER_SIZE > 0);
        assert!(performance::DEFAULT_BUFFER_SIZE > 0);
        assert!(performance::MAX_CONNECTIONS > 0);
    }

    #[test]
    fn test_concurrency_constants() {
        assert!(concurrency::DEFAULT_THREAD_POOL_SIZE > 0);
        assert!(concurrency::DEFAULT_MAX_CONNECTIONS > 0);
        assert!(concurrency::DEFAULT_WORKER_COUNT > 0);
        assert!(concurrency::DEFAULT_QUEUE_SIZE > 0);
    }

    #[test]
    fn test_sizes_constants() {
        assert!(sizes::DEFAULT_CACHE_SIZE > 0);
        assert!(sizes::DEFAULT_RECORD_SIZE > 0);
        assert!(sizes::DEFAULT_FILE_SIZE_LIMIT > 0);
        assert!(sizes::DEFAULT_MEMORY_LIMIT > 0);
    }

    #[test]
    fn test_timeouts_constants() {
        assert!(timeouts::DEFAULT_TIMEOUT_MS > 0);
        assert!(timeouts::CONNECTION_TIMEOUT_MS > 0);
        assert!(timeouts::REQUEST_TIMEOUT_MS > 0);
    }
}

pub mod timeouts {
    //! Timeout constants
    use std::time::Duration;

    /// Default timeout in milliseconds
    pub const DEFAULT_TIMEOUT_MS: u64 = 5000;

    /// Connection timeout in milliseconds (3 seconds)
    pub const CONNECTION_TIMEOUT_MS: u64 = 3000;

    /// Request timeout in milliseconds (10 seconds)
    pub const REQUEST_TIMEOUT_MS: u64 = 10000;

    /// Default connection timeout as Duration
    pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

    /// Default request timeout as Duration
    pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

    /// Default health check timeout as Duration
    pub const DEFAULT_HEALTH_TIMEOUT: Duration = Duration::from_secs(10);

    /// Default ZFS operation timeout as Duration
    pub const DEFAULT_ZFS_TIMEOUT: Duration = Duration::from_secs(300);

    /// Default test timeout as Duration
    pub const DEFAULT_TEST_TIMEOUT: Duration = Duration::from_secs(30);
}
