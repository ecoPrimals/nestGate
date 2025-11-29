use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use nestgate_core::unified_enums::StorageTier;
/// **PREDICTION TYPES - CANONICAL IMPLEMENTATION**
/// Clean type definitions for prediction and analysis functionality
/// Tier type enumeration for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Tier
pub enum TierType {
    /// Hot
    Hot,
    /// Warm
    Warm,
    /// Cold
    Cold,
}
/// Data pattern enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Datapattern
pub enum DataPattern {
    /// Sequential
    Sequential,
    /// Random
    Random,
    /// Mixed
    Mixed,
    /// Unknown
    Unknown,
}
/// File type enumeration  
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of File
pub enum FileType {
    /// Document
    Document,
    /// Image
    Image,
    /// Video
    Video,
    /// Archive
    Archive,
    /// Log
    Log,
    /// Backup
    Backup,
    /// Database
    Database,
    Other(String),
    /// Unknown
    Unknown,
}
/// Access type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Access
pub enum AccessType {
    /// Read
    Read,
    /// Write
    Write,
    /// Delete
    Delete,
    /// Modify
    Modify,
}
/// Access event structure with canonical fields
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Accessevent
pub struct AccessEvent {
    /// File Path
    pub file_path: String,
    /// Access Type
    pub access_type: AccessType,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Size Bytes
    pub size_bytes: u64,
}
/// File characteristics structure with canonical fields
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Filecharacteristics
pub struct FileCharacteristics {
    /// Estimated Compression Ratio
    pub estimated_compression_ratio: f64,
    /// Dedup Potential
    pub dedup_potential: f64,
    /// Access Frequency
    pub access_frequency: f64, // Accesses per day
    /// Size Category
    pub size_category: SizeCategory,
}
impl Default for FileCharacteristics {
    /// Returns the default instance
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
/// Sizecategory
pub enum SizeCategory {
    Small,  // < 1MB
    Medium, // 1MB - 100MB
    Large,  // 100MB - 1GB
    XLarge, // > 1GB
    /// Unknown
    Unknown,
}
/// Access pattern structure with canonical fields
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Accesspattern
pub struct AccessPattern {
    /// Accesses Last 24H
    pub accesses_last_24h: u32,
    /// Accesses Last Week
    pub accesses_last_week: u32,
    /// Accesses Last Month
    pub accesses_last_month: u32,
    /// Total Accesses
    pub total_accesses: u32,
    /// Last Access
    pub last_access: SystemTime,
    /// Peak Access Times
    pub peak_access_times: Vec<u8>, // Hours of day (0-23)
    /// Read Write Ratio
    pub read_write_ratio: f64, // Read operations / Write operations
}
impl Default for AccessPattern {
    /// Returns the default instance
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
/// Fileanalysis
pub struct FileAnalysis {
    /// File Path
    pub file_path: String,
    /// Size Bytes
    pub size_bytes: u64,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Modified At
    pub modified_at: SystemTime,
    /// Accessed At
    pub accessed_at: SystemTime,
    /// File Type
    pub file_type: String,
}
/// Tier prediction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tierprediction
pub struct TierPrediction {
    /// Predicted Tier
    pub predicted_tier: StorageTier,
    /// Confidence Score
    pub confidence_score: f64,
    /// Accesses Last 24H
    pub accesses_last_24h: u32,
    /// Accesses Last Week
    pub accesses_last_week: u32,
    /// Accesses Last Month
    pub accesses_last_month: u32,
    /// Size Bytes
    pub size_bytes: u64,
    /// File Type
    pub file_type: String,
    /// Recommendation Reason
    pub recommendation_reason: String,
}
/// Data migration instruction
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Datamigration
pub struct DataMigration {
    /// File Path
    pub file_path: String,
    /// Size Bytes
    pub size_bytes: u64,
    /// Current Tier
    pub current_tier: StorageTier,
    /// Target Tier
    pub target_tier: StorageTier,
    /// Migration Time
    pub migration_time: SystemTime,
    /// Accessed At
    pub accessed_at: SystemTime,
    /// Tier Prediction
    pub tier_prediction: TierPrediction,
}
/// Legacy tier prediction for compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Legacytierprediction
pub struct LegacyTierPrediction {
    /// Recommended Tier
    pub recommended_tier: StorageTier,
    /// Confidence
    pub confidence: f64,
    /// Reasoning
    pub reasoning: String,
    /// Alternative Tiers
    pub alternative_tiers: Vec<(StorageTier, f64)>,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Valid Until
    pub valid_until: SystemTime,
}
impl LegacyTierPrediction {
    #[must_use]
    pub fn is_valid(&self) -> bool {
        SystemTime::now() < self.valid_until
    }
}

/// Convert `TierType` to legacy `StorageTier`
impl From<TierType> for StorageTier {
    /// From
    fn from(tier_type: TierType) -> Self {
        match tier_type {
            TierType::Hot => StorageTier::Hot,
            TierType::Warm => StorageTier::Warm,
            TierType::Cold => StorageTier::Cold,
        }
    }
}
/// Convert legacy `StorageTier` to `TierType`
impl From<StorageTier> for TierType {
    /// From
    fn from(tier: StorageTier) -> Self {
        match tier {
            StorageTier::Hot => TierType::Hot,
            StorageTier::Warm => TierType::Warm,
            StorageTier::Cold => TierType::Cold,
            StorageTier::Cool => TierType::Cold, // Map cool to cold
            StorageTier::Frozen => TierType::Cold, // Map frozen to cold
            _ => TierType::Cold,                 // Default for any other variants
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tierclassification
pub enum TierClassification {
    /// Performance
    Performance,
    /// Balanced
    Balanced,
    /// Archive
    Archive,
}

impl From<StorageTier> for TierClassification {
    /// From
    fn from(tier: StorageTier) -> Self {
        match tier {
            StorageTier::Hot => TierClassification::Performance,
            StorageTier::Warm => TierClassification::Balanced,
            StorageTier::Cold => TierClassification::Archive,
            StorageTier::Cool => TierClassification::Archive, // Map cool to archive
            StorageTier::Frozen => TierClassification::Archive, // Map frozen to archive
            _ => TierClassification::Balanced,                // Default for any other variants
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_type_to_storage_tier() {
        let hot: StorageTier = TierType::Hot.into();
        let warm: StorageTier = TierType::Warm.into();
        let cold: StorageTier = TierType::Cold.into();

        match hot {
            StorageTier::Hot => {}
            _ => panic!("Expected Hot tier"),
        }
        match warm {
            StorageTier::Warm => {}
            _ => panic!("Expected Warm tier"),
        }
        match cold {
            StorageTier::Cold => {}
            _ => panic!("Expected Cold tier"),
        }
    }

    #[test]
    fn test_storage_tier_to_tier_type() {
        let hot: TierType = StorageTier::Hot.into();
        let warm: TierType = StorageTier::Warm.into();
        let cold: TierType = StorageTier::Cold.into();

        match hot {
            TierType::Hot => {}
            _ => panic!("Expected Hot tier"),
        }
        match warm {
            TierType::Warm => {}
            _ => panic!("Expected Warm tier"),
        }
        match cold {
            TierType::Cold => {}
            _ => panic!("Expected Cold tier"),
        }
    }

    #[test]
    fn test_tier_classification_from_storage_tier() {
        let performance: TierClassification = StorageTier::Hot.into();
        let balanced: TierClassification = StorageTier::Warm.into();
        let archive: TierClassification = StorageTier::Cold.into();

        match performance {
            TierClassification::Performance => {}
            _ => panic!("Expected Performance"),
        }
        match balanced {
            TierClassification::Balanced => {}
            _ => panic!("Expected Balanced"),
        }
        match archive {
            TierClassification::Archive => {}
            _ => panic!("Expected Archive"),
        }
    }

    #[test]
    fn test_file_characteristics_default() {
        let chars = FileCharacteristics::default();
        assert_eq!(chars.estimated_compression_ratio, 1.0);
        assert_eq!(chars.dedup_potential, 0.0);
        assert_eq!(chars.access_frequency, 0.0);
    }

    #[test]
    fn test_file_characteristics_custom() {
        let chars = FileCharacteristics {
            estimated_compression_ratio: 2.5,
            dedup_potential: 0.8,
            access_frequency: 15.0,
            size_category: SizeCategory::Large,
        };

        assert_eq!(chars.estimated_compression_ratio, 2.5);
        assert_eq!(chars.dedup_potential, 0.8);
        assert_eq!(chars.access_frequency, 15.0);
    }

    #[test]
    fn test_access_pattern_default() {
        let pattern = AccessPattern::default();
        assert_eq!(pattern.accesses_last_24h, 0);
        assert_eq!(pattern.accesses_last_week, 0);
        assert_eq!(pattern.accesses_last_month, 0);
        assert_eq!(pattern.read_write_ratio, 3.0);
    }

    #[test]
    fn test_access_pattern_custom() {
        let pattern = AccessPattern {
            accesses_last_24h: 50,
            accesses_last_week: 200,
            accesses_last_month: 800,
            total_accesses: 5000,
            last_access: std::time::SystemTime::now(),
            peak_access_times: vec![9, 10, 14, 15],
            read_write_ratio: 5.0,
        };

        assert_eq!(pattern.accesses_last_24h, 50);
        assert_eq!(pattern.accesses_last_week, 200);
        assert_eq!(pattern.read_write_ratio, 5.0);
    }

    #[test]
    fn test_legacy_tier_prediction_is_valid() {
        let now = std::time::SystemTime::now();
        let future = now + std::time::Duration::from_secs(3600);

        let prediction = LegacyTierPrediction {
            recommended_tier: StorageTier::Hot,
            confidence: 0.95,
            reasoning: "High access frequency".to_string(),
            alternative_tiers: vec![],
            created_at: now,
            valid_until: future,
        };

        assert!(prediction.is_valid());
    }

    #[test]
    fn test_legacy_tier_prediction_is_expired() {
        let now = std::time::SystemTime::now();
        let past = now - std::time::Duration::from_secs(3600);

        let prediction = LegacyTierPrediction {
            recommended_tier: StorageTier::Hot,
            confidence: 0.95,
            reasoning: "High access frequency".to_string(),
            alternative_tiers: vec![],
            created_at: past - std::time::Duration::from_secs(3600),
            valid_until: past,
        };

        assert!(!prediction.is_valid());
    }

    #[test]
    fn test_access_event_creation() {
        let event = AccessEvent {
            file_path: "/data/file.txt".to_string(),
            access_type: AccessType::Read,
            timestamp: std::time::SystemTime::now(),
            size_bytes: 1024,
        };

        assert_eq!(event.file_path, "/data/file.txt");
        assert_eq!(event.size_bytes, 1024);
    }

    #[test]
    fn test_tier_prediction_creation() {
        let prediction = TierPrediction {
            predicted_tier: StorageTier::Hot,
            confidence_score: 0.85,
            accesses_last_24h: 100,
            accesses_last_week: 500,
            accesses_last_month: 2000,
            size_bytes: 1024 * 1024,
            file_type: "document".to_string(),
            recommendation_reason: "High frequency access".to_string(),
        };

        assert_eq!(prediction.accesses_last_24h, 100);
        assert_eq!(prediction.confidence_score, 0.85);
    }

    #[test]
    fn test_data_migration_creation() {
        let now = std::time::SystemTime::now();

        let prediction = TierPrediction {
            predicted_tier: StorageTier::Warm,
            confidence_score: 0.9,
            accesses_last_24h: 5,
            accesses_last_week: 20,
            accesses_last_month: 50,
            size_bytes: 1024 * 1024,
            file_type: "document".to_string(),
            recommendation_reason: "Decreased access".to_string(),
        };

        let migration = DataMigration {
            file_path: "/data/file.txt".to_string(),
            size_bytes: 1024 * 1024,
            current_tier: StorageTier::Hot,
            target_tier: StorageTier::Warm,
            migration_time: now,
            accessed_at: now,
            tier_prediction: prediction,
        };

        assert_eq!(migration.current_tier, StorageTier::Hot);
        assert_eq!(migration.target_tier, StorageTier::Warm);
    }

    #[test]
    fn test_file_analysis_creation() {
        let now = std::time::SystemTime::now();

        let analysis = FileAnalysis {
            file_path: "/data/file.txt".to_string(),
            size_bytes: 2048,
            created_at: now,
            modified_at: now,
            accessed_at: now,
            file_type: "text".to_string(),
        };

        assert_eq!(analysis.file_path, "/data/file.txt");
        assert_eq!(analysis.size_bytes, 2048);
        assert_eq!(analysis.file_type, "text");
    }

    #[test]
    fn test_size_category_variants() {
        let _small = SizeCategory::Small;
        let _medium = SizeCategory::Medium;
        let _large = SizeCategory::Large;
        let _xlarge = SizeCategory::XLarge;
        let _unknown = SizeCategory::Unknown;
    }

    #[test]
    fn test_data_pattern_variants() {
        let _sequential = DataPattern::Sequential;
        let _random = DataPattern::Random;
        let _mixed = DataPattern::Mixed;
        let _unknown = DataPattern::Unknown;
    }

    #[test]
    fn test_file_type_variants() {
        let _doc = FileType::Document;
        let _image = FileType::Image;
        let _video = FileType::Video;
        let _archive = FileType::Archive;
        let _log = FileType::Log;
        let _backup = FileType::Backup;
        let _db = FileType::Database;
        let _other = FileType::Other("custom".to_string());
        let _unknown = FileType::Unknown;
    }

    #[test]
    fn test_access_type_variants() {
        let _read = AccessType::Read;
        let _write = AccessType::Write;
        let _delete = AccessType::Delete;
        let _modify = AccessType::Modify;
    }
}
