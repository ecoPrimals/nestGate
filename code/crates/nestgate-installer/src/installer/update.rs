// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Download and apply release updates in place.

use anyhow::{Context, Result};
use console::Style;
use dialoguer::Confirm;
use std::fs;
use tracing::info;

use super::NestGateInstaller;

impl NestGateInstaller {
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub async fn update(&mut self, version: Option<String>, yes: bool) -> Result<()> {
        let blue = Style::new().blue().bold();

        info!("{}", blue.apply_to("🔄 NestGate Update"));
        info!("");

        let installation_info = self
            .get_installation_info()
            .context("NestGate is not installed")?;

        let target_version = match version {
            Some(v) => v,
            None => self.downloader.check_latest_version().await?,
        };

        if target_version == installation_info.version {
            info!("Already up to date (version {target_version})");
            return Ok(());
        }

        if !yes
            && !Confirm::new()
                .with_prompt(format!(
                    "Update NestGate from {} to {}?",
                    installation_info.version, target_version
                ))
                .interact()?
        {
            info!("Update cancelled.");
            return Ok(());
        }

        // Backup current installation
        let backup_path = installation_info.install_path.with_extension("backup");
        if backup_path.exists() {
            fs::remove_dir_all(&backup_path)?;
        }
        fs::rename(&installation_info.install_path, &backup_path)?;

        // Download and install new version
        let temp_dir = std::env::temp_dir().join("nestgate-update");
        fs::create_dir_all(&temp_dir)?;

        let archive_path = self
            .downloader
            .download_release(&target_version, &temp_dir)
            .await?;
        self.downloader
            .extract_archive(&archive_path, &installation_info.install_path)?;

        // Restore configuration
        let old_config = backup_path.join("etc").join("nestgate.toml");
        let new_config = installation_info
            .install_path
            .join("etc")
            .join("nestgate.toml");
        if old_config.exists() {
            fs::copy(&old_config, &new_config)?;
        }

        // Update installation info
        let mut updated_info = installation_info;
        updated_info.version = target_version;
        self.save_installation_info(&updated_info)?;

        // Verify installation
        self.downloader
            .verify_installation(&updated_info.install_path)?;

        // Cleanup
        fs::remove_dir_all(&backup_path)?;
        fs::remove_dir_all(&temp_dir)?;

        info!(
            "Update completed successfully to version {}",
            updated_info.version
        );
        Ok(())
    }
}
