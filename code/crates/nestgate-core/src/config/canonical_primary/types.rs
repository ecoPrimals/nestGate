// Common types and enums used across configuration modules

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigMetadata {
    pub version: String,
    pub created_at: String,
    pub updated_at: String,
    pub checksum: String,
}

impl Default for ConfigMetadata {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            created_at: "2025-02-01T00:00:00Z".to_string(),
            updated_at: "2025-02-01T00:00:00Z".to_string(),
            checksum: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentEnvironment {
    Development,
    Testing,
    Staging,
    Production,
}

impl Default for DeploymentEnvironment {
    fn default() -> Self {
        Self::Development
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

/// Configuration for type system and validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypesConfig {
    /// Enable strict type checking
    pub strict_typing: bool,
    /// Type validation rules
    pub validation_rules: Vec<String>,
    /// Custom type definitions
    pub custom_types: std::collections::HashMap<String, String>,
}
impl Default for TypesConfig {
    fn default() -> Self {
        Self {
            strict_typing: true,
            validation_rules: vec![
                "required_fields".to_string(),
                "type_compatibility".to_string(),
            ],
            custom_types: std::collections::HashMap::new(),
        }
    }
}
