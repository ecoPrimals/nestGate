// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CONSOLIDATED DOMAIN CONFIGURATIONS**
//!
//! This module consolidates 100+ scattered Config structs across the NestGate ecosystem
//! into a single, canonical system organized by domain concern.
//!
//! ## Architecture
//!
//! Each domain has its own focused module:
//! - **`zfs`**: ZFS storage management configuration
//! - **`api`**: API and HTTP services configuration
//! - **`mcp`**: MCP protocol handling configuration
//! - **`services`**: Other service domains (Network, Automation, FsMonitor, etc.)
//! - **`validation`**: Configuration validation framework
//!
//! ## Usage
//!
//! ```rust
//! use nestgate_core::config::canonical_primary::domains::ConsolidatedDomainConfigs;
//!
//! let config = ConsolidatedDomainConfigs::default();
//! let zfs_config = &config.zfs;
//! let api_config = &config.api;
//! ```
//!
//! ## Benefits of This Structure
//!
//! - **Modularity**: Each domain is self-contained
//! - **Maintainability**: Easy to locate and modify domain-specific configs
//! - **Testability**: Domain configs can be tested independently
//! - **Scalability**: New domains can be added without bloating existing modules

use serde::{Deserialize, Serialize};

// Re-export all domain modules
pub mod api;
pub mod integration;
pub mod mcp;
pub mod services;
pub mod validation;
pub mod zfs;

// Re-export commonly used types
pub use api::{ApiDomainConfig, ApiServerConfig, ConsolidatedApiHandlersConfig};
pub use integration::{
    BiomeOsIntegrationConfig, CapabilityRoutingConfig, ConsolidatedIntegrationConfigs,
    EcosystemConfig, ExternalServiceConfig, PrimalEcosystemConfig, ProtocolConfigs,
    ServiceDiscoveryConfig,
};
pub use mcp::{McpDomainConfig, McpMessageFormat, McpProtocolConfig};
pub use services::{
    AutomationDomainConfig, BinaryDomainConfig, FsMonitorDomainConfig, InstallerDomainConfig,
    NetworkServicesDomainConfig, PerformanceDomainConfig,
};
pub use validation::{DomainConfigValidation, ValidationError};
pub use zfs::{
    ZfsAutoSnapshotConfig, ZfsCleanupConfig, ZfsCompressionConfig, ZfsDatasetConfig,
    ZfsDatasetDefaults, ZfsDatasetsConfig, ZfsDomainConfig, ZfsPoolConfig, ZfsPoolsConfig,
    ZfsRetentionConfig, ZfsSizeLimits, ZfsSnapshotIntervals, ZfsSnapshotsConfig,
};

// ==================== CORE DOMAIN CONFIGURATIONS ====================

/// **CONSOLIDATED DOMAIN CONFIGURATIONS**
///
/// This structure brings together all domain-specific configurations
/// under a single, well-organized hierarchy.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsolidatedDomainConfigs {
    /// ZFS storage management configuration
    pub zfs: ZfsDomainConfig,

    /// API and HTTP services configuration
    pub api: ApiDomainConfig,

    /// MCP protocol handling configuration
    pub mcp: McpDomainConfig,

    /// Network orchestration configuration
    pub network_services: NetworkServicesDomainConfig,

    /// Automation and workflows configuration
    pub automation: AutomationDomainConfig,

    /// File system monitoring configuration
    pub fsmonitor: FsMonitorDomainConfig,

    /// Installation and deployment configuration
    pub installer: InstallerDomainConfig,

    /// Performance monitoring and optimization
    pub performance: PerformanceDomainConfig,

    /// Binary and executable configuration
    pub binary: BinaryDomainConfig,
}

impl ConsolidatedDomainConfigs {
    /// Create new consolidated domain configurations with defaults
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate all domain configurations
    ///
    /// # Errors
    ///
    /// Returns error if any domain configuration is invalid
    pub fn validate(&self) -> nestgate_types::error::Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Collect warnings from each domain
        warnings.extend(self.zfs.validate()?);
        warnings.extend(self.api.validate()?);
        warnings.extend(self.mcp.validate()?);

        Ok(warnings)
    }

    /// Validate all domain configurations (strict)
    ///
    /// # Errors
    ///
    /// Returns error if any domain configuration is invalid
    pub fn validate_all(&self) -> nestgate_types::error::Result<()> {
        // Validate each domain strictly
        self.zfs.validate()?;
        self.api.validate()?;
        self.mcp.validate()?;

        Ok(())
    }

    /// Validate for specific environment (dev, staging, production)
    ///
    /// # Errors
    ///
    /// Returns error if configuration is invalid for the given environment
    pub fn validate_for_environment(&self, env: &str) -> nestgate_types::error::Result<()> {
        self.zfs.validate_for_environment(env)?;
        self.api.validate_for_environment(env)?;
        self.mcp.validate_for_environment(env)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consolidated_config_creation() {
        let config = ConsolidatedDomainConfigs::new();
        assert_eq!(config.zfs.pools.default_pool, "tank");
    }

    #[test]
    fn test_validation() {
        let config = ConsolidatedDomainConfigs::default();
        assert!(config.validate_all().is_ok());
    }

    #[test]
    fn test_environment_validation() {
        let config = ConsolidatedDomainConfigs::default();
        assert!(config.validate_for_environment("production").is_ok());
        assert!(config.validate_for_environment("dev").is_ok());
    }
}
