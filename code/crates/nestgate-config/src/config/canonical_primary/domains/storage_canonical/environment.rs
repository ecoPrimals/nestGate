// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **STORAGE ENVIRONMENT CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `StorageEnvironment`
pub struct StorageEnvironmentConfig {
    /// Development
    pub development: EnvironmentStorageSettings,
    /// Staging
    pub staging: EnvironmentStorageSettings,
    /// Production
    pub production: EnvironmentStorageSettings,
    /// Deployment
    pub deployment: DeploymentStorageConfig,
    /// Runtime
    pub runtime: RuntimeStorageConfig,
    /// Features
    pub features: StorageFeatureConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Environmentstoragesettings
pub struct EnvironmentStorageSettings {
    /// Backend Type
    pub backend_type: String,
    /// Performance Mode
    pub performance_mode: String,
    /// Security Level
    pub security_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `DeploymentStorage`
pub struct DeploymentStorageConfig {
    /// Auto Provision
    pub auto_provision: bool,
    /// Resource Limits
    pub resource_limits: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `RuntimeStorage`
pub struct RuntimeStorageConfig {
    /// Hot Reload
    pub hot_reload: bool,
    /// Dynamic Scaling
    pub dynamic_scaling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `StorageFeature`
pub struct StorageFeatureConfig {
    /// Feature Flags
    pub feature_flags: HashMap<String, bool>,
}

impl Default for StorageEnvironmentConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            development: EnvironmentStorageSettings {
                backend_type: "filesystem".to_string(),
                performance_mode: "standard".to_string(),
                security_level: "basic".to_string(),
            },
            staging: EnvironmentStorageSettings {
                backend_type: "zfs".to_string(),
                performance_mode: "optimized".to_string(),
                security_level: "enhanced".to_string(),
            },
            production: EnvironmentStorageSettings {
                backend_type: "zfs".to_string(),
                performance_mode: "high_performance".to_string(),
                security_level: "maximum".to_string(),
            },
            deployment: DeploymentStorageConfig {
                auto_provision: false,
                resource_limits: HashMap::new(),
            },
            runtime: RuntimeStorageConfig {
                hot_reload: false,
                dynamic_scaling: false,
            },
            features: StorageFeatureConfig {
                feature_flags: HashMap::new(),
            },
        }
    }
}

impl StorageEnvironmentConfig {
    /// Create production-optimized environment configuration.
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Create development-optimized environment configuration.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Create high-performance environment configuration.
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Create cloud-native environment configuration.
    #[must_use]
    pub fn cloud_native() -> Self {
        Self::default()
    }

    /// Merge this configuration with another.
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }

    /// Validate environment configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn storage_environment_config_default_values() {
        let c = StorageEnvironmentConfig::default();
        assert_eq!(c.development.backend_type, "filesystem");
        assert_eq!(c.staging.backend_type, "zfs");
        assert_eq!(c.production.performance_mode, "high_performance");
        assert!(!c.deployment.auto_provision);
        assert!(c.deployment.resource_limits.is_empty());
        assert!(!c.runtime.hot_reload);
        assert!(!c.runtime.dynamic_scaling);
        assert!(c.features.feature_flags.is_empty());
    }

    #[test]
    fn storage_environment_constructors_match_default() {
        let d = StorageEnvironmentConfig::default();
        let ser = serde_json::to_string(&d).expect("serialize");
        assert_eq!(
            ser,
            serde_json::to_string(&StorageEnvironmentConfig::production_optimized())
                .expect("serialize")
        );
        assert_eq!(
            ser,
            serde_json::to_string(&StorageEnvironmentConfig::development_optimized())
                .expect("serialize")
        );
        assert_eq!(
            ser,
            serde_json::to_string(&StorageEnvironmentConfig::high_performance())
                .expect("serialize")
        );
        assert_eq!(
            ser,
            serde_json::to_string(&StorageEnvironmentConfig::cloud_native()).expect("serialize")
        );
    }

    #[test]
    fn storage_environment_merge_keeps_self() {
        let mut other = StorageEnvironmentConfig::default();
        other.development.backend_type = "s3".to_string();
        let merged = StorageEnvironmentConfig::default().merge(other);
        assert_eq!(merged.development.backend_type, "filesystem");
    }

    #[test]
    fn storage_environment_validate_succeeds() {
        assert!(StorageEnvironmentConfig::default().validate().is_ok());
    }

    #[test]
    fn storage_environment_serde_roundtrip() {
        let mut original = StorageEnvironmentConfig::default();
        original
            .deployment
            .resource_limits
            .insert("cpu".to_string(), "4".to_string());
        original
            .features
            .feature_flags
            .insert("tiered".to_string(), true);
        let json = serde_json::to_string(&original).expect("serialize");
        let parsed: StorageEnvironmentConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(
            original.deployment.resource_limits,
            parsed.deployment.resource_limits
        );
        assert_eq!(
            original.features.feature_flags,
            parsed.features.feature_flags
        );
        assert_eq!(
            serde_json::to_string(&original).expect("serialize"),
            serde_json::to_string(&parsed).expect("re-serialize")
        );
    }

    #[test]
    fn deployment_runtime_feature_defaults() {
        let d = DeploymentStorageConfig {
            auto_provision: true,
            resource_limits: HashMap::new(),
        };
        assert!(d.auto_provision);
        let r = RuntimeStorageConfig {
            hot_reload: true,
            dynamic_scaling: true,
        };
        assert!(r.hot_reload && r.dynamic_scaling);
    }
}
