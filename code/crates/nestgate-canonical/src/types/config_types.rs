//! Configuration Type Definitions
//!
//! Canonical configuration types for services, networks, storage, security, and performance.

use serde::{Deserialize, Serialize};

/// Canonical Configuration - Top-level configuration container
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(deprecated)] // MIGRATION: Using deprecated types until v0.12.0 - will migrate to CanonicalNetworkConfig
pub struct CanonicalConfig {
    pub service_name: String,
    pub version: String,
    pub environment: String,
    pub debug_mode: bool,
    pub log_level: String,
    pub network: NetworkConfig,
    pub storage: StorageConfig,
    pub security: SecurityConfig,
    pub performance: PerformanceConfig,
}

/// Canonical Network Configuration
///
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `nestgate_core::config::canonical_primary::domains::network`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use nestgate_canonical::types::NetworkConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// ```
///
/// **Timeline**: This type will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.9.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub bind_endpoint: String,
    pub port: u16,
    pub timeout_seconds: u64,
    pub max_connections: u32,
    pub enable_tls: bool,
    pub websocket_port: Option<u16>,
}

/// Canonical Storage Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub backend_type: String,
    pub data_directory: String,
    pub cache_size_mb: u64,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub backup_enabled: bool,
}

/// Canonical Security Configuration
///
/// **⚠️ DEPRECATED**: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use nestgate_canonical::types::SecurityConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// ```
///
/// **Timeline**: This type will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub authentication_enabled: bool,
    pub authorization_enabled: bool,
    pub session_timeout_minutes: u64,
    pub max_login_attempts: u32,
    pub password_policy: PasswordPolicy,
}

/// Canonical Password Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)] // Policy flags are semantically correct here
pub struct PasswordPolicy {
    pub min_length: u32,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special: bool,
}

/// Canonical Performance Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub thread_pool_size: u32,
    pub buffer_size_kb: u32,
    pub batch_size: u32,
    pub enable_metrics: bool,
    pub metrics_interval_seconds: u64,
}

/// Authentication configuration for canonical modernization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry_minutes: u64,
    pub refresh_token_expiry_days: u32,
    pub require_email_verification: bool,
    pub enable_two_factor: bool,
}

// ==================== Default Implementations ====================

#[allow(deprecated)] // MIGRATION: Use CanonicalNetworkConfig in next major version
impl Default for CanonicalConfig {
    fn default() -> Self {
        Self {
            service_name: "nestgate".to_string(),
            version: "2.0.0".to_string(),
            environment: "production".to_string(),
            debug_mode: false,
            log_level: "info".to_string(),
            network: NetworkConfig::default(),
            storage: StorageConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

#[allow(deprecated)] // MIGRATION: Use CanonicalNetworkConfig in next major version
impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_endpoint: std::env::var("NESTGATE_BIND_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("NESTGATE_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            timeout_seconds: 30,
            max_connections: 1000,
            enable_tls: false,
            websocket_port: None,
        }
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend_type: "local".to_string(),
            data_directory: std::env::var("NESTGATE_DATA_DIR")
                .unwrap_or_else(|_| "./data".to_string()),
            cache_size_mb: 512,
            compression_enabled: true,
            encryption_enabled: false,
            backup_enabled: true,
        }
    }
}

