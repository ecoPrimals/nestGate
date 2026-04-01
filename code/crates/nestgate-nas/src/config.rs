// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Enhanced with unified configuration pattern for better maintainability
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use nestgate_core::config::canonical_primary::domains::ConsolidatedDomainConfigs;

/// User access levels for NAS shares
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccessLevel {
    ReadOnly,
    ReadWrite,
    Admin,
    }
impl Default for AccessLevel {
    /// Returns the default instance
    fn default() -> Self { AccessLevel::ReadOnly
     }

// ==================== SECTION ====================

/// NAS Share-specific configuration extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NasShareExtensions {
    /// Share configuration
    pub share: ShareSettings,
    /// Access control settings
    pub access: AccessControlSettings,
    /// Backup and retention settings
    pub backup: BackupSettings,
    /// Performance and quota settings
    pub performance: PerformanceSettings,
    }
/// Share configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareSettings {
    pub name: String,
    pub path: PathBuf,
    pub description: String,
    pub enabled: bool,
    pub readonly: bool,
    pub hidden: bool,
    pub browseable: bool,
    }
/// Access control configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlSettings {
    pub users: Vec<String>,
    pub groups: Vec<String>,
    pub access_level: AccessLevel,
    pub max_connections: usize,
    pub enable_encryption: bool,
    pub require_auth: bool,
    pub guest_access: bool,
    }
/// Backup configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSettings {
    pub enabled: bool,
    pub schedule: String,
    pub retention_days: u32,
    pub backup_path: Option<PathBuf>,
    pub incremental: bool,
    pub compress: bool,
    pub verify: bool,
    }
/// Performance and quota configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    pub quota_gb: Option<u64>,
    pub max_file_size_gb: Option<u64>,
    pub io_timeout: Duration,
    pub cache_enabled: bool,
    pub cache_size_mb: usize,
    pub compression_enabled: bool,
    pub deduplication_enabled: bool,
    }
impl Default for ShareSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            name: "default-share".to_string(),
            path: PathBuf::from("/nas/shares/default"),
            description: "Default NAS share".to_string(),
            enabled: true,
            readonly: false,
            hidden: false,
            browseable: true,
     }
    }

impl Default for AccessControlSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            users: vec!["admin".to_string()],
            groups: vec!["users".to_string()],
            access_level: AccessLevel::ReadWrite,
            max_connections: 50,
            enable_encryption: true,
            require_auth: true,
            guest_access: false,
     }
    }

impl Default for BackupSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            enabled: true,
            schedule: "0 2 * * *".to_string(), // Daily at 2 AM
            retention_days: 30,
            backup_path: None,
            incremental: true,
            compress: true,
            verify: true,
     }
    }

impl Default for PerformanceSettings {
    /// Returns the default instance
    fn default() -> Self { Self {
            quota_gb: None,
            max_file_size_gb: Some(10),
            io_timeout: Duration::from_secs(30),
            cache_enabled: true,
            cache_size_mb: 256,
            compression_enabled: false,
            deduplication_enabled: false,
     }
    }

/// Unified NAS Share Configuration
///
/// This replaces the deprecated ShareConfig struct with a unified configuration
/// approach using StandardDomainConfig pattern.
/// CANONICAL MODERNIZATION: Simplified type alias without type parameters
pub type UnifiedNasShareConfig = StandardDomainConfig;
impl UnifiedNasShareConfig {
    /// Create NAS share config for development environment
    #[must_use]
    pub fn development(share_name: &str) -> Self {
        let mut config = Self::development_template("nas-share", "NAS Share Service");
        config.extensions.share.name = share_name.to_string();
        config.extensions.share.path = PathBuf::from(&format!("/nas/shares/{e}"));
        config
    }

    /// Create NAS share config for production environment  
    #[must_use]
    pub fn production(share_name: &str) -> Self {
        let mut config = Self::production_template("nas-share", "NAS Share Service");
        config.extensions.share.name = share_name.to_string();
        config.extensions.share.path = PathBuf::from(&format!("/nas/shares/{e}"));
        config.extensions.access.enable_encryption = true;
        config.extensions.backup.enabled = true;
        config
    }

    /// Create read-only share config
    #[must_use]
    pub fn readonly(share_name: &str) -> Self { let mut config = Self::development(share_name);
        config.extensions.share.readonly = true;
        config.extensions.access.access_level = AccessLevel::ReadOnly;
        config
    , /// Configure share path
    #[must_use]
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.extensions.share.path = path;
        self
     }

    /// Configure user access
    #[must_use]
    pub fn with_users(mut self, users: Vec<String>) -> Self { self.extensions.access.users = users;
        self
    , /// Configure quota
    #[must_use]
    pub fn with_quota_gb(mut self, quota: u64) -> Self {
        self.extensions.performance.quota_gb = Some(quota);
        self
     }
    }

// Migration utilities for converting from old configurations to unified system
// All conversions now use the unified pattern established in nestgate-core
