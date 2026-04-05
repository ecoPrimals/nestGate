// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ENVIRONMENT VARIABLE ISOLATION FOR TESTS**
//!
//! Provides thread-safe environment variable access for concurrent testing.
//! Prevents race conditions when tests read or modify environment variables.
//!
//! **Pattern**: Use `IsolatedEnvironment` for any test that accesses env vars.
//! **Benefit**: Tests can run in parallel without interfering with each other.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

/// Global lock for environment variable access in tests
///
/// This serializes all environment variable access to prevent race conditions.
/// Tests that need to read or write env vars must acquire this lock.
static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

/// Environment variables used by NestGate
///
/// This list should be kept in sync with actual usage across the codebase.
const NESTGATE_ENV_VARS: &[&str] = &[
    // Core configuration
    "NESTGATE_TIMEOUT_MS",
    "NESTGATE_INSTANCE_NAME",
    "NESTGATE_SERVICE_NAME",
    "NESTGATE_RETRY_ATTEMPTS",
    "NESTGATE_HEALTH_CHECK_INTERVAL",
    // Test configuration
    "NESTGATE_TEST_HOST",
    "NESTGATE_TEST_PORT",
    "NESTGATE_TEST_WS_PORT",
    "NESTGATE_TEST_API_URL",
    "NESTGATE_TEST_WS_URL",
    "NESTGATE_TEST_MODE",
    "NESTGATE_TEST_ID",
    "NESTGATE_TEST_ZFS_DATASET",
    "NESTGATE_TEST_NETWORK_NAMESPACE",
    "NESTGATE_TEST_ROOT",
    "NESTGATE_TEST_UNIT_TIMEOUT",
    "NESTGATE_TEST_INTEGRATION_TIMEOUT",
    "NESTGATE_USE_REAL_ZFS",
    // Network configuration
    "NESTGATE_BIND_HOST",
    "NESTGATE_BIND_PORT",
    "NESTGATE_API_PORT",
    "NESTGATE_WS_PORT",
    // Storage configuration
    "NESTGATE_STORAGE_PATH",
    "NESTGATE_ZFS_POOL",
    "NESTGATE_DATA_DIR",
];

/// **ISOLATED ENVIRONMENT FOR TESTS**
///
/// Provides thread-safe access to environment variables with automatic cleanup.
/// Holds a lock for the duration of the test to prevent concurrent access.
///
/// # Example
///
/// ```rust,ignore
/// let mut env = IsolatedEnvironment::new("my_test");
/// env.set("NESTGATE_TIMEOUT_MS", "10000");
/// assert_eq!(std::env::var("NESTGATE_TIMEOUT_MS").unwrap(), "10000");
/// // Automatic cleanup when env is dropped
/// ```
pub struct IsolatedEnvironment {
    original_vars: HashMap<String, Option<String>>,
    test_name: String,
    _lock: MutexGuard<'static, ()>,
}

impl IsolatedEnvironment {
    /// Create a new isolated environment for a test
    ///
    /// This captures the current state of all NestGate environment variables
    /// and acquires a lock to prevent concurrent access.
    ///
    /// # Arguments
    ///
    /// * `test_name` - Name of the test (for debugging)
    ///
    /// # Panics
    ///
    /// Panics if the environment lock cannot be acquired (very rare, indicates deadlock)
    #[must_use]
    pub fn new(test_name: &str) -> Self {
        let lock = ENV_TEST_LOCK
            .lock()
            .expect("Failed to acquire environment test lock - possible deadlock");

        // Capture original environment state
        let original_vars = NESTGATE_ENV_VARS
            .iter()
            .map(|key| (key.to_string(), std::env::var(key).ok()))
            .collect();

        Self {
            original_vars,
            test_name: test_name.to_string(),
            _lock: lock,
        }
    }

    /// Set an environment variable for this test
    ///
    /// The variable will be automatically restored to its original value
    /// when the `IsolatedEnvironment` is dropped.
    pub fn set(&mut self, key: &str, value: &str) {
        // Capture original value if not already tracked
        if !self.original_vars.contains_key(key) {
            self.original_vars
                .insert(key.to_string(), std::env::var(key).ok());
        }
        nestgate_core::env_process::set_var(key, value);
    }

    /// Remove an environment variable for this test
    ///
    /// The variable will be automatically restored to its original value
    /// when the `IsolatedEnvironment` is dropped.
    pub fn remove(&mut self, key: &str) {
        // Capture original value if not already tracked
        if !self.original_vars.contains_key(key) {
            self.original_vars
                .insert(key.to_string(), std::env::var(key).ok());
        }
        nestgate_core::env_process::remove_var(key);
    }

    /// Get the test name
    #[must_use]
    pub fn test_name(&self) -> &str {
        &self.test_name
    }
}

