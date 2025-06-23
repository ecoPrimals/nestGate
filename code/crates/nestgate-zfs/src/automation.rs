//! ZFS Dataset Automation
//!
//! Intelligent dataset lifecycle management with automated tier assignment
//! and performance optimization based on data characteristics and access patterns.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use serde::{Serialize, Deserialize};

use nestgate_core::{Result as CoreResult, NestGateError};
use crate::{
    config::ZfsConfig,
    pool::ZfsPoolManager,
    dataset::{ZfsDatasetManager, DatasetConfig},
    types::{StorageTier, CompressionAlgorithm, DatasetProperty},
    error::ZfsError,
};

/// File characteristics for tier assignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCharacteristics {
    /// File size in bytes
    pub size: u64,
    /// File extension
    pub extension: String,
    /// MIME type
    pub mime_type: String,
    /// Creation time
    pub created_at: SystemTime,
    /// Last access time
    pub accessed_at: SystemTime,
    /// Last modification time
    pub modified_at: SystemTime,
    /// Access frequency (accesses per hour)
    pub access_frequency: f64,
    /// Read/write ratio (0.0 = write-only, 1.0 = read-only)
    pub read_write_ratio: f64,
}

/// Tier assignment recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierRecommendation {
    /// Recommended tier
    pub tier: StorageTier,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Reasoning for the recommendation
    pub reasoning: String,
    /// Expected performance characteristics
    pub expected_performance: PerformanceExpectation,
}

/// Expected performance characteristics for a tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceExpectation {
    /// Expected latency in milliseconds
    pub latency_ms: f64,
    /// Expected throughput in MB/s
    pub throughput_mbs: f64,
    /// Expected IOPS
    pub iops: f64,
    /// Compression ratio
    pub compression_ratio: f64,
}

/// Access pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    /// File path
    pub path: PathBuf,
    /// Access timestamps
    pub access_times: Vec<SystemTime>,
    /// Total accesses
    pub total_accesses: u64,
    /// Read operations
    pub read_operations: u64,
    /// Write operations
    pub write_operations: u64,
    /// Average time between accesses
    pub avg_access_interval: Duration,
    /// Peak access times (hour of day)
    pub peak_hours: Vec<u8>,
}

/// Dataset analyzer for intelligent tier assignment
#[derive(Debug)]
pub struct DatasetAnalyzer {
    config: ZfsConfig,
    pool_manager: Arc<ZfsPoolManager>,
    dataset_manager: Arc<ZfsDatasetManager>,
    access_patterns: Arc<RwLock<HashMap<PathBuf, AccessPattern>>>,
    tier_thresholds: TierThresholds,
}

/// Thresholds for tier assignment
#[derive(Debug, Clone)]
pub struct TierThresholds {
    /// Hot tier: access frequency threshold (accesses per hour)
    pub hot_access_frequency: f64,
    /// Warm tier: access frequency threshold (accesses per hour)
    pub warm_access_frequency: f64,
    /// File size thresholds for different tiers (bytes)
    pub size_thresholds: HashMap<StorageTier, u64>,
    /// Age thresholds for tier assignment (days)
    pub age_thresholds: HashMap<StorageTier, u64>,
}

impl Default for TierThresholds {
    fn default() -> Self {
        let mut size_thresholds = HashMap::new();
        size_thresholds.insert(StorageTier::Hot, 100 * 1024 * 1024);    // 100MB
        size_thresholds.insert(StorageTier::Warm, 1024 * 1024 * 1024);  // 1GB
        size_thresholds.insert(StorageTier::Cold, u64::MAX);            // No limit
        
        let mut age_thresholds = HashMap::new();
        age_thresholds.insert(StorageTier::Hot, 7);    // 7 days
        age_thresholds.insert(StorageTier::Warm, 30);  // 30 days
        age_thresholds.insert(StorageTier::Cold, 90);  // 90 days
        
        Self {
            hot_access_frequency: 10.0,   // 10 accesses per hour
            warm_access_frequency: 1.0,   // 1 access per hour
            size_thresholds,
            age_thresholds,
        }
    }
}

