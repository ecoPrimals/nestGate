// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS Configuration Types
//!
//! Configuration types for datasets and snapshots used by the universal ZFS handler layer.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dataset configuration for create/update operations in the universal ZFS stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetConfig {
    /// Dataset name
    pub name: String,
    /// Mount point
    pub mountpoint: Option<String>,
    /// Compression enabled
    pub compression: bool,
    /// Quota in bytes
    pub quota: Option<u64>,
    /// Reservation in bytes
    pub reservation: Option<u64>,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

/// Snapshot configuration for create operations in the universal ZFS stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotConfig {
    /// Snapshot name (e.g., "pool/dataset@snapshot-name")
    pub name: String,
    /// Dataset name (e.g., "pool/dataset")
    #[serde(default)]
    pub dataset: String,
    /// Properties to set
    pub properties: HashMap<String, String>,
}

// ==================== CANONICAL TYPE ALIASES ====================

/// Canonical name for [`DatasetConfig`] in the universal ZFS API surface.
pub type DatasetConfigCanonical = DatasetConfig;

/// Canonical name for [`SnapshotConfig`] in the universal ZFS API surface.
pub type SnapshotConfigCanonical = SnapshotConfig;
