// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Value types for file and dataset analysis.

use std::collections::HashMap;

use nestgate_core::unified_enums::StorageTier;
use serde::{Deserialize, Serialize};

use crate::types::prediction::{AccessPattern, DataPattern, SizeCategory};

/// File characteristics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCharacteristics {
    /// Size Category
    pub size_category: SizeCategory,
    /// Access Frequency
    pub access_frequency: u32,
    /// Whether frequently accessed
    pub is_frequently_accessed: bool,
    /// Whether sequential access
    pub is_sequential_access: bool,
    /// Data Pattern
    pub data_pattern: DataPattern,
}

/// Dataset analysis structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetAnalysis {
    /// Path
    pub path: String,
    /// Total Files
    pub total_files: u64,
    /// Total Size Bytes
    pub total_size_bytes: u64,
    /// File Types
    pub file_types: HashMap<String, u64>,
    /// Characteristics
    pub characteristics: FileCharacteristics,
}

/// Dataset summary with access patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetSummary {
    /// Dataset name
    pub dataset_name: String,
    /// Total Files
    pub total_files: usize,
    /// Total Size Bytes
    pub total_size_bytes: u64,
    /// Size of average file
    pub average_file_size: u64,
    /// File Types
    pub file_types: HashMap<String, usize>,
    /// Access Pattern
    pub access_pattern: AccessPattern,
    /// Compressible Files
    pub compressible_files: usize,
    /// Dedupable Files
    pub dedupable_files: usize,
}

/// Tier recommendation from path heuristics (extension-based).
pub fn storage_tier_from_extension(file_path: &str) -> StorageTier {
    let extension = std::path::Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension {
        "log" | "tmp" | "bak" => StorageTier::Cold,
        "mp4" | "mkv" | "avi" => StorageTier::Hot,
        "doc" | "pdf" | "txt" | _ => StorageTier::Warm,
    }
}
