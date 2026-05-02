// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// **Complete installation and deployment system for NestGate**
//
// This module provides the core installation functionality for NestGate storage system,
// including interactive setup, automated deployment, and cross-platform installation.
//
// ## Key Features
//
// - **Cross-Platform Installation**: Supports Windows, macOS, and Linux
// - **Interactive Setup**: Guided wizard with configuration validation
// - **Automated Deployment**: Unattended installation for CI/CD
// - **Service Integration**: System service setup and configuration
// - **Dependency Management**: Automatic resolution and installation
//
// ## Usage
//
// See `NestGateInstaller` and `InstallerConfig` in this crate; run the installation wizard or `install()` as documented on those types.

//! Installer module

mod configure;
mod doctor;
mod install_ops;
mod metadata;
mod requirements;
mod types;
mod uninstall;
mod update;

#[cfg(test)]
mod tests;

/// Stable public alias for `types::InstallationInfo`.
pub type InstallationInfo = types::InstallationInfo;

use crate::download::DownloadManager;
use crate::platform::PlatformInfo;
use std::path::PathBuf;

// `configure()` keeps `println!` for stdout (piping / shell redirection). Other subcommands use `tracing` for user-visible messages.

/// Orchestrates download, platform detection, and install steps for NestGate.
pub struct NestGateInstaller {
    pub(crate) platform: PlatformInfo,
    pub(crate) install_dir: Option<PathBuf>,
    pub(crate) downloader: DownloadManager,
}
