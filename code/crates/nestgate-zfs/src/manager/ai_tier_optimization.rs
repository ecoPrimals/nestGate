// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

use nestgate_core::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// AI-driven tier optimization for ZFS storage
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Aitieroptimizer
pub struct AiTierOptimizer {
    /// Configuration for AI optimization
    config: AiOptimizationConfig,
    /// Historical performance data
    performance_history: HashMap<String, Vec<PerformanceMetric>>,
}
/// Configuration for AI tier optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AiOptimizationConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AiOptimizationConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for `AiOptimization`
pub struct AiOptimizationConfig {
    /// Enable AI optimization
    pub enabled: bool,
    /// Optimization interval in seconds
    pub optimization_interval: u64,
    /// Minimum data points required for optimization
    pub min_data_points: usize,
    /// Performance threshold for tier migration
    pub performance_threshold: f64,
}
/// Performance metrics for AI optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancemetric
pub struct PerformanceMetric {
    /// Timestamp of the metric
    pub timestamp: u64,
    /// Read operations per second
    pub read_ops: u64,
    /// Write operations per second
    pub write_ops: u64,
    /// Average latency in microseconds
    pub avg_latency: f64,
    /// Data access pattern
    pub access_pattern: AccessPattern,
}
/// Data access patterns for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Accesspattern
pub enum AccessPattern {
    /// Sequential access pattern
    Sequential,
    /// Random access pattern
    Random,
    /// Mixed access pattern
    Mixed,
}
impl Default for AiOptimizationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            optimization_interval: 3600, // 1 hour
            min_data_points: 100,
            performance_threshold: 0.8,
        }
    }
}

impl AiTierOptimizer {
    /// Create a new AI tier optimizer
    #[must_use]
    pub fn new(config: AiOptimizationConfig) -> Self {
        Self {
            config,
            performance_history: HashMap::new(),
        }
    }

    /// Add a performance metric
    pub fn add_metric(&mut self, dataset: String, metric: PerformanceMetric) {
        self.performance_history
            .entry(dataset)
            .or_default()
            .push(metric);
    }

    /// Analyze performance and recommend tier optimizations
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn analyze_and_optimize(&self) -> Result<Vec<TierOptimizationRecommendation>> {
        if !self.config.enabled {
            debug!("AI tier optimization is disabled");
            return Ok(vec![]);
        }

        let mut recommendations = Vec::new();

        for (dataset, metrics) in &self.performance_history {
            if metrics.len() < self.config.min_data_points {
                debug!("Insufficient data points for dataset: {}", dataset);
                continue;
            }

            if let Some(recommendation) = self.analyze_dataset(dataset, metrics).await? {
                recommendations.push(recommendation);
            }
        }

        info!(
            "Generated {} tier optimization recommendations",
            recommendations.len()
        );
        Ok(recommendations)
    }

    /// Analyze a specific dataset
    async fn analyze_dataset(
        &self,
        dataset: &str,
        metrics: &[PerformanceMetric],
    ) -> Result<Option<TierOptimizationRecommendation>> {
        let avg_latency =
            metrics.iter().map(|m| m.avg_latency).sum::<f64>() / (metrics.len() as f64);
        let total_ops = metrics
            .iter()
            .map(|m| m.read_ops + m.write_ops)
            .sum::<u64>();

        let access_pattern = self.determine_access_pattern(metrics);
        let current_tier = self.determine_current_tier(dataset)?;

        let recommended_tier = match (avg_latency, total_ops, access_pattern) {
            (latency, ops, AccessPattern::Sequential) if latency < 100.0 && ops > 1000 => {
                TierType::Hot
            }
            (latency, ops, AccessPattern::Random) if latency < 500.0 && ops > 500 => TierType::Warm,
            (_, ops, _) if ops < 100 => TierType::Cold,
            _ => current_tier.clone(),
        };

        if recommended_tier == current_tier {
            Ok(None)
        } else {
            Ok(Some(TierOptimizationRecommendation {
                dataset: dataset.to_string(),
                current_tier: current_tier.clone(),
                recommended_tier: recommended_tier.clone(),
                confidence: self.calculate_confidence(metrics),
                reason: self.generate_reason(
                    &current_tier,
                    &recommended_tier,
                    avg_latency,
                    total_ops,
                ),
            }))
        }
    }

