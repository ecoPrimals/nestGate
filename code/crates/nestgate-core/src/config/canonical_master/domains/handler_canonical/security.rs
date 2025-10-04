// **SECURITY HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHandlerConfig {
    pub authentication: AuthenticationHandlerConfig,
    pub authorization: AuthorizationHandlerConfig,
    pub threat_detection: ThreatDetectionConfig,
    pub audit: AuditHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationHandlerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationHandlerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditHandlerConfig {
    pub enabled: bool,
}

impl Default for SecurityHandlerConfig {
    fn default() -> Self {
        Self {
            authentication: AuthenticationHandlerConfig { enabled: true },
            authorization: AuthorizationHandlerConfig { enabled: true },
            threat_detection: ThreatDetectionConfig { enabled: true },
            audit: AuditHandlerConfig { enabled: true },
        }
    }
}

impl SecurityHandlerConfig {
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
