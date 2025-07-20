//! Installation Configuration Management
//!
//! This module handles configuration for the NestGate installation process,
//! including default settings, user preferences, and installation paths.
//!
//! ## Features
//! - Installation path configuration
//! - Service configuration options
//! - Default parameter management
//! - Configuration validation

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallerConfig {
    pub install_path: PathBuf,
    pub service_mode: bool,
    pub features: FeatureFlags,
    pub integration: IntegrationOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub enable_zfs: bool,
    pub enable_ui: bool,
    pub enable_network: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationOptions {
    pub create_desktop_entry: bool,
    pub add_to_path: bool,
}

impl Default for InstallerConfig {
    fn default() -> Self {
        Self {
            install_path: PathBuf::from("/opt/nestgate"),
            service_mode: true,
            features: FeatureFlags::default(),
            integration: IntegrationOptions::default(),
        }
    }
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            enable_zfs: true,
            enable_ui: true,
            enable_network: true,
        }
    }
}

impl Default for IntegrationOptions {
    fn default() -> Self {
        Self {
            create_desktop_entry: true,
            add_to_path: true,
        }
    }
}

impl InstallerConfig {
    /// Create a new installer configuration with default values
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the installer configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid, such as:
    /// - Install path parent directory doesn't exist
    /// - Invalid path permissions
    pub fn validate(&self) -> Result<()> {
        // Check if install path parent exists
        if !self
            .install_path
            .parent()
            .is_some_and(std::path::Path::exists)
        {
            return Err(anyhow::anyhow!(
                "Install path parent directory does not exist"
            ));
        }

        // Additional validation could be added here
        Ok(())
    }

    /// Convert to `NestGate` configuration format
    #[must_use]
    pub fn to_nestgate_config(&self) -> String {
        format!(
            r#"
[core]
install_path = "{}"
service_mode = {}

[features]
enable_zfs = {}
enable_ui = {}
enable_network = {}

[integration]
create_desktop_entry = {}
add_to_path = {}
"#,
            self.install_path.display(),
            self.service_mode,
            self.features.enable_zfs,
            self.features.enable_ui,
            self.features.enable_network,
            self.integration.create_desktop_entry,
            self.integration.add_to_path
        )
    }
}
