// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Core constructor and install entrypoint.

use anyhow::Result;
use tracing::info;

use crate::config::InstallerConfig;
use crate::download::DownloadManager;
use crate::platform::PlatformInfo;

use super::NestGateInstaller;

impl NestGateInstaller {
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn new(install_dir: Option<std::path::PathBuf>) -> Result<Self> {
        let platform = PlatformInfo::detect();
        let downloader = DownloadManager::new();

        Ok(Self {
            platform,
            install_dir,
            downloader,
        })
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn install(&self, config: &InstallerConfig) -> Result<()> {
        // Note: domains field doesn't exist in canonical config - using system config instead
        let system_config = &config.base_config.system;

        // System integration logic would be based on system config
        // For now, just log that we're doing system integration
        info!(
            "Performing system integration for: {}",
            system_config.instance_name
        );

        Ok(())
    }
}
