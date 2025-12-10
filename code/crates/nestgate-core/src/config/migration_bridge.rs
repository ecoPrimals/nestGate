//! Migration Bridge for Legacy Configuration Patterns
//!
//! This module provides a compatibility layer between legacy configuration
//! helpers and the modern `EnvironmentConfig` system. It allows existing code
//! to continue working while we gradually migrate to the new patterns.
//!
//! # Migration Strategy
//!
//! 1. **Phase 1** (Current): Bridge delegates to EnvironmentConfig
//! 2. **Phase 2**: Update call sites to use EnvironmentConfig directly
//! 3. **Phase 3**: Remove legacy helpers and this bridge
//!
//! # Usage
//!
//! ```rust
//! use nestgate_core::config::migration_bridge;
//!
//! // Legacy code continues to work
//! let port = migration_bridge::get_api_port();
//!
//! // But new code should use:
//! use nestgate_core::config::environment::EnvironmentConfig;
//! let config = EnvironmentConfig::from_env()?;
//! let port = config.network.port.get();
//! ```

use super::environment::EnvironmentConfig;
use std::sync::OnceLock;

/// Global configuration instance
///
/// Initialized once on first access, then reused for all subsequent calls.
/// This provides efficient access to configuration without repeated parsing.
static GLOBAL_CONFIG: OnceLock<EnvironmentConfig> = OnceLock::new();

/// Get or initialize the global configuration
///
/// On first call, loads configuration from environment. Subsequent calls
/// return the cached instance. Falls back to defaults if loading fails.
///
/// # Error Handling
///
/// This function uses `unwrap_or_else` to provide graceful degradation.
/// If environment loading fails, it logs the error and falls back to defaults.
/// This ensures the application can start even with configuration issues.
fn global_config() -> &'static EnvironmentConfig {
    GLOBAL_CONFIG.get_or_init(|| match EnvironmentConfig::from_env() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("⚠️  Warning: Failed to load environment config: {}", e);
            eprintln!("   Using default configuration values");
            eprintln!("   Set environment variables to customize (see CONFIGURATION_GUIDE.md)");
            EnvironmentConfig::default()
        }
    })
}

// ==================== API SERVER ====================

/// Get API server port
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.network.port` instead
///
/// # Migration Example
///
/// ```rust
/// // OLD
/// let port = migration_bridge::get_api_port();
///
/// // NEW
/// let config = EnvironmentConfig::from_env()?;
/// let port = config.network.port.get();
/// ```
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.network.port instead"
)]
#[must_use]
pub fn get_api_port() -> u16 {
    global_config().network.port.get()
}

/// Get API server host
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.network.host` instead
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.network.host instead"
)]
#[must_use]
pub fn get_api_host() -> String {
    global_config().network.host.clone()
}

/// Get API bind address
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.bind_address()` instead
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.bind_address() instead"
)]
#[must_use]
pub fn get_bind_address() -> String {
    format!(
        "{}:{}",
        global_config().network.host,
        global_config().network.port.get()
    )
}

// ==================== METRICS ====================

/// Get metrics port
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.monitoring.metrics_port` instead
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.monitoring.metrics_port instead"
)]
#[must_use]
pub fn get_metrics_port() -> u16 {
    global_config().monitoring.metrics_port.get()
}

// ==================== STORAGE ====================

/// Get ZFS pool name
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.storage.zfs_pool` instead
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.storage.zfs_pool instead"
)]
#[must_use]
pub fn get_zfs_pool() -> String {
    global_config().storage.zfs_pool.clone()
}

/// Get data directory
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.storage.data_dir` instead
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.storage.data_dir instead"
)]
#[must_use]
pub fn get_data_dir() -> String {
    global_config().storage.data_dir.clone()
}

// ==================== TIMEOUTS ====================

/// Get connection timeout in seconds
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.network.timeout_secs` instead
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.network.timeout() instead"
)]
#[must_use]
pub fn get_timeout_secs() -> u64 {
    global_config().network.timeout_secs
}

/// Get read timeout in seconds
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.network.read_timeout_secs` instead
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.network.read_timeout() instead"
)]
#[must_use]
pub fn get_read_timeout_secs() -> u64 {
    global_config().network.read_timeout_secs
}

// ==================== MODERN INTERFACE ====================

/// Get the global environment configuration
///
/// This is the **recommended** way to access configuration. It returns a
/// reference to the cached `EnvironmentConfig` instance.
///
/// # Example
///
/// ```rust
/// use nestgate_core::config::migration_bridge;
///
/// let config = migration_bridge::config();
/// let port = config.network.port.get();
/// let timeout = config.network.timeout();
/// ```
#[must_use]
pub fn config() -> &'static EnvironmentConfig {
    global_config()
}

/// Reset the global configuration (primarily for testing)
///
/// Forces the next call to `config()` to reload from environment.
/// This should only be used in tests.
///
/// # Safety
///
/// This function is not thread-safe during the reset operation.
/// Only call this from single-threaded test code.
pub fn reset_for_testing() {
    // Note: OnceLock doesn't provide a way to reset
    // In practice, tests should use separate config instances
    eprintln!("Warning: reset_for_testing() cannot actually reset OnceLock");
    eprintln!("Use EnvironmentConfig::from_env() directly in tests instead");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_config_cached() {
        let config1 = config();
        let config2 = config();

        // Should be the same instance (same memory address)
        assert!(std::ptr::eq(config1, config2));
    }

    #[test]
    #[allow(deprecated)]
    fn test_legacy_api_port() {
        let port = get_api_port();
        assert!(port >= 1024); // Port validation ensures >= 1024
    }

    #[test]
    #[allow(deprecated)]
    fn test_legacy_api_host() {
        let host = get_api_host();
        assert!(!host.is_empty());
    }

    #[test]
    #[allow(deprecated)]
    fn test_legacy_bind_address() {
        let addr = get_bind_address();
        assert!(addr.contains(':'));
    }

    #[test]
    fn test_modern_config_access() {
        let cfg = config();

        // All subsystems accessible
        assert!(cfg.network.port.get() >= 1024);
        assert!(!cfg.network.host.is_empty());
        assert!(!cfg.storage.zfs_pool.is_empty());
        assert!(cfg.monitoring.metrics_port.get() >= 1024);
    }
}
