// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Session storage, encryption, security cookies, and refresh behavior.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Session lifecycle and storage configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session timeout duration
    pub timeout: Duration,
    /// Idle timeout duration
    pub idle_timeout: Duration,
    /// Maximum concurrent sessions per user
    pub max_concurrent_sessions: u32,
    /// Session storage configuration
    pub storage: SessionStorageConfig,
    /// Session security settings
    pub security: SessionSecurityConfig,
    /// Session refresh settings
    pub refresh: SessionRefreshConfig,
}

/// Backing store for session data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStorageConfig {
    /// Session storage type
    pub storage_type: SessionStorageType,
    /// Storage configuration
    pub config: HashMap<String, String>,
    /// Encryption settings
    pub encryption: SessionEncryptionConfig,
}

/// Session storage backend kind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStorageType {
    /// In-memory storage
    Memory,
    /// Redis storage
    Redis,
    /// Database storage
    Database,
    /// File-based storage
    File,
    /// Custom storage
    Custom(String),
}

/// Session payload encryption settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEncryptionConfig {
    /// Enable session encryption
    pub enabled: bool,
    /// Encryption algorithm
    pub algorithm: String,
    /// Key rotation settings
    pub key_rotation: KeyRotationConfig,
}

/// Key rotation policy for session encryption keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationConfig {
    /// Enable automatic key rotation
    pub enabled: bool,
    /// Key rotation interval
    pub interval: Duration,
    /// Number of old keys to keep
    pub keep_old_keys: u32,
}

/// Cookie and binding policy for sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSecurityConfig {
    /// Secure cookie settings
    pub secure_cookies: bool,
    /// HTTP-only cookies
    pub http_only: bool,
    /// `SameSite` cookie policy
    pub same_site: SameSitePolicy,
    /// Session fixation protection
    pub fixation_protection: bool,
    /// IP address validation
    pub ip_validation: bool,
    /// User agent validation
    pub user_agent_validation: bool,
}

/// `SameSite` cookie policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SameSitePolicy {
    /// Strict
    Strict,
    /// Lax
    Lax,
    /// None
    None,
}

/// Automatic session refresh windows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRefreshConfig {
    /// Enable automatic session refresh
    pub enabled: bool,
    /// Refresh threshold (percentage of session lifetime)
    pub threshold: f64,
    /// Refresh window duration
    pub window: Duration,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(24 * 60 * 60),     // 24 hours
            idle_timeout: Duration::from_secs(2 * 60 * 60), // 2 hours
            max_concurrent_sessions: 3,
            storage: SessionStorageConfig::default(),
            security: SessionSecurityConfig::default(),
            refresh: SessionRefreshConfig::default(),
        }
    }
}

impl Default for SessionStorageConfig {
    fn default() -> Self {
        Self {
            storage_type: SessionStorageType::Memory,
            config: HashMap::new(),
            encryption: SessionEncryptionConfig::default(),
        }
    }
}

impl Default for SessionEncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: "AES-256-GCM".to_string(),
            key_rotation: KeyRotationConfig::default(),
        }
    }
}

impl Default for KeyRotationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
            keep_old_keys: 3,
        }
    }
}

impl Default for SessionSecurityConfig {
    fn default() -> Self {
        Self {
            secure_cookies: true,
            http_only: true,
            same_site: SameSitePolicy::Strict,
            fixation_protection: true,
            ip_validation: false,
            user_agent_validation: false,
        }
    }
}

impl Default for SessionRefreshConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threshold: 0.8, // Refresh when 80% of session lifetime has passed
            window: Duration::from_secs(5 * 60), // 5 minutes
        }
    }
}
