/// Unified NAS Configuration - Standardized Pattern  
/// **CONSOLIDATED**: Migrated from 889-line fragmented config to StandardDomainConfig<T>
/// for full consistency with the unified configuration framework.
/// **PROBLEM SOLVED**: Eliminates NAS configuration fragmentation and size violation.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// Import the standardized config pattern
use nestgate_core::unified_config_consolidation::StandardDomainConfig;

// ==================== NAS UNIFIED CONFIGURATION ====================

/// **THE** unified NAS configuration using the standardized pattern
/// This replaces all fragmented NAS config structs with a single, consistent interface
pub type UnifiedNasConfig = StandardDomainConfig<NasExtensions>;

/// NAS-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasExtensions {
    /// Protocol support settings (SMB, NFS, FTP, SFTP, HTTP, WebDAV)
    pub protocols: NasProtocolSettings,
    /// Share management configuration
    pub shares: NasShareManagementSettings,
    /// Server configuration and runtime settings
    pub server: NasServerSettings,
    /// Storage management and optimization
    pub storage: NasStorageSettings,
    /// Access control and permissions
    pub access_control: NasAccessControlSettings,
    /// Backup and replication settings
    pub backup: NasBackupSettings,
    /// Performance optimization settings
    pub performance: NasPerformanceSettings,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasProtocolSettings {
    // SMB/CIFS Protocol
    pub smb_enabled: bool,
    pub smb_port: u16,
    pub smb_version: String,

    // NFS Protocol
    pub nfs_enabled: bool,
    pub nfs_port: u16,
    pub nfs_version: String,

    // FTP/FTPS Protocol
    pub ftp_enabled: bool,
    pub ftp_port: u16,
    pub ftps_enabled: bool,

    // SFTP Protocol
    pub sftp_enabled: bool,
    pub sftp_port: u16,

    // HTTP/HTTPS Protocol
    pub http_enabled: bool,
    pub http_port: u16,
    pub https_enabled: bool,

    // WebDAV Protocol
    pub webdav_enabled: bool,
    pub webdav_port: u16,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasShareManagementSettings {
    pub share_root: PathBuf,
    pub default_share_settings: NasShareConfig,
    pub shares: HashMap<String, NasShareConfig>,
    pub auto_discovery: bool,
    pub indexing: NasIndexingSettings,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasShareConfig {
    pub name: String,
    pub path: PathBuf,
    pub description: String,
    pub readonly: bool,
    pub guest_access: bool,
    pub encryption_enabled: bool,
    pub compression_enabled: bool,
    pub quota_gb: Option<u64>,
    pub allowed_users: Vec<String>,
    pub allowed_groups: Vec<String>,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasIndexingSettings {
    pub enable_indexing: bool,
    pub index_path: PathBuf,
    pub scan_interval_hours: u64,
    pub index_file_contents: bool,
    pub index_metadata: bool,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasServerSettings {
    pub server_name: String,
    pub workgroup: String,
    pub domain: Option<String>,
    pub max_connections: usize,
    pub connection_timeout: Duration,
    pub enable_logging: bool,
    pub log_level: String,
    pub maintenance_window_start: String, // Format: "HH:MM"
    pub maintenance_window_duration_hours: u8,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasStorageSettings {
    pub storage_root: PathBuf,
    pub enable_snapshots: bool,
    pub snapshot_retention_days: u32,
    pub enable_deduplication: bool,
    pub enable_compression: bool,
    pub raid_level: Option<String>,
    pub hot_spare_drives: u8,
    pub scrub_schedule: String, // Cron format
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasAccessControlSettings {
    pub enable_user_authentication: bool,
    pub enable_group_management: bool,
    pub password_policy: NasPasswordPolicy,
    pub session_timeout_minutes: u32,
    pub max_failed_login_attempts: u8,
    pub lockout_duration_minutes: u32,
    pub enable_two_factor_auth: bool,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasPasswordPolicy {
    pub min_length: u8,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special_chars: bool,
    pub password_expiry_days: Option<u32>,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasBackupSettings {
    pub enable_automatic_backup: bool,
    pub backup_schedule: String, // Cron format
    pub backup_destination: PathBuf,
    pub backup_retention_days: u32,
    pub enable_incremental_backup: bool,
    pub enable_remote_backup: bool,
    pub remote_backup_settings: Option<NasRemoteBackupSettings>,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasRemoteBackupSettings {
    pub remote_host: String,
    pub remote_port: u16,
    pub remote_username: String,
    pub remote_path: PathBuf,
    pub encryption_enabled: bool,
    pub compression_enabled: bool,
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasPerformanceSettings {
    pub max_concurrent_transfers: usize,
    pub read_buffer_size_kb: u32,
    pub write_buffer_size_kb: u32,
    pub enable_read_ahead: bool,
    pub enable_write_cache: bool,
    pub cache_size_mb: u32,
    pub enable_async_io: bool,
    pub io_queue_depth: u32,
    }

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for NasExtensions {
    fn default() -> Self {
        Self {
            protocols: NasProtocolSettings::default(),
            shares: NasShareManagementSettings::default(),
            server: NasServerSettings::default(),
            storage: NasStorageSettings::default(),
            access_control: NasAccessControlSettings::default(),
            backup: NasBackupSettings::default(),
            performance: NasPerformanceSettings::default(),
    }
    }
    }

impl Default for NasProtocolSettings {
    fn default() -> Self {
        Self {
            smb_enabled: true,
            smb_port: 445,
            smb_version: "3.0".to_string(),
            nfs_enabled: true,
            nfs_port: 2049,
            nfs_version: "4.1".to_string(),
            ftp_enabled: false,
            ftp_port: 21,
            ftps_enabled: false,
            sftp_enabled: true,
            sftp_port: 22,
            http_enabled: true,
            http_port: 80,
            https_enabled: true,
            webdav_enabled: false,
            webdav_port: 8080,
    }
    }
    }

impl Default for NasShareManagementSettings {
    fn default() -> Self {
        Self {
            share_root: PathBuf::from("/shares"),
            default_share_settings: NasShareConfig::default(),
            shares: HashMap::new(),
            auto_discovery: true,
            indexing: NasIndexingSettings::default(),
    }
    }
    }

impl Default for NasShareConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            path: PathBuf::from("/shares/default"),
            description: "Default share".to_string(),
            readonly: false,
            guest_access: false,
            encryption_enabled: true,
            compression_enabled: true,
            quota_gb: None,
            allowed_users: Vec::new(),
            allowed_groups: Vec::new(),
    }
    }
    }

impl Default for NasIndexingSettings {
    fn default() -> Self {
        Self {
            enable_indexing: true,
            index_path: PathBuf::from("/var/nas/index"),
            scan_interval_hours: 24,
            index_file_contents: false,
            index_metadata: true,
    }
    }
    }

impl Default for NasServerSettings {
    fn default() -> Self {
        Self {
            server_name: "nestgate-nas".to_string(),
            workgroup: "WORKGROUP".to_string(),
            domain: None,
            max_connections: 100,
            connection_timeout: Duration::from_secs(300),
            enable_logging: true,
            log_level: "INFO".to_string(),
            maintenance_window_start: "02:00".to_string(),
            maintenance_window_duration_hours: 2,
    }
    }
    }

impl Default for NasStorageSettings {
    fn default() -> Self {
        Self {
            storage_root: PathBuf::from("/storage"),
            enable_snapshots: true,
            snapshot_retention_days: 30,
            enable_deduplication: true,
            enable_compression: true,
            raid_level: Some("RAID5".to_string()),
            hot_spare_drives: 1,
            scrub_schedule: "0 2 * * 0".to_string(), // Weekly at 2 AM Sunday
    }
    }
    }

impl Default for NasAccessControlSettings {
    fn default() -> Self {
        Self {
            enable_user_authentication: true,
            enable_group_management: true,
            password_policy: NasPasswordPolicy::default(),
            session_timeout_minutes: 60,
            max_failed_login_attempts: 3,
            lockout_duration_minutes: 15,
            enable_two_factor_auth: false,
    }
    }
    }

impl Default for NasPasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special_chars: false,
            password_expiry_days: Some(90),
    }
    }
    }

impl Default for NasBackupSettings {
    fn default() -> Self {
        Self {
            enable_automatic_backup: true,
            backup_schedule: "0 1 * * *".to_string(), // Daily at 1 AM
            backup_destination: PathBuf::from("/backup"),
            backup_retention_days: 30,
            enable_incremental_backup: true,
            enable_remote_backup: false,
            remote_backup_settings: None,
    }
    }
    }

impl Default for NasPerformanceSettings {
    fn default() -> Self {
        Self {
            max_concurrent_transfers: 50,
            read_buffer_size_kb: 64,
            write_buffer_size_kb: 64,
            enable_read_ahead: true,
            enable_write_cache: true,
            cache_size_mb: 512,
            enable_async_io: true,
            io_queue_depth: 32,
    }
    }
    }

// ==================== CONVENIENCE CONSTRUCTORS ====================

impl UnifiedNasConfig {
    /// Create a new NAS configuration with default settings
    pub fn new_nas_server(server_name: &str) -> Self {
        let mut extensions = NasExtensions::default();
        extensions.server.server_name = server_name.to_string();

        Self::with_service(extensions, "nestgate-nas", "1.0.0")
    }

    /// Create a NAS configuration with specific protocol settings
    pub fn with_protocols(server_name: &str, protocols: NasProtocolSettings) -> Self {
        let mut extensions = NasExtensions::default();
        extensions.server.server_name = server_name.to_string();
        extensions.protocols = protocols;

        Self::with_service(extensions, "nestgate-nas", "1.0.0")
    }
    }
