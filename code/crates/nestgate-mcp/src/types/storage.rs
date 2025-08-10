//! Storage-related type definitions for MCP system

use serde::{Deserialize, Serialize};

// Import the canonical StorageTier from the unified system
pub use nestgate_core::types::StorageTier;

// Note: StorageTier now comes from nestgate_core::types with these variants:
// - Hot: High-performance storage for frequently accessed data
// - Warm: Medium-performance storage for moderately accessed data
// - Cold: Low-performance storage for rarely accessed data
// - Cache: Fast cache storage for temporary data
//
// The Archive tier from the old definition maps to Cold tier in the unified system

/// Storage protocols supported by providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StorageProtocol {
    /// NFS protocol with version.
    Nfs(NfsVersion),
    /// SMB protocol with version.
    Smb(SmbVersion),
    /// iSCSI protocol.
    Iscsi,
    /// S3 protocol.
    S3,
    /// Custom protocol with name.
    Custom(String),
}

/// NFS protocol versions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NfsVersion {
    /// NFS version 3.
    V3,
    /// NFS version 4.0.
    V4,
    /// NFS version 4.1.
    V41,
    /// NFS version 4.2.
    V42,
}

/// SMB protocol versions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SmbVersion {
    /// SMB version 2.
    V2,
    /// SMB version 3.0.
    V3,
    /// SMB version 3.1.
    V31,
}
