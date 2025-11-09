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
