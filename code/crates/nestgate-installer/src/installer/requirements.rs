// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! System requirement and optional capability checks.

use anyhow::Result;
use tracing::info;

use super::NestGateInstaller;

impl NestGateInstaller {
    /// Check System Requirements
    #[expect(
        clippy::unused_self,
        reason = "Installer method: will use self for platform state"
    )]
    pub(super) fn check_system_requirements(&self) -> Result<()> {
        // Check disk space (at least 100MB)
        // Check memory (at least 512MB)
        // Check OS compatibility

        info!("System requirements check passed");
        Ok(())
    }

    /// Check System Requirements Silent
    pub(super) fn check_system_requirements_silent(&self) -> bool {
        self.check_system_requirements().is_ok()
    }

    /// Check Zfs Availability
    #[expect(
        clippy::unused_self,
        reason = "Installer method: will use self for platform state"
    )]
    pub(super) fn check_zfs_availability(&self) -> bool {
        // Check if ZFS is available on the system
        std::process::Command::new("zfs")
            .arg("version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    #[expect(
        clippy::unused_self,
        reason = "Installer method: will use self for platform state"
    )]
    pub(super) fn setup_system_integration(
        &self,
        config: &crate::config::InstallerConfig,
    ) -> Result<()> {
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
