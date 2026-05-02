// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Core constructor and install entrypoint.

use anyhow::Result;

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
        self.setup_system_integration(config)
    }
}
