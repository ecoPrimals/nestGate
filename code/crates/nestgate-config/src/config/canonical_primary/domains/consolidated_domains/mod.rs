// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CONSOLIDATED DOMAIN CONFIGURATIONS**
//!
//! This module consolidates 100+ scattered Config structs across the `NestGate` ecosystem
//! into a single, canonical system organized by domain concern.
//!
//! ## Architecture
//!
//! Each domain has its own focused module:
//! - **`zfs`**: ZFS storage management configuration
//! - **`api`**: API and HTTP services configuration
//! - **`mcp`**: MCP protocol handling configuration
//! - **`services`**: Other service domains (Network, Automation, `FsMonitor`, etc.)
//! - **`validation`**: Configuration validation framework
//!
//! ## Usage
//!
//! ```rust,ignore
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
use std::collections::HashMap;

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

fn merge_json_values(base: &mut serde_json::Value, patch: &serde_json::Value) {
    match (base, patch) {
        (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
            for (k, v) in b {
                if let Some(existing) = a.get_mut(k) {
                    merge_json_values(existing, v);
                } else {
                    a.insert(k.clone(), v.clone());
                }
            }
        }
        (b, p) => *b = p.clone(),
    }
}

fn merge_json_patch<T>(
    target: &mut T,
    patch: &serde_json::Value,
) -> nestgate_types::error::Result<()>
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let mut base = serde_json::to_value(&*target).map_err(|e| {
        nestgate_types::error::NestGateError::configuration_error(
            "domain_overrides",
            format!("serialize domain config: {e}"),
        )
    })?;
    merge_json_values(&mut base, patch);
    *target = serde_json::from_value(base).map_err(|e| {
        nestgate_types::error::NestGateError::configuration_error(
            "domain_overrides",
            format!("merge domain override: {e}"),
        )
    })?;
    Ok(())
}

impl ConsolidatedDomainConfigs {
    /// Create new consolidated domain configurations with defaults
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Merge per-domain JSON patches into this consolidated configuration (deep-merge objects).
    pub fn merge_domain_json_overrides(&mut self, overrides: HashMap<String, serde_json::Value>) {
        for (domain, patch) in overrides {
            let res = match domain.as_str() {
                "zfs" => merge_json_patch(&mut self.zfs, &patch),
                "api" => merge_json_patch(&mut self.api, &patch),
                "mcp" => merge_json_patch(&mut self.mcp, &patch),
                "network_services" => merge_json_patch(&mut self.network_services, &patch),
                "automation" => merge_json_patch(&mut self.automation, &patch),
                "fsmonitor" => merge_json_patch(&mut self.fsmonitor, &patch),
                "installer" => merge_json_patch(&mut self.installer, &patch),
                "performance" => merge_json_patch(&mut self.performance, &patch),
                "binary" => merge_json_patch(&mut self.binary, &patch),
                _ => {
                    tracing::warn!("unknown domain override key: {domain}");
                    Ok(())
                }
            };
            if let Err(e) = res {
                tracing::warn!("failed to apply domain override for {domain}: {e}");
            }
        }
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