impl DatasetAnalyzer {
    /// Create a new dataset analyzer
    pub fn new(
        config: ZfsConfig,
        pool_manager: Arc<ZfsPoolManager>,
        dataset_manager: Arc<ZfsDatasetManager>,
    ) -> Self {
        Self {
            config,
            pool_manager,
            dataset_manager,
            access_patterns: Arc::new(RwLock::new(HashMap::new())),
            tier_thresholds: TierThresholds::default(),
        }
    }

    /// Analyze file characteristics for tier assignment
    pub async fn analyze_file(&self, file_path: &Path) -> CoreResult<FileCharacteristics> {
        debug!("Analyzing file characteristics: {:?}", file_path);
        
        let metadata = tokio::fs::metadata(file_path).await
            .map_err(|e| NestGateError::Internal(format!("Failed to read file metadata: {}", e)))?;
        
        let extension = file_path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        let mime_type = self.detect_mime_type(&extension);
        
        let created_at = metadata.created()
            .unwrap_or(UNIX_EPOCH);
        
        let accessed_at = metadata.accessed()
            .unwrap_or(UNIX_EPOCH);
        
        let modified_at = metadata.modified()
            .unwrap_or(UNIX_EPOCH);
        
        // Get access pattern if available
        let access_patterns = self.access_patterns.read().await;
        let (access_frequency, read_write_ratio) = if let Some(pattern) = access_patterns.get(file_path) {
            let frequency = pattern.total_accesses as f64 / 
                (pattern.access_times.len().max(1) as f64);
            let ratio = if pattern.read_operations + pattern.write_operations > 0 {
                pattern.read_operations as f64 / 
                (pattern.read_operations + pattern.write_operations) as f64
            } else {
                0.5 // Default to balanced
            };
            (frequency, ratio)
        } else {
            (0.0, 0.5) // No pattern data available
        };
        
        Ok(FileCharacteristics {
            size: metadata.len(),
            extension,
            mime_type,
            created_at,
            accessed_at,
            modified_at,
            access_frequency,
            read_write_ratio,
        })
    }

