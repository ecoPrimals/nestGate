//! Storage Type Definitions
//!
//! Storage tier classifications, file analysis, and access patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Canonical Storage Tier Classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageTier {
    Hot,
    Warm,
    Cold,
    Archive,
    Cache,
}

/// Canonical File Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub path: String,
    pub size: u64,
    pub access_pattern: AccessPattern,
    pub recommended_tier: StorageTier,
    pub compression_ratio: f64,
    pub last_accessed: SystemTime,
    pub metadata: HashMap<String, String>,
}

/// Canonical Access Pattern Classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessPattern {
    Sequential,
    Random,
    WriteOnce,
    ReadHeavy,
    WriteHeavy,
    Streaming,
}
