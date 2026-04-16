// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CANONICAL CONFIGURATION REGISTRY**
//!
//! Central registry of all configuration types across the NestGate ecosystem.
//!
//! # Modern Modular Architecture (Refactored Nov 13, 2025)
//!
//! This module was refactored from a single 1571-line file into a clean modular structure:
//!
//! - **`storage`** - Storage backend, connection, replication, tiering configurations (340 lines)
//! - **`security`** - Authentication, encryption, audit configurations (215 lines)
//! - **`monitoring`** - Metrics, logging, alerting configurations (557 lines)
//!
//! ## Benefits of Modular Structure
//!
//! ✅ **Maintained Cohesion** - Related types grouped logically by domain
//! ✅ **Better Discoverability** - Clear module boundaries for navigation
//! ✅ **Reduced Complexity** - Each file <600 lines, easier to understand
//! ✅ **Single Import Point** - Still use `config_registry::*` for all types
//! ✅ **No Circular Dependencies** - Clean module hierarchy
//!
//! ## Usage
//!
//! ```rust,ignore
//! // Import all config types (same as before refactoring)
//! use nestgate_core::canonical::types::config_registry::*;
//!
//! // Or import specific domains
//! use nestgate_core::canonical::types::config_registry::storage::*;
//! ```
//!
//! ## Module Purpose
//!
//! This module provides the single source of truth for all configuration types
//! across the NestGate ecosystem, eliminating fragmentation and duplication.
//!
//! **CONSOLIDATES AND REPLACES**:
//! - `StorageConfig` (7+ different definitions across crates)
//! - `NetworkConfig` (4+ different definitions)
//! - `SecurityConfig` (3+ different definitions)
//! - `MonitoringConfig` (5+ different definitions)
//! - All other fragmented config structures
//!
//! **Refactored**: November 13, 2025 (Technical Debt Elimination)
//! **Previous Size**: 1571 lines (single file)
//! **Current Size**: 4 focused modules (~350 lines each)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ==================== SUBMODULES ====================

pub mod monitoring;
pub mod security;
pub mod storage;

#[cfg(test)]
mod storage_tests;

// Re-export all types for convenience
pub use monitoring::*;
pub use security::*;
pub use storage::*;

// ==================== TOP-LEVEL CONFIGURATION TYPES ====================

/// **THE CANONICAL STORAGE CONFIGURATION**
///
/// Consolidates ALL storage configuration patterns:
/// - `StorageConfig` from `canonical_storage.rs`
/// - `StorageResourceConfig` from `unified_types/storage/config.rs`
/// - `StorageConfig` from config/storage.rs
/// - `CanonicalStorageConfig` from `config/canonical/NestGateCanonicalConfig/storage_configs.rs`
/// - `StorageConfig` from `unified_final_config/NestGateCanonicalConfig/storage.rs`
/// - `FsMonitorStorageSettings` from fsmonitor storage config
/// - `McpStorageConfig` from mcp storage config
///
/// Configuration for CanonicalStorage
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalStorageConfig {
    /// Storage backend type
    pub backend_type: storage::StorageBackendType,
    /// Connection settings
    pub connection: storage::StorageConnectionConfig,
    /// Performance settings
    pub performance: storage::StoragePerformanceConfig,
    /// Security settings
    pub security: storage::StorageSecurityConfig,
    /// Replication settings
    pub replication: storage::StorageReplicationConfig,
    /// Tier management
    pub tiers: storage::StorageTierConfig,
    /// Protocol support
    pub protocols: storage::StorageProtocolsConfig,
    /// Monitoring settings
    pub monitoring: storage::StorageMonitoringConfig,
    /// Resource limits
    pub resources: storage::StorageResourceConfig,
    /// Environment-specific overrides
    pub environment_overrides: HashMap<String, serde_json::Value>,
}

