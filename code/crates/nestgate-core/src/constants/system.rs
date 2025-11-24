//! **SYSTEM CONSTANTS**
//!
//! System-level constants and default values with environment variable support.

// Import the configuration module for concurrent-safe access
use super::system_config::SystemConfig;

/// Default instance name for `NestGate` services
pub const DEFAULT_INSTANCE_NAME: &str = "nestgate-default";

/// Default service name
pub const DEFAULT_SERVICE_NAME: &str = "nestgate";

/// Get timeout in milliseconds from environment or default
#[must_use]
pub fn timeout_ms() -> u64 {
    SystemConfig::from_env().timeout_ms()
}

/// Get max connections from environment or default
#[must_use]
pub fn max_connections() -> usize {
    SystemConfig::from_env().max_connections()
}

/// Get buffer size from environment or default
#[must_use]
pub fn buffer_size() -> usize {
    SystemConfig::from_env().buffer_size()
}

/// Get default retry attempts from environment or default
#[must_use]
pub fn default_retry_attempts() -> u32 {
    SystemConfig::from_env().retry_attempts()
}

/// Get health check interval from environment or default
#[must_use]
pub fn health_check_interval() -> u64 {
    SystemConfig::from_env().health_check_interval()
}

/// Get API port from environment or default
#[must_use]
pub fn api_port() -> u16 {
    SystemConfig::from_env().api_port()
}

/// Get bind host from environment or default
#[must_use]
pub fn bind_host() -> String {
    SystemConfig::from_env().bind_host()
}

/// Get API URL from environment or default
#[must_use]
pub fn api_url() -> String {
    SystemConfig::from_env().api_url()
}

