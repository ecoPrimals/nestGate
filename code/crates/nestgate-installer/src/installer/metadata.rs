// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Persisted installation metadata path and load/save helpers.

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use super::NestGateInstaller;
use super::types::InstallationInfo;

impl NestGateInstaller {
    /// Save Installation Info
    pub(super) fn save_installation_info(&self, info: &InstallationInfo) -> Result<()> {
        let info_path = self.get_installation_info_path();
        let info_json = serde_json::to_string_pretty(info)?;
        fs::write(&info_path, info_json)?;
        Ok(())
    }

    /// Gets Installation Info
    pub(super) fn get_installation_info(&self) -> Result<InstallationInfo> {
        let info_path = self.get_installation_info_path();
        let info_json = fs::read_to_string(&info_path).context("Installation info not found")?;
        let info: InstallationInfo =
            serde_json::from_str(&info_json).context("Invalid installation info format")?;
        Ok(info)
    }

    /// Gets Installation Info Path
    #[expect(
        clippy::unused_self,
        reason = "Installer method: will use self for platform state"
    )]
    pub(super) fn get_installation_info_path(&self) -> PathBuf {
        use etcetera::BaseStrategy;

        etcetera::base_strategy::choose_base_strategy()
            .ok()
            .map_or_else(
                || PathBuf::from(".nestgate-install-info.json"),
                |strategy| {
                    strategy
                        .data_dir()
                        .join("nestgate")
                        .join("install-info.json")
                },
            )
    }
}
