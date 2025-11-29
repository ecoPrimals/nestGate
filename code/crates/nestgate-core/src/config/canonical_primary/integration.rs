// Integration configuration structures

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Integration
pub struct IntegrationConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for UniversalAdapter
pub struct UniversalAdapterConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Primals
pub struct PrimalsConfig {
    /// Intelligence Ai Endpoint
    pub intelligence_ai_endpoint: String,
    /// Security Security Endpoint
    pub security_security_endpoint: String,
    /// Ecosystem Integration Enabled
    pub ecosystem_integration_enabled: bool,
}

impl Default for IntegrationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for UniversalAdapterConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for PrimalsConfig {
    /// Returns the default instance
    fn default() -> Self {
        use crate::constants::hardcoding::{addresses, ports};
        use std::env;
        
        let intelligence_endpoint = env::var("NESTGATE_AI_ENDPOINT")
            .unwrap_or_else(|_| {
                format!(
                    "http://{}:{}",
                    addresses::LOCALHOST_NAME,
                    env::var("NESTGATE_API_PORT")
                        .unwrap_or_else(|_| ports::HTTP_DEFAULT.to_string())
                )
            });
            
        let security_endpoint = env::var("NESTGATE_SECURITY_ENDPOINT")
            .unwrap_or_else(|_| {
                format!(
                    "http://{}:{}",
                    addresses::LOCALHOST_NAME,
                    env::var("NESTGATE_SECURITY_PORT")
                        .unwrap_or_else(|_| ports::HEALTH_CHECK.to_string())
                )
            });
        
        Self {
            intelligence_ai_endpoint: intelligence_endpoint,
            security_security_endpoint: security_endpoint,
            ecosystem_integration_enabled: true,
        }
    }
}
