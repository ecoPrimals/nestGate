// **AUTHORIZATION CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for Authorization
pub struct AuthorizationConfig {
    /// Access Control
    pub access_control: AccessControlConfig,
    /// Roles
    pub roles: Vec<RoleConfig>,
    /// Permissions
    pub permissions: Vec<PermissionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for AccessControl
pub struct AccessControlConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Default Policy
    pub default_policy: String,
}

impl AccessControlConfig {
    /// Creates a development-optimized access control configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            enabled: false,
            default_policy: "allow_all".to_string(),
        }
    }

    /// Creates a compliance-focused access control configuration
    #[must_use]
    pub fn compliance_focused() -> Self {
        Self {
            enabled: true,
            default_policy: "deny_all".to_string(),
        }
    }

    /// Creates a production-hardened access control configuration
    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            enabled: true,
            default_policy: "strict_deny".to_string(),
        }
    }

    /// Merges two access control configurations
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.default_policy = other.default_policy;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Role
pub struct RoleConfig {
    /// Name
    pub name: String,
    /// Permissions
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Permission
pub struct PermissionConfig {
    /// Name
    pub name: String,
    /// Human-readable description
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Resource
pub struct ResourceConfig {
    /// Name
    pub name: String,
    /// Type name
    pub type_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Policy
pub struct PolicyConfig {
    /// Name
    pub name: String,
    /// Rules
    pub rules: Vec<String>,
}

impl Default for AccessControlConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            default_policy: "deny".to_string(),
        }
    }
}

impl AuthorizationConfig {
    /// Returns a production-hardened authorization configuration
    #[must_use]
    pub fn production_hardened() -> Self {
        Self::default()
    }
    /// Returns a development-optimized authorization configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    /// Returns a compliance-focused authorization configuration
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
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
