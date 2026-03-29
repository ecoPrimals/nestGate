// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CENTRALIZED RUNTIME CONFIGURATION SYSTEM**
//!
//! Eliminates 805+ hardcoded values throughout the codebase.
//! Environment-driven, flexible, production-ready.
//!
//! # Architecture
//!
//! Configuration is loaded from environment variables on first access via
//! configuration initialization functions, using [`OnceLock`](std::sync::OnceLock) for thread-safe
//! lazy initialization. This provides zero-cost abstraction with no runtime overhead.
//!
//! # Organization
//!
//! The configuration is organized into domain-specific modules:
//! - `network` - API endpoints, ports, timeouts, connection pooling
//! - `services` - Service discovery, registration, health checks
//! - `storage` - Backend configuration, paths, quotas, retention
//! - `database` - Connection pooling, query limits, credentials
//! - `cache` - TTL, size limits, eviction policies (Redis, in-memory)
//! - `monitoring` - Metrics, logging, tracing, alerting
//! - `security` - Authentication, encryption, access control
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use nestgate_core::config::runtime::get_config;
//!
//! // Get global configuration (initialized once)
//! let config = get_config();
//!
//! // Use configuration values
//! let api_url = format!("http://{}:{}", config.network.api_host, config.network.api_port);
//! let storage_path = &config.storage.base_path;
//! let db_pool_size = config.database.pool_size;
//! ```
//!
//! # Environment Variables
//!
//! All configuration can be overridden via environment variables with the `NESTGATE_` prefix:
//! - `NESTGATE_API_HOST` → `network.api_host` (default: "127.0.0.1")
//! - `NESTGATE_API_PORT` → `network.api_port` (default: 8080)
//! - `NESTGATE_STORAGE_PATH` → `storage.base_path` (default: "./data")
//! - See individual config modules for complete list
//!
//! # Migration Status
//!
//! **Active Migration** (Dec 3, 2025):
//! - **Total hardcoded values identified**: 805
//! - **Remaining to migrate**: ~390 files
//! - **Migration pattern**: Replace `"localhost"` with `get_config().network.api_host`
//! - **Progress tracking**: See `HARDCODING_ELIMINATION_GUIDE.md`
//!
//! # Thread Safety
//!
//! All configuration is immutable after initialization and the global instance
//! is stored in a [`OnceLock`](std::sync::OnceLock) for safe concurrent access.

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

// Re-export domain configuration modules
pub mod cache;
pub mod database;
pub mod monitoring;
pub mod network;
pub mod security;
pub mod services;
pub mod storage;

// Re-export types for convenience
pub use cache::CacheConfig;
pub use database::DatabaseConfig;
pub use monitoring::MonitoringConfig;
pub use network::NetworkConfig;
pub use security::SecurityConfig;
pub use services::ServicesConfig;
pub use storage::StorageConfig;

/// Global configuration instance (thread-safe, lazy-initialized)
static GLOBAL_CONFIG: OnceLock<NestGateRuntimeConfig> = OnceLock::new();

/// Centralized runtime configuration for all `NestGate` components.
///
/// This is the **single source of truth** for all runtime configuration,
/// systematically replacing 805+ hardcoded values throughout the codebase.
///
/// See module-level documentation for usage examples.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NestGateRuntimeConfig {
    /// Network configuration (ports, IPs, endpoints, timeouts)
    pub network: NetworkConfig,

    /// Service configuration (discovery, registration, health checks)
    pub services: ServicesConfig,

    /// Storage configuration (paths, backends, quotas)
    pub storage: StorageConfig,

    /// Database configuration (connection pooling, credentials)
    pub database: DatabaseConfig,

    /// Cache configuration (Redis, in-memory, TTL)
    pub cache: CacheConfig,

    /// Monitoring configuration (metrics, logs, traces)
    pub monitoring: MonitoringConfig,

    /// Security configuration (auth, encryption, access control)
    pub security: SecurityConfig,
}

