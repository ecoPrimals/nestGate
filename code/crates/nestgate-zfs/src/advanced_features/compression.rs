// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// ZFS compression analysis and optimization recommendations

use nestgate_core::error::CanonicalResult as Result;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Compression analytics without AI recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Compressionanalytics
pub struct CompressionAnalytics {
    /// Current compression ratio
    pub compression_ratio: f64,
    /// Compression efficiency percentage
    pub efficiency: f64,
    /// Compression algorithm used
    pub algorithm: String,
}
impl CompressionAnalytics {
    /// Analyze compression performance for a dataset
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn analyze_compression(dataset: &str, data_sample: &[u8]) -> Result<Self> {
        debug!("Analyzing compression for dataset: {}", dataset);

        // Basic compression analysis
        let compression_ratio = Self::calculate_compression_ratio(data_sample);
        let efficiency = Self::calculate_efficiency(compression_ratio);

        Ok(Self {
            compression_ratio,
            efficiency,
            algorithm: "lz4".to_string(), // Default ZFS compression
        })
    }

    /// Calculate Compression Ratio
    fn calculate_compression_ratio(data: &[u8]) -> f64 {
        // Simple compression ratio calculation
        let original_size = data.len() as f64;
        let compressed_size = original_size * 0.7; // Mock compression
        original_size / compressed_size
    }

    /// Calculate Efficiency
    fn calculate_efficiency(ratio: f64) -> f64 {
        // Efficiency calculation as percentage
        (ratio - 1.0) / ratio * 100.0
    }

    /// Get basic compression recommendations
    #[must_use]
    pub fn get_compression_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.compression_ratio < 1.2 {
            recommendations.push("Consider disabling compression for this dataset".to_string());
        } else if self.compression_ratio < 1.5 {
            recommendations.push("lz4 compression is optimal for this data".to_string());
        } else {
            recommendations.push("Consider gzip compression for better ratio".to_string());
        }

        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_analytics_creation() {
        let data = vec![0u8; 1024];
        let analytics = CompressionAnalytics::analyze_compression("tank/data", &data).unwrap();

        assert!(analytics.compression_ratio > 0.0);
        assert_eq!(analytics.algorithm, "lz4");
    }

    #[test]
    fn test_compression_ratio_calculation() {
        let data = vec![0u8; 1000];
        let ratio = CompressionAnalytics::calculate_compression_ratio(&data);

        assert!(ratio > 1.0); // Should have some compression
    }

    #[test]
    fn test_efficiency_calculation() {
        let efficiency = CompressionAnalytics::calculate_efficiency(2.0);
        assert_eq!(efficiency, 50.0); // (2.0 - 1.0) / 2.0 * 100 = 50%
    }

    #[test]
    fn test_efficiency_no_compression() {
        let efficiency = CompressionAnalytics::calculate_efficiency(1.0);
        assert_eq!(efficiency, 0.0);
    }

    #[test]
    fn test_low_compression_recommendation() {
        let analytics = CompressionAnalytics {
            compression_ratio: 1.1,
            efficiency: 9.09,
            algorithm: "lz4".to_string(),
        };

        let recs = analytics.get_compression_recommendations();
        assert!(recs.iter().any(|r| r.contains("disabling compression")));
    }

    #[test]
    fn test_medium_compression_recommendation() {
        let analytics = CompressionAnalytics {
            compression_ratio: 1.3,
            efficiency: 23.08,
            algorithm: "lz4".to_string(),
        };

        let recs = analytics.get_compression_recommendations();
        assert!(recs.iter().any(|r| r.contains("lz4 compression")));
    }

    #[test]
    fn test_high_compression_recommendation() {
        let analytics = CompressionAnalytics {
            compression_ratio: 2.0,
            efficiency: 50.0,
            algorithm: "lz4".to_string(),
        };

        let recs = analytics.get_compression_recommendations();
        assert!(recs.iter().any(|r| r.contains("gzip")));
    }

    #[test]
    fn test_empty_data() {
        let data: Vec<u8> = vec![];
        let ratio = CompressionAnalytics::calculate_compression_ratio(&data);
        assert!(ratio.is_infinite() || ratio.is_nan()); // 0 / 0
    }

    #[test]
    fn test_small_data() {
        let data = vec![1u8; 10];
        let analytics = CompressionAnalytics::analyze_compression("test", &data).unwrap();
        assert!(analytics.compression_ratio > 0.0);
    }

    #[test]
    fn test_large_data() {
        let data = vec![0u8; 1024 * 1024]; // 1MB
        let analytics = CompressionAnalytics::analyze_compression("test", &data).unwrap();
        assert!(analytics.compression_ratio > 1.0);
    }

    #[test]
    fn test_analytics_clone() {
        let analytics1 = CompressionAnalytics {
            compression_ratio: 1.5,
            efficiency: 33.33,
            algorithm: "lz4".to_string(),
        };
        let analytics2 = analytics1.clone();

        assert_eq!(analytics1.compression_ratio, analytics2.compression_ratio);
    }

    #[test]
    fn test_analytics_serialization() {
        let analytics = CompressionAnalytics {
            compression_ratio: 1.5,
            efficiency: 33.33,
            algorithm: "lz4".to_string(),
        };
        let serialized = serde_json::to_string(&analytics).unwrap();

        assert!(serialized.contains("lz4"));
    }
}
