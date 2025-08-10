//! Performance Engine Types and Data Structures
//!
//! This module contains all the data structures, enums, and types used by the
//! performance optimization engine.

use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;

use crate::types::StorageTier;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Performance engine configuration
#[derive(Debug, Clone)]
pub struct PerformanceEngineConfig {
    pub monitoring_interval: std::time::Duration,
    pub optimization_interval: std::time::Duration,
    pub bottleneck_detection_interval: std::time::Duration,
    pub max_concurrent_optimizations: usize,
    pub enable_ai_guidance: bool,
}

impl Default for PerformanceEngineConfig {
    fn default() -> Self {
        Self {
            monitoring_interval: std::time::Duration::from_secs(5),
            optimization_interval: std::time::Duration::from_secs(30),
            bottleneck_detection_interval: std::time::Duration::from_secs(10),
            max_concurrent_optimizations: 3,
            enable_ai_guidance: true,
        }
    }
}

/// Optimization state tracking
#[derive(Debug, Clone, PartialEq, Default)]
pub enum OptimizationState {
    #[default]
    Idle,
    Collecting,
    Analyzing,
    Optimizing,
    Validating,
    Applied,
}

/// ZFS performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceMetrics {
    pub timestamp: SystemTime,
    pub pool_metrics: HashMap<String, ZfsPoolMetrics>,
    pub dataset_metrics: HashMap<String, ZfsDatasetMetrics>,
    pub system_memory: SystemMemoryUsage,
    pub arc_stats: ArcStatistics,
}

/// ZFS pool metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolMetrics {
    pub pool_name: String,
    pub read_ops: f64,
    pub write_ops: f64,
    pub read_bandwidth: f64,
    pub write_bandwidth: f64,
    pub latency: f64,
    pub cache_hit_ratio: f64,
    pub fragmentation: f64,
}

/// ZFS dataset metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDatasetMetrics {
    pub dataset_name: String,
    pub access_pattern: AccessPattern,
    pub dedup_ratio: f64,
    pub record_size: u64,
}

/// Access pattern classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccessPattern {
    Sequential,
    Random,
    Mixed,
}

/// System memory usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMemoryUsage {
    pub total: u64,
    pub available: u64,
    pub used: u64,
}

/// ARC statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcStatistics {
    pub size: u64,
    pub target_size: u64,
    pub hit_ratio: f64,
    pub miss_ratio: f64,
}

/// ZFS bottleneck detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsBottleneck {
    pub bottleneck_type: ZfsBottleneckType,
    pub severity: BottleneckSeverity,
    pub pool_name: String,
    pub dataset_name: Option<String>,
    pub description: String,
    pub impact_score: f64,
}

/// Types of ZFS bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsBottleneckType {
    HighLatency,
    LowThroughput,
    CacheMiss,
    Fragmentation,
    MemoryPressure,
    CpuUtilization,
    NetworkBandwidth,
    DiskIo,
}

/// Bottleneck severity levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Performance optimization result
#[derive(Debug, Clone, Default)]
pub struct PerformanceOptimizationResult {
    pub applied_optimizations: Vec<AppliedOptimization>,
    pub performance_improvement: f64,
    pub bottlenecks_resolved: Vec<ZfsBottleneck>,
    pub recommendations: Vec<String>,
}

impl PerformanceOptimizationResult {
    pub fn merge_with(&mut self, other: PerformanceOptimizationResult) {
        self.applied_optimizations
            .extend(other.applied_optimizations);
        self.performance_improvement += other.performance_improvement;
        self.bottlenecks_resolved.extend(other.bottlenecks_resolved);
        self.recommendations.extend(other.recommendations);
    }
}

/// Applied optimization tracking
#[derive(Debug, Clone)]
pub struct AppliedOptimization {
    pub optimization_type: OptimizationType,
    pub description: String,
    pub performance_impact: f64,
    pub applied_at: SystemTime,
}

/// Types of optimizations
#[derive(Debug, Clone)]
pub enum OptimizationType {
    CacheOptimization,
    LatencyOptimization,
    ThroughputOptimization,
    FragmentationDefrag,
    ArcTuning,
    RecordSizeOptimization,
    CompressionOptimization,
}

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub pool_name: String,
    pub dataset_name: Option<String>,
    pub description: String,
    pub timestamp: SystemTime,
}

