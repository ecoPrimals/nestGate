// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Interactive and read-only configuration flows.

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tracing::info;

use crate::config::InstallerConfig;
use crate::wizard::InstallationWizard;

use super::NestGateInstaller;

impl NestGateInstaller {
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn configure(&self, config_path: Option<PathBuf>) -> Result<()> {
        let installation_info = self
            .get_installation_info()
            .context("NestGate is not installed")?;

        let config_file = config_path.unwrap_or(installation_info.config_path);

        // User-facing interactive output — not log
        println!("Current configuration: {}", config_file.display());

        if config_file.exists() {
            let content = fs::read_to_string(&config_file)?;
            println!("{content}");
        } else {
            println!("Configuration file does not exist.");
        }
        Ok(())
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn run_configuration_wizard(&self) -> Result<()> {
        let installation_info = self
            .get_installation_info()
            .context("NestGate is not installed")?;

        let mut wizard = InstallationWizard::new(InstallerConfig::default());
        let config = wizard.run()?;

        // Save new configuration
        let config_toml = toml::to_string(&config)
            .map_err(|e| anyhow::anyhow!("Failed to serialize config: {e}"))?;
        fs::write(&installation_info.config_path, config_toml)?;

        info!(
            "Configuration updated: {}",
            installation_info.config_path.display()
        );
        Ok(())
    }
}
