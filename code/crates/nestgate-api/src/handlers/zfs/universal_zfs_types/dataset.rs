// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS Dataset and Snapshot Types
//!
//! Types for managing ZFS datasets and snapshots.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information about a ZFS dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Datasetinfo
pub struct DatasetInfo {
    /// Dataset name
    pub name: String,
    /// Dataset type
    pub dataset_type: DatasetType,
    /// Used space in bytes
    pub used: u64,
    /// Available space in bytes
    pub available: u64,
    /// Referenced space in bytes
    pub referenced: u64,
    /// Mount point
    pub mountpoint: Option<String>,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

/// Dataset type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of Dataset
pub enum DatasetType {
    /// Filesystem dataset
    Filesystem,
    /// Volume dataset
    Volume,
    /// Snapshot dataset
    Snapshot,
}

/// Information about a ZFS snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotinfo
pub struct SnapshotInfo {
    /// Snapshot name (e.g., "pool/dataset@snapshot-name")
    pub name: String,
    /// Creation time (Unix timestamp)
    pub creation_time: u64,
    /// Used space in bytes
    pub used: u64,
    /// Referenced space in bytes
    pub referenced: u64,
    /// Additional properties
    pub properties: HashMap<String, String>,
}