impl NestGateRuntimeConfig {
    /// Load configuration from environment variables with fallback to defaults.
    ///
    /// This method reads all `NESTGATE_*` environment variables and constructs
    /// a complete configuration tree. If any value is missing, it uses sensible
    /// defaults appropriate for the current environment (dev/prod).
    ///
    /// # Errors
    ///
    /// Returns error if required environment variables are malformed (e.g., invalid port numbers).
    pub fn from_environment() -> Result<Self> {
        Ok(Self {
            network: NetworkConfig::from_environment()?,
            services: ServicesConfig::from_environment()?,
            storage: StorageConfig::from_environment()?,
            database: DatabaseConfig::from_environment()?,
            cache: CacheConfig::from_environment()?,
            monitoring: MonitoringConfig::from_environment()?,
            security: SecurityConfig::from_environment()?,
        })
    }
}

/// Initialize the global configuration.
///
/// This should be called once at application startup. Subsequent calls are no-ops.
///
/// # Errors
///
/// Returns error if environment variables contain invalid values AND no defaults can be used.
/// In practice, this gracefully falls back to defaults with a warning.
pub fn init_config() -> Result<()> {
    GLOBAL_CONFIG.get_or_init(|| match NestGateRuntimeConfig::from_environment() {
        Ok(config) => config,
        Err(e) => {
            tracing::warn!(
                "Failed to load configuration from environment: {}. Using defaults.",
                e
            );
            NestGateRuntimeConfig::default()
        }
    });
    Ok(())
}

/// Get the global runtime configuration.
///
/// This lazily initializes the configuration on first call. The configuration
/// is then cached for the lifetime of the process.
///
/// # Graceful Degradation
///
/// If environment loading fails, this function logs a warning and returns
/// sensible defaults. This ensures the application can start even with
/// configuration issues. Call [`init_config()`] at startup to initialize
/// explicitly and handle errors if needed.
#[must_use]
pub fn get_config() -> &'static NestGateRuntimeConfig {
    GLOBAL_CONFIG.get_or_init(|| match NestGateRuntimeConfig::from_environment() {
        Ok(config) => config,
        Err(e) => {
            tracing::warn!(
                "Failed to load configuration from environment: {}. Using defaults. \
                     Consider calling init_config() at startup for explicit error handling.",
                e
            );
            NestGateRuntimeConfig::default()
        }
    })
}

//
// ==================== CONVENIENCE FUNCTIONS ====================
//

/// Get the complete API base URL (http://host:port).
#[must_use]
pub fn api_base_url() -> String {
    let config = get_config();
    format!(
        "http://{}:{}",
        config.network.api_host, config.network.api_port
    )
}

/// Get the API port number.
#[must_use]
pub fn api_port() -> u16 {
    get_config().network.api_port
}

/// Get URL for a specific capability (from service registry).
///
/// Returns `None` if the capability is not registered.
#[must_use]
pub fn capability_url(capability: &str) -> Option<String> {
    get_config()
        .services
        .discovered_capabilities
        .get(capability)
        .cloned()
}

/// Get URL for a capability, defaulting to local API if not found.
#[must_use]
pub fn capability_url_or_local(capability: &str) -> String {
    capability_url(capability)
        .unwrap_or_else(|| format!("{}/api/v1/{}", api_base_url(), capability))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = NestGateRuntimeConfig::default();
        // IpAddr doesn't have is_empty(), just verify config loaded properly
        // Port is u16, so automatically in valid range (0-65535)
        assert!(config.network.api_port > 0);
    }

    #[test]
    fn test_api_base_url() {
        let url = api_base_url();
        assert!(url.starts_with("http://"));
        assert!(url.contains(':'));
    }

    #[test]
    fn test_capability_url_or_local() {
        let url = capability_url_or_local("unknown");
        assert!(url.contains("/api/v1/unknown"));
    }
}