    /// Determine the access pattern from metrics
    fn determine_access_pattern(&self, metrics: &[PerformanceMetric]) -> AccessPattern {
        let sequential_count = metrics
            .iter()
            .filter(|m| matches!(m.access_pattern, AccessPattern::Sequential))
            .count();
        let random_count = metrics
            .iter()
            .filter(|m| matches!(m.access_pattern, AccessPattern::Random))
            .count();

        if sequential_count > random_count * 2 {
            AccessPattern::Sequential
        } else if random_count > sequential_count * 2 {
            AccessPattern::Random
        } else {
            AccessPattern::Mixed
        }
    }

    /// Determine the current tier of a dataset
    const fn determine_current_tier(&self, _dataset: &str) -> Result<TierType> {
        // This would integrate with actual ZFS tier information
        // For now, return a default
        Ok(TierType::Warm)
    }

    /// Calculate confidence in the recommendation
    fn calculate_confidence(&self, metrics: &[PerformanceMetric]) -> f64 {
        // Simple confidence calculation based on data consistency
        let variance = self.calculate_variance(metrics);
        if variance < 0.1 {
            0.9
        } else if variance < 0.3 {
            0.7
        } else {
            0.5
        }
    }

    /// Calculate variance in performance metrics
    fn calculate_variance(&self, metrics: &[PerformanceMetric]) -> f64 {
        if metrics.is_empty() {
            return 1.0;
        }

        let mean = metrics.iter().map(|m| m.avg_latency).sum::<f64>() / (metrics.len() as f64);
        let variance = metrics
            .iter()
            .map(|m| (m.avg_latency - mean).powi(2))
            .sum::<f64>()
            / (metrics.len() as f64);

        variance.sqrt() / mean
    }

    /// Generate a human-readable reason for the recommendation
    fn generate_reason(
        &self,
        current: &TierType,
        recommended: &TierType,
        latency: f64,
        ops: u64,
    ) -> String {
        match (current, recommended) {
            (TierType::Cold, TierType::Hot) => {
                format!("High activity detected: {ops} ops with {latency}μs latency")
            }
            (TierType::Cold, TierType::Warm) => format!("Moderate activity detected: {ops} ops"),
            (TierType::Warm, TierType::Hot) => {
                format!("Performance critical: {latency}μs latency")
            }
            (TierType::Hot, TierType::Warm) => format!("Activity decreased: {ops} ops"),
            (TierType::Warm, TierType::Cold) => format!("Low activity: {ops} ops"),
            (TierType::Hot, TierType::Cold) => format!("Minimal usage: {ops} ops"),
            _ => "No change recommended".to_string(),
        }
    }
}

/// Storage tier types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of Tier
pub enum TierType {
    /// Hot tier - fast access, expensive storage
    Hot,
    /// Warm tier - moderate access, balanced storage
    Warm,
    /// Cold tier - slow access, cheap storage
    Cold,
}
/// Tier optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tieroptimizationrecommendation
pub struct TierOptimizationRecommendation {
    /// Dataset name
    pub dataset: String,
    /// Current tier
    pub current_tier: TierType,
    /// Recommended tier
    pub recommended_tier: TierType,
    /// Confidence in the recommendation (0.0 to 1.0)
    pub confidence: f64,
    /// Human-readable reason for the recommendation
    pub reason: String,
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Aioptimizationconfigcanonical
pub type AiOptimizationConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AiOptimizationConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
#[expect(
    clippy::float_cmp,
    reason = "AI optimization tests compare fixed threshold literals and helper-constructed metrics"
)]
mod tests {
    use super::*;

    /// Creates  Test Config
    fn create_test_config() -> AiOptimizationConfig {
        AiOptimizationConfig {
            enabled: true,
            optimization_interval: 600,
            min_data_points: 10,
            performance_threshold: 0.75,
        }
    }

    /// Creates  Test Metric
    fn create_test_metric(
        read_ops: u64,
        write_ops: u64,
        avg_latency: f64,
        access_pattern: AccessPattern,
    ) -> PerformanceMetric {
        PerformanceMetric {
            timestamp: 1000,
            read_ops,
            write_ops,
            avg_latency,
            access_pattern,
        }
    }

