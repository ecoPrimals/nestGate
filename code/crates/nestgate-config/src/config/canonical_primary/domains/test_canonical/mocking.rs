// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
/// Configuration for `MockService`
pub struct MockServiceConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `TestDouble`
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
    /// Creates a CI-optimized mocking configuration
    #[must_use]
    pub fn ci_optimized() -> Self {
        Self::default()
    }
    /// Creates a development-optimized mocking configuration
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
