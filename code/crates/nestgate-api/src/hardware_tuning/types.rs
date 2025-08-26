//
// This module contains all the data structures, enums, and types
// used by the hardware tuning system.

use chrono::{DateTime, Utc};
use crate::canonical_modernization::UnifiedHealthStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Service registration for Toadstool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningServiceRegistration {
    /// Name of the tuning service
    pub name: String,
    /// Type of service (e.g., "cpu-optimizer", "memory-tuner")
    pub service_type: String,
    /// List of capabilities this service provides
    pub capabilities: Vec<String>,
    /// Resource requirements for running this service
    pub resource_requirements: ResourceRequirements,
    /// URL endpoint for health check monitoring
    pub health_check_url: String,
}

/// Compute resource request to Toadstool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResourceRequest {
    /// Unique identifier for this tuning session
    pub session_id: Uuid,
    /// Number of CPU cores required
    pub cpu_cores: u32,
    /// Amount of memory required in gigabytes
    pub memory_gb: u32,
    /// Whether GPU acceleration is required
    pub gpu_required: bool,
    /// Maximum duration for resource allocation in minutes
    pub duration_minutes: Option<u32>,
    /// Priority level for resource allocation
    pub priority: ComputePriority,
}

/// Compute allocation response from Toadstool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeAllocation {
    /// Unique identifier for this allocation
    pub allocation_id: String,
    /// Number of CPU cores allocated
    pub cpu_cores: u32,
    /// Amount of memory allocated in gigabytes
    pub memory_gb: u32,
    /// GPU allocation details if GPU was requested
    pub gpu_allocation: Option<GpuAllocation>,
    /// Timestamp when this allocation expires
    pub expires_at: DateTime<Utc>,
    /// Name of the compute node where resources are allocated
    pub compute_node: String,
}

/// GPU allocation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuAllocation {
    /// Number of GPUs allocated
    pub gpu_count: u32,
    /// Type/model of GPU allocated
    pub gpu_type: String,
    /// Amount of GPU memory in gigabytes
    pub memory_gb: u32,
}

/// Compute priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComputePriority {
    /// Low priority - can be preempted by higher priority tasks
    Low,
    /// Normal priority - standard processing priority
    Normal,
    /// High priority - prioritized over normal tasks
    High,
    /// Critical priority - highest priority, cannot be preempted
    Critical,
}

/// Compute request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeRequest {
    /// Number of CPU cores requested
    pub cpu_cores: u32,
    /// Amount of memory requested in gigabytes
    pub memory_gb: u32,
    /// Whether GPU acceleration is required
    pub gpu_required: bool,
    /// Maximum duration for the request in minutes
    pub duration_minutes: Option<u32>,
}

/// Compute resources structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResources {
    /// Number of CPU cores available
    pub cpu_cores: u32,
    /// Amount of memory available in gigabytes
    pub memory_gb: u32,
    /// Number of GPUs available
    pub gpu_count: u32,
    /// Amount of storage available in gigabytes
    pub storage_gb: u32,
}

/// Live hardware metrics from Toadstool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveHardwareMetrics {
    /// Timestamp when these metrics were collected
    pub timestamp: DateTime<Utc>,
    /// CPU usage percentage (0.0 to 100.0)
    pub _cpu_usage: f64,
    /// Memory usage percentage (0.0 to 100.0)
    pub memory_usage: f64,
    /// GPU usage percentage if GPU is available (0.0 to 100.0)
    pub gpu_usage: Option<f64>,
    /// System temperature in Celsius
    pub temperature: f64,
    /// Power consumption in watts
    pub power_consumption: f64,
    /// Network I/O statistics
    pub network_io: NetworkIoMetrics,
    /// Disk I/O statistics
    pub disk_io: DiskIoMetrics,
}

/// Network I/O metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIoMetrics {
    /// Total bytes sent over network
    pub bytes_sent: u64,
    /// Total bytes received over network
    pub bytes_received: u64,
    /// Total packets sent over network
    pub packets_sent: u64,
    /// Total packets received over network
    pub packets_received: u64,
}

/// Disk I/O performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIoMetrics {
    /// Number of bytes read from storage
    pub read_bytes: u64,
    /// Number of bytes written to storage
    pub write_bytes: u64,
    /// Number of read operations performed
    pub read_ops: u64,
    /// Number of write operations performed
    pub write_ops: u64,
}

/// Hardware event with timestamp and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareEvent {
    /// Timestamp when the event occurred
    pub timestamp: DateTime<Utc>,
    /// Type of hardware event
    pub event_type: HardwareEventType,
    /// Additional event data in JSON format
    pub data: serde_json::Value,
}

