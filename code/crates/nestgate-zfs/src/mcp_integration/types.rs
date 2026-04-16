// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! MCP request/response types and deprecated [`ZfsMcpConfig`].

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::types::StorageTier;
use nestgate_core::{NestGateError, Result};

/// MCP mount request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpMountRequest {
    /// Mount identifier
    pub mount_id: String,
    /// Mount Point
    pub mount_point: String,
    /// Tier
    pub tier: StorageTier,
    /// Size in gigabytes
    pub size_gb: u64,
}

/// MCP volume request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpVolumeRequest {
    /// Volume identifier
    pub volume_id: String,
    /// Tier
    pub tier: StorageTier,
    /// Size in gigabytes
    pub size_gb: u64,
}

/// Mount status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MountStatus {
    /// Active
    Active,
    /// Inactive
    Inactive,
    /// Error state with message
    Error(String),
}

/// Volume status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VolumeStatus {
    /// Active
    Active,
    /// Inactive
    Inactive,
    /// Error state with message
    Error(String),
}

/// ZFS mount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMountInfo {
    /// Mount identifier
    pub mount_id: String,
    /// Dataset Path
    pub dataset_path: String,
    /// Mount Point
    pub mount_point: String,
    /// Tier
    pub tier: StorageTier,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Status
    pub status: MountStatus,
}

/// ZFS volume information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsVolumeInfo {
    /// Volume identifier
    pub volume_id: String,
    /// Dataset Path
    pub dataset_path: String,
    /// Tier
    pub tier: StorageTier,
    /// Size Bytes
    pub size_bytes: u64,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Status
    pub status: VolumeStatus,
}

/// ZFS MCP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMcpConfig {
    /// Enable AI optimization
    pub enable_ai_optimization: bool,
    /// Maximum concurrent operations
    pub max_concurrent_operations: u32,
    /// Default tier for new resources
    pub default_tier: StorageTier,
}

impl Default for ZfsMcpConfig {
    fn default() -> Self {
        Self {
            enable_ai_optimization: true,
            max_concurrent_operations: 10,
            default_tier: StorageTier::Warm,
        }
    }
}

impl ZfsMcpConfig {
    /// Validate the configuration settings
    pub fn validate(&self) -> Result<()> {
        if self.max_concurrent_operations == 0 {
            return Err(NestGateError::validation(
                "max_concurrent_operations must be greater than 0",
            ));
        }

        if self.max_concurrent_operations > 1000 {
            return Err(NestGateError::validation(
                "max_concurrent_operations cannot exceed 1000",
            ));
        }
        Ok(())
    }

    /// Get tier configuration for a specific tier
    #[must_use]
    pub const fn get_tier_config(&self, tier: &StorageTier) -> TierConfig {
        match tier {
            StorageTier::Hot => TierConfig {
                priority: 1,
                cache_enabled: true,
                compression: false,
                replication: 2,
            },
            StorageTier::Warm => TierConfig {
                priority: 2,
                cache_enabled: true,
                compression: true,
                replication: 1,
            },
            StorageTier::Cold => TierConfig {
                priority: 3,
                cache_enabled: false,
                compression: true,
                replication: 1,
            },
            StorageTier::Cache => TierConfig {
                priority: 0,
                cache_enabled: true,
                compression: false,
                replication: 3,
            },
            StorageTier::Archive => TierConfig {
                priority: 4,
                cache_enabled: false,
                compression: true,
                replication: 1,
            },
        }
    }
}

/// Configuration for a storage tier
#[derive(Debug, Clone)]
pub struct TierConfig {
    /// Priority
    pub priority: u8,
    /// Cache Enabled
    pub cache_enabled: bool,
    /// Compression
    pub compression: bool,
    /// Replication
    pub replication: u32,
}
