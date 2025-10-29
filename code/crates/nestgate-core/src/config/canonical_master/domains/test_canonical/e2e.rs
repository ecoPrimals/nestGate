// **END-TO-END TEST CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct E2eTestConfig {
    pub browser: BrowserTestConfig,
    pub journey: UserJourneyConfig,
    pub scenario: ScenarioTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTestConfig {
    pub enabled: bool,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserJourneyConfig {
    pub enabled: bool,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioTestConfig {
    pub enabled: bool,
    pub timeout: Duration,
}

impl Default for BrowserTestConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(60),
        }
    }
}

impl Default for UserJourneyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(60),
        }
    }
}

impl Default for ScenarioTestConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(60),
        }
    }
}

impl E2eTestConfig {
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
