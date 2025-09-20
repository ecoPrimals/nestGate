/// **SECURITY CONFIGURATION**
///
/// Security and authentication configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable security
    pub enabled: bool,
    /// Authentication settings
    pub auth: AuthConfig,
    /// Security settings
    pub security_settings: HashMap<String, serde_json::Value>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Enable authentication
    pub enabled: bool,
    /// Auth provider
    pub provider: String,
    /// Auth settings
    pub auth_settings: HashMap<String, serde_json::Value>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auth: AuthConfig::default(),
            security_settings: HashMap::new(),
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "none".to_string(),
            auth_settings: HashMap::new(),
        }
    }
} 