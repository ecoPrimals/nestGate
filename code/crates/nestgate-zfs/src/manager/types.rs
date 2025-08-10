//! ZFS Manager Types - Data structures for ZFS management
//!
//! Contains all the data structures used by the ZFS manager including
//! service information, tier analysis, performance metrics, and status reporting.

use crate::types::StorageTier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Enhanced service information for orchestrator registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub endpoint: String,
    pub health_endpoint: String,
    pub capabilities: Vec<String>,
    pub metadata: HashMap<String, String>,
    /// AI capabilities
    pub ai_capabilities: Vec<String>,
    /// Performance monitoring features
    pub monitoring_features: Vec<String>,
}

/// Tier benefits analysis
#[derive(Debug, Clone)]
pub struct TierBenefits {
    pub performance_improvement: f64,
    pub cost_savings: f64,
    pub storage_efficiency: f64,
}

/// File analysis data for AI predictions
#[derive(Debug, Clone)]
pub struct FileAnalysisData {
    pub file_size: u64,
    pub file_type: String,
    pub access_frequency: String,
    pub last_accessed: u64,
    pub last_modified: u64,
    pub is_system_critical: bool,
    pub is_frequently_accessed_dir: bool,
    pub estimated_access_pattern: String,
}

/// Current metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentMetrics {
    pub operations_per_second: f64,
    pub throughput_bytes_per_second: u64,
    pub average_latency_ms: f64,
    pub error_rate: f64,
}

impl Default for CurrentMetrics {
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
pub struct EnhancedServiceStatus {
    pub overall_health: HealthState,
    pub pool_status: PoolOverallStatus,
    pub tier_status: TierOverallStatus,
    pub performance_metrics: crate::performance::CurrentPerformanceMetrics,
    pub ai_status: Option<AiIntegrationStatus>,
    pub migration_status: MigrationStatus,
    pub snapshot_status: SnapshotStatus,
    pub metrics: CurrentMetrics,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// AI integration status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiIntegrationStatus {
    pub enabled: bool,
    pub models_deployed: u32,
    pub optimization_active: bool,
    pub last_optimization: SystemTime,
    pub prediction_accuracy: f64,
}

/// Migration status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationStatus {
    pub active_jobs: u32,
    pub queued_jobs: u32,
    pub completed_jobs: u64,
    pub failed_jobs: u64,
    pub total_bytes_migrated: u64,
}

/// Snapshot status
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapshotStatus {
    pub total_snapshots: u64,
    pub active_policies: u32,
    pub pending_operations: u32,
    pub recent_failures: u32,
}

/// Performance analytics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnalytics {
    pub current_metrics: crate::performance::CurrentPerformanceMetrics,
    pub history: Vec<crate::performance::PerformanceSnapshot>,
    pub active_alerts: Vec<crate::performance::ActiveAlert>,
    pub tier_analytics: HashMap<StorageTier, crate::performance::TierPerformanceData>,
}

/// Optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    pub timestamp: SystemTime,
    pub results: Vec<String>,
    pub success: bool,
}

/// Health state enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthState {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// Pool overall status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolOverallStatus {
    pub pools_online: usize,
    pub pools_degraded: usize,
    pub total_capacity: u64,
    pub available_capacity: u64,
}

/// Tier overall status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierOverallStatus {
    pub hot_utilization: f64,
    pub warm_utilization: f64,
    pub cold_utilization: f64,
    pub migration_queue_size: usize,
}

/// File analysis data structure for heuristic tier prediction
#[derive(Debug, Clone)]
pub struct FileAnalysis {
    pub file_path: String,
    pub file_size: u64,
    pub file_extension: String,
    pub file_type: String,
    pub estimated_access_frequency: f64,
    pub is_system_critical: bool,
    pub estimated_compression_ratio: f64,
}

/// Capacity information helper struct
#[derive(Debug)]
pub struct CapacityInfo {
    pub used_bytes: u64,
    pub total_bytes: u64,
}
