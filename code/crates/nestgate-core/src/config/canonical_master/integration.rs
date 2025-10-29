// Integration configuration structures

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub enabled: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalAdapterConfig {
    pub enabled: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalsConfig {
    pub intelligence_ai_endpoint: String,
    pub security_security_endpoint: String,
    pub ecosystem_integration_enabled: bool,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for UniversalAdapterConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for PrimalsConfig {
    fn default() -> Self {
        Self {
            intelligence_ai_endpoint: "http://localhost:".to_string() + &env::var("NESTGATE_API_PORT").unwrap_or_else(|_| "8080".to_string()).to_string(),
            security_security_endpoint: "http://localhost:".to_string() + &env::var("NESTGATE_SECURITY_PORT").unwrap_or_else(|_| "8081".to_string()).to_string(),
            ecosystem_integration_enabled: true,
        }
    }
}
