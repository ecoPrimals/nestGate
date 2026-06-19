// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Configuration for ZFS security, encryption metadata tracking, and access control.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

fn nestgate_config_dir() -> PathBuf {
    if let Ok(v) = std::env::var("NESTGATE_CONFIG_DIR") {
        return PathBuf::from(v);
    }
    if let Ok(v) = std::env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(v).join("nestgate");
    }
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".config").join("nestgate");
    }
    PathBuf::from("/etc/nestgate")
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Security
pub struct SecurityConfig {
    /// Enable encryption metadata tracking (encryption handled by external providers)
    /// Note: `NestGate` tracks encryption state but does not perform encryption itself
    pub enable_encryption: bool,
    /// Default encryption algorithm hint for external providers (like security modules)
    pub encryption_algorithm: String,
    /// Key management settings
    pub key_management: KeyManagementConfig,
    /// Access control settings
    pub access_control: AccessControlConfig,
}
/// Key management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::KeyManagementConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::KeyManagementConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for `KeyManagement`
pub struct KeyManagementConfig {
    /// Key storage location
    pub key_storage_path: PathBuf,
    /// Key rotation interval in days
    pub rotation_interval_days: u32,
    /// Backup key locations
    pub backup_locations: Vec<PathBuf>,
}
/// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AccessControlConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AccessControlConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for `AccessControl`
pub struct AccessControlConfig {
    /// Default permissions for new datasets
    pub default_permissions: String,
    /// User access rules
    pub user_rules: HashMap<String, Vec<String>>,
    /// Group access rules
    pub group_rules: HashMap<String, Vec<String>>,
}
impl Default for SecurityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_encryption: false,
            encryption_algorithm: String::from("aes-256-gcm"),
            key_management: KeyManagementConfig::default(),
            access_control: AccessControlConfig::default(),
        }
    }
}

impl Default for KeyManagementConfig {
    fn default() -> Self {
        Self {
            key_storage_path: nestgate_config_dir().join("zfs").join("keys"),
            rotation_interval_days: 90,
            backup_locations: vec![],
        }
    }
}

impl KeyManagementConfig {
    /// Create production-optimized key management configuration
    #[must_use]
    pub fn production() -> Self {
        Self {
            key_storage_path: nestgate_config_dir()
                .join("zfs")
                .join("keys")
                .join("production"),
            rotation_interval_days: 30,
            backup_locations: vec![
                PathBuf::from("/backup/nestgate/keys"),
                PathBuf::from("/offsite/nestgate/keys"),
            ],
        }
    }
}

impl Default for AccessControlConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            default_permissions: String::from("755"),
            user_rules: HashMap::new(),
            group_rules: HashMap::new(),
        }
    }
}

impl AccessControlConfig {
    /// Create production-optimized access control configuration
    pub fn production() -> Self {
        let mut user_rules = HashMap::new();
        user_rules.insert(String::from("zfs-admin"), vec![String::from("all")]);
        user_rules.insert(String::from("backup"), vec![String::from("read")]);

        let mut group_rules = HashMap::new();
        group_rules.insert(
            String::from("zfs-operators"),
            [
                String::from("read"),
                String::from("create"),
                String::from("snapshot"),
            ]
            .iter()
            .map(std::string::ToString::to_string)
            .collect(),
        );

        Self {
            default_permissions: String::from("750"),
            user_rules,
            group_rules,
        }
    }
}

impl SecurityConfig {
    /// Create production-optimized security configuration
    #[must_use]
    pub fn production() -> Self {
        Self {
            enable_encryption: true,
            encryption_algorithm: String::from("aes-256-gcm"),
            key_management: KeyManagementConfig::production(),
            access_control: AccessControlConfig::production(),
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Accesscontrolconfigcanonical
pub type AccessControlConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AccessControlConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Keymanagementconfigcanonical
pub type KeyManagementConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using KeyManagementConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn security_config_default_production_and_json() {
        let d = SecurityConfig::default();
        assert!(!d.enable_encryption);
        let p = SecurityConfig::production();
        assert!(p.enable_encryption);
        let json = serde_json::to_string(&d).expect("serialize");
        let back: SecurityConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.encryption_algorithm, d.encryption_algorithm);
    }

    #[test]
    fn key_and_access_control_defaults_and_production() {
        let km = KeyManagementConfig::default();
        assert_eq!(km.rotation_interval_days, 90);
        let kprod = KeyManagementConfig::production();
        assert!(
            kprod
                .key_storage_path
                .to_string_lossy()
                .contains("production")
        );

        let ac = AccessControlConfig::default();
        assert_eq!(ac.default_permissions, "755");
        let acp = AccessControlConfig::production();
        assert!(acp.user_rules.contains_key("zfs-admin"));

        let json = serde_json::to_string(&km).unwrap();
        let _: KeyManagementConfig = serde_json::from_str(&json).unwrap();
        let ac_json = serde_json::to_string(&ac).unwrap();
        let _: AccessControlConfig = serde_json::from_str(&ac_json).unwrap();
    }

    #[test]
    fn key_management_path_roundtrip() {
        let km = KeyManagementConfig {
            key_storage_path: PathBuf::from("/tmp/nestgate-test-keys"),
            rotation_interval_days: 7,
            backup_locations: vec![PathBuf::from("/backup")],
        };
        let json = serde_json::to_string(&km).unwrap();
        let back: KeyManagementConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.rotation_interval_days, 7);
    }
}
