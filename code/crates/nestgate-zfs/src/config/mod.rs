//! ZFS Configuration Management
//!
//! Advanced configuration patterns with ZFS-specific settings.
//! This module is organized into focused submodules for better maintainability:
//!
//! - `main` - Main ZFS configuration and core settings
//! - `tiers` - Tier configurations and performance profiles
//! - `migration` - Migration settings and capacity limits
//! - `pool` - Pool discovery and management settings
//! - `health` - Health monitoring configuration  
//! - `metrics` - Metrics collection and export settings
//! - `security` - Security configuration and access control
//! - `automation` - Dataset automation and AI settings

pub mod automation;
pub mod health;
pub mod metrics;
pub mod migration;
pub mod pool;
pub mod security;
pub mod tiers;
pub mod unified_zfs_config;

// Re-export main types for backward compatibility
use serde::{Deserialize, Serialize};

/// Pool type configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PoolType {
    #[default]
    Standard,
    Mirror,
    RaidZ1,
    RaidZ2,
    RaidZ3,
}

/// Compression type configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CompressionType {
    #[default]
    None,
    Lz4,
    Gzip,
    Zstd,
}

pub use automation::{AiAutomationSettings, DatasetAutomationConfig};
pub use health::HealthMonitoringConfig;
pub use metrics::{MetricsConfig, MetricsFormat};
pub use migration::{CapacityLimits, MigrationConfig, MigrationRules};
pub use pool::PoolDiscoveryConfig;
pub use security::{AccessControlConfig, KeyManagementConfig, SecurityConfig};
pub use tiers::{PerformanceProfile, TierConfig, TierConfigurations};
pub use unified_zfs_config::ZfsConfig;
