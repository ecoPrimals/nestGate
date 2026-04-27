// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Remove NestGate files and installation metadata.

use anyhow::{Context, Result};
use console::Style;
use dialoguer::Confirm;
use std::fs;
use tracing::info;

use super::NestGateInstaller;

impl NestGateInstaller {
    /// Uninstall the application
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn uninstall(&self, remove_config: bool, remove_data: bool, force: bool) -> Result<()> {
        let red = Style::new().red().bold();
        let yellow = Style::new().yellow().bold();

        info!("{}", red.apply_to("NestGate Uninstallation"));
        info!("");

        let installation_info = self
            .get_installation_info()
            .context("NestGate is not installed or installation info is corrupted")?;

        if !force {
            let _message = format!(
                "This will remove NestGate from {}{}{}",
                installation_info.install_path.display(),
                if remove_config {
                    " (including config)"
                } else {
                    ""
                },
                if remove_data { " (including data)" } else { "" }
            );

            let install_dir = self
                .install_dir
                .as_ref()
                .context("BUG: install_dir must be set before uninstall")?;
            if !Confirm::new()
                .with_prompt(format!(
                    "Directory {} already exists. Overwrite?",
                    install_dir.display()
                ))
                .interact()?
            {
                info!("Uninstallation cancelled.");
                return Ok(());
            }
        }

        // Remove installation directory
        if installation_info.install_path.exists() {
            fs::remove_dir_all(&installation_info.install_path)?;
            info!(
                "Removed installation directory: {}",
                installation_info.install_path.display()
            );
        }

        // Remove configuration if requested
        if remove_config
            && installation_info.config_path.exists()
            && installation_info.config_path
                != installation_info
                    .install_path
                    .join("etc")
                    .join("nestgate.toml")
        {
            fs::remove_dir_all(&installation_info.config_path)?;
            info!(
                "Removed configuration: {}",
                installation_info.config_path.display()
            );
        }

        // Remove data if requested
        if remove_data && installation_info.data_path.exists() {
            fs::remove_dir_all(&installation_info.data_path)?;
            info!(
                "Removed data directory: {}",
                installation_info.data_path.display()
            );
        }

        // Remove installation info
        let info_path = self.get_installation_info_path();
        if info_path.exists() {
            fs::remove_file(&info_path)?;
        }

        info!("{}", yellow.apply_to("NestGate uninstalled successfully"));
        Ok(())
    }
}
