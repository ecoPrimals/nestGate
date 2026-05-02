// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Post-install verification and environment diagnostics.

use anyhow::Result;
use console::Style;
use tracing::{info, warn};

use super::NestGateInstaller;

impl NestGateInstaller {
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn doctor(&self) -> Result<()> {
        let green = Style::new().green();
        let red = Style::new().red();
        let yellow = Style::new().yellow();

        info!("NestGate System Check");
        info!("");

        let mut issues = 0;

        // Check if installed
        if let Ok(info) = self.get_installation_info() {
            info!("{} NestGate is installed", green.apply_to("OK"));
            info!("   Version: {}", info.version);
            info!("   Path: {}", info.install_path.display());
            info!(
                "   Service: {}",
                if info.service_installed { "Yes" } else { "No" }
            );
        } else {
            warn!("{} NestGate is not installed", red.apply_to("MISSING"));
            issues += 1;
        }

        // Check platform support
        info!(
            "{} Platform: {}-{}",
            green.apply_to("PASS"),
            self.platform.os,
            self.platform.arch
        );
        if self.platform.service_install_supported() {
            info!("{} Service installation supported", green.apply_to("OK"));
        } else {
            warn!(
                "{} Service installation not supported",
                yellow.apply_to("WARN")
            );
        }

        // Check system requirements
        let requirements_ok = self.check_system_requirements_silent();
        if requirements_ok {
            info!("{} System requirements met", green.apply_to("PASS"));
        } else {
            warn!("{} System requirements not met", red.apply_to("FAIL"));
            issues += 1;
        }

        // Check ZFS availability
        if self.check_zfs_availability() {
            info!("{} ZFS available", green.apply_to("OK"));
        } else {
            warn!("{} ZFS not available", yellow.apply_to("WARN"));
        }

        info!("");
        if issues == 0 {
            info!("{} All checks passed", green.apply_to("OK"));
        } else {
            warn!("{} {} issues found", red.apply_to("FAIL"), issues);
        }
        Ok(())
    }
}
