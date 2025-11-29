//
// Contains all the data structures used by the ZFS manager including
// service information, tier analysis, performance metrics, and status reporting.

use crate::types::StorageTier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Enhanced service information for orchestrator registration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceinfo
pub struct ServiceInfo {
    /// Name
    pub name: String,
    /// Endpoint
    pub endpoint: String,
    /// Health Endpoint
    pub health_endpoint: String,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// AI capabilities
    pub ai_capabilities: Vec<String>,
    /// Performance monitoring features
    pub monitoring_features: Vec<String>,
}
/// Tier benefits analysis
#[derive(Debug, Clone)]
/// Tierbenefits
pub struct TierBenefits {
    /// Performance Improvement
    pub performance_improvement: f64,
    /// Cost Savings
    pub cost_savings: f64,
    /// Storage Efficiency
    pub storage_efficiency: f64,
}
/// File analysis data for AI predictions
#[derive(Debug, Clone)]
/// Fileanalysisdata
pub struct FileAnalysisData {
    /// Size of file
    pub file_size: u64,
    /// File Type
    pub file_type: String,
    /// Access Frequency
    pub access_frequency: String,
    /// Last Accessed
    pub last_accessed: u64,
    /// Last Modified
    pub last_modified: u64,
    /// Whether system critical
    pub is_system_critical: bool,
    /// Whether frequently accessed dir
    pub is_frequently_accessed_dir: bool,
    /// Estimated Access Pattern
    pub estimated_access_pattern: String,
}
/// Current metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Currentmetrics
pub struct CurrentMetrics {
    /// Operations Per Second
    pub operations_per_second: f64,
    /// Throughput Bytes Per Second
    pub throughput_bytes_per_second: u64,
    /// Average Latency Ms
    pub average_latency_ms: f64,
    /// Error Rate
    pub error_rate: f64,
}
impl Default for CurrentMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            throughput_bytes_per_second: 0,
            average_latency_ms: 0.0,
            error_rate: 0.0,
        }
    }
}

/// Enhanced service status for health reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enhancedservicestatus
pub struct EnhancedServiceStatus {
    /// Overall Health
    pub overall_health: HealthState,
    /// Pool Status
    pub pool_status: PoolOverallStatus,
    /// Tier Status
    pub tier_status: TierOverallStatus,
    /// Performance Metrics
    pub performance_metrics: crate::performance::CurrentPerformanceMetrics,
    /// Ai Status
    pub ai_status: Option<AiIntegrationStatus>,
    /// Migration Status
    pub migration_status: MigrationStatus,
    /// Snapshot Status
    pub snapshot_status: SnapshotStatus,
    /// Metrics
    pub metrics: CurrentMetrics,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
/// AI integration status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Aiintegrationstatus
pub struct AiIntegrationStatus {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Models Deployed
    pub models_deployed: u32,
    /// Optimization Active
    pub optimization_active: bool,
    /// Last Optimization
    pub last_optimization: SystemTime,
    /// Prediction Accuracy
    pub prediction_accuracy: f64,
}
/// Migration status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Migrationstatus
pub struct MigrationStatus {
    /// Active Jobs
    pub active_jobs: u32,
    /// Queued Jobs
    pub queued_jobs: u32,
    /// Completed Jobs
    pub completed_jobs: u64,
    /// Failed Jobs
    pub failed_jobs: u64,
    /// Total Bytes Migrated
    pub total_bytes_migrated: u64,
}
/// Snapshot status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Snapshotstatus
pub struct SnapshotStatus {
    /// Total Snapshots
    pub total_snapshots: u64,
    /// Active Policies
    pub active_policies: u32,
    /// Pending Operations
    pub pending_operations: u32,
    /// Recent Failures
    pub recent_failures: u32,
}
/// Performance analytics data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceanalytics
pub struct PerformanceAnalytics {
    /// Current Metrics
    pub current_metrics: crate::performance::CurrentPerformanceMetrics,
    /// History
    pub history: Vec<crate::performance::PerformanceSnapshot>,
    /// Active Alerts
    pub active_alerts: Vec<crate::performance::ActiveAlert>,
    /// Tier Analytics
    pub tier_analytics: HashMap<StorageTier, crate::performance::TierPerformanceData>,
}
/// Optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Optimizationresult
pub struct OptimizationResult {
    /// Timestamp
    pub timestamp: SystemTime,
    /// Results
    pub results: Vec<String>,
    /// Success
    pub success: bool,
}
/// Health state enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthstate
pub enum HealthState {
    /// Healthy
    Healthy,
    /// Warning
    Warning,
    /// Critical
    Critical,
    /// Unknown
    Unknown,
}
/// Pool overall status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Pooloverallstatus
pub struct PoolOverallStatus {
    /// Pools Online
    pub pools_online: usize,
    /// Pools Degraded
    pub pools_degraded: usize,
    /// Total Capacity
    pub total_capacity: u64,
    /// Available Capacity
    pub available_capacity: u64,
}
/// Tier overall status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tieroverallstatus
pub struct TierOverallStatus {
    /// Hot Utilization
    pub hot_utilization: f64,
    /// Warm Utilization
    pub warm_utilization: f64,
    /// Cold Utilization
    pub cold_utilization: f64,
    /// Size of migration queue
    pub migration_queue_size: usize,
}
/// File analysis data structure for heuristic tier prediction
#[derive(Debug, Clone)]
/// Fileanalysis
pub struct FileAnalysis {
    /// File Path
    pub file_path: String,
    /// Size of file
    pub file_size: u64,
    /// File Extension
    pub file_extension: String,
    /// File Type
    pub file_type: String,
    /// Estimated Access Frequency
    pub estimated_access_frequency: f64,
    /// Whether system critical
    pub is_system_critical: bool,
    /// Estimated Compression Ratio
    pub estimated_compression_ratio: f64,
}
/// Capacity information helper struct
#[derive(Debug)]
/// Capacityinfo
pub struct CapacityInfo {
    /// Used Bytes
    pub used_bytes: u64,
    /// Total Bytes
    pub total_bytes: u64,
}
