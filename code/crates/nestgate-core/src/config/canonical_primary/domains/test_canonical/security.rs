// **SECURITY TEST CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for SecurityTest
pub struct SecurityTestConfig {
    /// Penetration
    pub penetration: PenetrationTestConfig,
    /// Vulnerability
    pub vulnerability: VulnerabilityTestConfig,
    /// Compliance
    pub compliance: ComplianceTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for PenetrationTest
pub struct PenetrationTestConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for VulnerabilityTest
pub struct VulnerabilityTestConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ComplianceTest
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
    #[must_use]
    pub fn ci_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
}