    /// Recommend tier assignment based on file characteristics
    pub async fn recommend_tier(&self, characteristics: &FileCharacteristics) -> CoreResult<TierRecommendation> {
        debug!("Recommending tier for file with characteristics: {:?}", characteristics);
        
        let mut score_hot = 0.0;
        let mut score_warm = 0.0;
        let mut score_cold = 0.0;
        let mut reasoning = Vec::new();
        
        // Access frequency scoring
        if characteristics.access_frequency >= self.tier_thresholds.hot_access_frequency {
            score_hot += 0.4;
            reasoning.push("High access frequency favors hot tier".to_string());
        } else if characteristics.access_frequency >= self.tier_thresholds.warm_access_frequency {
            score_warm += 0.3;
            reasoning.push("Moderate access frequency favors warm tier".to_string());
        } else {
            score_cold += 0.3;
            reasoning.push("Low access frequency favors cold tier".to_string());
        }
        
        // File age scoring
        let age_days = characteristics.created_at.elapsed()
            .unwrap_or(Duration::ZERO)
            .as_secs() / (24 * 3600);
        
        if age_days <= *self.tier_thresholds.age_thresholds.get(&StorageTier::Hot).unwrap_or(&7) {
            score_hot += 0.2;
            reasoning.push("Recent file favors hot tier".to_string());
        } else if age_days <= *self.tier_thresholds.age_thresholds.get(&StorageTier::Warm).unwrap_or(&30) {
            score_warm += 0.2;
            reasoning.push("Moderately old file favors warm tier".to_string());
        } else {
            score_cold += 0.3;
            reasoning.push("Old file favors cold tier".to_string());
        }
        
        // File size scoring
        if characteristics.size <= *self.tier_thresholds.size_thresholds.get(&StorageTier::Hot).unwrap_or(&(100 * 1024 * 1024)) {
            score_hot += 0.2;
            reasoning.push("Small file size favors hot tier".to_string());
        } else if characteristics.size <= *self.tier_thresholds.size_thresholds.get(&StorageTier::Warm).unwrap_or(&(1024 * 1024 * 1024)) {
            score_warm += 0.2;
            reasoning.push("Medium file size favors warm tier".to_string());
        } else {
            score_cold += 0.2;
            reasoning.push("Large file size favors cold tier".to_string());
        }
        
        // File type scoring
        match characteristics.mime_type.as_str() {
            mime if mime.starts_with("application/") => {
                score_hot += 0.2;
                reasoning.push("Application file type favors hot tier".to_string());
            }
            mime if mime.starts_with("image/") || mime.starts_with("video/") => {
                score_warm += 0.3;
                reasoning.push("Media file type favors warm tier".to_string());
            }
            _ => {
                score_cold += 0.2;
                reasoning.push("Generic file type favors cold tier".to_string());
            }
        }
        
        // Determine recommended tier
        let (tier, confidence) = if score_hot >= score_warm && score_hot >= score_cold {
            (StorageTier::Hot, score_hot)
        } else if score_warm >= score_cold {
            (StorageTier::Warm, score_warm)
        } else {
            (StorageTier::Cold, score_cold)
        };
        
        let expected_performance = self.get_expected_performance(tier);
        
        Ok(TierRecommendation {
            tier,
            confidence,
            reasoning: reasoning.join("; "),
            expected_performance,
        })
    }

    /// Get expected performance characteristics for a tier
    fn get_expected_performance(&self, tier: StorageTier) -> PerformanceExpectation {
        match tier {
            StorageTier::Hot => PerformanceExpectation {
                latency_ms: 0.5,
                throughput_mbs: 3000.0,
                iops: 100000.0,
                compression_ratio: 1.2,
            },
            StorageTier::Warm => PerformanceExpectation {
                latency_ms: 5.0,
                throughput_mbs: 1000.0,
                iops: 10000.0,
                compression_ratio: 2.0,
            },
            StorageTier::Cold => PerformanceExpectation {
                latency_ms: 50.0,
                throughput_mbs: 500.0,
                iops: 1000.0,
                compression_ratio: 4.0,
            },
        }
    }

    /// Detect MIME type based on file extension
    fn detect_mime_type(&self, extension: &str) -> String {
        match extension {
            "txt" | "md" | "rst" => "text/plain",
            "html" | "htm" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            "json" => "application/json",
            "xml" => "application/xml",
            "pdf" => "application/pdf",
            "zip" => "application/zip",
            "tar" | "gz" => "application/gzip",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "svg" => "image/svg+xml",
            "mp4" => "video/mp4",
            "avi" => "video/avi",
            "mov" => "video/quicktime",
            "mp3" => "audio/mpeg",
            "wav" => "audio/wav",
            "flac" => "audio/flac",
            _ => "application/octet-stream",
        }.to_string()
    }