/// Types of hardware events that can be monitored
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HardwareEventType {
    /// System metrics have been updated
    MetricsUpdate,
    /// A performance threshold has been exceeded
    ThresholdExceeded,
    /// Resources have been allocated to a task
    ResourceAllocation,
    /// Resources have been deallocated from a task
    ResourceDeallocation,
    /// A system alert has been triggered
    SystemAlert,
}

/// Hardware requirements for a workload or service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// Minimum number of CPU cores required
    pub min_cpu_cores: u32,
    /// Minimum amount of memory in GB required
    pub min_memory_gb: u32,
    /// Preferred number of CPU cores for optimal performance
    pub preferred_cpu_cores: u32,
    /// Preferred amount of memory in GB for optimal performance
    pub preferred_memory_gb: u32,
    /// Whether GPU acceleration is required
    pub gpu_required: bool,
}

/// Information about a compute node in the cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformInfo {
    /// Operating system running on the node
    pub os: String,
    /// CPU architecture (e.g., x86_64, arm64)
    pub architecture: String,
    /// Number of CPU cores available
    pub cpu_cores: u32,
    /// Amount of memory in GB available
    pub memory_gb: u32,
    /// Number of GPUs available
    pub gpu_count: u32,
    /// Storage devices attached to this node
    pub storage_devices: Vec<StorageDevice>,
}

/// Information about a storage device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    /// Device name or identifier
    pub name: String,
    /// Type of storage device (SSD, HDD, NVMe, etc.)
    pub device_type: String,
    /// Total capacity in GB
    pub capacity_gb: u64,
    /// Available space in GB
    pub available_gb: u64,
    /// Mount point where the device is accessible
    pub mount_point: String,
}

/// Storage I/O metrics for performance monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageIoMetrics {
    /// Number of bytes read from storage
    pub read_bytes: u64,
    /// Number of bytes written to storage
    pub write_bytes: u64,
    /// Number of read operations performed
    pub read_ops: u64,
    /// Number of write operations performed
    pub write_ops: u64,
}

/// System load averages over different time periods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLoadMetrics {
    /// Load average over 1 minute
    pub load_1m: f64,
    /// Load average over 5 minutes
    pub load_5m: f64,
    /// Load average over 15 minutes
    pub load_15m: f64,
}

/// Real-time performance metrics for a compute node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Timestamp when metrics were collected
    pub timestamp: DateTime<Utc>,
    /// CPU usage as a percentage (0.0-100.0)
    pub _cpu_usage: f64,
    /// Memory usage as a percentage (0.0-100.0)
    pub memory_usage: f64,
    /// GPU usage as a percentage (0.0-100.0), if GPU is present
    pub gpu_usage: Option<f64>,
    /// Network I/O metrics
    pub network_io: NetworkIoMetrics,
    /// Disk I/O metrics
    pub disk_io: DiskIoMetrics,
    /// Storage I/O metrics
    pub storage_io: StorageIoMetrics,
    /// System load metrics
    pub system_load: SystemLoadMetrics,
}

/// Information about a compute node in the cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeNode {
    /// Operating system running on the node
    pub os: String,
    /// CPU architecture (e.g., x86_64, arm64)
    pub architecture: String,
    /// Number of CPU cores available
    pub cpu_cores: u32,
    /// Amount of memory in GB available
    pub memory_gb: u32,
    /// Number of GPUs available
    pub gpu_count: u32,
    /// Storage devices attached to this node
    pub storage_devices: Vec<StorageDevice>,
}

/// Compute discovery information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeDiscovery {
    /// All compute nodes in the cluster
    pub nodes: Vec<ComputeNode>,
    /// Total CPU cores across all nodes
    pub total_cpu_cores: u32,
    /// Total memory in GB across all nodes
    pub total_memory_gb: u32,
    /// Total number of GPUs across all nodes
    pub total_gpu_count: u32,
    /// Available CPU cores not currently allocated
    pub available_cpu_cores: u32,
    /// Available memory in GB not currently allocated
    pub available_memory_gb: u32,
    /// Available GPUs not currently allocated
    pub available_gpu_count: u32,
}

/// System health information
///
/// Comprehensive health status for all major system components,
/// including any active alerts or warnings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Overall system health status
    pub overall_status: UnifiedHealthStatus,
    /// CPU subsystem health status
    pub cpu_status: UnifiedHealthStatus,
    /// Memory subsystem health status
    pub memory_status: UnifiedHealthStatus,
    /// Storage subsystem health status
    pub storage_status: UnifiedHealthStatus,
    /// Network subsystem health status
    pub network_status: UnifiedHealthStatus,
    /// List of active system alerts
    pub alerts: Vec<SystemAlert>,
}

