// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Storage resources, capabilities, and permissions
//!
//! Defines storage resources (pools, datasets, volumes), their capabilities,
//! permissions, and health status.

use super::providers::UniversalStorageType;
use nestgate_types::unified_enums::UnifiedTierType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **THE** Universal Storage Resource - consolidates all storage resource types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalStorageResource {
    /// Unique resource identifier
    pub resource_id: String,
    /// Human-readable name
    pub name: String,
    /// Storage type
    pub storage_type: UniversalStorageType,
    /// Resource type (dataset, pool, volume, etc.)
    pub resource_type: StorageResourceType,
    /// Resource size in bytes
    pub size_bytes: u64,
    /// Available space in bytes
    pub available_bytes: u64,
    /// Used space in bytes
    pub used_bytes: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    /// Last accessed timestamp
    pub accessed_at: Option<DateTime<Utc>>,
    /// Storage tier
    pub tier: UnifiedTierType,
    /// Resource capabilities
    pub capabilities: Vec<StorageCapability>,
    /// Performance metrics
    pub performance: super::metrics::StoragePerformanceMetrics,
    /// Resource metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Resource tags
    pub tags: Vec<String>,
    /// Access permissions
    pub permissions: StoragePermissions,
    /// Health status
    pub health_status: StorageHealthStatus,
}

/// Storage resource types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageResourceType {
    /// Storage pool
    Pool,
    /// Dataset within a pool
    Dataset,
    /// Volume (block device)
    Volume,
    /// Snapshot
    Snapshot,
    /// Backup
    Backup,
    /// Cache
    Cache,
    /// Custom resource type
    Custom(String),
}

/// Storage capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageCapability {
    /// Read-write access
    ReadWrite,
    /// Read-only access
    ReadOnly,
    /// Streaming support
    Streaming,
    /// Replication support
    Replication,
    /// Snapshot support
    Snapshots,
    /// Compression support
    Compression,
    /// Deduplication support
    Deduplication,
    /// Encryption support
    Encryption,
    /// Versioning support
    Versioning,
    /// Backup support
    Backup,
    /// Restore operations (data recovery)
    Restore,
    /// Monitoring and observability
    Monitoring,
    /// Custom capability type with arbitrary name
    Custom(String),
}

/// Storage permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePermissions {
    /// Owner permissions
    pub owner: Vec<String>,
    /// Group permissions
    pub group: Vec<String>,
    /// Other permissions
    pub other: Vec<String>,
    /// Access control list
    pub acl: HashMap<String, Vec<String>>,
}

/// Storage health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageHealthStatus {
    /// Healthy - operating normally
    Healthy,
    /// Warning - potential issues detected
    Warning,
    /// Critical - immediate attention required
    Critical,
    /// Offline - not accessible
    Offline,
    /// Maintenance - scheduled maintenance mode
    Maintenance,
    /// Unknown - health status cannot be determined
    Unknown,
}
