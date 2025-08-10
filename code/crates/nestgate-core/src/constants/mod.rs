//! Constants Module - Consolidated Domain Constants System
//!
//! This module provides organized, domain-specific constants that eliminate
//! duplication across the codebase and provide a single source of truth.
//!
//! **NEW UNIFIED SYSTEM**: Domain-specific constant hierarchies
//! **LEGACY MODULES**: Configuration-aware constants (deprecated)

// ==================== NEW DOMAIN CONSTANTS SYSTEM ====================

/// **THE** Domain-specific constants - consolidated from across the codebase
/// This is the primary constants system that should be used going forward
pub mod domain_constants;

// Re-export the new domain constants for easy access
pub use domain_constants::{
    // API constants
    api,
    // Network constants
    network,
    // Performance constants
    performance,
    // Security constants
    security,

    // Service constants
    services,

    // Storage constants
    storage,
    // Timeout constants
    timeouts,

    // Utility functions
    utils,
    CURRENT_API_VERSION,

    DEFAULT_API_PORT,
    DEFAULT_BUFFER_SIZE,

    DEFAULT_HEALTH_PORT,

    PROTOCOL_NFS,
    PROTOCOL_SMB,
    PROTOCOL_ZFS,

    TIER_COLD,
    TIER_HOT,
    TIER_WARM,
};

// Re-export test constants for test modules
#[cfg(test)]
pub use domain_constants::test;

// ==================== LEGACY CONSTANTS MODULES (DEPRECATED) ====================

// ==================== DEPRECATED MODULES ELIMINATED ====================
// All deprecated constant modules have been removed and consolidated into domain_constants:
// - addresses.rs → domain_constants::network::addresses
// - limits.rs → domain_constants::network::limits
// - port_defaults.rs → domain_constants::network::ports
// - test_defaults.rs → domain_constants::timeouts
// - time.rs → domain_constants::timeouts
// - timeout_defaults.rs → domain_constants::timeouts
// - network.rs → domain_constants::network::ports
// - strings.rs → domain_constants::api::messages
// - test.rs → domain_constants::timeouts
// - timeouts.rs → domain_constants::timeouts

// Re-export unified domain constants (eliminates fragmented imports)
pub use domain_constants::api::messages::*;
pub use domain_constants::network::{addresses::*, limits::*, ports::*};
pub use domain_constants::timeouts::*;

// REMOVED: References to deleted test_defaults and timeouts modules
// Use domain_constants::timeouts instead for all timeout constants

use crate::config::canonical::CanonicalConfig;
use crate::error::{NestGateError, Result};
use std::sync::OnceLock;

/// Global configuration instance
static CONFIG: OnceLock<CanonicalConfig> = OnceLock::new();

/// Initialize the global configuration
pub fn init_config(config: CanonicalConfig) -> Result<()> {
    CONFIG.set(config).map_err(|_| NestGateError::Validation {
        field: "global_config".to_string(),
        message: "Configuration already initialized".to_string(),
        current_value: None,
        expected: Some("uninitialized config".to_string()),
        user_error: false,
    })?;
    Ok(())
}

/// Get the global configuration
pub fn get_config() -> Result<&'static CanonicalConfig> {
    CONFIG.get().ok_or_else(|| NestGateError::Validation {
        field: "global_config".to_string(),
        message: "Configuration not initialized".to_string(),
        current_value: None,
        expected: Some("initialized config".to_string()),
        user_error: false,
    })
}

/// Configuration-aware constants
pub mod configurable {
    use super::*;

    /// Get the API server port from configuration
    pub fn api_port() -> u16 {
        get_config().map(|c| c.network.api.port).unwrap_or(8080)
    }

    /// Get the API server host from configuration
    pub fn api_host() -> std::net::IpAddr {
        get_config()
            .map(|c| c.network.api.host)
            .unwrap_or([127, 0, 0, 1].into())
    }

    /// Get the data directory from configuration
    pub fn data_dir() -> std::path::PathBuf {
        get_config()
            .map(|c| c.system.data_dir.clone())
            .unwrap_or_else(|_| "./data".into())
    }

    /// Get the log level from configuration
    pub fn log_level() -> String {
        get_config()
            .map(|c| c.system.log_level.clone())
            .unwrap_or_else(|_| "info".to_string())
    }

    /// Get whether development mode is enabled
    pub fn dev_mode() -> bool {
        get_config().map(|c| c.system.dev_mode).unwrap_or(true)
    }

    /// Get the cache size from configuration
    pub fn cache_size_mb() -> usize {
        get_config()
            .map(|c| (c.performance.memory.pool_size / (1024 * 1024)) as usize)
            .unwrap_or(512)
    }

    /// Get the number of API workers from configuration
    pub fn api_workers() -> usize {
        get_config()
            .map(|c| c.performance.threads.worker_threads.unwrap_or(4))
            .unwrap_or(4)
    }

    /// Get the maximum number of connections from configuration
    pub fn max_connections() -> usize {
        get_config()
            .map(|c| c.network.api.max_connections)
            .unwrap_or(1000)
    }

    /// Get whether metrics are enabled from configuration
    pub fn metrics_enabled() -> bool {
        get_config()
            .map(|c| c.monitoring.metrics.enabled)
            .unwrap_or(true)
    }

    /// Get the metrics endpoint from configuration
    pub fn metrics_endpoint() -> String {
        get_config()
            .map(|c| c.monitoring.metrics.endpoint.clone())
            .unwrap_or_else(|_| "/metrics".to_string())
    }

    /// Get whether health checks are enabled from configuration
    pub fn health_enabled() -> bool {
        get_config()
            .map(|c| c.monitoring.metrics.enabled)
            .unwrap_or(true)
    }

    /// Get the health check endpoint from configuration
    pub fn health_endpoint() -> String {
        get_config()
            .map(|_c| "/health".to_string()) // Fixed endpoint
            .unwrap_or_else(|_| "/health".to_string())
    }
}

/// Backward compatibility - these functions now use configuration
/// but maintain the same API for existing code
pub mod compat {
    use super::*;

    /// Get default API port (now from configuration)
    pub fn default_api_port() -> u16 {
        configurable::api_port()
    }

    /// Get default host (now from configuration)  
    pub fn default_host() -> std::net::IpAddr {
        configurable::api_host()
    }

    /// Get default cache size (now from configuration)
    pub fn default_cache_size() -> usize {
        configurable::cache_size_mb()
    }
}