/// Hardware health status enumeration
///
/// **DEPRECATED**: Use `nestgate_core::unified_enums::UnifiedHealthStatus` instead.
/// This enum has been superseded by the unified health status system which provides
/// consistent health reporting across all NestGate components.
///
/// **MIGRATION GUIDE**:
/// ```rust
/// // OLD: Using hardware_tuning::HealthStatus
/// use crate::hardware_tuning::types::HealthStatus;
///
/// // NEW: Using unified health status
/// use crate::canonical_modernization::UnifiedHealthStatus;
/// ```
///
/// **MAPPING**:
/// - `Healthy` → `UnifiedHealthStatus::Healthy`
/// - `Warning` → `UnifiedHealthStatus::Degraded`
/// - `Critical` → `UnifiedHealthStatus::Unhealthy`
/// - `Unknown` → `UnifiedHealthStatus::Unknown`
// Deprecated HealthStatus removed - use crate::canonical_modernization::UnifiedHealthStatus
pub use crate::canonical_modernization::UnifiedHealthStatus as HealthStatus;

/// System alert information
///
/// Represents an alert or notification about system conditions
/// that require attention or monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAlert {
    /// Unique identifier for this alert
    pub alert_id: String,
    /// Severity level of the alert
    pub severity: AlertSeverity,
    /// Human-readable alert message
    pub message: String,
    /// Timestamp when the alert was generated
    pub timestamp: DateTime<Utc>,
    /// System component that generated the alert
    pub component: String,
}

/// Alert severity levels
///
/// Categorizes alerts by their importance and urgency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Informational message, no action required
    Info,
    /// Warning condition, monitoring recommended
    Warning,
    /// Error condition, action may be required
    Error,
    /// Critical condition, immediate action required
    Critical,
}

/// Storage workload information
///
/// Represents a storage-intensive workload that needs to be
/// scheduled and managed by the tuning system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageWorkload {
    /// Unique identifier for this workload
    pub workload_id: String,
    /// Type of workload (e.g., "backup", "sync", "optimization")
    pub workload_type: String,
    /// Priority level for resource allocation
    pub priority: ComputePriority,
    /// Resource requirements for this workload
    pub resource_requirements: WorkloadResourceRequirements,
    /// Estimated duration in minutes
    pub estimated_duration_minutes: u32,
    /// Current status of the workload
    pub status: WorkloadStatus,
}

/// Workload resource requirements
///
/// Specifies the computational and storage resources
/// needed to execute a workload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadResourceRequirements {
    /// Number of CPU cores required
    pub cpu_cores: u32,
    /// Amount of memory required in gigabytes
    pub memory_gb: u32,
    /// Amount of storage required in gigabytes
    pub storage_gb: u64,
    /// Network bandwidth required in gigabits per second
    pub network_bandwidth_gbps: f64,
}

/// Workload execution information
///
/// Tracks the execution of a workload including progress,
/// resource allocation, and timing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadExecution {
    /// Unique identifier for this execution instance
    pub execution_id: String,
    /// ID of the workload being executed
    pub workload_id: String,
    /// Timestamp when execution started
    pub start_time: DateTime<Utc>,
    /// Estimated completion time
    pub estimated_end_time: DateTime<Utc>,
    /// Current progress as a percentage (0.0 to 100.0)
    pub progress_percent: f64,
    /// Resources allocated to this execution
    pub allocated_resources: AllocatedResources,
    /// Current execution status
    pub status: WorkloadStatus,
}

/// Allocated resources information
///
/// Represents the actual resources allocated to a workload
/// execution, including which compute nodes are being used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocatedResources {
    /// Number of CPU cores allocated
    pub cpu_cores: u32,
    /// Amount of memory allocated in gigabytes
    pub memory_gb: u32,
    /// Amount of storage allocated in gigabytes
    pub storage_gb: u64,
    /// Network bandwidth allocated in gigabits per second
    pub network_bandwidth_gbps: f64,
    /// List of compute nodes where resources are allocated
    pub compute_nodes: Vec<String>,
}

/// Workload status
///
/// Represents the current state of a workload in the execution pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkloadStatus {
    /// Workload is waiting to be scheduled
    Pending,
    /// Workload is currently executing
    Running,
    /// Workload has completed successfully
    Completed,
    /// Workload execution failed
    Failed,
    /// Workload was cancelled before completion
    Cancelled,
}