    /// Record file access for pattern learning
    pub async fn record_access(&self, file_path: &Path, is_read: bool) -> CoreResult<()> {
        let mut patterns = self.access_patterns.write().await;
        let pattern = patterns.entry(file_path.to_path_buf()).or_insert_with(|| {
            AccessPattern {
                path: file_path.to_path_buf(),
                access_times: Vec::new(),
                total_accesses: 0,
                read_operations: 0,
                write_operations: 0,
                avg_access_interval: Duration::ZERO,
                peak_hours: Vec::new(),
            }
        });
        
        let now = SystemTime::now();
        pattern.access_times.push(now);
        pattern.total_accesses += 1;
        
        if is_read {
            pattern.read_operations += 1;
        } else {
            pattern.write_operations += 1;
        }
        
        // Update average access interval
        if pattern.access_times.len() > 1 {
            let total_duration: Duration = pattern.access_times.windows(2)
                .map(|window| window[1].duration_since(window[0]).unwrap_or(Duration::ZERO))
                .sum();
            pattern.avg_access_interval = total_duration / (pattern.access_times.len() - 1) as u32;
        }
        
        // Update peak hours
        if let Ok(duration) = now.duration_since(UNIX_EPOCH) {
            let hour = (duration.as_secs() / 3600) % 24;
            if !pattern.peak_hours.contains(&(hour as u8)) {
                pattern.peak_hours.push(hour as u8);
            }
        }
        
        debug!("Recorded access for {:?}: {} total accesses", file_path, pattern.total_accesses);
        Ok(())
    }
}

/// Automated dataset creator for intelligent tier management
#[derive(Debug)]
pub struct AutomatedDatasetCreator {
    config: ZfsConfig,
    pool_manager: Arc<ZfsPoolManager>,
    dataset_manager: Arc<ZfsDatasetManager>,
    analyzer: Arc<DatasetAnalyzer>,
    tier_configs: HashMap<StorageTier, DatasetConfig>,
}

impl AutomatedDatasetCreator {
    /// Create a new automated dataset creator
    pub fn new(
        config: ZfsConfig,
        pool_manager: Arc<ZfsPoolManager>,
        dataset_manager: Arc<ZfsDatasetManager>,
        analyzer: Arc<DatasetAnalyzer>,
    ) -> Self {
        let tier_configs = Self::create_tier_configs();
        
        Self {
            config,
            pool_manager,
            dataset_manager,
            analyzer,
            tier_configs,
        }
    }

    /// Create tier-specific dataset configurations
    fn create_tier_configs() -> HashMap<StorageTier, DatasetConfig> {
        let mut configs = HashMap::new();
        
        // Hot tier configuration - optimized for performance
        configs.insert(StorageTier::Hot, DatasetConfig {
            name: "hot".to_string(),
            parent: "nestpool".to_string(),
            tier: nestgate_core::StorageTier::Hot,
            compression: CompressionAlgorithm::Lz4,
            record_size: 128 * 1024, // 128K for high performance
            quota: Some(500 * 1024 * 1024 * 1024), // 500GB quota
            reservation: Some(100 * 1024 * 1024 * 1024), // 100GB reservation
            properties: vec![
                DatasetProperty::new("atime", "off"), // Disable access time updates
                DatasetProperty::new("sync", "disabled"), // Disable sync for performance
                DatasetProperty::new("primarycache", "all"), // Cache everything
                DatasetProperty::new("secondarycache", "all"),
            ],
        });
        
        // Warm tier configuration - balanced performance and compression
        configs.insert(StorageTier::Warm, DatasetConfig {
            name: "warm".to_string(),
            parent: "nestpool".to_string(),
            tier: nestgate_core::StorageTier::Warm,
            compression: CompressionAlgorithm::Zstd,
            record_size: 1024 * 1024, // 1MB for balanced performance
            quota: Some(1000 * 1024 * 1024 * 1024), // 1TB quota
            reservation: Some(200 * 1024 * 1024 * 1024), // 200GB reservation
            properties: vec![
                DatasetProperty::new("atime", "off"),
                DatasetProperty::new("sync", "standard"),
                DatasetProperty::new("primarycache", "metadata"),
                DatasetProperty::new("secondarycache", "all"),
            ],
        });
        
        // Cold tier configuration - optimized for storage efficiency
        configs.insert(StorageTier::Cold, DatasetConfig {
            name: "cold".to_string(),
            parent: "nestpool".to_string(),
            tier: nestgate_core::StorageTier::Cold,
            compression: CompressionAlgorithm::Gzip9,
            record_size: 1024 * 1024, // 1MB for compression efficiency
            quota: None, // No quota limit for cold storage
            reservation: None, // No reservation for cold storage
            properties: vec![
                DatasetProperty::new("atime", "off"),
                DatasetProperty::new("sync", "always"), // Ensure data integrity
                DatasetProperty::new("primarycache", "none"), // Minimal caching
                DatasetProperty::new("secondarycache", "metadata"),
                DatasetProperty::new("dedup", "on"), // Enable deduplication
            ],
        });
        
        configs
    }

