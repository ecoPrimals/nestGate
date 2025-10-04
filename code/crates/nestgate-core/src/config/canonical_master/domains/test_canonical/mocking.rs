// **MOCKING TEST CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MockingConfig {
    pub services: MockServiceConfig,
    pub doubles: TestDoubleConfig,
    pub stubs: StubConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockServiceConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDoubleConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StubConfig {
    pub enabled: bool,
}

impl Default for MockServiceConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for TestDoubleConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for StubConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl MockingConfig {
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