/// Alert types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlertType {
    PerformanceDegradation,
    BottleneckDetected,
    ThresholdExceeded,
    OptimizationFailed,
}

/// Alert severity levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Alert response
#[derive(Debug, Clone, Default)]
pub struct AlertResponse {
    pub mitigation_applied: bool,
    pub optimization_result: Option<PerformanceOptimizationResult>,
    pub follow_up_required: bool,
}

/// Workload pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadPattern {
    pub access_pattern: AccessPattern,
    pub io_size_distribution: HashMap<String, f64>,
    pub read_write_ratio: f64,
    pub temporal_locality: f64,
}

/// ZFS configuration context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsConfigurationContext {
    pub pool_name: String,
    pub dataset_name: Option<String>,
    pub current_configuration: HashMap<String, String>,
    pub workload_pattern: WorkloadPattern,
    pub system_capabilities: SystemCapabilities,
}

/// ZFS expertise context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsExpertiseContext {
    pub storage_tier: StorageTier,
    pub access_patterns: Vec<AccessPattern>,
    pub current_performance: ZfsPerformanceMetrics,
    pub identified_bottlenecks: Vec<ZfsBottleneck>,
}

/// System capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCapabilities {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub storage_type: String,
    pub network_bandwidth_gbps: f64,
}

/// ZFS tuning result
#[derive(Debug, Clone, Default)]
pub struct ZfsTuningResult {
    pub tuning_applied: bool,
    pub parameter_changes: HashMap<String, String>,
    pub expected_improvement: f64,
    pub validation_required: bool,
}

/// Performance optimization request
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceOptimizationRequest {
    pub pool_name: String,
    pub dataset_name: Option<String>,
    pub optimization_strategy: EcosystemOptimizationStrategy,
    pub current_metrics: ZfsPerformanceMetrics,
    pub configuration_context: ZfsConfigurationContext,
}

/// Ecosystem optimization strategy
#[derive(Debug, Serialize, Deserialize)]
pub enum EcosystemOptimizationStrategy {
    LatencyOptimization,
    ThroughputOptimization,
    BalancedOptimization,
    CustomStrategy(String),
}

/// AI optimization recommendation
#[derive(Debug, Serialize, Deserialize)]
pub struct AiOptimizationRecommendation {
    pub strategy: EcosystemOptimizationStrategy,
    pub confidence_score: f64,
    pub expected_improvement: f64,
    pub parameter_recommendations: HashMap<String, String>,
    pub risk_assessment: String,
}

/// ZFS tuning request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsTuningRequest {
    pub pool_name: String,
    pub dataset_name: Option<String>,
    pub workload_pattern: WorkloadPattern,
    pub current_configuration: HashMap<String, String>,
    pub performance_goals: Vec<String>,
    pub system_capabilities: SystemCapabilities,
    pub configuration_context: ZfsConfigurationContext,
    pub expertise_context: ZfsExpertiseContext,
}

/// Ecosystem tuning recommendations
#[derive(Debug, Serialize, Deserialize)]
pub struct EcosystemTuningRecommendations {
    pub recommendations: Vec<ZfsTuningRecommendation>,
    pub overall_confidence: f64,
    pub estimated_improvement: f64,
}

/// ZFS tuning recommendation
#[derive(Debug, Serialize, Deserialize)]
pub struct ZfsTuningRecommendation {
    pub parameter: String,
    pub recommended_value: String,
    pub confidence: f64,
    pub expected_impact: f64,
}

/// Performance alert analysis request
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceAlertAnalysisRequest {
    pub alert: PerformanceAlert,
    pub historical_metrics: Vec<ZfsPerformanceMetrics>,
    pub current_configuration: HashMap<String, String>,
    pub system_capabilities: SystemCapabilities,
}

/// Ecosystem alert analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct EcosystemAlertAnalysis {
    pub root_cause_analysis: String,
    pub recommended_actions: Vec<String>,
    pub confidence_score: f64,
    pub urgency_level: AlertSeverity,
}

/// Custom serialization for SystemTime
pub mod system_time_serde {
    use super::*;
    use std::time::UNIX_EPOCH;

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0));
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)
            .map_err(|e| de::Error::custom(format!("deserialization error: {}", e)))?;
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
    }
}
