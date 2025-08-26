/// Security and access control configuration - extracted from monolithic config
/// Handles access control, encryption, audit logging, authentication, and authorization
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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
    /// IP whitelist
    pub ip_whitelist: Vec<String>,
    /// IP blacklist
    pub ip_blacklist: Vec<String>,
    /// User whitelist
    pub user_whitelist: Vec<String>,
    /// User blacklist
    pub user_blacklist: Vec<String>,
    /// Process whitelist
    pub process_whitelist: Vec<String>,
    /// Process blacklist
    pub process_blacklist: Vec<String>,
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
    fn default() -> Self {
        Self {
            enabled: true,
            enable_access_logging: false,
            allowed_operations: vec![
                "read".to_string(),
                "write".to_string(),
                "create".to_string(),
                "delete".to_string(),
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
    fn default() -> Self {
        Self {
            enabled: false,
            default_policy: "allow".to_string(),
            rules: Vec::new(),
            ip_whitelist: Vec::new(),
            ip_blacklist: Vec::new(),
            user_whitelist: Vec::new(),
            user_blacklist: Vec::new(),
            process_whitelist: Vec::new(),
            process_blacklist: Vec::new(),
        }
    }
}

impl Default for EncryptionSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: "AES-256-GCM".to_string(),
            key_management: KeyManagementSettings::default(),
            encrypt_at_rest: false,
            encrypt_in_transit: false,
        }
    }
}

impl Default for KeyManagementSettings {
    fn default() -> Self {
        Self {
            provider: "local".to_string(),
            rotation_interval: Duration::from_secs(86400 * 30), // 30 days
            kdf: "PBKDF2".to_string(),
            key_size: 256,
            storage_location: "/etc/nestgate/keys".to_string(),
        }
    }
}

impl Default for AuditLoggingSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            log_level: "info".to_string(),
            log_file: "/var/log/nestgate/audit.log".to_string(),
            rotation: LogRotationSettings::default(),
            include_sensitive_data: false,
            retention_period: Duration::from_secs(86400 * 90), // 90 days
        }
    }
}

impl Default for LogRotationSettings {
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
    fn default() -> Self {
        Self {
            enabled: false,
            methods: vec!["password".to_string()],
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
            model: "rbac".to_string(),
            default_permissions: vec!["read".to_string()],
            cache_timeout: Duration::from_secs(300),
            roles: HashMap::new(),
        }
    }
}
