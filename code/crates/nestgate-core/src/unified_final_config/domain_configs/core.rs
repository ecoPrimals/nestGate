use std::collections::HashMap;
//
// Core domain-specific configuration structures extracted from the monolithic domain_configs.rs
// for better maintainability and focused responsibility.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfig {
    pub enabled: bool,
    pub check_interval: Duration,
    pub alert_thresholds: AlertThresholds,
    pub notification_channels: Vec<String>,
}

/// Alert thresholds configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub temperature_celsius: f64,
}

/// AI automation configuration for ZFS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAutomationConfig {
    pub enabled: bool,
    pub predictive_scaling: bool,
    pub auto_optimization: bool,
    pub learning_mode: bool,
}

/// Pool management configuration for ZFS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolManagementConfig {
    pub auto_scrub: bool,
    pub scrub_interval: Duration,
    pub auto_trim: bool,
    pub trim_interval: Duration,
}

/// Performance configuration for ZFS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDomainConfig {
    pub cache_optimization: bool,
    pub prefetch_enabled: bool,
    pub l2arc_enabled: bool,
    pub zil_enabled: bool,
    pub tuning_parameters: HashMap<String, String>,
}

/// Storage tiers configuration for ZFS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageTiersConfig {
    pub hot_tier: TierConfig,
    pub warm_tier: TierConfig,
    pub cold_tier: TierConfig,
    pub archive_tier: TierConfig,
}

/// Individual tier configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    pub enabled: bool,
    pub storage_class: String,
    pub compression: String,
    pub deduplication: bool,
    pub encryption: bool,
}

impl Default for HealthMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(300), // 5 minutes
            alert_thresholds: AlertThresholds::default(),
            notification_channels: vec!["email".to_string(), "webhook".to_string()],
        }
    }
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 80.0,
            memory_usage_percent: 85.0,
            disk_usage_percent: 90.0,
            temperature_celsius: 70.0,
        }
    }
}

impl Default for AiAutomationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            predictive_scaling: false,
            auto_optimization: false,
            learning_mode: false,
        }
    }
}

impl Default for PoolManagementConfig {
    fn default() -> Self {
        Self {
            auto_scrub: true,
            scrub_interval: Duration::from_secs(86400 * 7), // Weekly
            auto_trim: true,
            trim_interval: Duration::from_secs(86400), // Daily
        }
    }
}

impl Default for PerformanceDomainConfig {
    fn default() -> Self {
        let mut tuning_parameters = HashMap::new();
        tuning_parameters.insert("zfs_arc_max".to_string(), "8G".to_string());
        tuning_parameters.insert("zfs_vdev_cache_size".to_string(), "10M".to_string());

        Self {
            cache_optimization: true,
            prefetch_enabled: true,
            l2arc_enabled: false,
            zil_enabled: true,
            tuning_parameters,
        }
    }
}

impl Default for StorageTiersConfig {
    fn default() -> Self {
        Self {
            hot_tier: TierConfig {
                enabled: true,
                storage_class: "nvme".to_string(),
                compression: "lz4".to_string(),
                deduplication: false,
                encryption: true,
            },
            warm_tier: TierConfig {
                enabled: true,
                storage_class: "ssd".to_string(),
                compression: "gzip".to_string(),
                deduplication: true,
                encryption: true,
            },
            cold_tier: TierConfig {
                enabled: true,
                storage_class: "hdd".to_string(),
                compression: "gzip-9".to_string(),
                deduplication: true,
                encryption: true,
            },
            archive_tier: TierConfig {
                enabled: false,
                storage_class: "tape".to_string(),
                compression: "gzip-9".to_string(),
                deduplication: true,
                encryption: true,
            },
        }
    }
}
