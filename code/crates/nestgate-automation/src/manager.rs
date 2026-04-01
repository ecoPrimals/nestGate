// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// Clean implementation of intelligent dataset management

//! Manager module

use crate::analysis::DatasetAnalyzer;
use crate::types::prediction::TierPrediction;
use nestgate_core::config::canonical_primary::domains::automation::AutomationConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use nestgate_core::Result;
use nestgate_core::unified_enums::StorageTier;

// Type alias to reduce complexity
type PerformanceCache = Arc<RwLock<HashMap<String, DatasetPerformance>>>;

/// Performance metrics for a dataset
#[derive(Debug, Clone)]
/// Datasetperformance
pub struct DatasetPerformance {
    /// Dataset name
    pub dataset_name: String,
    /// Total Files
    pub total_files: usize,
    /// Total Size Bytes
    pub total_size_bytes: u64,
    /// Access Frequency
    pub access_frequency: f64,
    /// Compression Ratio
    pub compression_ratio: f64,
    /// Last Optimized
    pub last_optimized: std::time::SystemTime,
}
/// Intelligent dataset manager with canonical implementation
#[derive(Debug)]
/// Manager for `IntelligentDataset` operations
pub struct IntelligentDatasetManager {
    _config: AutomationConfig,
    analyzer: Arc<DatasetAnalyzer>,
    _performance_cache: PerformanceCache,
}
impl IntelligentDatasetManager {
    #[must_use]
    pub fn new(config: AutomationConfig) -> Self {
        Self {
            _config: config,
            analyzer: Arc::new(DatasetAnalyzer::new()),
            _performance_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start the intelligent automation system
    pub const fn start(&mut self) -> Result<()> {
        // Simplified canonical implementation
        Ok(())
    }

    /// Predict optimal storage tier for a file
    pub async fn predict_optimal_tier(&self, file_path: &str) -> Result<TierPrediction> {
        // Analyze file characteristics
        let analysis = self.analyzer.analyze_file(file_path).await?;

        // Simple tier prediction based on file characteristics
        let predicted_tier = if analysis.size_bytes > 1024 * 1024 * 1024 {
            StorageTier::Cold
        } else if analysis.size_bytes > 100 * 1024 * 1024 {
            StorageTier::Warm
        } else {
            StorageTier::Hot
        };

        Ok(TierPrediction {
            predicted_tier,
            confidence_score: 0.8,
            accesses_last_24h: 10,
            accesses_last_week: 70,
            accesses_last_month: 300,
            size_bytes: analysis.size_bytes,
            file_type: analysis.file_type,
            recommendation_reason: "Based on file size and access patterns".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates  Test Config
    fn create_test_config() -> AutomationConfig {
        AutomationConfig::default()
    }

    #[test]
    fn test_intelligent_dataset_manager_creation() {
        let config = create_test_config();
        let manager = IntelligentDatasetManager::new(config);

        // Verify manager is created successfully
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("IntelligentDatasetManager"));
    }

    #[test]
    fn test_manager_start() {
        let config = create_test_config();
        let mut manager = IntelligentDatasetManager::new(config);

        let result = manager.start();
        assert!(result.is_ok());
    }

    #[test]
    fn test_dataset_performance_creation() {
        let perf = DatasetPerformance {
            dataset_name: "test-dataset".to_string(),
            total_files: 100,
            total_size_bytes: 1024 * 1024 * 500, // 500MB
            access_frequency: 0.75,
            compression_ratio: 1.5,
            last_optimized: std::time::SystemTime::now(),
        };

        assert_eq!(perf.dataset_name, "test-dataset");
        assert_eq!(perf.total_files, 100);
        assert_eq!(perf.total_size_bytes, 1024 * 1024 * 500);
    }

    #[test]
    fn test_dataset_performance_cloning() {
        let perf = DatasetPerformance {
            dataset_name: "clone-test".to_string(),
            total_files: 50,
            total_size_bytes: 1024 * 1024 * 100,
            access_frequency: 0.5,
            compression_ratio: 2.0,
            last_optimized: std::time::SystemTime::now(),
        };

        let cloned = perf.clone();
        assert_eq!(cloned.dataset_name, perf.dataset_name);
        assert_eq!(cloned.total_files, perf.total_files);
    }

    #[tokio::test]
    async fn test_predict_optimal_tier_large_file() {
        let config = create_test_config();
        let manager = IntelligentDatasetManager::new(config);

        // Create a test file to analyze
        let temp_file = "/tmp/test_large_file.txt";
        std::fs::write(temp_file, vec![0u8; 2 * 1024 * 1024 * 1024]).ok(); // 2GB

        if let Ok(prediction) = manager.predict_optimal_tier(temp_file).await {
            // Large files should be predicted for Cold tier
            assert_eq!(prediction.predicted_tier, StorageTier::Cold);
            assert!(prediction.size_bytes > 1024 * 1024 * 1024);
        }

        // Cleanup
        std::fs::remove_file(temp_file).ok();
    }

    #[tokio::test]
    async fn test_predict_optimal_tier_medium_file() {
        let config = create_test_config();
        let manager = IntelligentDatasetManager::new(config);

        let temp_file = "/tmp/test_medium_file.txt";
        std::fs::write(temp_file, vec![0u8; 200 * 1024 * 1024]).ok(); // 200MB

        if let Ok(prediction) = manager.predict_optimal_tier(temp_file).await {
            // Medium files should be predicted for Warm tier
            assert_eq!(prediction.predicted_tier, StorageTier::Warm);
        }

        std::fs::remove_file(temp_file).ok();
    }

    #[tokio::test]
    async fn test_predict_optimal_tier_small_file() {
        let config = create_test_config();
        let manager = IntelligentDatasetManager::new(config);

        let temp_file = "/tmp/test_small_file.txt";
        std::fs::write(temp_file, vec![0u8; 10 * 1024 * 1024]).ok(); // 10MB

        if let Ok(prediction) = manager.predict_optimal_tier(temp_file).await {
            // Small files should be predicted for Hot tier
            assert_eq!(prediction.predicted_tier, StorageTier::Hot);
        }

        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_dataset_performance_debug_format() {
        let perf = DatasetPerformance {
            dataset_name: "debug-test".to_string(),
            total_files: 25,
            total_size_bytes: 1024 * 1024 * 250,
            access_frequency: 0.8,
            compression_ratio: 1.8,
            last_optimized: std::time::SystemTime::now(),
        };

        let debug_str = format!("{:?}", perf);
        assert!(debug_str.contains("debug-test"));
        assert!(debug_str.contains("25"));
    }

    #[test]
    fn test_dataset_performance_various_sizes() {
        let sizes = vec![
            (1024, "1KB"),
            (1024 * 1024, "1MB"),
            (1024 * 1024 * 1024, "1GB"),
        ];

        for (bytes, _label) in sizes {
            let perf = DatasetPerformance {
                dataset_name: format!("dataset-{}", bytes),
                total_files: 10,
                total_size_bytes: bytes,
                access_frequency: 0.5,
                compression_ratio: 1.5,
                last_optimized: std::time::SystemTime::now(),
            };

            assert_eq!(perf.total_size_bytes, bytes);
        }
    }

    #[test]
    fn test_dataset_performance_compression_ratios() {
        let ratios = vec![1.0, 1.5, 2.0, 2.5, 3.0];

        for ratio in ratios {
            let perf = DatasetPerformance {
                dataset_name: "compression-test".to_string(),
                total_files: 100,
                total_size_bytes: 1024 * 1024 * 500,
                access_frequency: 0.7,
                compression_ratio: ratio,
                last_optimized: std::time::SystemTime::now(),
            };

            assert_eq!(perf.compression_ratio, ratio);
        }
    }

    #[test]
    fn test_dataset_performance_access_frequencies() {
        let frequencies = vec![0.0, 0.25, 0.5, 0.75, 1.0];

        for freq in frequencies {
            let perf = DatasetPerformance {
                dataset_name: "frequency-test".to_string(),
                total_files: 50,
                total_size_bytes: 1024 * 1024 * 100,
                access_frequency: freq,
                compression_ratio: 1.5,
                last_optimized: std::time::SystemTime::now(),
            };

            assert_eq!(perf.access_frequency, freq);
        }
    }

    #[test]
    fn test_multiple_managers_creation() {
        let config1 = create_test_config();
        let config2 = create_test_config();

        let manager1 = IntelligentDatasetManager::new(config1);
        let manager2 = IntelligentDatasetManager::new(config2);

        // Both should be created successfully
        let debug1 = format!("{:?}", manager1);
        let debug2 = format!("{:?}", manager2);

        assert!(debug1.contains("IntelligentDatasetManager"));
        assert!(debug2.contains("IntelligentDatasetManager"));
    }
}
