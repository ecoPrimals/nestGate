// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// ZFS replication analysis and optimization recommendations

use crate::types::ReplicationPerformance;
use nestgate_core::error::CanonicalResult as Result;
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Replication analytics without AI recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Replicationanalytics
pub struct ReplicationAnalytics {
    /// Current replication strategy
    pub strategy: String,
    /// Performance metrics
    pub performance: ReplicationPerformance,
    /// Basic recommendations
    pub recommendations: Vec<String>,
}
impl ReplicationAnalytics {
    /// Analyze replication performance
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn analyze_replication(
        source: &str,
        targets: &[String],
        performance_data: &ReplicationPerformance,
    ) -> Result<Self> {
        debug!("Analyzing replication from {} to {:?}", source, targets);

        let mut recommendations = Vec::new();

        // Basic replication analysis
        if performance_data.transfer_rate < 10.0 {
            recommendations.push("Consider async replication for better performance".to_string());
        }

        if performance_data.transfer_rate < 1.0 {
            recommendations.push("Investigate network connectivity issues".to_string());
        }

        if performance_data.compression_ratio < 1.2 {
            recommendations.push("Consider enabling compression for better efficiency".to_string());
        }
        Ok(Self {
            strategy: "sync".to_string(),
            performance: performance_data.clone(),
            recommendations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates  Test Performance
    fn create_test_performance(
        transfer_rate: f64,
        compression_ratio: f64,
    ) -> ReplicationPerformance {
        ReplicationPerformance {
            source_dataset: "tank/source".to_string(),
            target_dataset: "tank/target".to_string(),
            transfer_rate,
            compression_ratio,
            estimated_completion: std::time::SystemTime::now(),
        }
    }

    #[test]
    fn test_replication_analytics_creation() {
        let perf = create_test_performance(50.0, 1.5);
        let analytics = ReplicationAnalytics::analyze_replication(
            "tank/source",
            &["tank/target1".to_string()],
            &perf,
        )
        .unwrap();

        assert_eq!(analytics.strategy, "sync");
        assert_eq!(analytics.performance.transfer_rate, 50.0);
    }

    #[test]
    fn test_slow_replication_recommendation() {
        let perf = create_test_performance(5.0, 1.5);
        let analytics = ReplicationAnalytics::analyze_replication(
            "tank/source",
            &["tank/target1".to_string()],
            &perf,
        )
        .unwrap();

        assert!(analytics
            .recommendations
            .iter()
            .any(|r| r.contains("async replication")));
    }

    #[test]
    fn test_very_slow_replication_recommendation() {
        let perf = create_test_performance(0.5, 1.5);
        let analytics = ReplicationAnalytics::analyze_replication(
            "tank/source",
            &["tank/target1".to_string()],
            &perf,
        )
        .unwrap();

        assert!(analytics
            .recommendations
            .iter()
            .any(|r| r.contains("network connectivity")));
    }

    #[test]
    fn test_low_compression_recommendation() {
        let perf = create_test_performance(50.0, 1.1);
        let analytics = ReplicationAnalytics::analyze_replication(
            "tank/source",
            &["tank/target1".to_string()],
            &perf,
        )
        .unwrap();

        assert!(analytics
            .recommendations
            .iter()
            .any(|r| r.contains("compression")));
    }

    #[test]
    fn test_fast_replication_no_recommendations() {
        let perf = create_test_performance(50.0, 2.0);
        let analytics = ReplicationAnalytics::analyze_replication(
            "tank/source",
            &["tank/target1".to_string()],
            &perf,
        )
        .unwrap();

        assert!(analytics.recommendations.is_empty());
    }

    #[test]
    fn test_multiple_targets() {
        let perf = create_test_performance(50.0, 1.5);
        let targets = vec![
            "target1".to_string(),
            "target2".to_string(),
            "target3".to_string(),
        ];
        let analytics =
            ReplicationAnalytics::analyze_replication("source", &targets, &perf).unwrap();

        assert_eq!(analytics.strategy, "sync");
    }

    #[test]
    fn test_analytics_clone() {
        let perf = create_test_performance(50.0, 1.5);
        let analytics1 = ReplicationAnalytics::analyze_replication("source", &[], &perf).unwrap();
        let analytics2 = analytics1.clone();

        assert_eq!(analytics1.strategy, analytics2.strategy);
    }

    #[test]
    fn test_analytics_serialization() {
        let perf = create_test_performance(50.0, 1.5);
        let analytics = ReplicationAnalytics::analyze_replication("source", &[], &perf).unwrap();
        let serialized = serde_json::to_string(&analytics).unwrap();

        assert!(serialized.contains("sync"));
    }
}