#[allow(deprecated)]
impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            authentication_enabled: true,
            authorization_enabled: true,
            session_timeout_minutes: 60,
            max_login_attempts: 5,
            password_policy: PasswordPolicy::default(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            thread_pool_size: u32::try_from(num_cpus::get()).unwrap_or(4),
            buffer_size_kb: 1024,
            batch_size: 100,
            enable_metrics: true,
            metrics_interval_seconds: 60,
        }
    }
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special: true,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "change-me-in-production".to_string()),
            token_expiry_minutes: 60,
            refresh_token_expiry_days: 30,
            require_email_verification: true,
            enable_two_factor: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_config_default() {
        let config = CanonicalConfig::default();
        assert_eq!(config.service_name, "nestgate");
        assert_eq!(config.version, "2.0.0");
        assert_eq!(config.environment, "production");
        assert!(!config.debug_mode);
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_canonical_config_serialization() {
        let config = CanonicalConfig::default();
        let json = serde_json::to_string(&config).expect("Failed to serialize");
        let deserialized: CanonicalConfig =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(config.service_name, deserialized.service_name);
        assert_eq!(config.version, deserialized.version);
    }

    #[test]
    #[allow(deprecated)]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_connections, 1000);
        assert!(!config.enable_tls);
        assert!(config.websocket_port.is_none());
    }

    #[test]
    fn test_storage_config_default() {
        let config = StorageConfig::default();
        assert_eq!(config.backend_type, "local");
        assert_eq!(config.cache_size_mb, 512);
        assert!(config.compression_enabled);
        assert!(!config.encryption_enabled);
        assert!(config.backup_enabled);
    }

    #[test]
    #[allow(deprecated)]
    fn test_security_config_default() {
        let config = SecurityConfig::default();
        assert!(config.authentication_enabled);
        assert!(config.authorization_enabled);
        assert_eq!(config.session_timeout_minutes, 60);
        assert_eq!(config.max_login_attempts, 5);
    }

    #[test]
    fn test_password_policy_default() {
        let policy = PasswordPolicy::default();
        assert_eq!(policy.min_length, 8);
        assert!(policy.require_uppercase);
        assert!(policy.require_lowercase);
        assert!(policy.require_numbers);
        assert!(policy.require_special);
    }

    #[test]
    fn test_performance_config_default() {
        let config = PerformanceConfig::default();
        assert!(config.thread_pool_size >= 1); // At least 1 CPU
        assert_eq!(config.buffer_size_kb, 1024);
        assert_eq!(config.batch_size, 100);
        assert!(config.enable_metrics);
        assert_eq!(config.metrics_interval_seconds, 60);
    }

    #[test]
    fn test_auth_config_default() {
        let config = AuthConfig::default();
        assert_eq!(config.token_expiry_minutes, 60);
        assert_eq!(config.refresh_token_expiry_days, 30);
        assert!(config.require_email_verification);
        assert!(!config.enable_two_factor);
    }

    #[test]
    fn test_storage_config_custom_values() {
        let config = StorageConfig {
            backend_type: "zfs".to_string(),
            data_directory: "/tank/data".to_string(),
            cache_size_mb: 2048,
            compression_enabled: false,
            encryption_enabled: true,
            backup_enabled: false,
        };

        assert_eq!(config.backend_type, "zfs");
        assert_eq!(config.data_directory, "/tank/data");
        assert_eq!(config.cache_size_mb, 2048);
        assert!(!config.compression_enabled);
        assert!(config.encryption_enabled);
        assert!(!config.backup_enabled);
    }

    #[test]
    fn test_password_policy_custom_values() {
        let policy = PasswordPolicy {
            min_length: 12,
            require_uppercase: false,
            require_lowercase: true,
            require_numbers: true,
            require_special: false,
        };

        assert_eq!(policy.min_length, 12);
        assert!(!policy.require_uppercase);
        assert!(policy.require_lowercase);
        assert!(policy.require_numbers);
        assert!(!policy.require_special);
    }

    #[test]
    fn test_auth_config_serialization() {
        let config = AuthConfig {
            jwt_secret: "test-secret".to_string(),
            token_expiry_minutes: 120,
            refresh_token_expiry_days: 90,
            require_email_verification: false,
            enable_two_factor: true,
        };

        let json = serde_json::to_string(&config).expect("Failed to serialize");
        let deserialized: AuthConfig = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(config.jwt_secret, deserialized.jwt_secret);
        assert_eq!(
            config.token_expiry_minutes,
            deserialized.token_expiry_minutes
        );
        assert_eq!(
            config.refresh_token_expiry_days,
            deserialized.refresh_token_expiry_days
        );
        assert_eq!(
            config.require_email_verification,
            deserialized.require_email_verification
        );
        assert_eq!(config.enable_two_factor, deserialized.enable_two_factor);
    }
}
