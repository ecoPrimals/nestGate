// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Advanced configuration patterns with ZFS-specific settings.
// This module is organized into focused submodules for better maintainability:
//
// - `main` - Main ZFS configuration and core settings
// - `tiers` - Tier configurations and performance profiles
// - `migration` - Migration settings and capacity limits
// - `pool` - Pool discovery and management settings
// - `health` - Health monitoring configuration
// - `metrics` - Metrics collection and export settings
// - `security` - Security configuration and access control
// - `automation` - Dataset automation and AI settings

//! Config module

/// Automation configuration settings
pub mod automation;
/// Health monitoring configuration
pub mod health;
pub mod metrics;
// Migration module removed as part of cleanup
/// Pool configuration settings
pub mod pool;
/// Security configuration settings
pub mod security;
pub mod tiers;
// unified_zfs_config removed - use canonical types from nestgate_core

// Re-export main types for backward compatibility
use serde::{Deserialize, Serialize};

/// Pool type configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Types of Pool
pub enum PoolType {
    #[default]
    /// Standard
    Standard,
    /// Mirror
    Mirror,
    /// Raidz1
    RaidZ1,
    /// Raidz2
    RaidZ2,
    /// Raidz3
    RaidZ3,
}
/// Compression type configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Types of Compression
pub enum CompressionType {
    #[default]
    /// None
    None,
    /// Lz4
    Lz4,
    /// Gzip
    Gzip,
    /// Zstd
    Zstd,
}
pub use crate::automation::policies::MigrationRules;
pub use automation::{AiAutomationSettings, DatasetAutomationConfig};
pub use health::HealthMonitoringConfig;
pub use nestgate_core::services::storage::config::ZfsConfig;
pub use pool::PoolDiscoveryConfig;
pub use security::{AccessControlConfig, KeyManagementConfig, SecurityConfig};
pub use tiers::CapacityLimits; // Using local definition from tiers.rs
pub use tiers::{PerformanceProfile, TierConfig, TierConfigurations};
