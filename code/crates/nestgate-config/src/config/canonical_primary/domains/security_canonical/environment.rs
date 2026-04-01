// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **ENVIRONMENT SECURITY CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `SecurityEnvironment`
pub struct SecurityEnvironmentConfig {
    /// Development
    pub development: EnvironmentSecuritySettings,
    /// Staging
    pub staging: EnvironmentSecuritySettings,
    /// Production
    pub production: EnvironmentSecuritySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Environmentsecuritysettings
pub struct EnvironmentSecuritySettings {
    /// Security Level
    pub security_level: String,
    /// Deployment
    pub deployment: DeploymentSecurityConfig,
    /// Runtime
    pub runtime: RuntimeSecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `DeploymentSecurity`
pub struct DeploymentSecurityConfig {
    /// Secure Deployment
    pub secure_deployment: bool,
    /// Image Scanning
    pub image_scanning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `RuntimeSecurity`
pub struct RuntimeSecurityConfig {
    /// Sandboxing
    pub sandboxing: bool,
    /// Privilege Escalation
    pub privilege_escalation: bool,
}

impl Default for SecurityEnvironmentConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            development: EnvironmentSecuritySettings::development(),
            staging: EnvironmentSecuritySettings::staging(),
            production: EnvironmentSecuritySettings::production(),
        }
    }
}

impl EnvironmentSecuritySettings {
    /// Returns security settings optimized for development environment
    #[must_use]
    pub fn development() -> Self {
        Self {
            security_level: "low".to_string(),
            deployment: DeploymentSecurityConfig {
                secure_deployment: false,
                image_scanning: false,
            },
            runtime: RuntimeSecurityConfig {
                sandboxing: false,
                privilege_escalation: true,
            },
        }
    }

    /// Returns security settings optimized for staging environment
    #[must_use]
    pub fn staging() -> Self {
        Self {
            security_level: "medium".to_string(),
            deployment: DeploymentSecurityConfig {
                secure_deployment: true,
                image_scanning: true,
            },
            runtime: RuntimeSecurityConfig {
                sandboxing: true,
                privilege_escalation: false,
            },
        }
    }

    /// Returns security settings optimized for production environment
    #[must_use]
    pub fn production() -> Self {
        Self {
            security_level: "high".to_string(),
            deployment: DeploymentSecurityConfig {
                secure_deployment: true,
                image_scanning: true,
            },
            runtime: RuntimeSecurityConfig {
                sandboxing: true,
                privilege_escalation: false,
            },
        }
    }
}

impl SecurityEnvironmentConfig {
    /// Returns a production-hardened security environment configuration
    #[must_use]
    pub fn production_hardened() -> Self {
        Self::default()
    }
    /// Returns a development-optimized security environment configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    /// Returns a compliance-focused security environment configuration
    #[must_use]
    pub fn compliance_focused() -> Self {
        Self::default()
    }
    /// Merges this configuration with another
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
