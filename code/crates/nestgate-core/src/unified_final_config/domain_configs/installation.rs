use std::collections::HashMap;
//
// Installation-specific configuration structures extracted from the monolithic domain_configs.rs
// for better maintainability and focused responsibility.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// Installation domain configuration for installer crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationDomainConfig {
    pub install_dir: PathBuf,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub mode: String, // "development" or "production"
    pub interactive: bool,
    pub verbose: bool,
    pub force_install: bool,
}

/// Components domain configuration for installer crate
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComponentsDomainConfig {
    pub selected_components: SelectedComponents,
}

/// Selected components for installation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedComponents {
    pub install_api: bool,
    pub install_zfs: bool,
    pub install_ui: bool,
    pub install_nas: bool,
    pub install_mcp: bool,
    pub install_network: bool,
    pub install_monitoring: bool,
    pub install_security: bool,
    pub install_automation: bool,
    pub install_fsmonitor: bool,
    pub custom_components: Vec<String>,
}

/// System integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemIntegrationConfig {
    pub install_as_service: bool,
    pub add_to_path: bool,
    pub desktop_integration: bool,
}

/// Datasets domain configuration for ZFS crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetsDomainConfig {
    pub auto_create: bool,
    pub default_compression: String,
    pub default_recordsize: String,
    pub enable_snapshots: bool,
    pub snapshot_interval: Duration,
    pub quota_enforcement: bool,
    pub default_properties: HashMap<String, String>,
    pub default_quota_bytes: Option<u64>,
    pub default_reservation_bytes: Option<u64>,
    pub max_datasets_per_pool: u32,
    pub snapshot_settings: SnapshotSettings,
}

/// Validation domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDomainConfig {
    pub system_requirements: SystemRequirements,
    pub pre_install_checks: PreInstallChecks,
    pub post_install_validation: bool,
}

/// System requirements for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRequirements {
    pub min_ram_mb: u64,
    pub min_disk_space_mb: u64,
    pub required_features: Vec<String>,
}

/// Pre-installation checks configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreInstallChecks {
    pub check_dependencies: bool,
    pub check_permissions: bool,
    pub check_network: bool,
    pub check_storage: bool,
}

/// Snapshot settings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotSettings {
    pub auto_snapshot: bool,
    pub retention_count: u32,
    pub compression: String,
    pub retention_period: Duration,
    pub snapshot_frequency: Duration,
}

impl Default for InstallationDomainConfig {
    fn default() -> Self {
        Self {
            install_dir: PathBuf::from("/opt/nestgate"),
            config_dir: PathBuf::from("/etc/nestgate"),
            data_dir: PathBuf::from("/var/lib/nestgate"),
            mode: "production".to_string(),
            interactive: true,
            verbose: false,
            force_install: false,
        }
    }
}

impl Default for SelectedComponents {
    fn default() -> Self {
        Self {
            install_api: true,
            install_zfs: true,
            install_ui: false,
            install_nas: false,
            install_mcp: false,
            install_network: true,
            install_monitoring: true,
            install_security: true,
            install_automation: false,
            install_fsmonitor: false,
            custom_components: Vec::new(),
        }
    }
}

impl Default for SystemIntegrationConfig {
    fn default() -> Self {
        Self {
            install_as_service: true,
            add_to_path: true,
            desktop_integration: false,
        }
    }
}

impl Default for DatasetsDomainConfig {
    fn default() -> Self {
        Self {
            auto_create: true,
            default_compression: "lz4".to_string(),
            default_recordsize: "128K".to_string(),
            enable_snapshots: true,
            snapshot_interval: Duration::from_secs(3600),
            quota_enforcement: false,
            default_properties: HashMap::new(),
            default_quota_bytes: None,
            default_reservation_bytes: None,
            max_datasets_per_pool: 1000,
            snapshot_settings: SnapshotSettings::default(),
        }
    }
}

impl Default for ValidationDomainConfig {
    fn default() -> Self {
        Self {
            system_requirements: SystemRequirements::default(),
            pre_install_checks: PreInstallChecks::default(),
            post_install_validation: true,
        }
    }
}

impl Default for SystemRequirements {
    fn default() -> Self {
        Self {
            min_ram_mb: 2048,
            min_disk_space_mb: 10240,
            required_features: vec!["zfs".to_string()],
        }
    }
}

impl Default for PreInstallChecks {
    fn default() -> Self {
        Self {
            check_dependencies: true,
            check_permissions: true,
            check_network: true,
            check_storage: true,
        }
    }
}

impl Default for SnapshotSettings {
    fn default() -> Self {
        Self {
            auto_snapshot: true,
            retention_count: 24,
            compression: "lz4".to_string(),
            retention_period: Duration::from_secs(86400 * 7), // 7 days
            snapshot_frequency: Duration::from_secs(3600),    // 1 hour
        }
    }
}