    #[test]
    fn test_ai_optimization_config_default() {
        let config = AiOptimizationConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.optimization_interval, 3600);
        assert_eq!(config.min_data_points, 100);
        assert_eq!(config.performance_threshold, 0.8);
    }

    #[test]
    fn test_ai_tier_optimizer_creation() {
        let config = create_test_config();
        let optimizer = AiTierOptimizer::new(config.clone());
        assert!(optimizer.config.enabled);
        assert_eq!(optimizer.config.min_data_points, 10);
    }

    #[test]
    fn test_add_metric() {
        let config = create_test_config();
        let mut optimizer = AiTierOptimizer::new(config);

        let metric = create_test_metric(100, 50, 50.0, AccessPattern::Sequential);
        optimizer.add_metric("dataset1".to_string(), metric.clone());

        assert_eq!(optimizer.performance_history.len(), 1);
        assert_eq!(
            optimizer.performance_history.get("dataset1").unwrap().len(),
            1
        );
    }

    #[test]
    fn test_add_multiple_metrics() {
        let config = create_test_config();
        let mut optimizer = AiTierOptimizer::new(config);

        for i in 0..5 {
            let metric = create_test_metric(100 + i, 50, 50.0, AccessPattern::Sequential);
            optimizer.add_metric("dataset1".to_string(), metric);
        }

        assert_eq!(
            optimizer.performance_history.get("dataset1").unwrap().len(),
            5
        );
    }

    #[test]
    fn test_performance_metric_creation() {
        let metric = create_test_metric(100, 50, 25.5, AccessPattern::Random);
        assert_eq!(metric.read_ops, 100);
        assert_eq!(metric.write_ops, 50);
        assert_eq!(metric.avg_latency, 25.5);
        assert!(matches!(metric.access_pattern, AccessPattern::Random));
    }

    #[test]
    fn test_access_pattern_variants() {
        let sequential = AccessPattern::Sequential;
        let random = AccessPattern::Random;
        let mixed = AccessPattern::Mixed;

        // Just ensure all variants are constructible
        assert!(matches!(sequential, AccessPattern::Sequential));
        assert!(matches!(random, AccessPattern::Random));
        assert!(matches!(mixed, AccessPattern::Mixed));
    }

    #[test]
    fn test_tier_type_equality() {
        assert_eq!(TierType::Hot, TierType::Hot);
        assert_eq!(TierType::Warm, TierType::Warm);
        assert_eq!(TierType::Cold, TierType::Cold);
        assert_ne!(TierType::Hot, TierType::Warm);
        assert_ne!(TierType::Warm, TierType::Cold);
    }

    #[test]
    fn test_tier_type_clone() {
        let tier = TierType::Hot;
        let cloned = tier.clone();
        assert_eq!(tier, cloned);
    }

    #[test]
    fn test_tier_optimization_recommendation_creation() {
        let recommendation = TierOptimizationRecommendation {
            dataset: "test_dataset".to_string(),
            current_tier: TierType::Cold,
            recommended_tier: TierType::Hot,
            confidence: 0.85,
            reason: "High activity".to_string(),
        };

        assert_eq!(recommendation.dataset, "test_dataset");
        assert_eq!(recommendation.current_tier, TierType::Cold);
        assert_eq!(recommendation.recommended_tier, TierType::Hot);
        assert_eq!(recommendation.confidence, 0.85);
    }

    #[test]
    fn test_determine_access_pattern_sequential() {
        let config = create_test_config();
        let optimizer = AiTierOptimizer::new(config);

        let metrics = vec![
            create_test_metric(100, 50, 50.0, AccessPattern::Sequential),
            create_test_metric(100, 50, 50.0, AccessPattern::Sequential),
            create_test_metric(100, 50, 50.0, AccessPattern::Sequential),
            create_test_metric(100, 50, 50.0, AccessPattern::Random),
        ];

        let pattern = optimizer.determine_access_pattern(&metrics);
        assert!(matches!(pattern, AccessPattern::Sequential));
    }

    #[test]
    fn test_determine_access_pattern_random() {
        let config = create_test_config();
        let optimizer = AiTierOptimizer::new(config);

        let metrics = vec![
            create_test_metric(100, 50, 50.0, AccessPattern::Random),
            create_test_metric(100, 50, 50.0, AccessPattern::Random),
            create_test_metric(100, 50, 50.0, AccessPattern::Random),
            create_test_metric(100, 50, 50.0, AccessPattern::Sequential),
        ];

        let pattern = optimizer.determine_access_pattern(&metrics);
        assert!(matches!(pattern, AccessPattern::Random));
    }

    #[test]
    fn test_determine_access_pattern_mixed() {
        let config = create_test_config();
        let optimizer = AiTierOptimizer::new(config);

        let metrics = vec![
            create_test_metric(100, 50, 50.0, AccessPattern::Sequential),
            create_test_metric(100, 50, 50.0, AccessPattern::Sequential),
            create_test_metric(100, 50, 50.0, AccessPattern::Random),
            create_test_metric(100, 50, 50.0, AccessPattern::Random),
        ];

        let pattern = optimizer.determine_access_pattern(&metrics);
        assert!(matches!(pattern, AccessPattern::Mixed));
    }

    #[test]
    fn test_calculate_confidence_low_variance() {
        let config = create_test_config();
        let optimizer = AiTierOptimizer::new(config);

        let metrics = vec![
            create_test_metric(100, 50, 50.0, AccessPattern::Sequential),
            create_test_metric(100, 50, 51.0, AccessPattern::Sequential),
            create_test_metric(100, 50, 49.0, AccessPattern::Sequential),
        ];

        let confidence = optimizer.calculate_confidence(&metrics);
        assert!(confidence > 0.6); // Should have relatively high confidence
    }

    #[test]
    fn test_calculate_variance_empty_metrics() {
        let config = create_test_config();
        let optimizer = AiTierOptimizer::new(config);

        let metrics: Vec<PerformanceMetric> = vec![];
        let variance = optimizer.calculate_variance(&metrics);
        assert_eq!(variance, 1.0);
    }

    #[test]
    fn test_calculate_variance_single_metric() {
        let config = create_test_config();
        let optimizer = AiTierOptimizer::new(config);

        let metrics = vec![create_test_metric(100, 50, 50.0, AccessPattern::Sequential)];
        let variance = optimizer.calculate_variance(&metrics);
        assert_eq!(variance, 0.0); // Single point has no variance
    }

    #[test]
    fn test_generate_reason_cold_to_hot() {
        let config = create_test_config();
        let optimizer = AiTierOptimizer::new(config);

        let reason = optimizer.generate_reason(&TierType::Cold, &TierType::Hot, 50.0, 5000);
        assert!(reason.contains("High activity"));
        assert!(reason.contains("5000"));
    }

    #[test]
    fn test_generate_reason_hot_to_cold() {
        let config = create_test_config();
        let optimizer = AiTierOptimizer::new(config);

        let reason = optimizer.generate_reason(&TierType::Hot, &TierType::Cold, 500.0, 50);
        assert!(reason.contains("Minimal usage"));
        assert!(reason.contains("50"));
    }

    #[test]
    fn test_config_clone() {
        let config1 = create_test_config();
        let config2 = config1.clone();
        assert_eq!(config1.enabled, config2.enabled);
        assert_eq!(config1.optimization_interval, config2.optimization_interval);
    }

    #[test]
    fn test_optimizer_multiple_datasets() {
        let config = create_test_config();
        let mut optimizer = AiTierOptimizer::new(config);

        optimizer.add_metric(
            "dataset1".to_string(),
            create_test_metric(100, 50, 50.0, AccessPattern::Sequential),
        );
        optimizer.add_metric(
            "dataset2".to_string(),
            create_test_metric(200, 100, 75.0, AccessPattern::Random),
        );

        assert_eq!(optimizer.performance_history.len(), 2);
    }

    #[tokio::test]
    async fn test_analyze_and_optimize_disabled() {
        let mut config = create_test_config();
        config.enabled = false;
        let optimizer = AiTierOptimizer::new(config);

        let result = optimizer.analyze_and_optimize().await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[tokio::test]
    async fn test_analyze_and_optimize_insufficient_data() {
        let config = create_test_config();
        let mut optimizer = AiTierOptimizer::new(config);

        // Add only 5 metrics when min_data_points is 10
        for _ in 0..5 {
            optimizer.add_metric(
                "dataset1".to_string(),
                create_test_metric(100, 50, 50.0, AccessPattern::Sequential),
            );
        }

        let result = optimizer.analyze_and_optimize().await.unwrap();
        assert_eq!(result.len(), 0); // Should not recommend anything
    }

    #[tokio::test]
    async fn test_determine_current_tier() {
        let config = create_test_config();
        let optimizer = AiTierOptimizer::new(config);

        let tier = optimizer.determine_current_tier("any_dataset").unwrap();
        assert_eq!(tier, TierType::Warm); // Default implementation returns Warm
    }

    #[test]
    fn test_metric_serialization() {
        let metric = create_test_metric(100, 50, 50.0, AccessPattern::Sequential);
        let serialized = serde_json::to_string(&metric).unwrap();
        assert!(serialized.contains("100"));
        assert!(serialized.contains("50.0"));
    }

    #[test]
    fn test_tier_type_serialization() {
        let tier = TierType::Hot;
        let serialized = serde_json::to_string(&tier).unwrap();
        let deserialized: TierType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tier, deserialized);
    }
}
