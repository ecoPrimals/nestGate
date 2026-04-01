// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **NESTGATE CONSTANTS MODULE**
//!
//! Centralized constants to eliminate hardcoding throughout the codebase.
//! This module provides environment-driven configuration for all system constants
//! including ports, addresses, timeouts, and system limits.
//!
//! # Architecture
//!
//! Constants are organized into several categories:
//!
//! - **Network**: IP addresses, hostnames, bind addresses
//! - **Ports**: Service ports (API, metrics, discovery, databases)
//! - **Timeouts**: Connection, request, idle, keepalive timeouts
//! - **System**: Resource limits, buffer sizes, thread counts
//! - **Sovereignty**: Compliance and governance settings
//!
//! # Migration Strategy
//!
//! This module is part of an ongoing migration from hardcoded values to
//! environment-driven configuration. See `HARDCODING_MIGRATION_TRACKER.md`
//! for the complete migration plan.
//!
//! # Usage
//!
//! ```rust,ignore
//! use nestgate_core::constants::{PortConfig, NetworkDefaultsConfig};
//!
//! // Load port configuration from environment
//! let ports = PortConfig::from_env();
//! let api_port = ports.get_api_port();
//! let metrics_port = ports.get_metrics_port();
//!
//! // Load network configuration
//! let network = NetworkDefaultsConfig::from_env();
//! let bind_addr = network.get_bind_address();
//! ```
//!
//! # Environment Variables
//!
//! All configuration can be customized via environment variables with the
//! `NESTGATE_` prefix:
//!
//! - `NESTGATE_API_PORT`: API server port (default: 8080)
//! - `NESTGATE_METRICS_PORT`: Metrics port (default: 9090)
//! - `NESTGATE_HOST`: Bind address (default: 127.0.0.1)
//! - `NESTGATE_CONNECTION_TIMEOUT`: Connection timeout in seconds
//!
//! See individual module documentation for complete variable lists.

pub mod canonical;
pub mod canonical_defaults; // Canonical constants - single source of truth
// magic_numbers_replacement removed - was migration artifact with zero usages
pub mod capability_port_discovery;
pub mod network;
pub mod network_defaults;
/// Network configuration with environment-driven defaults.
pub mod network_defaults_config;
pub mod network_hardcoded;

/// Environment-driven network configuration (modern, preferred)
///
/// **NEW**: Use this instead of hardcoded constants!
///
/// Provides environment-variable driven configuration with sensible defaults.
/// Integrates with capability-based discovery for true runtime flexibility.
///
/// # Migration
///
/// **Before** (hardcoded):
/// ```rust,ignore
/// use nestgate_core::constants::network_hardcoded::ports::API_DEFAULT;
/// let port = API_DEFAULT; // HARDCODED!
/// ```
///
/// **After** (environment-driven):
/// ```rust,ignore
/// use nestgate_core::constants::network_environment::api_port;
/// let port = api_port(); // Checks NESTGATE_API_PORT, defaults to 8080
/// ```
pub mod network_environment; // NEW: Centralized network constants (Nov 6, 2025)
pub mod port_defaults;
/// Port configuration with environment-driven defaults.
pub mod port_defaults_config;
pub mod ports; // ✅ NEW (Nov 28, 2025): Centralized port configuration with env var support
pub mod shared;
pub mod sovereignty_helpers;
pub mod sovereignty_helpers_config;
pub mod system;
pub mod system_config;
pub mod testing;
pub mod timeouts;
/// Timeout configuration with environment-driven defaults.
pub mod timeouts_config;

/// Centralized constants to eliminate hardcoding (addresses, ports, limits)
pub mod hardcoding;

/// **NEW: Consolidated constants module (Nov 13, 2025)**
/// Single source of truth for ALL hardcoded values (888+ instances)
/// - 447 hardcoded IPs → Environment-driven
/// - 441 hardcoded ports → Environment-driven
/// - Thread-safe, zero-copy, production-ready
pub mod consolidated;

// Re-export commonly used constants for backwards compatibility
// Port constants - use port_defaults as single source of truth
pub use port_defaults::{
    DEFAULT_ADMIN_PORT, DEFAULT_API_PORT, DEFAULT_GRAFANA_PORT, DEFAULT_HEALTH_PORT,
    DEFAULT_METRICS_PORT, DEFAULT_POSTGRES_PORT, DEFAULT_REDIS_PORT, get_admin_port, get_api_port,
    get_dev_port, get_grafana_port, get_health_port, get_metrics_port, get_postgres_port,
    get_prometheus_port, get_redis_port,
};
pub use port_defaults_config::{PortConfig, SharedPortConfig};

// System configuration - thread-safe config for system constants
pub use system_config::{SharedSystemConfig, SystemConfig};

// Sovereignty helpers configuration - thread-safe config for sovereignty-compliant helpers
pub use sovereignty_helpers_config::{SharedSovereigntyHelpersConfig, SovereigntyHelpersConfig};

// Network address constants - environment-aware (legacy helpers remain for compatibility)
#[allow(deprecated)]
pub use network_defaults::{
    BIND_ALL_IPV4, BIND_ALL_IPV6, LOCALHOST_IPV4, LOCALHOST_IPV6, LOCALHOST_NAME, get_api_host,
    get_bind_address, get_db_host, get_redis_host, is_development, is_production,
};
pub use network_defaults_config::{NetworkDefaultsConfig, SharedNetworkDefaultsConfig};

// Network constants (non-port)
pub use network::{
    DEFAULT_BIND_ADDRESS,
    DEFAULT_TIMEOUT_MS, // Re-exported from canonical::timeouts (30,000 ms)
    LOCALHOST,
    NETWORK_BUFFER_SIZE, // Explicit network buffer size (8192 bytes)
};
pub use shared::DEFAULT_MAX_CONNECTIONS;

// Timeout and retry constants - use canonical as single source of truth:
pub use self::canonical::timeouts::{
    DEFAULT_RETRY_ATTEMPTS, DEFAULT_RETRY_DELAY_MS, DEFAULT_TIMEOUT_SECS,
};

// Timeout configuration - thread-safe config for timeout constants
pub use timeouts::{
    DEFAULT_CONNECTION_TIMEOUT_SECS, DEFAULT_HEALTH_CHECK_INTERVAL_SECS, DEFAULT_IDLE_TIMEOUT_SECS,
    DEFAULT_KEEPALIVE_SECS, DEFAULT_REQUEST_TIMEOUT_SECS, connection_timeout, idle_timeout,
    keepalive_interval, request_timeout, retry_delay,
};
pub use timeouts_config::{SharedTimeoutsConfig, TimeoutsConfig};

// For buffer sizes, use domain-specific constants from canonical_defaults:
// - network::NETWORK_BUFFER_SIZE (8192 bytes) for network I/O
// - canonical_defaults::performance::DEFAULT_BUFFER_SIZE (4096 bytes) for disk/general I/O
pub use testing::*;

// Single source of truth modules:
// - canonical:: - All consolidated constants (PREFERRED)
// - port_defaults:: - Service ports with environment variable support
// - timeouts:: - Timeout values with environment variable support
// - network_defaults:: - IPv4/IPv6 defaults with environment variable support
