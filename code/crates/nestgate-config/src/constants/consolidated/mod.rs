// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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

mod defaults;
mod network;
mod performance;
mod security;
mod storage;

pub use network::NetworkConstants;
pub use performance::PerformanceConstants;
pub use security::SecurityConstants;
pub use storage::StorageConstants;

use std::sync::Arc;

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

#[cfg(test)]
mod tests {
    use super::defaults::{env_or, env_or_parse};
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
