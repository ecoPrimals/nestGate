// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Installation metadata types.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Metadata recorded about a completed or in-progress installation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationInfo {
    /// Version
    pub version: String,
    /// Install Date
    pub install_date: chrono::DateTime<chrono::Utc>,
    /// Install Path
    pub install_path: PathBuf,
    /// Configuration for path
    pub config_path: PathBuf,
    /// Data Path
    pub data_path: PathBuf,
    /// Service Installed
    pub service_installed: bool,
    /// Features
    pub features: Vec<String>,
}
