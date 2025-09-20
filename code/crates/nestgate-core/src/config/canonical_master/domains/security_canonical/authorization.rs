// **AUTHORIZATION CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorizationConfig {
    pub access_control: AccessControlConfig,
    pub roles: Vec<RoleConfig>,
    pub permissions: Vec<PermissionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    pub enabled: bool,
    pub default_policy: String,
}

impl AccessControlConfig {
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self {
            enabled: false,
            default_policy: "allow_all".to_string(),
        }
    }

    #[must_use]
    pub const fn compliance_focused() -> Self {
        Self {
            enabled: true,
            default_policy: "deny_all".to_string(),
        }
    }

    #[must_use]
    pub const fn production_hardened() -> Self {
        Self {
            enabled: true,
            default_policy: "strict_deny".to_string(),
        }
    }

    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.default_policy = other.default_policy;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleConfig {
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub name: String,
    pub rules: Vec<String>,
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_policy: "deny".to_string(),
        }
    }
}

impl AuthorizationConfig {
    #[must_use]
    pub const fn production_hardened() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn compliance_focused() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
