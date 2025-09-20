use std::collections::HashMap;
//
// This module provides detailed performance analysis, optimization recommendations,
// and deduplication analysis for enterprise storage systems.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Detailed performance metrics for storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedMetrics {
    pub read_ops_per_sec: f64,
    pub write_ops_per_sec: f64,
    pub avg_read_latency_ms: f64,
    pub avg_write_latency_ms: f64,
    pub throughput_mb_per_sec: f64,
    pub cache_hit_ratio: f64,
    pub error_rate: f64,
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: u64,
    pub disk_usage_percent: f64,
    pub network_usage_mb_per_sec: f64,
    pub concurrent_operations: u32,
    pub queue_depth: u32,
    pub uptime_seconds: u64,
    pub last_updated: SystemTime,
    pub custom_metrics: HashMap<String, f64>,
}
/// Storage optimization analysis and recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationReport {
    pub generated_at: SystemTime,
    pub analysis_duration: Duration,
    pub recommendations: Vec<OptimizationRecommendation>,
    pub estimated_total_savings: EstimatedSavings,
    pub performance_insights: Vec<PerformanceInsight>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub id: String,
    pub category: OptimizationCategory,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub estimated_savings: EstimatedSavings,
    pub implementation_effort: ImplementationEffort,
    pub prerequisites: Vec<String>,
    pub risks: Vec<String>,
    pub metrics_improvement: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationCategory {
    Compression,
    Caching,
    Tiering,
    Deduplication,
    NetworkOptimization,
    IndexOptimization,
    MemoryManagement,
    IOOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Minimal,   // < 1 hour
    Low,       // 1-8 hours
    Medium,    // 1-3 days
    High,      // 1-2 weeks
    Extensive, // > 2 weeks
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatedSavings {
    pub storage_space_mb: u64,
    pub cost_savings_percent: f32,
    pub performance_improvement_percent: f32,
    pub energy_savings_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceInsight {
    pub metric_name: String,
    pub currentvalue: f64,
    pub optimal_range: (f64, f64),
    pub trend: String, // "improving", "degrading", "stable"
    pub impact_assessment: String,
    pub suggested_actions: Vec<String>,
}

/// Deduplication analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationReport {
    pub total_files_analyzed: u64,
    pub total_size_bytes: u64,
    pub duplicate_groups: Vec<DuplicateGroup>,
    pub potential_savings_bytes: u64,
    pub deduplication_ratio: f32,
    pub analysis_duration: Duration,
    pub generated_at: SystemTime,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub content_hash: String,
    pub file_paths: Vec<String>,
    pub file_size: u64,
    pub duplicate_count: u32,
    pub wasted_space_bytes: u64,
}

impl Default for DetailedMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl DetailedMetrics {
    #[must_use]
    pub fn new() -> Self {
        Self {
            read_ops_per_sec: 0.0,
            write_ops_per_sec: 0.0,
            avg_read_latency_ms: 0.0,
            avg_write_latency_ms: 0.0,
            throughput_mb_per_sec: 0.0,
            cache_hit_ratio: 0.0,
            error_rate: 0.0,
            cpu_usage_percent: 0.0,
            memory_usage_mb: 0,
            disk_usage_percent: 0.0,
            network_usage_mb_per_sec: 0.0,
            concurrent_operations: 0,
            queue_depth: 0,
            uptime_seconds: 0,
            last_updated: SystemTime::now(),
            custom_metrics: HashMap::new(),
        }
    }

    pub fn add_custom_metric(&mut self, name: String, value: f64) {
        self.custom_metrics.insert(name, value);
    }

    pub fn update_timestamp(&mut self) {
        self.last_updated = SystemTime::now();
    }
}

impl Default for OptimizationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationReport {
    #[must_use]
    pub fn new() -> Self {
        Self {
            generated_at: SystemTime::now(),
            analysis_duration: Duration::from_secs(0),
            recommendations: Vec::new(),
            estimated_total_savings: EstimatedSavings {
                storage_space_mb: 0,
                cost_savings_percent: 0.0,
                performance_improvement_percent: 0.0,
                energy_savings_percent: 0.0,
            },
            performance_insights: Vec::new(),
        }
    }

    pub fn add_recommendation(&mut self, recommendation: OptimizationRecommendation) {
        self.recommendations.push(recommendation);
    }

    pub fn add_insight(&mut self, insight: PerformanceInsight) {
        self.performance_insights.push(insight);
    }
}

impl DeduplicationReport {
    pub const fn calculate_savings_ratio(&self) -> f32 {
        if self.total_size_bytes > 0 {
            self.f32::from(potential_savings_bytes) / self.f32::from(total_size_bytes)
        } else {
            0.0
        }
    }

    pub fn get_top_duplicate_groups(&self, limit: usize) -> Vec<&DuplicateGroup> {
        let mut groups: Vec<&DuplicateGroup> = self.duplicate_groups.iter().collect();
        groups.sort_by(|a, b| b.wasted_space_bytes.cmp(&a.wasted_space_bytes));
        groups.into_iter().take(limit).collect()
    }
}
