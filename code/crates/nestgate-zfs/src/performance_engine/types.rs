// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// This module contains all the data structures, enums, and types used by the
// performance optimization engine.

use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;

use crate::types::StorageTier;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Performance engine configuration
///
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::PerformanceEngineConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::PerformanceEngineConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[derive(Debug, Clone)]
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct PerformanceEngineConfig {
    /// Interval between monitoring cycles
    pub monitoring_interval: std::time::Duration,
    /// Interval between optimization runs
    pub optimization_interval: std::time::Duration,
    /// Interval for bottleneck detection
    pub bottleneck_detection_interval: std::time::Duration,
    /// Maximum number of concurrent optimizations
    pub max_concurrent_optimizations: usize,
    /// Enable AI-powered guidance
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
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OptimizationState {
    /// No optimization in progress
    #[default]
    Idle,
    /// Collecting metrics
    Collecting,
    /// Analyzing performance data
    Analyzing,
    /// Applying optimizations
    Optimizing,
    /// Validating optimization results
    Validating,
    /// Optimization successfully applied
    Applied,
}
/// ZFS performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceMetrics {
    /// When these metrics were collected
    pub timestamp: SystemTime,
    /// Per-pool performance metrics
    pub pool_metrics: HashMap<String, ZfsPoolMetrics>,
    /// Per-dataset performance metrics
    pub dataset_metrics: HashMap<String, ZfsDatasetMetrics>,
    /// System memory usage statistics
    pub system_memory: SystemMemoryUsage,
    /// ARC (Adaptive Replacement Cache) statistics
    pub arc_stats: ArcStatistics,
}
/// ZFS pool metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolMetrics {
    /// Name of the ZFS pool
    pub pool_name: String,
    /// Read operations per second
    pub read_ops: f64,
    /// Write operations per second
    pub write_ops: f64,
    /// Read bandwidth in bytes/second
    pub read_bandwidth: f64,
    /// Write bandwidth in bytes/second
    pub write_bandwidth: f64,
    /// Average latency in milliseconds
    pub latency: f64,
    /// Cache hit ratio (0.0 to 1.0)
    pub cache_hit_ratio: f64,
    /// Pool fragmentation percentage (0.0 to 100.0)
    pub fragmentation: f64,
}
/// ZFS dataset metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDatasetMetrics {
    /// Name of the ZFS dataset
    pub dataset_name: String,
    /// Detected I/O access pattern
    pub access_pattern: AccessPattern,
    /// Deduplication ratio (1.0 = no dedup, >1.0 = space saved)
    pub dedup_ratio: f64,
    /// Record size in bytes
    pub record_size: u64,
}
/// Access pattern classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessPattern {
    /// Sequential I/O pattern
    Sequential,
    /// Random I/O pattern
    Random,
    /// Mixed sequential and random I/O
    Mixed,
}
/// System memory usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMemoryUsage {
    /// Total system memory in bytes
    pub total: u64,
    /// Available memory in bytes
    pub available: u64,
    /// Used memory in bytes
    pub used: u64,
}
/// ARC statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcStatistics {
    /// Current ARC size in bytes
    pub size: u64,
    /// Target ARC size in bytes
    pub target_size: u64,
    /// Cache hit ratio (0.0 to 1.0)
    pub hit_ratio: f64,
    /// Cache miss ratio (0.0 to 1.0)
    pub miss_ratio: f64,
}
/// ZFS bottleneck detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsBottleneck {
    /// Type of bottleneck detected
    pub bottleneck_type: ZfsBottleneckType,
    /// Severity level of the bottleneck
    pub severity: BottleneckSeverity,
    /// Name of the affected ZFS pool
    pub pool_name: String,
    /// Name of the affected dataset (if applicable)
    pub dataset_name: Option<String>,
    /// Human-readable description of the bottleneck
    pub description: String,
    /// Impact score (0.0 to 1.0, higher = more severe)
    pub impact_score: f64,
}
/// Types of ZFS bottlenecks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsBottleneckType {
    /// High latency detected
    HighLatency,
    /// Low throughput detected
    LowThroughput,
    /// High cache miss rate
    CacheMiss,
    /// High pool fragmentation
    Fragmentation,
    /// System memory pressure
    MemoryPressure,
    /// High CPU utilization
    CpuUtilization,
    /// Network bandwidth saturation
    NetworkBandwidth,
    /// Disk I/O saturation
    DiskIo,
}
/// Bottleneck severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BottleneckSeverity {
    /// Low severity - minor impact
    Low,
    /// Medium severity - noticeable impact
    Medium,
    /// High severity - significant impact
    High,
    /// Critical severity - severe impact requiring immediate attention
    Critical,
}
/// Performance optimization result
#[derive(Debug, Clone, Default)]
pub struct PerformanceOptimizationResult {
    /// List of optimizations that were applied
    pub applied_optimizations: Vec<AppliedOptimization>,
    /// Overall performance improvement percentage
    pub performance_improvement: f64,
    /// List of bottlenecks that were resolved
    pub bottlenecks_resolved: Vec<ZfsBottleneck>,
    /// Additional recommendations for further optimization
    pub recommendations: Vec<String>,
}
impl PerformanceOptimizationResult {
    /// Merge another optimization result into this one
    ///
    /// Combines optimizations, improvements, bottlenecks, and recommendations
    /// from multiple optimization runs.
    pub fn merge_with(&mut self, other: Self) {
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
    /// Type of optimization applied
    pub optimization_type: OptimizationType,
    /// Human-readable description of the optimization
    pub description: String,
    /// Measured performance impact (percentage improvement)
    pub performance_impact: f64,
    /// Timestamp when optimization was applied
    pub applied_at: SystemTime,
}
/// Types of optimizations
#[derive(Debug, Clone)]
pub enum OptimizationType {
    /// Cache-related optimization
    CacheOptimization,
    /// Latency reduction optimization
    LatencyOptimization,
    /// Throughput improvement optimization
    ThroughputOptimization,
    /// Defragmentation operation
    FragmentationDefrag,
    /// ARC (cache) tuning
    ArcTuning,
    /// Record size optimization
    RecordSizeOptimization,
    /// Compression algorithm optimization
    CompressionOptimization,
}
/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Type of performance alert
    pub alert_type: AlertType,
    /// Severity level of the alert
    pub severity: AlertSeverity,
    /// Name of the affected ZFS pool
    pub pool_name: String,
    /// Name of the affected dataset (if applicable)
    pub dataset_name: Option<String>,
    /// Human-readable alert description
    pub description: String,
    /// When the alert was triggered
    pub timestamp: SystemTime,
}
/// Alert types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertType {
    /// Performance has degraded below acceptable thresholds
    PerformanceDegradation,
    /// A performance bottleneck has been identified
    BottleneckDetected,
    /// A performance threshold has been exceeded
    ThresholdExceeded,
    /// An optimization attempt has failed
    OptimizationFailed,
}
/// Alert severity levels for performance monitoring
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Informational alert - no action required
    Info,
    /// Warning alert - attention recommended
    Warning,
    /// Error alert - action required
    Error,
    /// Critical alert - immediate action required
    Critical,
}
/// Alert response
#[derive(Debug, Clone, Default)]
pub struct AlertResponse {
    /// Whether mitigation was applied successfully
    pub mitigation_applied: bool,
    /// Result of optimization if applied
    pub optimization_result: Option<PerformanceOptimizationResult>,
    /// Whether follow-up action is required
    pub follow_up_required: bool,
}
/// Workload pattern analysis for storage optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadPattern {
    /// Detected I/O access pattern (sequential, random, or mixed)
    pub access_pattern: AccessPattern,
    /// Distribution of I/O request sizes as histogram
    pub io_size_distribution: HashMap<String, f64>,
    /// Ratio of read operations to write operations
    pub read_write_ratio: f64,
    /// Temporal locality score (0.0 to 1.0, higher = better cache performance)
    pub temporal_locality: f64,
}
/// ZFS configuration context for optimization decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsConfigurationContext {
    /// Name of the ZFS pool being configured
    pub pool_name: String,
    /// Optional dataset name for dataset-specific configuration
    pub dataset_name: Option<String>,
    /// Current ZFS configuration parameters (key-value pairs)
    pub current_configuration: HashMap<String, String>,
    /// Detected workload pattern for this storage
    pub workload_pattern: WorkloadPattern,
    /// System hardware and resource capabilities
    pub system_capabilities: SystemCapabilities,
}
/// ZFS expertise context for intelligent optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsExpertiseContext {
    /// Storage tier classification (hot, warm, cold, archive)
    pub storage_tier: StorageTier,
    /// Historical access patterns observed
    pub access_patterns: Vec<AccessPattern>,
    /// Current performance metrics snapshot
    pub current_performance: ZfsPerformanceMetrics,
    /// List of identified performance bottlenecks
    pub identified_bottlenecks: Vec<ZfsBottleneck>,
}
/// System hardware capabilities and resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCapabilities {
    /// Number of CPU cores available
    pub cpu_cores: u32,
    /// Total system memory in gigabytes
    pub memory_gb: u32,
    /// Type of storage hardware (e.g., "`NVMe`", "SSD", "HDD")
    pub storage_type: String,
    /// Network bandwidth capacity in gigabits per second
    pub network_bandwidth_gbps: f64,
}
/// ZFS tuning operation result
#[derive(Debug, Clone, Default)]
pub struct ZfsTuningResult {
    /// Whether tuning parameters were successfully applied
    pub tuning_applied: bool,
    /// Map of parameter names to their new values
    pub parameter_changes: HashMap<String, String>,
    /// Expected performance improvement as percentage
    pub expected_improvement: f64,
    /// Whether validation testing is required after tuning
    pub validation_required: bool,
}
/// Performance optimization request
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceOptimizationRequest {
    /// Name of the ZFS pool to optimize
    pub pool_name: String,
    /// Optional dataset name for specific dataset optimization
    pub dataset_name: Option<String>,
    /// Current optimization strategy being used
    pub optimization_strategy: EcosystemOptimizationStrategy,
    /// Current performance metrics
    pub current_metrics: ZfsPerformanceMetrics,
    /// ZFS configuration context
    pub configuration_context: ZfsConfigurationContext,
}
/// Ecosystem optimization strategy
#[derive(Debug, Serialize, Deserialize)]
pub enum EcosystemOptimizationStrategy {
    /// Optimize for minimum latency
    LatencyOptimization,
    /// Optimize for maximum throughput
    ThroughputOptimization,
    /// Balance latency and throughput
    BalancedOptimization,
    /// Custom optimization strategy with specified name
    CustomStrategy(String),
}
/// AI optimization recommendation
#[derive(Debug, Serialize, Deserialize)]
pub struct AiOptimizationRecommendation {
    /// Recommended optimization strategy
    pub strategy: EcosystemOptimizationStrategy,
    /// Confidence score for the recommendation (0.0-1.0)
    pub confidence_score: f64,
    /// Expected performance improvement percentage
    pub expected_improvement: f64,
    /// Recommended parameter values
    pub parameter_recommendations: HashMap<String, String>,
    /// Risk assessment description
    pub risk_assessment: String,
}
/// ZFS tuning request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsTuningRequest {
    /// Name of the ZFS pool
    pub pool_name: String,
    /// Optional dataset name
    pub dataset_name: Option<String>,
    /// Detected workload pattern
    pub workload_pattern: WorkloadPattern,
    /// Current ZFS configuration parameters
    pub current_configuration: HashMap<String, String>,
    /// Performance goals for tuning
    pub performance_goals: Vec<String>,
    /// System hardware capabilities
    pub system_capabilities: SystemCapabilities,
    /// ZFS configuration context
    pub configuration_context: ZfsConfigurationContext,
    /// Expertise level context for tuning
    pub expertise_context: ZfsExpertiseContext,
}
/// Ecosystem tuning recommendations
#[derive(Debug, Serialize, Deserialize)]
pub struct EcosystemTuningRecommendations {
    /// List of tuning recommendations
    pub recommendations: Vec<ZfsTuningRecommendation>,
    /// Overall confidence score (0.0-1.0)
    pub overall_confidence: f64,
    /// Estimated performance improvement percentage
    pub estimated_improvement: f64,
}
/// ZFS tuning recommendation
#[derive(Debug, Serialize, Deserialize)]
pub struct ZfsTuningRecommendation {
    /// ZFS parameter to tune
    pub parameter: String,
    /// Recommended value for the parameter
    pub recommendedvalue: String,
    /// Confidence level in this recommendation (0.0-1.0)
    pub confidence: f64,
    /// Expected performance impact percentage
    pub expected_impact: f64,
}
/// Performance alert analysis request
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceAlertAnalysisRequest {
    /// The performance alert being analyzed
    pub alert: PerformanceAlert,
    /// Historical performance metrics for trend analysis
    pub historical_metrics: Vec<ZfsPerformanceMetrics>,
    /// Current ZFS configuration parameters
    pub current_configuration: HashMap<String, String>,
    /// System hardware and resource capabilities
    pub system_capabilities: SystemCapabilities,
}
/// Ecosystem alert analysis
#[derive(Debug, Serialize, Deserialize)]
pub struct EcosystemAlertAnalysis {
    /// Root cause analysis description
    pub root_cause_analysis: String,
    /// Recommended remediation actions
    pub recommended_actions: Vec<String>,
    /// Confidence score for the analysis (0.0 to 1.0)
    pub confidence_score: f64,
    /// Alert urgency level classification
    pub urgency_level: AlertSeverity,
}
/// Custom serialization for `SystemTime`
pub mod system_time_serde {
    use super::{Deserialize, Deserializer, Duration, Serializer, SystemTime, de};
    use std::time::UNIX_EPOCH;
    #[allow(clippy::type_complexity)]
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0));
        serializer.serialize_u64(duration.as_secs())
    }

    /// Deserialize a `SystemTime` from Unix timestamp
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails or timestamp is invalid
    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)
            .map_err(|_e| de::Error::custom("deserialization error: error details".to_string()))?;
        Ok(UNIX_EPOCH + std::time::Duration::from_secs(secs))
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type PerformanceEngineConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using PerformanceEngineConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use super::system_time_serde;
    use super::*;
    use crate::types::StorageTier;
    use std::collections::HashMap;
    use std::time::{Duration, UNIX_EPOCH};

    #[derive(serde::Serialize, serde::Deserialize)]
    struct TimeWrap {
        #[serde(with = "system_time_serde")]
        t: std::time::SystemTime,
    }

    #[test]
    fn performance_engine_config_default() {
        let c = PerformanceEngineConfig::default();
        assert!(c.enable_ai_guidance);
        assert_eq!(c.max_concurrent_optimizations, 3);
    }

    #[test]
    fn optimization_state_variants() {
        let states = [
            OptimizationState::Idle,
            OptimizationState::Collecting,
            OptimizationState::Analyzing,
            OptimizationState::Optimizing,
            OptimizationState::Validating,
            OptimizationState::Applied,
        ];
        assert_eq!(states[0], OptimizationState::default());
        assert_ne!(states[1], states[0]);
    }

    #[test]
    fn performance_optimization_result_merge_with() {
        let mut a = PerformanceOptimizationResult {
            applied_optimizations: vec![AppliedOptimization {
                optimization_type: OptimizationType::CacheOptimization,
                description: "a".to_string(),
                performance_impact: 1.0,
                applied_at: UNIX_EPOCH + Duration::from_secs(1),
            }],
            performance_improvement: 2.0,
            bottlenecks_resolved: vec![],
            recommendations: vec!["r".to_string()],
        };
        let b = PerformanceOptimizationResult {
            applied_optimizations: vec![AppliedOptimization {
                optimization_type: OptimizationType::ArcTuning,
                description: "b".to_string(),
                performance_impact: 3.0,
                applied_at: UNIX_EPOCH + Duration::from_secs(2),
            }],
            performance_improvement: 4.0,
            bottlenecks_resolved: vec![ZfsBottleneck {
                bottleneck_type: ZfsBottleneckType::HighLatency,
                severity: BottleneckSeverity::Low,
                pool_name: "p".to_string(),
                dataset_name: None,
                description: "d".to_string(),
                impact_score: 0.1,
            }],
            recommendations: vec!["x".to_string()],
        };
        a.merge_with(b);
        assert_eq!(a.applied_optimizations.len(), 2);
        assert!((a.performance_improvement - 6.0).abs() < f64::EPSILON);
        assert_eq!(a.bottlenecks_resolved.len(), 1);
        assert_eq!(a.recommendations.len(), 2);
    }

    #[test]
    fn alert_response_and_zfs_tuning_result_default() {
        let a = AlertResponse::default();
        assert!(!a.mitigation_applied);
        let z = ZfsTuningResult::default();
        assert!(!z.tuning_applied);
    }

    #[test]
    fn system_time_serde_roundtrip_and_epoch_fallback() {
        let t = UNIX_EPOCH + Duration::from_secs(12345);
        let w = TimeWrap { t };
        let v = serde_json::to_value(&w).expect("ser");
        let back: TimeWrap = serde_json::from_value(v).expect("de");
        assert_eq!(back.t, w.t);

        let before_epoch = TimeWrap {
            t: UNIX_EPOCH - Duration::from_secs(10),
        };
        let v2 = serde_json::to_value(&before_epoch).expect("ser");
        let back2: TimeWrap = serde_json::from_value(v2).expect("de");
        assert_eq!(back2.t, UNIX_EPOCH);
    }

    #[test]
    fn system_time_serde_deserialize_invalid_type_errors() {
        let err = serde_json::from_str::<TimeWrap>(r#"{"t":null}"#);
        assert!(err.is_err());
    }

    #[test]
    fn serde_roundtrip_core_metrics_and_enums() {
        let m = ZfsPerformanceMetrics {
            timestamp: UNIX_EPOCH,
            pool_metrics: HashMap::from([(
                "tank".to_string(),
                ZfsPoolMetrics {
                    pool_name: "tank".to_string(),
                    read_ops: 1.0,
                    write_ops: 2.0,
                    read_bandwidth: 3.0,
                    write_bandwidth: 4.0,
                    latency: 5.0,
                    cache_hit_ratio: 0.5,
                    fragmentation: 10.0,
                },
            )]),
            dataset_metrics: HashMap::from([(
                "tank/d".to_string(),
                ZfsDatasetMetrics {
                    dataset_name: "tank/d".to_string(),
                    access_pattern: AccessPattern::Sequential,
                    dedup_ratio: 1.0,
                    record_size: 4096,
                },
            )]),
            system_memory: SystemMemoryUsage {
                total: 1,
                available: 1,
                used: 0,
            },
            arc_stats: ArcStatistics {
                size: 1,
                target_size: 2,
                hit_ratio: 0.9,
                miss_ratio: 0.1,
            },
        };
        let j = serde_json::to_string(&m).expect("ser");
        let _: ZfsPerformanceMetrics = serde_json::from_str(&j).expect("de");

        let patterns = [
            AccessPattern::Sequential,
            AccessPattern::Random,
            AccessPattern::Mixed,
        ];
        for p in patterns {
            let v = serde_json::to_string(&p).unwrap();
            let _: AccessPattern = serde_json::from_str(&v).unwrap();
        }

        let bottlenecks = [
            ZfsBottleneckType::HighLatency,
            ZfsBottleneckType::LowThroughput,
            ZfsBottleneckType::CacheMiss,
            ZfsBottleneckType::Fragmentation,
            ZfsBottleneckType::MemoryPressure,
            ZfsBottleneckType::CpuUtilization,
            ZfsBottleneckType::NetworkBandwidth,
            ZfsBottleneckType::DiskIo,
        ];
        for b in bottlenecks {
            let v = serde_json::to_string(&b).unwrap();
            let _: ZfsBottleneckType = serde_json::from_str(&v).unwrap();
        }

        let sev = [
            BottleneckSeverity::Low,
            BottleneckSeverity::Medium,
            BottleneckSeverity::High,
            BottleneckSeverity::Critical,
        ];
        for s in sev {
            let v = serde_json::to_string(&s).unwrap();
            let _: BottleneckSeverity = serde_json::from_str(&v).unwrap();
        }
    }

    #[test]
    fn serde_performance_alert_and_strategies() {
        let alert = PerformanceAlert {
            alert_type: AlertType::BottleneckDetected,
            severity: AlertSeverity::Warning,
            pool_name: "p".to_string(),
            dataset_name: Some("d".to_string()),
            description: "x".to_string(),
            timestamp: UNIX_EPOCH,
        };
        let j = serde_json::to_string(&alert).expect("ser");
        let _: PerformanceAlert = serde_json::from_str(&j).expect("de");

        let strategies = [
            EcosystemOptimizationStrategy::LatencyOptimization,
            EcosystemOptimizationStrategy::ThroughputOptimization,
            EcosystemOptimizationStrategy::BalancedOptimization,
            EcosystemOptimizationStrategy::CustomStrategy("c".to_string()),
        ];
        for s in strategies {
            let v = serde_json::to_string(&s).unwrap();
            let _: EcosystemOptimizationStrategy = serde_json::from_str(&v).unwrap();
        }

        let alert_types = [
            AlertType::PerformanceDegradation,
            AlertType::BottleneckDetected,
            AlertType::ThresholdExceeded,
            AlertType::OptimizationFailed,
        ];
        for a in alert_types {
            let v = serde_json::to_string(&a).unwrap();
            let _: AlertType = serde_json::from_str(&v).unwrap();
        }

        let sev = [
            AlertSeverity::Info,
            AlertSeverity::Warning,
            AlertSeverity::Error,
            AlertSeverity::Critical,
        ];
        for s in sev {
            let v = serde_json::to_string(&s).unwrap();
            let _: AlertSeverity = serde_json::from_str(&v).unwrap();
        }
    }

    #[test]
    fn serde_workload_and_tuning_requests() {
        let wp = WorkloadPattern {
            access_pattern: AccessPattern::Random,
            io_size_distribution: HashMap::from([("4k".to_string(), 0.5)]),
            read_write_ratio: 1.0,
            temporal_locality: 0.25,
        };
        let j = serde_json::to_string(&wp).expect("ser");
        let _: WorkloadPattern = serde_json::from_str(&j).expect("de");

        let caps = SystemCapabilities {
            cpu_cores: 8,
            memory_gb: 16,
            storage_type: "NVMe".to_string(),
            network_bandwidth_gbps: 10.0,
        };
        let ctx = ZfsConfigurationContext {
            pool_name: "p".to_string(),
            dataset_name: None,
            current_configuration: HashMap::from([("key".to_string(), "val".to_string())]),
            workload_pattern: wp.clone(),
            system_capabilities: caps.clone(),
        };
        let zpm = ZfsPerformanceMetrics {
            timestamp: UNIX_EPOCH,
            pool_metrics: HashMap::new(),
            dataset_metrics: HashMap::new(),
            system_memory: SystemMemoryUsage {
                total: 1,
                available: 1,
                used: 0,
            },
            arc_stats: ArcStatistics {
                size: 1,
                target_size: 1,
                hit_ratio: 0.5,
                miss_ratio: 0.5,
            },
        };
        let expert = ZfsExpertiseContext {
            storage_tier: StorageTier::Hot,
            access_patterns: vec![AccessPattern::Mixed],
            current_performance: zpm.clone(),
            identified_bottlenecks: vec![],
        };
        let req = ZfsTuningRequest {
            pool_name: "p".to_string(),
            dataset_name: None,
            workload_pattern: wp,
            current_configuration: HashMap::new(),
            performance_goals: vec!["latency".to_string()],
            system_capabilities: caps,
            configuration_context: ctx.clone(),
            expertise_context: expert,
        };
        let j = serde_json::to_string(&req).expect("ser");
        let _: ZfsTuningRequest = serde_json::from_str(&j).expect("de");

        let po = PerformanceOptimizationRequest {
            pool_name: "p".to_string(),
            dataset_name: None,
            optimization_strategy: EcosystemOptimizationStrategy::BalancedOptimization,
            current_metrics: zpm,
            configuration_context: ctx,
        };
        let j = serde_json::to_string(&po).expect("ser");
        let _: PerformanceOptimizationRequest = serde_json::from_str(&j).expect("de");
    }

    #[test]
    fn serde_ai_and_ecosystem_tuning_and_alert_analysis() {
        let ai = AiOptimizationRecommendation {
            strategy: EcosystemOptimizationStrategy::LatencyOptimization,
            confidence_score: 0.9,
            expected_improvement: 12.0,
            parameter_recommendations: HashMap::from([("a".to_string(), "b".to_string())]),
            risk_assessment: "low".to_string(),
        };
        let j = serde_json::to_string(&ai).expect("ser");
        let _: AiOptimizationRecommendation = serde_json::from_str(&j).expect("de");

        let eco = EcosystemTuningRecommendations {
            recommendations: vec![ZfsTuningRecommendation {
                parameter: "recordsize".to_string(),
                recommendedvalue: "128k".to_string(),
                confidence: 0.8,
                expected_impact: 5.0,
            }],
            overall_confidence: 0.8,
            estimated_improvement: 5.0,
        };
        let j = serde_json::to_string(&eco).expect("ser");
        let _: EcosystemTuningRecommendations = serde_json::from_str(&j).expect("de");

        let analysis = EcosystemAlertAnalysis {
            root_cause_analysis: "cpu".to_string(),
            recommended_actions: vec!["tune".to_string()],
            confidence_score: 0.7,
            urgency_level: AlertSeverity::Error,
        };
        let j = serde_json::to_string(&analysis).expect("ser");
        let _: EcosystemAlertAnalysis = serde_json::from_str(&j).expect("de");

        let par = PerformanceAlertAnalysisRequest {
            alert: PerformanceAlert {
                alert_type: AlertType::ThresholdExceeded,
                severity: AlertSeverity::Info,
                pool_name: "p".to_string(),
                dataset_name: None,
                description: "d".to_string(),
                timestamp: UNIX_EPOCH,
            },
            historical_metrics: vec![],
            current_configuration: HashMap::new(),
            system_capabilities: SystemCapabilities {
                cpu_cores: 1,
                memory_gb: 1,
                storage_type: "HDD".to_string(),
                network_bandwidth_gbps: 1.0,
            },
        };
        let j = serde_json::to_string(&par).expect("ser");
        let _: PerformanceAlertAnalysisRequest = serde_json::from_str(&j).expect("de");
    }

    #[test]
    fn optimization_type_debug_covers_all_variants() {
        let opts = [
            OptimizationType::CacheOptimization,
            OptimizationType::LatencyOptimization,
            OptimizationType::ThroughputOptimization,
            OptimizationType::FragmentationDefrag,
            OptimizationType::ArcTuning,
            OptimizationType::RecordSizeOptimization,
            OptimizationType::CompressionOptimization,
        ];
        let joined = opts
            .iter()
            .map(|o| format!("{o:?}"))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(joined.contains("Cache"));
        assert!(joined.contains("Compression"));
    }
}
