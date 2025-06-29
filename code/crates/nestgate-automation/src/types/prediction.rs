//! Prediction types for tier prediction and ML models

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Storage tier types for prediction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TierType {
    Hot,
    Warm,
    Cold,
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

/// Access pattern data for prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    pub accesses_last_24h: u32,
    pub accesses_last_week: u32,
    pub accesses_last_month: u32,
    pub total_accesses: u64,
    pub last_access: SystemTime,
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
    pub recommended_tier: nestgate_core::StorageTier,
    pub confidence: f64,
    pub reasoning: String,
    pub alternative_tiers: Vec<(nestgate_core::StorageTier, f64)>,
    pub created_at: SystemTime,
    pub valid_until: SystemTime,
}

impl LegacyTierPrediction {
    pub fn is_valid(&self) -> bool {
        SystemTime::now() < self.valid_until
    }
}

/// AI prediction result from ecosystem services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiPredictionResult {
    pub predicted_tier: nestgate_core::StorageTier,
    pub confidence: f64,
    pub reasoning: String,
    pub file_size: u64,
    pub access_frequency: u32,
    pub alternative_predictions: Vec<(nestgate_core::StorageTier, f64)>,
}

impl Default for AiPredictionResult {
    fn default() -> Self {
        Self {
            predicted_tier: nestgate_core::StorageTier::Warm,
            confidence: 0.5,
            reasoning: "Default prediction".to_string(),
            file_size: 0,
            access_frequency: 0,
            alternative_predictions: vec![],
        }
    }
}

/// Convert TierType to legacy StorageTier
impl From<TierType> for nestgate_core::StorageTier {
    fn from(tier: TierType) -> Self {
        match tier {
            TierType::Hot => nestgate_core::StorageTier::Hot,
            TierType::Warm => nestgate_core::StorageTier::Warm,
            TierType::Cold => nestgate_core::StorageTier::Cold,
        }
    }
}

/// Convert legacy StorageTier to TierType
impl From<nestgate_core::StorageTier> for TierType {
    fn from(tier: nestgate_core::StorageTier) -> Self {
        match tier {
            nestgate_core::StorageTier::Hot => TierType::Hot,
            nestgate_core::StorageTier::Warm => TierType::Warm,
            nestgate_core::StorageTier::Cold => TierType::Cold,
            nestgate_core::StorageTier::Cache => TierType::Hot, // Map cache to hot tier
        }
    }
}