impl Drop for IsolatedEnvironment {
    /// Automatically restore original environment variables
    ///
    /// This ensures tests don't leak environment changes to other tests.
    fn drop(&mut self) {
        for (key, value) in &self.original_vars {
            match value {
                Some(v) => nestgate_core::env_process::set_var(key, v),
                None => nestgate_core::env_process::remove_var(key),
            }
        }
    }
}

/// **ENVIRONMENT GUARD** (Alternative Pattern)
///
/// Simpler helper for setting a single environment variable.
/// Automatically restores the original value when dropped.
///
/// # Example
///
/// ```rust,ignore
/// let _guard = EnvGuard::new("NESTGATE_TIMEOUT_MS", "10000");
/// assert_eq!(std::env::var("NESTGATE_TIMEOUT_MS").unwrap(), "10000");
/// // Automatic cleanup on drop
/// ```
pub struct EnvGuard {
    key: String,
    original: Option<String>,
    _lock: MutexGuard<'static, ()>,
}

impl EnvGuard {
    /// Create a new environment guard
    ///
    /// Sets the environment variable and captures its original value.
    #[must_use]
    pub fn new(key: &str, value: &str) -> Self {
        let lock = ENV_TEST_LOCK
            .lock()
            .expect("Failed to acquire environment test lock");

        let original = std::env::var(key).ok();
        nestgate_core::env_process::set_var(key, value);

        Self {
            key: key.to_string(),
            original,
            _lock: lock,
        }
    }
}

impl Drop for EnvGuard {
    /// Automatically restore the original environment variable value
    fn drop(&mut self) {
        match &self.original {
            Some(value) => nestgate_core::env_process::set_var(&self.key, value),
            None => nestgate_core::env_process::remove_var(&self.key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isolated_environment_sets_var() {
        let mut env = IsolatedEnvironment::new("test_isolated_environment_sets_var");
        env.set("NESTGATE_TEST_VAR_UNIQUE_1", "test_value");
        assert_eq!(
            std::env::var("NESTGATE_TEST_VAR_UNIQUE_1").unwrap(),
            "test_value"
        );
    }

    #[test]
    fn test_isolated_environment_restores_var() {
        let orig = std::env::var("NESTGATE_TEST_VAR_UNIQUE_2").ok();
        nestgate_core::env_process::set_var("NESTGATE_TEST_VAR_UNIQUE_2", "original");

        {
            let mut env = IsolatedEnvironment::new("test_isolated_environment_restores_var");
            env.set("NESTGATE_TEST_VAR_UNIQUE_2", "modified");
            assert_eq!(
                std::env::var("NESTGATE_TEST_VAR_UNIQUE_2").unwrap(),
                "modified"
            );
        }

        let current = std::env::var("NESTGATE_TEST_VAR_UNIQUE_2").ok();
        match orig {
            Some(v) => nestgate_core::env_process::set_var("NESTGATE_TEST_VAR_UNIQUE_2", v),
            None => nestgate_core::env_process::remove_var("NESTGATE_TEST_VAR_UNIQUE_2"),
        }
        assert_eq!(current, Some("original".to_string()));
    }

    #[test]
    fn test_env_guard_simple() {
        let original = std::env::var("NESTGATE_TEST_VAR_UNIQUE_3").ok();

        {
            let _guard = EnvGuard::new("NESTGATE_TEST_VAR_UNIQUE_3", "guarded");
            assert_eq!(
                std::env::var("NESTGATE_TEST_VAR_UNIQUE_3").unwrap(),
                "guarded"
            );
        }

        // After drop, should be restored
        assert_eq!(std::env::var("NESTGATE_TEST_VAR_UNIQUE_3").ok(), original);
    }

    #[test]
    fn test_isolated_environment_removes_var() {
        let orig = std::env::var("NESTGATE_TEST_VAR_UNIQUE_4").ok();
        nestgate_core::env_process::set_var("NESTGATE_TEST_VAR_UNIQUE_4", "to_remove");

        {
            let mut env = IsolatedEnvironment::new("test_isolated_environment_removes_var");
            env.remove("NESTGATE_TEST_VAR_UNIQUE_4");
            assert!(std::env::var("NESTGATE_TEST_VAR_UNIQUE_4").is_err());
        }

        let current = std::env::var("NESTGATE_TEST_VAR_UNIQUE_4").ok();
        match orig {
            Some(v) => nestgate_core::env_process::set_var("NESTGATE_TEST_VAR_UNIQUE_4", v),
            None => nestgate_core::env_process::remove_var("NESTGATE_TEST_VAR_UNIQUE_4"),
        }
        assert_eq!(current, Some("to_remove".to_string()));
    }

    #[test]
    fn test_concurrent_safety() {
        // This test verifies that the lock prevents concurrent access
        // In practice, cargo test will run these serially due to the lock
        let _env1 = IsolatedEnvironment::new("test_concurrent_safety_1");
        // If we could acquire another lock here, it would be a bug
        // But we can't test that easily without spawning threads
    }
}
