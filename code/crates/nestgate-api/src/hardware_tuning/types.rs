//! Hardware Tuning Types and Data Structures
//!
//! This module contains all the data structures, enums, and types
//! used by the hardware tuning system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Service registration for Toadstool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningServiceRegistration {
    pub name: String,
    pub service_type: String,
    pub capabilities: Vec<String>,
    pub resource_requirements: ResourceRequirements,
    pub health_check_url: String,
}

/// Compute resource request to Toadstool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResourceRequest {
    pub session_id: Uuid,
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub gpu_required: bool,
    pub duration_minutes: Option<u32>,
    pub priority: ComputePriority,
}

/// Compute allocation response from Toadstool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeAllocation {
    pub allocation_id: String,
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub gpu_allocation: Option<GpuAllocation>,
    pub expires_at: DateTime<Utc>,
    pub compute_node: String,
}

/// GPU allocation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuAllocation {
    pub gpu_count: u32,
    pub gpu_type: String,
    pub memory_gb: u32,
}

/// Compute priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComputePriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Live hardware metrics from Toadstool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveHardwareMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub gpu_usage: Option<f64>,
    pub temperature: f64,
    pub power_consumption: f64,
    pub network_io: NetworkIoMetrics,
    pub disk_io: DiskIoMetrics,
}

/// Network I/O metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIoMetrics {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
}

/// Disk I/O metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIoMetrics {
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_ops: u64,
    pub write_ops: u64,
}

/// Hardware event from live feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: HardwareEventType,
    pub data: serde_json::Value,
}

/// Hardware event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HardwareEventType {
    MetricsUpdate,
    ThresholdExceeded,
    ResourceAllocation,
    ResourceDeallocation,
    SystemAlert,
}

/// Resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub min_cpu_cores: u32,
    pub min_memory_gb: u32,
    pub preferred_cpu_cores: u32,
    pub preferred_memory_gb: u32,
    pub gpu_required: bool,
}

/// Platform information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    pub os: String,
    pub architecture: String,
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub gpu_count: u32,
    pub storage_devices: Vec<StorageDevice>,
}

/// Storage device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    pub name: String,
    pub device_type: String,
    pub capacity_gb: u64,
    pub available_gb: u64,
    pub mount_point: String,
}

/// Real-time metrics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub gpu_usage: Option<f64>,
    pub network_io: NetworkIoMetrics,
    pub disk_io: DiskIoMetrics,
    pub storage_io: StorageIoMetrics,
    pub system_load: SystemLoadMetrics,
}

/// Storage I/O metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageIoMetrics {
    pub read_iops: u64,
    pub write_iops: u64,
    pub read_bandwidth_mbps: f64,
    pub write_bandwidth_mbps: f64,
}

/// System load metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLoadMetrics {
    pub load_1m: f64,
    pub load_5m: f64,
    pub load_15m: f64,
}

/// Compute discovery information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeDiscovery {
    pub nodes: Vec<ComputeNode>,
    pub total_cpu_cores: u32,
    pub total_memory_gb: u32,
    pub total_gpu_count: u32,
    pub available_cpu_cores: u32,
    pub available_memory_gb: u32,
    pub available_gpu_count: u32,
}

/// Compute node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeNode {
    pub node_id: String,
    pub hostname: String,
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub gpu_devices: Vec<GpuDevice>,
    pub network_interfaces: Vec<NetworkInterface>,
    pub storage_pools: Vec<StoragePool>,
}

/// GPU device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuDevice {
    pub device_id: String,
    pub gpu_type: String,
    pub memory_gb: u32,
    pub compute_capability: String,
}

/// Network interface information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub speed_gbps: f64,
    pub ip_address: String,
}

/// Storage pool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePool {
    pub pool_name: String,
    pub total_capacity_gb: u64,
    pub available_capacity_gb: u64,
    pub pool_type: String,
    pub performance_tier: String,
}

/// System health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub overall_status: HealthStatus,
    pub cpu_status: HealthStatus,
    pub memory_status: HealthStatus,
    pub storage_status: HealthStatus,
    pub network_status: HealthStatus,
    pub alerts: Vec<SystemAlert>,
}

/// Health status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// System alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAlert {
    pub alert_id: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub component: String,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Storage workload information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageWorkload {
    pub workload_id: String,
    pub workload_type: String,
    pub priority: ComputePriority,
    pub resource_requirements: WorkloadResourceRequirements,
    pub estimated_duration_minutes: u32,
    pub status: WorkloadStatus,
}

/// Workload resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadResourceRequirements {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub storage_gb: u64,
    pub network_bandwidth_gbps: f64,
}

