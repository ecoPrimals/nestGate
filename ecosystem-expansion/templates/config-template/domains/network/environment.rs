//! **NETWORK ENVIRONMENT CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkEnvironmentConfig {
    pub environment_name: String,
    pub overrides: HashMap<String, String>,
    pub feature_flags: HashMap<String, bool>,
}

impl NetworkEnvironmentConfig {
    pub fn development_optimized() -> Self {
        let mut feature_flags = HashMap::new();
        feature_flags.insert("debug_logging".to_string(), true);
        
        Self { 
            environment_name: "development".to_string(), 
            overrides: HashMap::new(),
            feature_flags,
        }
    }

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

    pub fn validate(&self) -> Result<()> { Ok(()) }
    
    pub fn merge(mut self, other: Self) -> Self {
        self.environment_name = other.environment_name;
        self.overrides = other.overrides;
        self.feature_flags = other.feature_flags;
        self
    }
} 