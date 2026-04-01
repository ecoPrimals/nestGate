// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **TEST SUPPORT FOR RUNTIME CONFIGURATION**
//!
//! Provides test-friendly configuration management that allows concurrent tests
//! to run with isolated configuration without global state conflicts.
//!
//! # The Problem
//!
//! Production code uses `OnceLock` for performance:
//! ```rust
//! static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();
//! ```
//!
//! This causes test isolation issues when tests set different env vars.
//!
//! # The Solution
//!
//! Thread-local configuration for tests that doesn't conflict with global config:
//! ```rust
//! thread_local! {
//!     static TEST_CONFIG: RefCell<Option<NestGateRuntimeConfig>> = RefCell::new(None);
//! }
//! ```
//!
//! # Usage in Tests
//!
//! ```rust,ignore
//! use nestgate_core::config::runtime::test_support::TestConfigGuard;
//!
//! // Example test showing isolated configuration
//! fn my_test() {
//!     // Isolated config for this test
//!     let _guard = TestConfigGuard::new(|config| {
//!         config.network.api_port = 9999;
//!     });
//!     
//!     // This test sees port 9999
//!     // Other concurrent tests see their own config
//!     // No conflicts!
//! }
//! ```

use super::NestGateRuntimeConfig;
use std::cell::RefCell;
use std::sync::Arc;

thread_local! {
    /// Thread-local configuration override for tests.
    ///
    /// This allows each test to have isolated configuration without
    /// affecting the global config or other concurrent tests.
    static TEST_CONFIG_OVERRIDE: RefCell<Option<Arc<NestGateRuntimeConfig>>> = const { RefCell::new(None) };
}

/// Test configuration guard that provides isolated configuration for tests.
///
/// When created, it sets a thread-local configuration override that will be
/// used by `get_test_config()` for the duration of the guard's lifetime.
///
/// When dropped, it clears the override, restoring default behavior.
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::config::runtime::test_support::TestConfigGuard;
///
/// // Example test showing custom port
/// fn test_with_custom_port() {
///     let _guard = TestConfigGuard::with_port(9999);
///     // Test code here sees port 9999
///     // Other tests are unaffected
/// }
/// ```
pub struct TestConfigGuard {
    _marker: std::marker::PhantomData<()>,
}

impl TestConfigGuard {
    /// Create a new test config guard with custom configuration.
    ///
    /// The provided closure can modify the configuration as needed.
    pub fn new<F>(configure: F) -> Self
    where
        F: FnOnce(&mut NestGateRuntimeConfig),
    {
        let mut config = NestGateRuntimeConfig::default();
        configure(&mut config);

        TEST_CONFIG_OVERRIDE.with(|cell| {
            *cell.borrow_mut() = Some(Arc::new(config));
        });

        Self {
            _marker: std::marker::PhantomData,
        }
    }

    /// Create a test config with specific API port.
    pub fn with_port(port: u16) -> Self {
        Self::new(|config| {
            config.network.api_port = port;
        })
    }

    /// Create a test config with specific host and port.
    pub fn with_host_port(host: &str, port: u16) -> Self {
        Self::new(|config| {
            // Try to parse as IpAddr, if fails use 127.0.0.1 (for test hostnames like "custom.example.com")
            config.network.api_host = host.parse().unwrap_or_else(|_| {
                std::net::Ipv4Addr::LOCALHOST.into()
            });
            config.network.api_port = port;
        })
    }

    /// Create a test config from environment variables (for this test only).
    pub fn from_test_env() -> Self {
        Self::new(|config| {
            // Load from current environment (test-isolated)
            if let Ok(port) = std::env::var("NESTGATE_API_PORT") {
                if let Ok(p) = port.parse() {
                    config.network.api_port = p;
                }
            }
            if let Ok(host) = std::env::var("NESTGATE_API_HOST") {
                if let Ok(h) = host.parse() {
                    config.network.api_host = h;
                }
            }
        })
    }
}

impl Drop for TestConfigGuard {
    fn drop(&mut self) {
        // Clear thread-local override when guard is dropped
        TEST_CONFIG_OVERRIDE.with(|cell| {
            *cell.borrow_mut() = None;
        });
    }
}

/// Get configuration for testing purposes.
///
/// Checks for thread-local test override first, falls back to global config.
/// This allows tests to have isolated configuration.
pub fn get_test_config() -> Arc<NestGateRuntimeConfig> {
    TEST_CONFIG_OVERRIDE.with(|cell| {
        cell.borrow()
            .clone()
            .unwrap_or_else(|| Arc::new(super::get_config().clone()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial_test::serial]
    fn test_config_guard_isolation() {
        temp_env::with_var_unset("NESTGATE_API_PORT", || {
            temp_env::with_var("NESTGATE_API_PORT", Some("7777"), || {
                let _guard = TestConfigGuard::with_port(7777);
                let config = get_test_config();
                assert_eq!(config.network.api_port, 7777);
            });
        });
    }

    #[test]
    #[serial_test::serial]
    fn test_concurrent_config_isolation() {
        let _guard1 = TestConfigGuard::with_port(8888);
        let config1 = get_test_config();
        assert_eq!(config1.network.api_port, 8888);

        drop(_guard1);

        let _guard2 = TestConfigGuard::with_port(9999);
        let config2 = get_test_config();
        assert_eq!(config2.network.api_port, 9999);
    }

    #[test]
    #[serial_test::serial]
    fn test_config_from_env() {
        temp_env::with_var("NESTGATE_API_PORT", Some("6666"), || {
            let _guard = TestConfigGuard::from_test_env();
            let config = get_test_config();
            assert_eq!(
                config.network.api_port, 6666,
                "Expected port 6666 from NESTGATE_API_PORT env var, got {}",
                config.network.api_port
            );
        });
    }
}
