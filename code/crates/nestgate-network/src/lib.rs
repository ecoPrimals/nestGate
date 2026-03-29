// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![forbid(unsafe_code)]
#![cfg_attr(
    test,
    allow(clippy::panic, clippy::too_many_lines, clippy::cognitive_complexity,)
)]
#![allow(
    deprecated,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::type_complexity,
    clippy::redundant_clone,
    clippy::unreadable_literal,
    clippy::ip_constant,
    clippy::unnecessary_literal_unwrap,
    clippy::module_inception,
    clippy::field_reassign_with_default,
    clippy::no_effect_underscore_binding,
    clippy::manual_string_new,
    clippy::manual_range_contains,
    clippy::needless_collect,
    clippy::items_after_statements,
    clippy::uninlined_format_args,
    clippy::float_cmp,
    clippy::unwrap_used,
    clippy::expect_used
)]

//! **NESTGATE NETWORK CRATE**
//!
//! This crate provides network functionality for the `NestGate` ecosystem,
//! including connection management, protocol handling, and service discovery.

use std::time::Duration;

/// CANONICAL MODERNIZATION: Use canonical Result type from nestgate-core
/// Type alias for Results used throughout the crate - migrated to canonical
pub use nestgate_core::Result;
// ==================== SECTION ====================

/// API module for network services
pub mod api;
#[cfg(test)]
mod comprehensive_coverage_tests;
/// Protocol handlers and management
pub mod handlers;
#[cfg(test)]
mod network_coverage_expansion;
/// Port allocation and management
pub mod ports;
#[cfg(test)]
mod ports_tests;
/// Protocol definitions
pub mod protocol;
#[cfg(test)]
mod protocol_comprehensive_tests;
/// Main network service implementation
pub mod service;
/// Network types and configuration
pub mod types;
#[cfg(test)]
mod types_comprehensive_tests;
/// Unified network configuration
pub mod unified_network_config;
/// Unified network extensions
pub mod unified_network_extensions;
// Removed: Zero-cost orchestration types (delegated to orchestration primal)
// Removed: OrchestrationAdapter (delegated to orchestration primal via capability discovery)
/// Configuration migration utilities
/// These utilities help migrate from legacy configurations
/// to the new modular network system.
/// Main network service
pub use service::RealNetworkService as NetworkService;
/// Network configuration
pub use types::{NetworkConfig, NetworkConfigBuilder};
// Removed: Universal orchestration modules (delegated to orchestration primal via capability discovery)
// ==================== SECTION ====================

// **DEPRECATED CODE REMOVED**
//
// The following deprecated compatibility layers have been eliminated:
// - `real_network_service.rs` (893 lines) - Deprecated compatibility layer
//
// **Migration Path**:
// All functionality has been migrated to the modular system:
// - Use `service::NetworkService` for main network operations
// - Use `types::NetworkConfig` for configuration
// - Use `handlers::*` for protocol-specific operations
//
// **Performance Impact**:
// - Removed 893 lines of deprecated code
// - Eliminated compatibility overhead
// - Improved compile times and memory usage

// ==================== CONFIGURATION FUNCTIONS ====================

/// Default network configuration
#[must_use]
pub fn default_network_config() -> NetworkConfig {
    NetworkConfig::default()
}
/// Create production network configuration
#[must_use]
pub fn production_network_config() -> NetworkConfig {
    let mut config = NetworkConfig::default();
    config.api.max_connections = 2000;
    config.api.connection_timeout = Duration::from_secs(10);
    config
}
/// Create development network configuration  
#[must_use]
pub fn development_network_config() -> NetworkConfig {
    let mut config = NetworkConfig::default();
    config.api.max_connections = 100;
    config.api.connection_timeout = Duration::from_secs(30);
    config
}
// ==================== SECTION ====================

/// Error handling module
pub mod error;

/// Network-specific result type
// Re-export from local error module
pub use crate::error::NetworkResult;
/// Network error types - re-exported from error module
pub use error::NetworkError;
// ==================== SECTION ====================

/// Network constants - use canonical constants system
///
/// # Primal Sovereignty
///
/// These constants are environment-driven. For dynamic discovery,
/// use `ServiceRegistry` instead.
pub mod constants {
    use std::env;

    /// Get API port from environment or use safe default
    ///
    /// # Environment Variables
    ///
    /// - `NESTGATE_API_PORT`: API server port
    #[must_use]
    pub fn api_port() -> u16 {
        env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8080) // Safe default
    }

    /// Get internal health check port from environment or use safe default
    ///
    /// # Environment Variables
    ///
    /// - `NESTGATE_HEALTH_PORT`: Health check port
    #[must_use]
    pub fn internal_port() -> u16 {
        env::var("NESTGATE_HEALTH_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8081) // Safe default
    }

    /// Default connection timeout
    pub const DEFAULT_CONNECTION_TIMEOUT_SECONDS: u64 = 30;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_creation() {
        let config = default_network_config();
        // Test that config is created successfully
        assert!(config.api.port_range_start == 0 || config.api.port_range_start >= 1024);
    }
    #[test]
    fn test_production_config() {
        let config = production_network_config();
        // Test production settings - config creation successful
        assert!(config.performance.keep_alive_timeout_seconds > 0);
    }

    #[test]
    fn test_development_config() {
        let config = development_network_config();
        // Test development settings - config creation successful
        assert!(config.performance.keep_alive_timeout_seconds > 0);
    }
}
