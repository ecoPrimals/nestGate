// **ENVIRONMENT SECURITY CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEnvironmentConfig {
    pub development: EnvironmentSecuritySettings,
    pub staging: EnvironmentSecuritySettings,
    pub production: EnvironmentSecuritySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSecuritySettings {
    pub security_level: String,
    pub deployment: DeploymentSecurityConfig,
    pub runtime: RuntimeSecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSecurityConfig {
    pub secure_deployment: bool,
    pub image_scanning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSecurityConfig {
    pub sandboxing: bool,
    pub privilege_escalation: bool,
}

impl Default for SecurityEnvironmentConfig {
    fn default() -> Self {
        Self {
            development: EnvironmentSecuritySettings::development(),
            staging: EnvironmentSecuritySettings::staging(),
            production: EnvironmentSecuritySettings::production(),
        }
    }
}

impl EnvironmentSecuritySettings {
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
    #[must_use]
    pub fn production_hardened() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn compliance_focused() -> Self {
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