/// **THE CANONICAL MONITORING CONFIGURATION**
///
/// Consolidates ALL monitoring configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for CanonicalMonitoring
pub struct CanonicalMonitoringConfig {
    /// Metrics collection
    pub metrics: monitoring::MonitoringMetricsConfig,
    /// Logging settings
    pub logging: monitoring::MonitoringLoggingConfig,
    /// Tracing settings
    pub tracing: monitoring::MonitoringTracingConfig,
    /// Health checks
    pub health_checks: monitoring::MonitoringHealthCheckConfig,
    /// Alerting settings
    pub alerting: monitoring::MonitoringAlertingConfig,
    /// Dashboard settings
    pub dashboards: monitoring::MonitoringDashboardConfig,
    /// Performance monitoring
    pub performance: monitoring::MonitoringPerformanceConfig,
}

// ==================== REGISTRY UTILITY ====================

/// Configuration type registry for runtime type lookup and validation
pub struct CanonicalConfigTypeRegistry;

impl CanonicalConfigTypeRegistry {
    /// Get all registered configuration type names
    #[must_use]
    pub fn all_types() -> Vec<&'static str> {
        vec![
            "CanonicalStorageConfig",
            "CanonicalMonitoringConfig",
            // Storage subtypes
            "StorageBackendType",
            "StorageConnectionConfig",
            "StoragePerformanceConfig",
            "StorageSecurityConfig",
            "StorageReplicationConfig",
            "StorageTierConfig",
            "StorageProtocolsConfig",
            "StorageMonitoringConfig",
            "StorageResourceConfig",
            "CanonicalNetworkConfig",
            // Security subtypes
            "SecurityAuthenticationConfig",
            "SecurityAuthorizationConfig",
            "SecurityEncryptionConfig",
            "SecurityTlsConfig",
            "SecurityAuditConfig",
            "SecurityThreatDetectionConfig",
            "SecurityComplianceConfig",
            // Monitoring subtypes
            "MonitoringMetricsConfig",
            "MonitoringLoggingConfig",
            "MonitoringTracingConfig",
            "MonitoringHealthCheckConfig",
            "MonitoringAlertingConfig",
            "MonitoringDashboardConfig",
            "MonitoringPerformanceConfig",
        ]
    }

    /// Check if a configuration type is registered
    #[must_use]
    pub fn is_registered(type_name: &str) -> bool {
        Self::all_types().contains(&type_name)
    }

    /// Get configuration type category (storage, network, security, monitoring)
    #[must_use]
    pub fn get_category(type_name: &str) -> Option<&'static str> {
        if type_name.contains("Storage") {
            Some("storage")
        } else if type_name.contains("Network") {
            Some("network")
        } else if type_name.contains("Security") {
            Some("security")
        } else if type_name.contains("Monitoring") {
            Some("monitoring")
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_all_types() {
        let types = CanonicalConfigTypeRegistry::all_types();
        assert!(!types.is_empty());
        assert!(types.len() >= 25);
    }

    #[test]
    fn test_registry_is_registered() {
        assert!(CanonicalConfigTypeRegistry::is_registered(
            "CanonicalStorageConfig"
        ));
        assert!(CanonicalConfigTypeRegistry::is_registered(
            "StorageBackendType"
        ));
        assert!(!CanonicalConfigTypeRegistry::is_registered(
            "NonexistentConfig"
        ));
    }

    #[test]
    fn test_registry_get_category() {
        assert_eq!(
            CanonicalConfigTypeRegistry::get_category("StorageBackendType"),
            Some("storage")
        );
        assert_eq!(
            CanonicalConfigTypeRegistry::get_category("CanonicalNetworkConfig"),
            Some("network")
        );
        assert_eq!(
            CanonicalConfigTypeRegistry::get_category("SecurityAuthenticationConfig"),
            Some("security")
        );
        assert_eq!(
            CanonicalConfigTypeRegistry::get_category("MonitoringMetricsConfig"),
            Some("monitoring")
        );
        assert_eq!(
            CanonicalConfigTypeRegistry::get_category("UnknownConfig"),
            None
        );
    }

    #[test]
    fn test_canonical_storage_config_default() {
        let config = CanonicalStorageConfig::default();
        // Verify default initialization works
        assert!(config.environment_overrides.is_empty());
    }

    #[test]
    fn test_canonical_monitoring_config_default() {
        let config = CanonicalMonitoringConfig::default();
        // Verify default initialization works
        let _ = config;
    }
}
