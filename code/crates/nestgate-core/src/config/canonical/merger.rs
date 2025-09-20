// Configuration Merging
//! Merger functionality and utilities.
// This module handles merging multiple configurations.
// Single responsibility: Combine configurations with proper precedence.

use super::types::*;
use crate::Result;

/// Configuration merger
pub struct ConfigMerger;
impl ConfigMerger {
    /// Merge two configurations with override precedence
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn merge(
        base: CanonicalConfig,
        override_config: CanonicalConfig,
    ) -> Result<CanonicalConfig>  {
        let mut merged = base;

        // Override system config
        merged.system = override_config.system;
        merged.network = override_config.network;
        merged.storage = override_config.storage;
        merged.security = override_config.security;
        merged.performance = override_config.performance;
        merged.monitoring = override_config.monitoring;
        merged.integrations = override_config.integrations;
        merged.environment = override_config.environment;

        Ok(merged)
    }
}
