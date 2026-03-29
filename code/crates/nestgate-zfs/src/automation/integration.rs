// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// This module provides integration functions for initializing ZFS automation
// with the NestGate ecosystem and external services.

// Removed unresolved automation imports - use local implementations
//! Integration module

use nestgate_core::Result;
use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;

/// Intelligent dataset manager for automated operations
pub struct IntelligentDatasetManager;
/// Configuration for Automation
pub struct AutomationConfig;

impl Default for AutomationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self
    }
}

/// Initialize automation integration with canonical configuration
pub fn initialize_automation(config: NestGateCanonicalConfig) -> Result<IntelligentDatasetManager> {
    let _automation_config = AutomationConfig;
    let _config = config; // Use config parameter to avoid warnings
    // Placeholder implementation until automation crate is fully integrated
    Ok(IntelligentDatasetManager)
}
/// Initialize automation with custom config
pub fn initialize_automation_with_config(
    config: NestGateCanonicalConfig,
    automation_config: AutomationConfig,
) -> Result<IntelligentDatasetManager> {
    let _config = config; // Use parameters to avoid warnings
    let _automation_config = automation_config;
    // Placeholder implementation until automation crate is fully integrated
    Ok(IntelligentDatasetManager)
}
/// Check if ecosystem services are available for ZFS automation
#[cfg(feature = "network-integration")]
#[must_use]
pub const fn check_zfs_ecosystem_availability() -> bool {
    // Default implementation for development - ecosystem always available
    // Production implementation would check actual automation capabilities
    true
}
#[cfg(not(feature = "network-integration"))]
/// Checks if ZFS ecosystem is available (stub for non-network builds).
#[must_use]
pub fn check_zfs_ecosystem_availability() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automation_config_default() {
        let _config = AutomationConfig;
        // Just verify it doesn't panic
        // Automatically dropped at end of scope
    }

    #[test]
    fn test_initialize_automation() {
        let config = NestGateCanonicalConfig::default();
        let result = initialize_automation(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_initialize_automation_with_config() {
        let config = NestGateCanonicalConfig::default();
        let automation_config = AutomationConfig;
        let result = initialize_automation_with_config(config, automation_config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_ecosystem_availability() {
        let available = check_zfs_ecosystem_availability();
        // Availability depends on features
        #[cfg(feature = "network-integration")]
        assert!(available);
        #[cfg(not(feature = "network-integration"))]
        assert!(!available);
    }

    #[test]
    fn test_multiple_initializations() {
        let config1 = NestGateCanonicalConfig::default();
        let config2 = NestGateCanonicalConfig::default();

        let result1 = initialize_automation(config1);
        let result2 = initialize_automation(config2);

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }
}
