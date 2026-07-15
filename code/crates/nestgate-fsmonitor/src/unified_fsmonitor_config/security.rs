// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Security and access control configuration - extracted from monolithic config
/// Handles access control, encryption, audit logging, authentication, and authorization
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// XDG-compliant config directory for key storage.
///
/// Checks `$NESTGATE_CONFIG_DIR`, then `$XDG_CONFIG_HOME/nestgate/keys`,
/// then `$HOME/.config/nestgate/keys`, and falls back to FHS.
fn default_key_storage_path() -> String {
    if let Ok(dir) = std::env::var("NESTGATE_CONFIG_DIR") {
        return PathBuf::from(dir)
            .join("keys")
            .to_string_lossy()
            .into_owned();
    }
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg)
            .join("nestgate/keys")
            .to_string_lossy()
            .into_owned();
    }
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home)
            .join(".config/nestgate/keys")
            .to_string_lossy()
            .into_owned();
    }
    "/etc/nestgate/keys".into()
}

/// XDG-compliant state directory for audit logs.
///
/// Checks `$NESTGATE_LOG_DIR`, then `$XDG_STATE_HOME/nestgate`,
/// then `$HOME/.local/state/nestgate`, and falls back to FHS.
fn default_audit_log_path() -> String {
    if let Ok(dir) = std::env::var("NESTGATE_LOG_DIR") {
        return PathBuf::from(dir)
            .join("audit.log")
            .to_string_lossy()
            .into_owned();
    }
    if let Ok(xdg) = std::env::var("XDG_STATE_HOME") {
        return PathBuf::from(xdg)
            .join("nestgate/audit.log")
            .to_string_lossy()
            .into_owned();
    }
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home)
            .join(".local/state/nestgate/audit.log")
            .to_string_lossy()
            .into_owned();
    }
    "/var/log/nestgate/audit.log".into()
}
/// Security and access control settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsMonitorSecuritySettings {
    /// Enable security features
    pub enabled: bool,
    /// Enable access logging
    pub enable_access_logging: bool,
    /// Allowed operations
    pub allowed_operations: Vec<String>,
    /// Access control settings
    pub access_control: AccessControlSettings,
    /// Encryption settings
    pub encryption: EncryptionSettings,
    /// Audit logging settings
    pub audit_logging: AuditLoggingSettings,
    /// Authentication settings
    pub authentication: AuthenticationSettings,
    /// Authorization settings
    pub authorization: AuthorizationSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlSettings {
    /// Enable access control
    pub enabled: bool,
    /// Default access policy (allow/deny)
    pub default_policy: String,
    /// Access control rules
    pub rules: Vec<String>,
    /// IP allowlist
    pub ip_allowlist: Vec<String>,
    /// IP denylist
    pub ip_denylist: Vec<String>,
    /// User allowlist
    pub user_allowlist: Vec<String>,
    /// User denylist
    pub user_denylist: Vec<String>,
    /// Process allowlist
    pub process_allowlist: Vec<String>,
    /// Process denylist
    pub process_denylist: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionSettings {
    /// Enable encryption
    pub enabled: bool,
    /// Encryption algorithm
    pub algorithm: String,
    /// Key management settings
    pub key_management: KeyManagementSettings,
    /// Encrypt data at rest
    pub encrypt_at_rest: bool,
    /// Encrypt data in transit
    pub encrypt_in_transit: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementSettings {
    /// Key provider (local, hsm, cloud)
    pub provider: String,
    /// Key rotation interval
    pub rotation_interval: Duration,
    /// Key derivation function
    pub kdf: String,
    /// Key size in bits
    pub key_size: u32,
    /// Key storage location
    pub storage_location: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLoggingSettings {
    /// Enable audit logging
    pub enabled: bool,
    /// Audit log level
    pub log_level: String,
    /// Log file path
    pub log_file: String,
    /// Log rotation settings
    pub rotation: LogRotationSettings,
    /// Include sensitive data in logs
    pub include_sensitive_data: bool,
    /// Log retention period
    pub retention_period: Duration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationSettings {
    /// Enable log rotation
    pub enabled: bool,
    /// Maximum log file size (bytes)
    pub max_size: u64,
    /// Number of log files to keep
    pub keep_files: u32,
    /// Rotation interval
    pub interval: Duration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationSettings {
    /// Enable authentication
    pub enabled: bool,
    /// Authentication methods
    pub methods: Vec<String>,
    /// Session timeout
    pub session_timeout: Duration,
    /// Maximum failed attempts
    pub max_failed_attempts: u32,
    /// Account lockout duration
    pub lockout_duration: Duration,
    /// Password policy
    pub password_policy: HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationSettings {
    /// Enable authorization
    pub enabled: bool,
    /// Authorization model (rbac, abac, acl)
    pub model: String,
    /// Default permissions
    pub default_permissions: Vec<String>,
    /// Permission cache timeout
    pub cache_timeout: Duration,
    /// Role definitions
    pub roles: HashMap<String, Vec<String>>,
}
impl Default for FsMonitorSecuritySettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            enable_access_logging: false,
            allowed_operations: vec![
                "read".into(),
                "write".into(),
                "create".into(),
                "delete".into(),
            ],
            access_control: AccessControlSettings::default(),
            encryption: EncryptionSettings::default(),
            audit_logging: AuditLoggingSettings::default(),
            authentication: AuthenticationSettings::default(),
            authorization: AuthorizationSettings::default(),
        }
    }
}

impl Default for AccessControlSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            default_policy: "allow".into(),
            rules: Vec::new(),
            ip_allowlist: Vec::new(),
            ip_denylist: Vec::new(),
            user_allowlist: Vec::new(),
            user_denylist: Vec::new(),
            process_allowlist: Vec::new(),
            process_denylist: Vec::new(),
        }
    }
}

