use std::collections::HashMap;
//
// This module provides detailed performance analysis, optimization recommendations,
// and deduplication analysis for enterprise storage systems.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Detailed performance metrics for storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Detailedmetrics
pub struct DetailedMetrics {
    /// Read Ops Per Sec
    pub read_ops_per_sec: f64,
    /// Write Ops Per Sec
    pub write_ops_per_sec: f64,
    /// Avg Read Latency Ms
    pub avg_read_latency_ms: f64,
    /// Avg Write Latency Ms
    pub avg_write_latency_ms: f64,
    /// Throughput Mb Per Sec
    pub throughput_mb_per_sec: f64,
    /// Cache Hit Ratio
    pub cache_hit_ratio: f64,
    /// Error Rate
    pub error_rate: f64,
    /// Cpu Usage Percent
    pub cpu_usage_percent: f64,
    /// Memory Usage in megabytes
    pub memory_usage_mb: u64,
    /// Disk Usage Percent
    pub disk_usage_percent: f64,
    /// Network Usage Mb Per Sec
    pub network_usage_mb_per_sec: f64,
    /// Concurrent Operations
    pub concurrent_operations: u32,
    /// Queue Depth
    pub queue_depth: u32,
    /// Uptime Seconds
    pub uptime_seconds: u64,
    /// Last Updated
    pub last_updated: SystemTime,
    /// Custom Metrics
    pub custom_metrics: HashMap<String, f64>,
}
/// Storage optimization analysis and recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Optimizationreport
pub struct OptimizationReport {
    /// Generated At
    pub generated_at: SystemTime,
    /// Analysis Duration
    pub analysis_duration: Duration,
    /// Recommendations
    pub recommendations: Vec<OptimizationRecommendation>,
    /// Estimated Total Savings
    pub estimated_total_savings: EstimatedSavings,
    /// Performance Insights
    pub performance_insights: Vec<PerformanceInsight>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Optimizationrecommendation
pub struct OptimizationRecommendation {
    /// Unique identifier
    pub id: String,
    /// Category
    pub category: OptimizationCategory,
    /// Title
    pub title: String,
    /// Human-readable description
    pub description: String,
    /// Priority
    pub priority: Priority,
    /// Estimated Savings
    pub estimated_savings: EstimatedSavings,
    /// Implementation Effort
    pub implementation_effort: ImplementationEffort,
    /// Prerequisites
    pub prerequisites: Vec<String>,
    /// Risks
    pub risks: Vec<String>,
    /// Metrics Improvement
    pub metrics_improvement: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Optimizationcategory
pub enum OptimizationCategory {
    /// Compression
    Compression,
    /// Caching
    Caching,
    /// Tiering
    Tiering,
    /// Deduplication
    Deduplication,
    /// Networkoptimization
    NetworkOptimization,
    /// Indexoptimization
    IndexOptimization,
    /// Memorymanagement
    MemoryManagement,
    /// Iooptimization
    IOOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Priority
pub enum Priority {
    /// Critical
    Critical,
    /// High
    High,
    /// Medium
    Medium,
    /// Low
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Implementationeffort
pub enum ImplementationEffort {
    Minimal,   // < 1 hour
    Low,       // 1-8 hours
    Medium,    // 1-3 days
    High,      // 1-2 weeks
    Extensive, // > 2 weeks
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Estimatedsavings
pub struct EstimatedSavings {
    /// Storage Space in megabytes
    pub storage_space_mb: u64,
    /// Cost Savings Percent
    pub cost_savings_percent: f32,
    /// Performance Improvement Percent
    pub performance_improvement_percent: f32,
    /// Energy Savings Percent
    pub energy_savings_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceinsight
pub struct PerformanceInsight {
    /// Metric name
    pub metric_name: String,
    /// Currentvalue
    pub currentvalue: f64,
    /// Optimal Range
    pub optimal_range: (f64, f64),
    /// Trend
    pub trend: String, // "improving", "degrading", "stable"
    /// Impact Assessment
    pub impact_assessment: String,
    /// Suggested Actions
    pub suggested_actions: Vec<String>,
}

/// Deduplication analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Deduplicationreport
pub struct DeduplicationReport {
    /// Total Files Analyzed
    pub total_files_analyzed: u64,
    /// Total Size Bytes
    pub total_size_bytes: u64,
    /// Duplicate Groups
    pub duplicate_groups: Vec<DuplicateGroup>,
    /// Potential Savings Bytes
    pub potential_savings_bytes: u64,
    /// Deduplication Ratio
    pub deduplication_ratio: f32,
    /// Analysis Duration
    pub analysis_duration: Duration,
    /// Generated At
    pub generated_at: SystemTime,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Duplicategroup
pub struct DuplicateGroup {
    /// Content Hash
    pub content_hash: String,
    /// File Paths
    pub file_paths: Vec<String>,
    /// Size of file
    pub file_size: u64,
    /// Count of duplicate
    pub duplicate_count: u32,
    /// Wasted Space Bytes
    pub wasted_space_bytes: u64,
}

impl Default for DetailedMetrics {
    /// Returns the default instance
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

    /// Add Custom Metric
    pub fn add_custom_metric(&mut self, name: String, value: f64) {
        self.custom_metrics.insert(name, value);
    }

    /// Updates  Timestamp
    pub fn update_timestamp(&mut self) {
        self.last_updated = SystemTime::now();
    }
}

impl Default for OptimizationReport {
    /// Returns the default instance
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

    /// Add Recommendation
    pub fn add_recommendation(&mut self, recommendation: OptimizationRecommendation) {
        self.recommendations.push(recommendation);
    }

    /// Add Insight
    pub fn add_insight(&mut self, insight: PerformanceInsight) {
        self.performance_insights.push(insight);
    }
}

impl DeduplicationReport {
    /// Calculate Savings Ratio
    pub fn calculate_savings_ratio(&self) -> f32 {
        if self.total_size_bytes > 0 {
            self.f32::from(potential_savings_bytes) / self.f32::from(total_size_bytes)
        } else {
            0.0
        }
    }

    /// Gets Top Duplicate Groups
    pub fn get_top_duplicate_groups(&self, limit: usize) -> Vec<&DuplicateGroup> {
        let mut groups: Vec<&DuplicateGroup> = self.duplicate_groups.iter().collect();
        groups.sort_by(|a, b| b.wasted_space_bytes.cmp(&a.wasted_space_bytes));
        groups.into_iter().take(limit).collect()
    }
}
