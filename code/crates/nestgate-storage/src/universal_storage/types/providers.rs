// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage type providers and protocol versions
//!
//! Defines the fundamental storage types, network protocols, and cloud providers
//! supported by the universal storage system.

use serde::{Deserialize, Serialize};

/// **THE** Universal Storage Type - replaces all `StorageType` enums
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UniversalStorageType {
    /// Local file system storage
    Local,
    /// Network file system (NFS)
    Nfs {
        /// NFS protocol version
        version: NfsVersion,
    },
    /// Server Message Block (SMB/CIFS)
    Smb {
        /// SMB protocol version
        version: SmbVersion,
    },
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
    Cloud {
        /// Cloud storage provider (AWS, Azure, GCP, or custom)
        provider: CloudProvider,
    },
    /// Distributed storage
    Distributed,
    /// Custom storage type
    Custom(String),
}

/// NFS protocol versions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NfsVersion {
    /// `NFSv3`
    V3,
    /// `NFSv4`
    V4,
    /// NFSv4.1
    V41,
    /// NFSv4.2
    V42,
}

/// SMB protocol versions  
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SmbVersion {
    /// `SMBv2`
    V2,
    /// `SMBv3`
    V3,
    /// SMBv3.1
    V31,
}

/// Cloud storage providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CloudProvider {
    /// AWS cloud provider
    AWS {
        /// AWS region identifier (e.g., "us-east-1")
        region: String,
    },
    /// Azure cloud provider
    Azure {
        /// Azure subscription ID
        subscription_id: String,
    },
    /// Google Cloud Platform provider
    GCP {
        /// GCP project ID
        project_id: String,
    },
    /// Custom cloud provider
    Custom {
        /// Custom endpoint URL
        endpoint: String,
    },
}