/// Storage resource request
///
/// Request for specific storage resources to support a workload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceRequest {
    /// Unique identifier for this resource request
    pub request_id: String,
    /// ID of the workload requesting resources
    pub workload_id: String,
    /// Type of resource being requested (e.g., "disk", "bandwidth")
    pub resource_type: String,
    /// Quantity of the resource being requested
    pub quantity: u64,
    /// Duration for which the resource is requested in minutes
    pub duration_minutes: u32,
}

/// Storage resource allocation
///
/// Represents the actual allocation of storage resources
/// in response to a resource request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceAllocation {
    /// Unique identifier for this allocation
    pub allocation_id: String,
    /// ID of the original resource request
    pub request_id: String,
    /// Actual quantity of resources allocated
    pub allocated_quantity: u64,
    /// List of nodes where resources are allocated
    pub allocated_nodes: Vec<String>,
    /// Timestamp when this allocation expires
    pub expires_at: DateTime<Utc>,
}

/// Storage process request
///
/// Request to execute a specific storage-related process
/// with configurable parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProcessRequest {
    /// Name of the process to execute
    pub process_name: String,
    /// Type/category of the process
    pub process_type: String,
    /// Process-specific configuration parameters
    pub parameters: serde_json::Value,
}

/// Process management information
///
/// Information about a running or completed storage process,
/// including status and resource usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessManagement {
    /// Unique identifier for this process instance
    pub process_id: String,
    /// Human-readable name of the process
    pub process_name: String,
    /// Current status of the process
    pub status: String,
    /// Timestamp when the process started
    pub start_time: DateTime<Utc>,
    /// Current resource usage by this process
    pub resource_usage: ResourceUsage,
}

/// Resource usage information
///
/// Current resource consumption metrics for a process or workload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// CPU usage as a percentage (0.0 to 100.0)
    pub cpu_percent: f64,
    /// Memory usage in megabytes
    pub memory_mb: u64,
    /// Disk read activity in megabytes
    pub disk_read_mb: u64,
    /// Disk write activity in megabytes
    pub disk_write_mb: u64,
}

/// Storage optimization request
///
/// Request to perform optimization on storage systems
/// to improve performance or efficiency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOptimizationRequest {
    /// Type of optimization to perform
    pub optimization_type: String,
    /// List of storage pools to optimize
    pub target_storage_pools: Vec<String>,
    /// Intensity level of optimization
    pub optimization_level: String,
    /// Optimization-specific parameters
    pub parameters: serde_json::Value,
}

/// Storage optimization result
///
/// Results and metrics from a completed storage optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageOptimization {
    /// Unique identifier for this optimization
    pub optimization_id: String,
    /// Type of optimization that was performed
    pub optimization_type: String,
    /// Performance improvement as a percentage
    pub performance_improvement: f64,
    /// Resources saved by the optimization
    pub resource_savings: ResourceSavings,
    /// Timestamp when optimization completed
    pub completion_time: DateTime<Utc>,
}

/// Resource savings information
///
/// Quantifies the resource savings achieved by an optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSavings {
    /// CPU usage reduction as a percentage
    pub cpu_percent_saved: f64,
    /// Memory usage reduction in megabytes
    pub memory_mb_saved: u64,
    /// Storage space saved in gigabytes
    pub storage_gb_saved: u64,
    /// Network bandwidth saved in gigabits per second
    pub network_bandwidth_saved_gbps: f64,
}

/// Tuning mode enum
///
/// Different modes for hardware tuning behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TuningMode {
    /// Automatic tuning based on system analysis
    Auto,
    /// Manual tuning with user control
    Manual,
    /// Aggressive tuning for maximum performance
    Aggressive,
    /// Conservative tuning prioritizing stability
    Conservative,
    /// Balanced tuning between performance and stability
    Balanced,
}

/// Session status enum
///
/// Current state of a tuning session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    /// Session is currently active
    Active,
    /// Session is inactive but can be resumed
    Inactive,
    /// Session is temporarily suspended
    Suspended,
    /// Session has expired and cannot be resumed
    Expired,
    /// Session encountered an error
    Error,
    /// Session completed successfully
    Completed,
}

