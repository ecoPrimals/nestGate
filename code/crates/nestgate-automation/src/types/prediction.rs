
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use nestgate_core::unified_enums::StorageTier;
/// **PREDICTION TYPES - CANONICAL IMPLEMENTATION**
/// Clean type definitions for prediction and analysis functionality

/// Tier type enumeration for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TierType {
    Hot,
    Warm,
    Cold,
}

/// Data pattern enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataPattern {
    Sequential,
    Random,
    Mixed,
    Unknown,
}

/// File type enumeration  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    Document,
    Image,
    Video,
    Archive,
    Log,
    Backup,
    Database,
    Other(String),
    Unknown,
}

/// Access type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessType {
    Read,
    Write,
    Delete,
    Modify,
}

/// Access event structure with canonical fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessEvent {
    pub file_path: String,
    pub access_type: AccessType,
    pub timestamp: SystemTime,
    pub size_bytes: u64,
}

/// File characteristics structure with canonical fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCharacteristics {
    pub estimated_compression_ratio: f64,
    pub dedup_potential: f64,
    pub access_frequency: f64, // Accesses per day
    pub size_category: SizeCategory,
}

impl Default for FileCharacteristics {
    fn default() -> Self {
        Self {
            estimated_compression_ratio: 1.0,
            dedup_potential: 0.0,
            access_frequency: 0.0,
            size_category: SizeCategory::Small,
        }
    }
}

/// Size category enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SizeCategory {
    Small,  // < 1MB
    Medium, // 1MB - 100MB
    Large,  // 100MB - 1GB
    XLarge, // > 1GB
    Unknown,
}

/// Access pattern structure with canonical fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    pub accesses_last_24h: u32,
    pub accesses_last_week: u32,
    pub accesses_last_month: u32,
    pub total_accesses: u32,
    pub last_access: SystemTime,
    pub peak_access_times: Vec<u8>, // Hours of day (0-23)
    pub read_write_ratio: f64,      // Read operations / Write operations
}

impl Default for AccessPattern {
    fn default() -> Self {
        Self {
            accesses_last_24h: 0,
            accesses_last_week: 0,
            accesses_last_month: 0,
            total_accesses: 0,
            last_access: SystemTime::now(),
            peak_access_times: vec![9, 10, 11, 14, 15, 16], // Default business hours
            read_write_ratio: 3.0,                          // Default 3:1 read/write ratio
        }
    }
}

/// File analysis structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAnalysis {
    pub file_path: String,
    pub size_bytes: u64,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
    pub accessed_at: SystemTime,
    pub file_type: String,
}

/// Tier prediction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPrediction {
    pub predicted_tier: StorageTier,
    pub confidence_score: f64,
    pub accesses_last_24h: u32,
    pub accesses_last_week: u32,
    pub accesses_last_month: u32,
    pub size_bytes: u64,
    pub file_type: String,
    pub recommendation_reason: String,
}

/// Data migration instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMigration {
    pub file_path: String,
    pub size_bytes: u64,
    pub current_tier: StorageTier,
    pub target_tier: StorageTier,
    pub migration_time: SystemTime,
    pub accessed_at: SystemTime,
    pub tier_prediction: TierPrediction,
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
            StorageTier::Cool => TierType::Cold,  // Map cool to cold
            StorageTier::Frozen => TierType::Cold, // Map frozen to cold
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TierClassification {
    Performance,
    Balanced,
    Archive,
}

impl From<StorageTier> for TierClassification {
    fn from(tier: StorageTier) -> Self {
        match tier {
            StorageTier::Hot => TierClassification::Performance,
            StorageTier::Warm => TierClassification::Balanced,
            StorageTier::Cold => TierClassification::Archive,
            StorageTier::Cool => TierClassification::Archive,  // Map cool to archive
            StorageTier::Frozen => TierClassification::Archive, // Map frozen to archive
        }
    }
}