    /// Automatically create or update dataset for a file
    pub async fn ensure_dataset_for_file(&self, file_path: &Path) -> CoreResult<String> {
        info!("Ensuring dataset for file: {:?}", file_path);
        
        // Analyze file characteristics
        let characteristics = self.analyzer.analyze_file(file_path).await?;
        
        // Get tier recommendation
        let recommendation = self.analyzer.recommend_tier(&characteristics).await?;
        
        info!("Tier recommendation for {:?}: {:?} (confidence: {:.2})", 
              file_path, recommendation.tier, recommendation.confidence);
        
        // Get or create dataset for the tier
        let dataset_name = self.ensure_tier_dataset(recommendation.tier).await?;
        
        // Record the tier assignment decision
        self.record_tier_assignment(file_path, &recommendation).await?;
        
        Ok(dataset_name)
    }

    /// Ensure a dataset exists for the specified tier
    async fn ensure_tier_dataset(&self, tier: StorageTier) -> CoreResult<String> {
        let config = self.tier_configs.get(&tier).ok_or_else(|| {
            NestGateError::Internal(format!("No configuration for tier: {:?}", tier))
        })?;
        
        let dataset_name = format!("{}/{}", self.config.default_pool, config.name);
        
        // Create dataset if it doesn't exist
        self.dataset_manager.create_dataset(&config.name, &config.parent, config.tier.clone()).await
            .map_err(|e| NestGateError::Internal(format!("Failed to create dataset: {}", e)))?;
        
        // Verify dataset was created
        self.dataset_manager.get_dataset_info(&dataset_name).await
            .map_err(|e| NestGateError::Internal(format!("Failed to verify dataset creation: {}", e)))?;
        
        Ok(dataset_name)
    }

    /// Record tier assignment decision for analysis and optimization
    async fn record_tier_assignment(&self, file_path: &Path, recommendation: &TierRecommendation) -> CoreResult<()> {
        // In a real implementation, this would store the decision in a database
        // or persistent storage for later analysis and model improvement
        debug!("Recording tier assignment: {:?} -> {:?} (confidence: {:.2}, reasoning: {})",
               file_path, recommendation.tier, recommendation.confidence, recommendation.reasoning);
        
        // For now, just log the decision
        info!("📊 Tier Assignment: {} -> {:?} ({:.1}% confidence)", 
              file_path.display(), recommendation.tier, recommendation.confidence * 100.0);
        
        Ok(())
    }

    /// Get dataset path for a tier
    pub fn get_dataset_path(&self, tier: StorageTier) -> PathBuf {
        let tier_name = match tier {
            StorageTier::Hot => "hot",
            StorageTier::Warm => "warm",
            StorageTier::Cold => "cold",
        };
        
        PathBuf::from(format!("/nestpool/{}", tier_name))
    }

