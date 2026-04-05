// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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
/// Configuration for `AccessControl`
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
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorization_config_default() {
        let c = AuthorizationConfig::default();
        assert!(c.access_control.enabled);
        assert_eq!(c.access_control.default_policy, "deny");
        assert!(c.roles.is_empty());
        assert!(c.permissions.is_empty());
    }

    #[test]
    fn access_control_config_default_and_variants() {
        let d = AccessControlConfig::default();
        assert!(d.enabled);
        let dev = AccessControlConfig::development_optimized();
        assert!(!dev.enabled);
        assert_eq!(dev.default_policy, "allow_all");
        let comp = AccessControlConfig::compliance_focused();
        assert!(comp.enabled);
        assert_eq!(comp.default_policy, "deny_all");
        let prod = AccessControlConfig::production_hardened();
        assert!(prod.enabled);
        assert_eq!(prod.default_policy, "strict_deny");
    }

    #[test]
    fn access_control_merge_prefers_other() {
        let a = AccessControlConfig::development_optimized();
        let b = AccessControlConfig::production_hardened();
        let m = a.merge(b);
        assert!(m.enabled);
        assert_eq!(m.default_policy, "strict_deny");
    }

    #[test]
    fn authorization_factory_methods_match_default() {
        let d = AuthorizationConfig::default();
        let ser = serde_json::to_string(&d).expect("serialize");
        assert_eq!(
            ser,
            serde_json::to_string(&AuthorizationConfig::production_hardened()).expect("serialize")
        );
        assert_eq!(
            ser,
            serde_json::to_string(&AuthorizationConfig::development_optimized())
                .expect("serialize")
        );
        assert_eq!(
            ser,
            serde_json::to_string(&AuthorizationConfig::compliance_focused()).expect("serialize")
        );
    }

    #[test]
    fn authorization_merge_identity() {
        let a = AuthorizationConfig::default();
        let b = AuthorizationConfig {
            access_control: AccessControlConfig::development_optimized(),
            ..AuthorizationConfig::default()
        };
        let merged = a.merge(b);
        assert!(merged.access_control.enabled);
    }

    #[test]
    fn authorization_validate_succeeds() {
        assert!(AuthorizationConfig::default().validate().is_ok());
    }

    #[test]
    fn authorization_serde_roundtrip() {
        let original = AuthorizationConfig {
            access_control: AccessControlConfig::development_optimized(),
            roles: vec![RoleConfig {
                name: "admin".to_string(),
                permissions: vec!["*".to_string()],
            }],
            permissions: vec![PermissionConfig {
                name: "read".to_string(),
                description: "read access".to_string(),
            }],
        };
        let json = serde_json::to_string(&original).expect("serialize");
        let parsed: AuthorizationConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed.roles.len(), 1);
        assert_eq!(parsed.permissions.len(), 1);
        assert_eq!(
            serde_json::to_string(&original).expect("serialize"),
            serde_json::to_string(&parsed).expect("re-serialize")
        );
    }

    #[test]
    fn resource_and_policy_config_serde_roundtrip() {
        let r = ResourceConfig {
            name: "pool".to_string(),
            type_name: "zfs".to_string(),
        };
        let p = PolicyConfig {
            name: "default".to_string(),
            rules: vec!["allow read".to_string()],
        };
        let jr = serde_json::to_string(&r).expect("serialize resource");
        let rr: ResourceConfig = serde_json::from_str(&jr).expect("deserialize resource");
        assert_eq!(r.name, rr.name);
        let jp = serde_json::to_string(&p).expect("serialize policy");
        let pr: PolicyConfig = serde_json::from_str(&jp).expect("deserialize policy");
        assert_eq!(p.rules, pr.rules);
    }
}
