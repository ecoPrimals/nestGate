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
pub mod main;
pub mod metrics;
pub mod migration;
pub mod pool;
pub mod security;
pub mod tiers;

// Re-export main types for backward compatibility
pub use automation::{AiAutomationSettings, DatasetAutomationConfig};
pub use health::HealthMonitoringConfig;
pub use main::ZfsConfig;
pub use metrics::{MetricsConfig, MetricsFormat};
pub use migration::{CapacityLimits, MigrationConfig, MigrationRules};
pub use pool::PoolDiscoveryConfig;
pub use security::{AccessControlConfig, KeyManagementConfig, SecurityConfig};
pub use tiers::{PerformanceProfile, TierConfig, TierConfigurations};
