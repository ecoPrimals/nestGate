use std::collections::HashMap;
//
// This module handles intelligent storage tiering, access pattern analysis,
// and automated data migration between storage tiers.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Storage tiering analysis and optimization report
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tieringreport
pub struct TieringReport {
    /// Generated At
    pub generated_at: SystemTime,
    /// Tier Distributions
    pub tier_distributions: Vec<TierDistribution>,
    /// Recommended Migrations
    pub recommended_migrations: Vec<TierMigration>,
    /// Access Patterns
    pub access_patterns: Vec<AccessPattern>,
    /// Potential Cost Savings
    pub potential_cost_savings: f32,
    /// Performance Impact Assessment
    pub performance_impact_assessment: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tierdistribution
pub struct TierDistribution {
    /// Tier name
    pub tier_name: String,
    /// Tier Type
    pub tier_type: String, // "hot", "warm", "cold", "archive"
    /// Total Files
    pub total_files: u64,
    /// Total Size Bytes
    pub total_size_bytes: u64,
    /// Utilization Percent
    pub utilization_percent: f32,
    /// Average Access Frequency
    pub average_access_frequency: f32,
    /// Cost Per Gb Per Month
    pub cost_per_gb_per_month: f32,
    /// Performance Characteristics
    pub performance_characteristics: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tiermigration
pub struct TierMigration {
    /// Current Tier
    pub current_tier: String,
    /// Recommended Tier
    pub recommended_tier: String,
    /// Migration Reason
    pub migration_reason: String,
    /// Confidence Score
    pub confidence_score: f32,
    /// Estimated Cost Impact
    pub estimated_cost_impact: f32,
    /// Estimated Performance Impact
    pub estimated_performance_impact: String,
    /// File Size Bytes
    pub file_size_bytes: u64,
    /// Last Accessed
    pub last_accessed: SystemTime,
    /// Access Frequency
    pub access_frequency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Accesspattern
pub struct AccessPattern {
    /// Access Count Last 30 Days
    pub access_count_last_30_days: u32,
    /// Access Count Last 90 Days
    pub access_count_last_90_days: u32,
    /// Average Access Interval
    pub average_access_interval: Duration,
    /// Peak Access Times
    pub peak_access_times: Vec<String>, // Hour ranges like "09:00-11:00"
    /// Access Trend
    pub access_trend: String,           // "increasing", "decreasing", "stable", "sporadic"
    /// Read Write Ratio
    pub read_write_ratio: f32,
    /// Sequential Vs Random
    pub sequential_vs_random: f32, // 1.0 = fully sequential, 0.0 = fully random
    /// Typical Access Size Bytes
    pub typical_access_size_bytes: u64,
}

impl Default for TieringReport {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl TieringReport {
    #[must_use]
    pub fn new() -> Self {
        Self {
            generated_at: SystemTime::now(),
            tier_distributions: Vec::new(),
            recommended_migrations: Vec::new(),
            access_patterns: Vec::new(),
            potential_cost_savings: 0.0,
            performance_impact_assessment: String::new(),
        }
    }

    /// Add Tier Distribution
    pub fn add_tier_distribution(&mut self, distribution: TierDistribution) {
        self.tier_distributions.push(distribution);
    }

    /// Add Migration Recommendation
    pub fn add_migration_recommendation(&mut self, migration: TierMigration) {
        self.recommended_migrations.push(migration);
    }

    /// Add Access Pattern
    pub fn add_access_pattern(&mut self, pattern: AccessPattern) {
        self.access_patterns.push(pattern);
    }

    /// Calculate Total Potential Savings
    pub fn calculate_total_potential_savings(&mut self) {
        self.potential_cost_savings = self
            .recommended_migrations
            .iter()
            .map(|m| m.estimated_cost_impact)
            .sum();
    }

    /// Gets High Confidence Migrations
    pub fn get_high_confidence_migrations(&self, min_confidence: f32) -> Vec<&TierMigration> {
        self.recommended_migrations
            .iter()
            .filter(|m| m.confidence_score >= min_confidence)
            .collect()
    }
}

impl TierDistribution {
    #[must_use]
    pub fn new(tier_name: String, tier_type: String) -> Self {
        Self {
            tier_name,
            tier_type,
            total_files: 0,
            total_size_bytes: 0,
            utilization_percent: 0.0,
            average_access_frequency: 0.0,
            cost_per_gb_per_month: 0.0,
            performance_characteristics: HashMap::new(),
        }
    }

    /// Updates  Statistics
    pub fn update_statistics(&mut self, files: u64, size_bytes: u64) {
        self.total_files = files;
        self.total_size_bytes = size_bytes;
    }

    /// Add Performance Metric
    pub fn add_performance_metric(&mut self, metric: String, value: f32) {
        self.performance_characteristics.insert(metric, value);
    }

    /// Calculate Utilization
    pub fn calculate_utilization(&mut self, capacity_bytes: u64) {
        if capacity_bytes > 0 {
            self.utilization_percent =
                (self.f32::from(total_size_bytes) / f32::from(capacity_bytes)) * 100.0;
        }
    }
}

impl AccessPattern {
        Self {
            file_path,
            access_count_last_30_days: 0,
            access_count_last_90_days: 0,
            average_access_interval: Duration::from_secs(0),
            peak_access_times: Vec::new(),
            access_trend: "unknown".to_string(),
            read_write_ratio: 0.0,
            sequential_vs_random: 0.0,
            typical_access_size_bytes: 0,
        }
    }

    /// Checks if Frequently Accessed
    pub fn is_frequently_accessed(&self) -> bool {
        self.access_count_last_30_days > 10
            || self.average_access_interval < Duration::from_secs(86400) // Daily
    }

    /// Checks if Archive Candidate
    pub fn is_archive_candidate(&self) -> bool {
        self.access_count_last_90_days == 0
            || self.average_access_interval > Duration::from_secs(86400 * 90) // 90 days
    }

    /// Gets Tier Recommendation
    pub fn get_tier_recommendation(&self) -> String {
        if self.access_count_last_30_days > 50 {
            "hot".to_string()
        } else if self.access_count_last_30_days > 5 {
            "warm".to_string()
        } else if self.access_count_last_90_days > 0 {
            "cold".to_string()
        } else {
            "archive".to_string()
        }
    }
}
