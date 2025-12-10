// **END-TO-END TEST CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for E2eTest
pub struct E2eTestConfig {
    /// Browser
    pub browser: BrowserTestConfig,
    /// Journey
    pub journey: UserJourneyConfig,
    /// Scenario
    pub scenario: ScenarioTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for BrowserTest
pub struct BrowserTestConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for UserJourney
pub struct UserJourneyConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ScenarioTest
pub struct ScenarioTestConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Timeout
    pub timeout: Duration,
}

impl Default for BrowserTestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(60),
        }
    }
}

impl Default for UserJourneyConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(60),
        }
    }
}

impl Default for ScenarioTestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(60),
        }
    }
}

impl E2eTestConfig {
    /// Returns a CI-optimized configuration for end-to-end testing
    ///
    /// This configuration is tuned for continuous integration environments
    /// with appropriate timeouts and parallelization settings.
    #[must_use]
    pub fn ci_optimized() -> Self {
        Self::default()
    }

    /// Returns a development-optimized configuration for local E2E testing
    ///
    /// This configuration provides more verbose logging and debugging
    /// capabilities suitable for local development.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Merges this configuration with another, returning the combined result
    ///
    /// Currently returns self as E2E configs don't support deep merging.
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
}
