// **SECURITY TEST CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityTestConfig {
    pub penetration: PenetrationTestConfig,
    pub vulnerability: VulnerabilityTestConfig,
    pub compliance: ComplianceTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PenetrationTestConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityTestConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceTestConfig {
    pub enabled: bool,
}

impl Default for VulnerabilityTestConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for ComplianceTestConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl SecurityTestConfig {
    #[must_use]
    pub const fn ci_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
}
