//! Integration test configuration module
//! Provides unified integration testing configuration and settings.

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for IntegrationTest
pub struct IntegrationTestConfig {
    /// Database test configuration
    pub database: DatabaseTestConfig,

    /// Service test configuration
    pub service: ServiceTestConfig,

    /// API test configuration
    pub api: ApiTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for DatabaseTest
pub struct DatabaseTestConfig {
    /// Enable database tests
    pub enabled: bool,

    /// Test timeout
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ServiceTest
pub struct ServiceTestConfig {
    /// Enable service tests
    pub enabled: bool,

    /// Test timeout
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ApiTest
pub struct ApiTestConfig {
    /// Enable API tests
    pub enabled: bool,

    /// Test timeout
    pub timeout: Duration,
}

impl Default for DatabaseTestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(30),
        }
    }
}

impl Default for ServiceTestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(30),
        }
    }
}

impl Default for ApiTestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(30),
        }
    }
}

impl IntegrationTestConfig {
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