/// Legacy constants for backward compatibility
pub const DEFAULT_TIMEOUT_MS: u64 = 5000;
pub const MAX_CONNECTIONS: usize = 1000;
pub const BUFFER_SIZE: usize = 8192;
pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;
pub const DEFAULT_HEALTH_CHECK_INTERVAL: u64 = 30;
pub const DEFAULT_BIND_HOST: &str = "127.0.0.1";

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    // Global mutex to serialize environment variable tests
    static ENV_TEST_LOCK: Mutex<()> = Mutex::new(());

    /// Helper to set and restore environment variables for testing
    struct EnvGuard {
        key: String,
        original: Option<String>,
    }

    impl EnvGuard {
        fn new(key: &str, value: &str) -> Self {
            let original = env::var(key).ok();
            env::set_var(key, value);
            Self {
                key: key.to_string(),
                original,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.original {
                Some(value) => env::set_var(&self.key, value),
                None => env::remove_var(&self.key),
            }
        }
    }

    #[test]
    fn test_default_constants() {
        assert_eq!(DEFAULT_INSTANCE_NAME, "nestgate-default");
        assert_eq!(DEFAULT_SERVICE_NAME, "nestgate");
        assert_eq!(DEFAULT_TIMEOUT_MS, 5000);
        assert_eq!(MAX_CONNECTIONS, 1000);
        assert_eq!(BUFFER_SIZE, 8192);
        assert_eq!(DEFAULT_RETRY_ATTEMPTS, 3);
        assert_eq!(DEFAULT_HEALTH_CHECK_INTERVAL, 30);
    }

    #[test]
    fn test_timeout_ms_default() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        // Save original value, remove var, test default, then restore
        let original = env::var("NESTGATE_TIMEOUT_MS").ok();
        env::remove_var("NESTGATE_TIMEOUT_MS");
        assert_eq!(timeout_ms(), 5000);
        // Restore original value if it existed
        if let Some(value) = original {
            env::set_var("NESTGATE_TIMEOUT_MS", value);
        }
    }

    #[test]
    fn test_timeout_ms_from_env() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        let _guard = EnvGuard::new("NESTGATE_TIMEOUT_MS", "10000");
        assert_eq!(timeout_ms(), 10000);
    }

    #[test]
    fn test_timeout_ms_invalid_env() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        let _guard = EnvGuard::new("NESTGATE_TIMEOUT_MS", "invalid");
        assert_eq!(timeout_ms(), 5000);
    }

    #[test]
    fn test_max_connections_default() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        // Save original value, remove var, test default, then restore
        let original = env::var("NESTGATE_MAX_CONNECTIONS").ok();
        env::remove_var("NESTGATE_MAX_CONNECTIONS");
        assert_eq!(max_connections(), 1000);
        // Restore original value if it existed
        if let Some(value) = original {
            env::set_var("NESTGATE_MAX_CONNECTIONS", value);
        }
    }

    #[test]
    fn test_max_connections_from_env() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        let _guard = EnvGuard::new("NESTGATE_MAX_CONNECTIONS", "2000");
        assert_eq!(max_connections(), 2000);
    }

    #[test]
    fn test_max_connections_invalid_env() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        let _guard = EnvGuard::new("NESTGATE_MAX_CONNECTIONS", "not_a_number");
        assert_eq!(max_connections(), 1000);
    }

    #[test]
    fn test_buffer_size_default() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        // Save original value, remove var, test default, then restore
        let original = env::var("NESTGATE_BUFFER_SIZE").ok();
        env::remove_var("NESTGATE_BUFFER_SIZE");
        assert_eq!(buffer_size(), 8192);
        // Restore original value if it existed
        if let Some(value) = original {
            env::set_var("NESTGATE_BUFFER_SIZE", value);
        }
    }

    #[test]
    fn test_buffer_size_from_env() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        // This test reads from the environment variable set by the guard
        let _guard = EnvGuard::new("NESTGATE_BUFFER_SIZE", "16384");
        assert_eq!(buffer_size(), 16384);
    }

    #[test]
    fn test_buffer_size_invalid_env() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        let _guard = EnvGuard::new("NESTGATE_BUFFER_SIZE", "invalid");
        assert_eq!(buffer_size(), 8192);
    }

    #[test]
    fn test_default_retry_attempts_default() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        // Save original value, remove var, test default, then restore
        let original = env::var("NESTGATE_RETRY_ATTEMPTS").ok();
        env::remove_var("NESTGATE_RETRY_ATTEMPTS");
        assert_eq!(default_retry_attempts(), 3);
        // Restore original value if it existed
        if let Some(value) = original {
            env::set_var("NESTGATE_RETRY_ATTEMPTS", value);
        }
    }

    #[test]
    fn test_default_retry_attempts_from_env() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        let _guard = EnvGuard::new("NESTGATE_RETRY_ATTEMPTS", "5");
        assert_eq!(default_retry_attempts(), 5);
    }

    #[test]
    fn test_default_retry_attempts_invalid_env() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        let _guard = EnvGuard::new("NESTGATE_RETRY_ATTEMPTS", "not_a_number");
        assert_eq!(default_retry_attempts(), 3);
    }

    #[test]
    fn test_health_check_interval_default() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        let _guard = EnvGuard::new("NESTGATE_HEALTH_CHECK_INTERVAL", "");
        env::remove_var("NESTGATE_HEALTH_CHECK_INTERVAL");
        assert_eq!(health_check_interval(), 30);
    }

    #[test]
    fn test_health_check_interval_from_env() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        // Set the test value directly
        let _test_guard = EnvGuard::new("NESTGATE_HEALTH_CHECK_INTERVAL", "60");

        // Verify the environment variable is actually set
        assert_eq!(
            env::var("NESTGATE_HEALTH_CHECK_INTERVAL")
                .expect("Failed to read environment variable"),
            "60"
        );

        // Test the function
        let result = health_check_interval();
        assert_eq!(result, 60, "Expected 60, got {result}");
    }

    #[test]
    fn test_health_check_interval_invalid_env() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        let _guard = EnvGuard::new("NESTGATE_HEALTH_CHECK_INTERVAL", "invalid");
        assert_eq!(health_check_interval(), 30);
    }

    #[test]
    fn test_edge_cases() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        // Test zero values
        let _guard1 = EnvGuard::new("NESTGATE_TIMEOUT_MS", "0");
        assert_eq!(timeout_ms(), 0);

        // Test negative values (should parse as default due to type constraints)
        let _guard2 = EnvGuard::new("NESTGATE_MAX_CONNECTIONS", "-1");
        assert_eq!(max_connections(), 1000);

        // Test very large values
        let _guard3 = EnvGuard::new("NESTGATE_BUFFER_SIZE", "1048576");
        assert_eq!(buffer_size(), 1048576);
    }

    #[test]
    fn test_multiple_env_vars_simultaneously() {
        let _lock = ENV_TEST_LOCK.lock().expect("Failed to acquire lock");
        let _guard1 = EnvGuard::new("NESTGATE_TIMEOUT_MS", "2000");
        let _guard2 = EnvGuard::new("NESTGATE_MAX_CONNECTIONS", "500");
        let _guard3 = EnvGuard::new("NESTGATE_BUFFER_SIZE", "4096");

        assert_eq!(timeout_ms(), 2000);
        assert_eq!(max_connections(), 500);
        assert_eq!(buffer_size(), 4096);
    }
}