/// Workload execution information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadExecution {
    pub execution_id: String,
    pub workload_id: String,
    pub start_time: DateTime<Utc>,
    pub estimated_end_time: DateTime<Utc>,
    pub progress_percent: f64,
    pub allocated_resources: AllocatedResources,
    pub status: WorkloadStatus,
}

/// Allocated resources information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocatedResources {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub storage_gb: u64,
    pub network_bandwidth_gbps: f64,
    pub compute_nodes: Vec<String>,
}

/// Workload status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkloadStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Storage resource request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceRequest {
    pub request_id: String,
    pub workload_id: String,
    pub resource_type: String,
    pub quantity: u64,
    pub duration_minutes: u32,
}

/// Storage resource allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceAllocation {
    pub allocation_id: String,
    pub request_id: String,
    pub allocated_quantity: u64,
    pub allocated_nodes: Vec<String>,
    pub expires_at: DateTime<Utc>,
}

/// Storage process request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProcessRequest {
    pub process_name: String,
    pub process_type: String,
    pub parameters: serde_json::Value,
}

/// Process management information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessManagement {
    pub process_id: String,
    pub process_name: String,
    pub status: String,
    pub start_time: DateTime<Utc>,
    pub resource_usage: ResourceUsage,
}

/// Resource usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f64,
    pub memory_mb: u64,
    pub disk_read_mb: u64,
    pub disk_write_mb: u64,
}

/// Storage optimization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOptimizationRequest {
    pub optimization_type: String,
    pub target_storage_pools: Vec<String>,
    pub optimization_level: String,
    pub parameters: serde_json::Value,
}

/// Storage optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOptimization {
    pub optimization_id: String,
    pub optimization_type: String,
    pub performance_improvement: f64,
    pub resource_savings: ResourceSavings,
    pub completion_time: DateTime<Utc>,
}

/// Resource savings information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSavings {
    pub cpu_percent_saved: f64,
    pub memory_mb_saved: u64,
    pub storage_gb_saved: u64,
    pub network_bandwidth_saved_gbps: f64,
}

/// Tuning mode enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TuningMode {
    Auto,
    Manual,
    Aggressive,
    Conservative,
    Balanced,
}

/// Session status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Inactive,
    Suspended,
    Expired,
    Error,
    Completed,
}

/// Tuning session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningSession {
    pub session_id: Uuid,
    pub user_id: String,
    pub start_time: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub status: SessionStatus,
    pub tuning_mode: TuningMode,
    pub active_profiles: Vec<String>,
}

/// Hardware tuning request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareTuningRequest {
    pub session_id: Uuid,
    pub tuning_mode: TuningMode,
    pub target_hardware: Vec<String>,
}

/// Hardware tuning response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareTuningResponse {
    pub session_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub status: SessionStatus,
    pub hardware_config: Option<serde_json::Value>,
    pub result: Option<TuningResult>,
    pub performance_improvement: Option<f64>,
    pub external_access_status: ExternalAccessStatus,
    pub recommendations: TuningRecommendations,
    pub warnings: Vec<String>,
}

/// Tuning query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningQuery {
    pub session_id: Option<Uuid>,
    pub include_metrics: Option<bool>,
    pub include_recommendations: Option<bool>,
}

/// External access requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAccessRequirements {
    pub access_type: String,
    pub required_permissions: Vec<String>,
    pub duration_hours: u32,
    pub justification: String,
    pub requester_id: String,
}

/// Benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub benchmark_id: String,
    pub benchmark_name: String,
    pub score: f64,
    pub duration_ms: u64,
    pub metadata: serde_json::Value,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_score: f64,
    pub memory_score: f64,
    pub disk_score: f64,
    pub network_score: f64,
    pub overall_score: f64,
    pub benchmark_results: Vec<BenchmarkResult>,
}

/// External access status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAccessStatus {
    pub access_granted: bool,
    pub access_level: String,
    pub restrictions: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Tuning recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningRecommendations {
    pub cpu_recommendations: Vec<String>,
    pub memory_recommendations: Vec<String>,
    pub storage_recommendations: Vec<String>,
    pub network_recommendations: Vec<String>,
}

/// Live performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivePerformanceMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub gpu_usage: Option<f64>,
    pub disk_io: DiskIoMetrics,
    pub network_io: NetworkIoMetrics,
    pub system_load: SystemLoadMetrics,
    pub temperature: f64,
    pub power_consumption: f64,
}

// Re-export from nestgate_core for convenience
pub use nestgate_core::hardware_tuning::{
    CopyleftRequirements, CryptographicProof, ExternalLockType, ExtractionLock,
    ExtractionRestrictions, HardwareAgnosticTuner, HardwareConfiguration, TimeRestrictions,
    TuningProfile, TuningResult,
};
