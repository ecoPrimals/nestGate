// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// **SECURITY CONFIGURATION**
///
/// Security and authentication configuration types.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Security
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
/// Configuration for Auth
pub struct AuthConfig {
    /// Enable authentication
    pub enabled: bool,
    /// Auth provider
    pub provider: String,
    /// Auth settings
    pub auth_settings: HashMap<String, serde_json::Value>,
}
impl Default for SecurityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            auth: AuthConfig::default(),
            security_settings: HashMap::new(),
        }
    }
}

impl Default for AuthConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "none".to_string(),
            auth_settings: HashMap::new(),
        }
    }
}
