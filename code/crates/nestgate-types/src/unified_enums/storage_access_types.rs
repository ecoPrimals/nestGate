// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Storage and Access Control Classification Enums
/// This module contains enums related to storage types, access control,
/// and data tier management.
use serde::{Deserialize, Serialize};
use std::fmt;
// ==================== SECTION ====================

/// **THE** `StorageType` - unified across all modules
/// Replaces `StorageType` definitions in service discovery and storage modules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedStorage`
pub enum UnifiedStorageType {
    /// Local file system storage
    Local,
    /// Network file system (NFS)
    Nfs,
    /// Server Message Block (SMB/CIFS)
    Smb,
    /// Object storage (S3-compatible)
    Object,
    /// Block storage
    Block,
    /// ZFS-based storage
    Zfs,
    /// Database storage
    Database,
    /// In-memory storage
    Memory,
    /// Cache storage
    Cache,
    /// Cloud storage
    Cloud,
    /// Distributed storage
    Distributed,
    /// Custom storage type
    Custom(String),
}
impl Default for UnifiedStorageType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Local
    }
}

impl fmt::Display for UnifiedStorageType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::Nfs => write!(f, "nfs"),
            Self::Smb => write!(f, "smb"),
            Self::Object => write!(f, "object"),
            Self::Block => write!(f, "block"),
            Self::Zfs => write!(f, "zfs"),
            Self::Database => write!(f, "database"),
            Self::Memory => write!(f, "memory"),
            Self::Cache => write!(f, "cache"),
            Self::Cloud => write!(f, "cloud"),
            Self::Distributed => write!(f, "distributed"),
            Self::Custom(storage_type) => write!(f, "{storage_type}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `AccessType` - unified across all modules
/// Replaces `AccessType` definitions in automation and other modules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedAccess`
pub enum UnifiedAccessType {
    /// Read access only
    Read,
    /// Write access only
    Write,
    /// Both read and write access
    ReadWrite,
    /// Execute access
    Execute,
    /// Administrative access
    Admin,
    /// No access
    None,
    /// Custom access type
    Custom(String),
}
impl Default for UnifiedAccessType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Read
    }
}

impl fmt::Display for UnifiedAccessType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read => write!(f, "read"),
            Self::Write => write!(f, "write"),
            Self::ReadWrite => write!(f, "read_write"),
            Self::Execute => write!(f, "execute"),
            Self::Admin => write!(f, "admin"),
            Self::None => write!(f, "none"),
            Self::Custom(access_type) => write!(f, "{access_type}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `TierType` - unified across all modules
/// Replaces `TierType` definitions in automation and storage modules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of `UnifiedTier`
pub enum UnifiedTierType {
    /// Hot tier - frequently accessed data
    Hot,
    /// Warm tier - occasionally accessed data
    Warm,
    /// Cool tier - rarely accessed data
    Cool,
    /// Cold tier - archived data
    Cold,
    /// Frozen tier - long-term archived data
    Frozen,
    /// Custom tier type
    Custom(String),
}
impl Default for UnifiedTierType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Hot
    }
}

impl fmt::Display for UnifiedTierType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hot => write!(f, "hot"),
            Self::Warm => write!(f, "warm"),
            Self::Cool => write!(f, "cool"),
            Self::Cold => write!(f, "cold"),
            Self::Frozen => write!(f, "frozen"),
            Self::Custom(tier_type) => write!(f, "{tier_type}"),
        }
    }
}
