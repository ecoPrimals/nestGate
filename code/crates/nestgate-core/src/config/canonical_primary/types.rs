// Common types and enums used across configuration modules

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configmetadata
pub struct ConfigMetadata {
    /// Version
    pub version: String,
    /// Timestamp when this was created
    pub created_at: String,
    /// Timestamp of last update
    pub updated_at: String,
    /// Checksum
    pub checksum: String,
}

impl Default for ConfigMetadata {
    /// Returns the default instance
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
/// Deploymentenvironment
pub enum DeploymentEnvironment {
    /// Development
    Development,
    /// Testing
    Testing,
    /// Staging
    Staging,
    /// Production
    Production,
}

impl Default for DeploymentEnvironment {
    /// Returns the default instance
    fn default() -> Self {
        Self::Development
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loglevel
pub enum LogLevel {
    /// Error
    Error,
    /// Warn
    Warn,
    /// Info
    Info,
    /// Debug
    Debug,
    /// Trace
    Trace,
}

impl Default for LogLevel {
    /// Returns the default instance
    fn default() -> Self {
        Self::Info
    }
}

/// Configuration for type system and validation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Types
pub struct TypesConfig {
    /// Enable strict type checking
    pub strict_typing: bool,
    /// Type validation rules
    pub validation_rules: Vec<String>,
    /// Custom type definitions
    pub custom_types: std::collections::HashMap<String, String>,
}
impl Default for TypesConfig {
    /// Returns the default instance
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
