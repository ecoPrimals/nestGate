// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Core storage backend type definitions.
//!
//! Defines the fundamental backend variants, their specific configurations,
//! and supporting types for distributed and in-memory storage.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::connection::StorageConnectionConfig;
use super::health::{StorageHealthCheckConfig, StorageLimitsConfig};

/// Types of storage backends available in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageBackendType {
    /// Local filesystem backend
    Filesystem,
    /// ZFS storage backend
    Zfs,
    /// S3-compatible object storage
    S3Compatible,
    /// Azure Blob Storage
    Azure,
    /// Google Cloud Storage
    Gcs,
    /// In-memory storage (volatile)
    Memory,
    /// Distributed multi-node storage
    Distributed,
    /// Custom storage backend type
    Custom(String),
}

/// A configured storage backend instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBackend {
    /// Backend type
    pub backend_type: StorageBackendType,
    /// Backend-specific configuration
    pub config: StorageBackendSpecificConfig,
    /// Connection settings
    pub connection: StorageConnectionConfig,
    /// Capacity and limits
    pub limits: StorageLimitsConfig,
    /// Health check configuration
    pub health_check: StorageHealthCheckConfig,
}

/// Backend-specific configuration variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageBackendSpecificConfig {
    /// Filesystem backend configuration
    Filesystem {
        /// Root path for filesystem storage
        root_path: PathBuf,
        /// File permissions (Unix mode)
        permissions: u32,
        /// Whether to create directories if they don't exist
        create_dirs: bool,
    },
    /// ZFS backend configuration
    Zfs {
        /// ZFS pool name
        pool_name: String,
        /// Dataset prefix for namespacing
        dataset_prefix: String,
        /// ZFS compression algorithm
        compression: ZfsCompression,
        /// Enable ZFS deduplication
        deduplication: bool,
    },
    /// S3-compatible storage configuration
    S3Compatible {
        /// S3 endpoint URL
        endpoint: String,
        /// AWS region
        region: String,
        /// S3 bucket name
        bucket: String,
        /// AWS access key ID
        access_key_id: String,
        /// AWS secret access key
        secret_access_key: String,
        /// Use SSL/TLS for connections
        use_ssl: bool,
    },
    /// Azure Blob Storage configuration
    Azure {
        /// Azure storage account name
        account_name: String,
        /// Azure storage account key
        account_key: String,
        /// Azure blob container name
        container: String,
        /// Optional Azure endpoint suffix (e.g., for sovereign clouds)
        endpoint_suffix: Option<String>,
    },
    /// Google Cloud Storage configuration
    Gcs {
        /// GCP project ID
        project_id: String,
        /// GCS bucket name
        bucket: String,
        /// Path to service account credentials JSON file
        credentials_path: Option<PathBuf>,
        /// Service account key as JSON string
        service_account_key: Option<String>,
    },
    /// In-memory storage configuration
    Memory {
        /// Maximum memory size in bytes
        max_size: usize,
        /// Memory eviction policy
        eviction_policy: MemoryEvictionPolicy,
    },
    /// Distributed storage configuration
    Distributed {
        /// Distributed storage nodes
        nodes: Vec<DistributedStorageNode>,
        /// Consistency level for distributed operations
        consistency_level: ConsistencyLevel,
        /// Replication factor across nodes
        replication_factor: u32,
    },
    /// Custom storage backend configuration
    Custom {
        /// Custom JSON configuration
        config: serde_json::Value,
    },
}

/// ZFS compression algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsCompression {
    /// Disabled
    Off,
    /// LZJB (legacy)
    Lzjb,
    /// Gzip
    Gzip,
    /// ZLE
    Zle,
    /// LZ4 (fast, good ratio)
    Lz4,
    /// Zstandard (best ratio)
    Zstd,
}

/// Memory cache eviction policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryEvictionPolicy {
    /// Least Recently Used
    Lru,
    /// Least Frequently Used
    Lfu,
    /// First In, First Out
    Fifo,
    /// Random eviction
    Random,
    /// Time-To-Live based eviction
    Ttl,
}

/// A node in a distributed storage cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedStorageNode {
    /// Node identifier
    pub id: String,
    /// Node endpoint
    pub endpoint: String,
    /// Node weight for load balancing
    pub weight: u32,
    /// Node availability zone
    pub availability_zone: Option<String>,
}

/// Consistency levels for distributed storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsistencyLevel {
    /// Eventual consistency
    Eventual,
    /// Strong consistency
    Strong,
    /// Session consistency
    Session,
    /// Bounded staleness
    BoundedStaleness,
}
