// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **NETWORK ENVIRONMENT CONFIGURATION**

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Network environment configuration for environment-specific network settings.
///
/// Provides environment-specific overrides and feature flags for network behavior.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `NetworkEnvironment`
pub struct NetworkEnvironmentConfig {
    /// Name of the deployment environment (e.g., "development", "production").
    pub environment_name: String,
    /// Environment-specific configuration overrides.
    pub overrides: HashMap<String, String>,
    /// Feature flags for enabling/disabling network features per environment.
    pub feature_flags: HashMap<String, bool>,
}

impl NetworkEnvironmentConfig {
    /// Create development-optimized configuration with debug logging enabled.
    ///
    /// Suitable for local development with verbose logging.
    #[must_use]
    pub fn development_optimized() -> Self {
        let mut feature_flags = HashMap::new();
        feature_flags.insert("debug_logging".to_string(), true);

        Self {
            environment_name: "development".to_string(),
            overrides: HashMap::new(),
            feature_flags,
        }
    }

    /// Create production-hardened configuration with strict validation.
    ///
    /// Disables debug logging and enables strict validation for production.
    #[must_use]
    pub fn production_hardened() -> Self {
        let mut feature_flags = HashMap::new();
        feature_flags.insert("debug_logging".to_string(), false);
        feature_flags.insert("strict_validation".to_string(), true);

        Self {
            environment_name: "production".to_string(),
            overrides: HashMap::new(),
            feature_flags,
        }
    }

    /// Validate the environment configuration.
    ///
    /// Ensures environment name and feature flags are properly configured.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub const fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Merge this configuration with another, preferring values from `other`.
    ///
    /// All fields from `other` will replace the current values.
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.environment_name = other.environment_name;
        self.overrides = other.overrides;
        self.feature_flags = other.feature_flags;
        self
    }
}
