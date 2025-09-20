use std::collections::HashMap;
//
// This module handles intelligent storage tiering, access pattern analysis,
// and automated data migration between storage tiers.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Storage tiering analysis and optimization report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieringReport {
    pub generated_at: SystemTime,
    pub tier_distributions: Vec<TierDistribution>,
    pub recommended_migrations: Vec<TierMigration>,
    pub access_patterns: Vec<AccessPattern>,
    pub potential_cost_savings: f32,
    pub performance_impact_assessment: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierDistribution {
    pub tier_name: String,
    pub tier_type: String, // "hot", "warm", "cold", "archive"
    pub total_files: u64,
    pub total_size_bytes: u64,
    pub utilization_percent: f32,
    pub average_access_frequency: f32,
    pub cost_per_gb_per_month: f32,
    pub performance_characteristics: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierMigration {
    pub current_tier: String,
    pub recommended_tier: String,
    pub migration_reason: String,
    pub confidence_score: f32,
    pub estimated_cost_impact: f32,
    pub estimated_performance_impact: String,
    pub file_size_bytes: u64,
    pub last_accessed: SystemTime,
    pub access_frequency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    pub access_count_last_30_days: u32,
    pub access_count_last_90_days: u32,
    pub average_access_interval: Duration,
    pub peak_access_times: Vec<String>, // Hour ranges like "09:00-11:00"
    pub access_trend: String,           // "increasing", "decreasing", "stable", "sporadic"
    pub read_write_ratio: f32,
    pub sequential_vs_random: f32, // 1.0 = fully sequential, 0.0 = fully random
    pub typical_access_size_bytes: u64,
}

impl Default for TieringReport {
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

    pub fn add_tier_distribution(&mut self, distribution: TierDistribution) {
        self.tier_distributions.push(distribution);
    }

    pub fn add_migration_recommendation(&mut self, migration: TierMigration) {
        self.recommended_migrations.push(migration);
    }

    pub fn add_access_pattern(&mut self, pattern: AccessPattern) {
        self.access_patterns.push(pattern);
    }

    pub fn calculate_total_potential_savings(&mut self) {
        self.potential_cost_savings = self
            .recommended_migrations
            .iter()
            .map(|m| m.estimated_cost_impact)
            .sum();
    }

    pub const fn get_high_confidence_migrations(&self, min_confidence: f32) -> Vec<&TierMigration> {
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

    pub fn update_statistics(&mut self, files: u64, size_bytes: u64) {
        self.total_files = files;
        self.total_size_bytes = size_bytes;
    }

    pub fn add_performance_metric(&mut self, metric: String, value: f32) {
        self.performance_characteristics.insert(metric, value);
    }

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

    pub const fn is_frequently_accessed(&self) -> bool {
        self.access_count_last_30_days > 10
            || self.average_access_interval < Duration::from_secs(86400) // Daily
    }

    pub const fn is_archive_candidate(&self) -> bool {
        self.access_count_last_90_days == 0
            || self.average_access_interval > Duration::from_secs(86400 * 90) // 90 days
    }

    pub const fn get_tier_recommendation(&self) -> String {
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
