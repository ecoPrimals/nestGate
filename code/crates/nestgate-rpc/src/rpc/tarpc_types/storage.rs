// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Dataset and object types for the tarpc [`super::NestGateRpc`] surface.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dataset creation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetParams {
    /// Dataset description
    pub description: Option<String>,

    /// Compression settings
    pub compression: Option<String>,

    /// Encryption enabled
    pub encrypted: bool,

    /// Deduplication enabled
    pub deduplicated: bool,

    /// Quota in bytes (None = unlimited)
    pub quota: Option<u64>,

    /// Additional custom properties
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

impl Default for DatasetParams {
    fn default() -> Self {
        Self {
            description: None,
            compression: Some("lz4".to_string()),
            encrypted: true,
            deduplicated: true,
            quota: None,
            properties: HashMap::new(),
        }
    }
}

/// Dataset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    /// Dataset name
    pub name: String,

    /// Dataset description
    pub description: Option<String>,

    /// Creation timestamp
    pub created_at: i64,

    /// Last modified timestamp
    pub modified_at: i64,

    /// Total size in bytes
    pub size_bytes: u64,

    /// Number of objects
    pub object_count: u64,

    /// Compression ratio (1.0 = no compression)
    pub compression_ratio: f64,

    /// Configuration parameters
    pub params: DatasetParams,

    /// Dataset status
    pub status: String,
}

/// Object information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInfo {
    /// Object key
    pub key: String,

    /// Dataset name
    pub dataset: String,

    /// Content size in bytes
    pub size_bytes: u64,

    /// Creation timestamp
    pub created_at: i64,

    /// Last modified timestamp
    pub modified_at: i64,

    /// Content type (MIME)
    pub content_type: Option<String>,

    /// Checksum (SHA-256)
    pub checksum: Option<String>,

    /// Encrypted
    pub encrypted: bool,

    /// Compressed
    pub compressed: bool,

    /// Custom metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    /// Success flag
    pub success: bool,

    /// Result message
    pub message: String,

    /// Operation metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}
