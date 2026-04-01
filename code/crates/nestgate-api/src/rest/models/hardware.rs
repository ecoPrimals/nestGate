// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Hardware analysis and auto-configuration models

use serde::{Deserialize, Serialize};

// Hardware detection and system information structures

/// System hardware information and specifications
///
/// Provides comprehensive hardware details for system analysis,
/// performance planning, and compatibility assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Hardwareinfo
pub struct HardwareInfo {
    /// Number of CPU cores available to the system
    pub cpu_cores: u32,
}
