#![cfg(feature = "dev-stubs")]

// **MOCKING TEST CONFIGURATION**
//
// **⚠️ TEST ONLY**: This module is only available with `dev-stubs` feature

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for Mocking
pub struct MockingConfig {
    /// Services
    pub services: MockServiceConfig,
    /// Doubles
    pub doubles: TestDoubleConfig,
    /// Stubs
    pub stubs: StubConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for MockService
pub struct MockServiceConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for TestDouble
pub struct TestDoubleConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Stub
pub struct StubConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for MockServiceConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for TestDoubleConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for StubConfig {
    /// Returns the default instance
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
