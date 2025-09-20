//! **TEST ENVIRONMENT CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TestEnvironmentConfig {
    pub infrastructure: TestInfrastructureConfig,
    pub resources: TestResourceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestInfrastructureConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResourceConfig {
    pub enabled: bool,
}


impl Default for TestInfrastructureConfig {
    fn default() -> Self { Self { enabled: true } }
}

impl Default for TestResourceConfig {
    fn default() -> Self { Self { enabled: true } }
}

impl TestEnvironmentConfig {
    pub fn ci_optimized() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
} 