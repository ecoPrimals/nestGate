//! Integration functions for ZFS automation
//!
//! This module provides integration functions for initializing ZFS automation
//! with the NestGate ecosystem and external services.

use nestgate_automation::{
    AutomationConfig, IntelligentDatasetManager, Result as AutomationResult,
};

/// Initialize automation for ZFS with default configuration
pub async fn initialize_zfs_automation() -> AutomationResult<IntelligentDatasetManager> {
    let zfs_config = nestgate_core::config::Config::default();
    nestgate_automation::initialize_automation(zfs_config).await
}

/// Initialize automation for ZFS with custom configuration
pub async fn initialize_zfs_automation_with_config(
    automation_config: AutomationConfig,
) -> AutomationResult<IntelligentDatasetManager> {
    let zfs_config = nestgate_core::config::Config::default();
    nestgate_automation::initialize_automation_with_config(zfs_config, automation_config).await
}

/// Check if ecosystem services are available for ZFS automation
#[cfg(feature = "network-integration")]
pub async fn check_zfs_ecosystem_availability() -> bool {
    nestgate_automation::check_ecosystem_capabilities().await
}

#[cfg(not(feature = "network-integration"))]
pub async fn check_zfs_ecosystem_availability() -> bool {
    false
}