impl Default for EncryptionSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: "AES-256-GCM".into(),
            key_management: KeyManagementSettings::default(),
            encrypt_at_rest: false,
            encrypt_in_transit: false,
        }
    }
}

impl Default for KeyManagementSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            provider: "local".into(),
            rotation_interval: Duration::from_secs(86400 * 30), // 30 days
            kdf: "PBKDF2".into(),
            key_size: 256,
            storage_location: default_key_storage_path(),
        }
    }
}

impl Default for AuditLoggingSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            log_level: "info".into(),
            log_file: default_audit_log_path(),
            rotation: LogRotationSettings::default(),
            include_sensitive_data: false,
            retention_period: Duration::from_secs(86400 * 90), // 90 days
        }
    }
}

impl Default for LogRotationSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            max_size: 100 * 1024 * 1024, // 100MB
            keep_files: 10,
            interval: Duration::from_secs(86400), // Daily
        }
    }
}

impl Default for AuthenticationSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            methods: vec!["password".into()],
            session_timeout: Duration::from_secs(3600), // 1 hour
            max_failed_attempts: 5,
            lockout_duration: Duration::from_secs(300), // 5 minutes
            password_policy: HashMap::new(),
        }
    }
}

impl Default for AuthorizationSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            model: "rbac".into(),
            default_permissions: vec!["read".into()],
            cache_timeout: Duration::from_secs(300),
            roles: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn security_disabled_by_default() {
        let settings = FsMonitorSecuritySettings::default();
        assert!(!settings.enabled);
    }

    #[test]
    fn audit_log_path_is_not_tmp() {
        let settings = AuditLoggingSettings::default();
        assert!(
            !settings.log_file.starts_with("/tmp"),
            "audit log must not default to /tmp — got {:?}",
            settings.log_file
        );
    }

    #[test]
    fn key_storage_not_tmp() {
        let settings = EncryptionSettings::default();
        assert!(
            !settings.storage_location.starts_with("/tmp"),
            "key storage must not default to /tmp — got {:?}",
            settings.storage_location
        );
    }

    #[test]
    fn authorization_defaults_to_rbac() {
        let settings = AuthorizationSettings::default();
        assert_eq!(settings.model, "rbac");
        assert!(settings.default_permissions.contains(&String::from("read")));
    }

    #[test]
    fn audit_log_resolves_nestgate_log_dir_env() {
        temp_env::with_var("NESTGATE_LOG_DIR", Some("/custom/logs"), || {
            let path = default_audit_log_path();
            assert_eq!(path, "/custom/logs/audit.log");
        });
    }

    #[test]
    fn key_storage_resolves_nestgate_config_dir_env() {
        temp_env::with_var("NESTGATE_CONFIG_DIR", Some("/custom/config"), || {
            let path = default_key_storage_path();
            assert_eq!(path, "/custom/config/keys");
        });
    }

    #[test]
    fn security_serialization_roundtrip() {
        let original = FsMonitorSecuritySettings::default();
        let json = serde_json::to_string(&original).expect("serialize");
        let deserialized: FsMonitorSecuritySettings =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original.enabled, deserialized.enabled);
    }
}
