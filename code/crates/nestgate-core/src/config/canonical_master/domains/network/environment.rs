// **NETWORK ENVIRONMENT CONFIGURATION**

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkEnvironmentConfig {
    pub environment_name: String,
    pub overrides: HashMap<String, String>,
    pub feature_flags: HashMap<String, bool>,
}

impl NetworkEnvironmentConfig {
    #[must_use]
    pub fn development_optimized() -> Self {
        let mut feature_flags = HashMap::new();
        feature_flags.insert("debug_logging".to_string(), true);

        Self {
            environment_name: "development".to_string(),
            overrides: HashMap::new(),
            feature_flags,
        }
    }

    #[must_use]
    pub fn production_hardened() -> Self {
        let mut feature_flags = HashMap::new();
        feature_flags.insert("debug_logging".to_string(), false);
        feature_flags.insert("strict_validation".to_string(), true);

        Self {
            environment_name: "production".to_string(),
            overrides: HashMap::new(),
            feature_flags,
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.environment_name = other.environment_name;
        self.overrides = other.overrides;
        self.feature_flags = other.feature_flags;
        self
    }
}