    /// Get tier statistics
    pub async fn get_tier_statistics(&self) -> CoreResult<HashMap<StorageTier, TierStatistics>> {
        let mut stats = HashMap::new();
        
        for tier in [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold] {
            let dataset_name = format!("nestpool/{}", match tier {
                StorageTier::Hot => "hot",
                StorageTier::Warm => "warm", 
                StorageTier::Cold => "cold",
            });
            
            if let Ok(dataset_info) = self.dataset_manager.get_dataset_info(&dataset_name).await {
                let tier_stats = TierStatistics {
                    tier,
                    total_space: dataset_info.used_space + dataset_info.available_space,
                    used_space: dataset_info.used_space,
                    available_space: dataset_info.available_space,
                    file_count: dataset_info.file_count.unwrap_or(0),
                    compression_ratio: dataset_info.compression_ratio.unwrap_or(1.0),
                    average_file_size: if dataset_info.file_count.unwrap_or(0) > 0 {
                        dataset_info.used_space / dataset_info.file_count.unwrap_or(1)
                    } else {
                        0
                    },
                };
                stats.insert(tier, tier_stats);
            }
        }
        
        Ok(stats)
    }

    /// Optimize tier configurations based on usage patterns
    pub async fn optimize_tier_configurations(&self) -> CoreResult<()> {
        info!("🔧 Optimizing tier configurations based on usage patterns");
        
        let statistics = self.get_tier_statistics().await?;
        
        for (tier, stats) in statistics {
            let utilization = stats.used_space as f64 / stats.total_space as f64;
            
            info!("📈 Tier {:?}: {:.1}% utilized, {} files, {:.2}x compression",
                  tier, utilization * 100.0, stats.file_count, stats.compression_ratio);
            
            // Suggest optimizations based on utilization
            if utilization > 0.9 {
                warn!("⚠️  Tier {:?} is over 90% utilized - consider expanding", tier);
            } else if utilization < 0.1 {
                info!("💡 Tier {:?} is under-utilized - consider reducing allocation", tier);
            }
        }
        
        Ok(())
    }
}

/// Statistics for a storage tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierStatistics {
    /// Storage tier
    pub tier: StorageTier,
    /// Total space in bytes
    pub total_space: u64,
    /// Used space in bytes
    pub used_space: u64,
    /// Available space in bytes
    pub available_space: u64,
    /// Number of files
    pub file_count: u64,
    /// Compression ratio
    pub compression_ratio: f64,
    /// Average file size in bytes
    pub average_file_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_tier_recommendation() {
        let config = ZfsConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new(&config).await.unwrap());
        let dataset_manager = Arc::new(ZfsDatasetManager::new(config.clone(), pool_manager.clone()));
        let analyzer = DatasetAnalyzer::new(config, pool_manager, dataset_manager);
        
        // Test hot tier recommendation for frequently accessed small file
        let characteristics = FileCharacteristics {
            size: 1024 * 1024, // 1MB
            extension: "txt".to_string(),
            mime_type: "text/plain".to_string(),
            created_at: SystemTime::now(),
            accessed_at: SystemTime::now(),
            modified_at: SystemTime::now(),
            access_frequency: 15.0, // High frequency
            read_write_ratio: 0.8,
        };
        
        let recommendation = analyzer.recommend_tier(&characteristics).await.unwrap();
        assert_eq!(recommendation.tier, StorageTier::Hot);
        assert!(recommendation.confidence > 0.5);
    }

    #[tokio::test]
    async fn test_access_pattern_recording() {
        let temp_dir = tempdir().unwrap();
        let config = ZfsConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new(&config).await.unwrap());
        let dataset_manager = Arc::new(ZfsDatasetManager::new(config.clone(), pool_manager.clone()));
        let analyzer = DatasetAnalyzer::new(config, pool_manager, dataset_manager);
        let test_path = temp_dir.path().join("test.txt");
        
        // Record multiple accesses
        analyzer.record_access(&test_path, true).await.unwrap();
        analyzer.record_access(&test_path, false).await.unwrap();
        analyzer.record_access(&test_path, true).await.unwrap();
        
        let patterns = analyzer.access_patterns.read().await;
        let pattern = patterns.get(&test_path).unwrap();
        
        assert_eq!(pattern.total_accesses, 3);
        assert_eq!(pattern.read_operations, 2);
        assert_eq!(pattern.write_operations, 1);
    }
} 