//! Security Configuration Module
//!
//! Configuration for ZFS security, encryption metadata tracking, and access control.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable encryption metadata tracking (encryption handled by external providers)
    /// Note: NestGate tracks encryption state but does not perform encryption itself
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
pub struct AccessControlConfig {
    /// Default permissions for new datasets
    pub default_permissions: String,
    /// User access rules
    pub user_rules: HashMap<String, Vec<String>>,
    /// Group access rules
    pub group_rules: HashMap<String, Vec<String>>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_encryption: false,
            encryption_algorithm: "aes-256-gcm".to_string(),
            key_management: KeyManagementConfig::default(),
            access_control: AccessControlConfig::default(),
        }
    }
}

impl Default for KeyManagementConfig {
    fn default() -> Self {
        Self {
            key_storage_path: PathBuf::from("/etc/nestgate/zfs/keys"),
            rotation_interval_days: 90,
            backup_locations: vec![],
        }
    }
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            default_permissions: "755".to_string(),
            user_rules: HashMap::new(),
            group_rules: HashMap::new(),
        }
    }
}
