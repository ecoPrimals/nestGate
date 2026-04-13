// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! GCS-specific data types (pools, datasets, storage class).

use nestgate_core::canonical_types::StorageTier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GCS-backed pool (maps to GCS bucket)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcsPool {
    /// Pool name
    pub name: String,
    /// GCS bucket name
    pub bucket: String,
    /// Bucket location
    pub location: String,
    /// Creation time
    pub created_at: std::time::SystemTime,
    /// Pool metadata
    pub metadata: HashMap<String, String>,
}

/// GCS-backed dataset (maps to object prefix with storage class)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcsDataset {
    /// Dataset name
    pub name: String,
    /// Pool name
    pub pool: String,
    /// Object prefix
    pub prefix: String,
    /// Storage tier
    pub tier: StorageTier,
    /// GCS storage class
    pub storage_class: GcsStorageClass,
    /// Creation time
    pub created_at: std::time::SystemTime,
}

/// GCS-backed snapshot (maps to object versioning or generation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcsSnapshot {
    /// Snapshot name
    pub name: String,
    /// Dataset name
    pub dataset: String,
    /// GCS generation identifier
    pub generation: String,
    /// Creation time
    pub created_at: std::time::SystemTime,
}

/// GCS pool properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcsProperties {
    /// GCP project ID
    pub project_id: String,
    /// Bucket location
    pub location: String,
    /// Versioning enabled
    pub versioning: bool,
    /// Uniform bucket-level access
    pub uniform_access: bool,
    /// Lifecycle rules active
    pub lifecycle_enabled: bool,
    /// Additional properties
    pub custom: HashMap<String, String>,
}

/// GCS storage class mapping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GcsStorageClass {
    /// Standard storage (frequent access)
    Standard,
    /// Nearline storage (monthly access)
    Nearline,
    /// Coldline storage (quarterly access)
    Coldline,
    /// Archive storage (yearly access)
    Archive,
}
