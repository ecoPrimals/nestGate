//! Prediction types for heuristic tier prediction

use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use nestgate_core::types::StorageTier;

/// Storage tier types for prediction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TierType {
    Hot,
    Warm,
    Cold,
}

/// File type classification for intelligent processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FileType {
    Database,
    Document,
    Image,
    Archive,
    Log,
    Backup,
    Unknown,
}

/// Access pattern data for prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPattern {
    pub accesses_last_24h: u32,
    pub accesses_last_week: u32,
    pub accesses_last_month: u32,
    pub total_accesses: u64,
    pub last_access: SystemTime,
}

/// Prediction result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    pub recommended_tier: TierType,
    pub confidence: f64,
    pub reasoning: String,
    pub alternative_tiers: Vec<TierType>,
    pub prediction_score: f64,
}

/// Access type for tracking file operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AccessType {
    Read,
    Write,
    Delete,
    Move,
    Copy,
}

impl std::fmt::Display for TierType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TierType::Hot => write!(f, "Hot"),
            TierType::Warm => write!(f, "Warm"),
            TierType::Cold => write!(f, "Cold"),
        }
    }
}

/// Confidence levels for predictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Confidence {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// File analysis result for prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub file_path: String,
    pub size_bytes: u64,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
    pub accessed_at: SystemTime,
    pub file_type: String,
}

/// Tier prediction result with confidence and alternatives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPrediction {
    pub recommended_tier: TierType,
    pub confidence: Confidence,
    pub reasoning: String,
    pub alternative_tiers: Vec<TierType>,
    pub prediction_score: f64,
}

/// Legacy tier prediction for compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyTierPrediction {
    pub recommended_tier: StorageTier,
    pub confidence: f64,
    pub reasoning: String,
    pub alternative_tiers: Vec<(StorageTier, f64)>,
    pub created_at: SystemTime,
    pub valid_until: SystemTime,
}

impl LegacyTierPrediction {
    pub fn is_valid(&self) -> bool {
        SystemTime::now() < self.valid_until
    }
}

/// Convert TierType to legacy StorageTier
impl From<TierType> for StorageTier {
    fn from(tier_type: TierType) -> Self {
        match tier_type {
            TierType::Hot => StorageTier::Hot,
            TierType::Warm => StorageTier::Warm,
            TierType::Cold => StorageTier::Cold,
        }
    }
}

/// Convert legacy StorageTier to TierType
impl From<StorageTier> for TierType {
    fn from(tier: StorageTier) -> Self {
        match tier {
            StorageTier::Hot => TierType::Hot,
            StorageTier::Warm => TierType::Warm,
            StorageTier::Cold => TierType::Cold,
            StorageTier::Cache => TierType::Hot, // Map cache to hot tier
        }
    }
}

pub enum TierClassification {
    Performance,
    Capacity,
    Archive,
}

impl From<TierClassification> for StorageTier {
    fn from(tier_class: TierClassification) -> Self {
        match tier_class {
            TierClassification::Performance => StorageTier::Hot,
            TierClassification::Capacity => StorageTier::Warm,
            TierClassification::Archive => StorageTier::Cold,
        }
    }
}

impl From<StorageTier> for TierClassification {
    fn from(tier: StorageTier) -> Self {
        match tier {
            StorageTier::Hot => TierClassification::Performance,
            StorageTier::Warm => TierClassification::Capacity,
            StorageTier::Cold => TierClassification::Archive,
            StorageTier::Cache => TierClassification::Performance, // Map cache to performance
        }
    }
}
