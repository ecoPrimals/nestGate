// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Storage Type Definitions
//!
//! Storage tier classifications, file analysis, and access patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Canonical Storage Tier Classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Storagetier
pub enum StorageTier {
    /// Hot
    Hot,
    /// Warm
    Warm,
    /// Cold
    Cold,
    /// Archive
    Archive,
    /// Cache
    Cache,
}

/// Canonical File Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Fileanalysis
pub struct FileAnalysis {
    /// Path
    pub path: String,
    /// Size
    pub size: u64,
    /// Access Pattern
    pub access_pattern: AccessPattern,
    /// Recommended Tier
    pub recommended_tier: StorageTier,
    /// Compression Ratio
    pub compression_ratio: f64,
    /// Last Accessed
    pub last_accessed: SystemTime,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

/// Canonical Access Pattern Classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Accesspattern
pub enum AccessPattern {
    /// Sequential
    Sequential,
    /// Random
    Random,
    /// Writeonce
    WriteOnce,
    /// Readheavy
    ReadHeavy,
    /// Writeheavy
    WriteHeavy,
    /// Streaming
    Streaming,
}
