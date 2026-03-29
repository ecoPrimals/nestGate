// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **SECURITY TEST CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `SecurityTest`
pub struct SecurityTestConfig {
    /// Penetration
    pub penetration: PenetrationTestConfig,
    /// Vulnerability
    pub vulnerability: VulnerabilityTestConfig,
    /// Compliance
    pub compliance: ComplianceTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `PenetrationTest`
pub struct PenetrationTestConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `VulnerabilityTest`
pub struct VulnerabilityTestConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ComplianceTest`
pub struct ComplianceTestConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for VulnerabilityTestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for ComplianceTestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl SecurityTestConfig {
    /// Creates a CI-optimized security test configuration
    #[must_use]
    pub fn ci_optimized() -> Self {
        Self::default()
    }
    /// Creates a development-optimized security test configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    /// Merges this configuration with another, taking precedence
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
}