/// Tuning session information
///
/// Information about an active or completed hardware tuning session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningSession {
    /// Unique identifier for this tuning session
    pub session_id: Uuid,
    /// ID of the user who started this session
    pub user_id: String,
    /// Timestamp when the session was started
    pub start_time: DateTime<Utc>,
    /// Timestamp of the last activity in this session
    pub last_activity: DateTime<Utc>,
    /// Current status of the session
    pub status: SessionStatus,
    /// Tuning mode being used in this session
    pub tuning_mode: TuningMode,
    /// List of active tuning profiles
    pub active_profiles: Vec<String>,
}

/// Hardware tuning request
///
/// Request to start or modify a hardware tuning operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareTuningRequest {
    /// Session ID for this tuning request
    pub session_id: Uuid,
    /// Tuning mode to use for this request
    pub tuning_mode: TuningMode,
    /// List of hardware components to target
    pub target_hardware: Vec<String>,
}

/// Hardware tuning response
///
/// Response containing the results of a hardware tuning operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareTuningResponse {
    /// Session ID for this tuning response
    pub session_id: Uuid,
    /// Timestamp when the response was generated
    pub timestamp: DateTime<Utc>,
    /// Current status of the tuning operation
    pub status: SessionStatus,
    /// Hardware configuration data if available
    pub hardware_config: Option<serde_json::Value>,
    /// Tuning result if operation completed
    pub result: Option<TuningResult>,
    /// Performance improvement achieved as a percentage
    pub performance_improvement: Option<f64>,
    /// Status of external access requirements
    pub external_access_status: ExternalAccessStatus,
    /// Tuning recommendations for the user
    pub recommendations: TuningRecommendations,
    /// Any warnings or issues encountered
    pub warnings: Vec<String>,
}

/// Tuning query parameters
///
/// Parameters for querying tuning session information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningQuery {
    /// Optional session ID to filter results
    pub session_id: Option<Uuid>,
    /// Whether to include performance metrics in results
    pub include_metrics: Option<bool>,
    /// Whether to include tuning recommendations in results
    pub include_recommendations: Option<bool>,
}

/// External access requirements
///
/// Requirements for external access to hardware tuning systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAccessRequirements {
    /// Type of access being requested
    pub access_type: String,
    /// List of permissions required for this access
    pub required_permissions: Vec<String>,
    /// Duration of access needed in hours
    pub duration_hours: u32,
    /// Justification for why this access is needed
    pub justification: String,
    /// ID of the user requesting access
    pub requester_id: String,
}

/// Benchmark result
///
/// Results from running a performance benchmark.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Unique identifier for this benchmark
    pub benchmark_id: String,
    /// Human-readable name of the benchmark
    pub benchmark_name: String,
    /// Benchmark score (higher is typically better)
    pub score: f64,
    /// Time taken to complete the benchmark in milliseconds
    pub duration_ms: u64,
    /// Additional benchmark-specific metadata
    pub metadata: serde_json::Value,
}

/// External access status
///
/// Status information about external access permissions
/// for hardware tuning operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAccessStatus {
    /// Whether external access has been granted
    pub access_granted: bool,
    /// Level of access granted (e.g., "read-only", "full")
    pub access_level: String,
    /// List of restrictions on the granted access
    pub restrictions: Vec<String>,
    /// Optional expiration time for the access
    pub expires_at: Option<DateTime<Utc>>,
}

/// Tuning recommendations
///
/// Hardware-specific recommendations for improving system performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningRecommendations {
    /// CPU-related performance recommendations
    pub cpu_recommendations: Vec<String>,
    /// Memory-related performance recommendations
    pub memory_recommendations: Vec<String>,
    /// Storage-related performance recommendations
    pub storage_recommendations: Vec<String>,
    /// Network-related performance recommendations
    pub network_recommendations: Vec<String>,
}

/// Live performance metrics
///
/// Real-time performance metrics collected from the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivePerformanceMetrics {
    /// Timestamp when these metrics were collected
    pub timestamp: DateTime<Utc>,
    /// Current CPU usage percentage (0.0 to 100.0)
    pub _cpu_usage: f64,
    /// Current memory usage percentage (0.0 to 100.0)
    pub memory_usage: f64,
    /// Current GPU usage percentage if GPU is available
    pub gpu_usage: Option<f64>,
    /// Current disk I/O metrics
    pub disk_io: DiskIoMetrics,
    /// Current network I/O metrics
    pub network_io: NetworkIoMetrics,
    /// Current system load metrics
    pub system_load: SystemLoadMetrics,
    /// Current system temperature in Celsius
    pub temperature: f64,
    /// Current power consumption in watts
    pub power_consumption: f64,
}

// Re-export from nestgate_core for convenience
// Removed import - hardware_tuning module moved or deprecated
