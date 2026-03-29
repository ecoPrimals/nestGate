// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **STORAGE TYPES** — Storage operations and management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Storage tiers for data classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Storagetier
pub enum StorageTier {
    /// Frequent access - NVMe/SSD tier
    Hot,
    /// Regular access - SSD tier
    Warm,
    /// Infrequent access - HDD tier
    Cold,
    /// Ultra-fast cache - RAM/NVMe cache tier
    Cache,
    /// Long-term storage - Tape/Cloud tier
    Archive,
}

/// Storage operation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Storageoperation
pub enum StorageOperation {
    /// Read data from storage
    Read,
    /// Write data to storage
    Write,
    /// Delete data from storage
    Delete,
    /// Copy data within storage
    Copy,
    /// Move data within storage
    Move,
    /// Backup data to another location
    Backup,
    /// Restore data from backup
    Restore,
    /// Compress data for storage efficiency
    Compress,
    /// Decompress data for access
    Decompress,
}

/// Storage metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagemetadata
pub struct StorageMetadata {
    /// Size of the stored data in bytes
    pub size_bytes: u64,
    /// Timestamp when the data was created
    pub created_at: SystemTime,
    /// Timestamp when the data was last modified
    pub modified_at: SystemTime,
    /// Timestamp when the data was last accessed
    pub accessed_at: SystemTime,
    /// Storage tier where the data resides
    pub tier: StorageTier,
    /// Whether the data is compressed
    pub compressed: bool,
    /// Whether the data is encrypted
    pub encrypted: bool,
    /// Optional checksum for data integrity verification
    pub checksum: Option<String>,
    /// User-defined tags for categorization and metadata
    pub tags: HashMap<String, String>,
}

/// Storage resource
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storageresource
pub struct StorageResource {
    /// Unique identifier for the resource
    pub id: String,
    /// Metadata about the storage resource
    pub metadata: StorageMetadata,
    /// Optional permissions string
    pub permissions: Option<String>,
    /// Optional owner identifier
    pub owner: Option<String>,
}
