// **COMPUTE TRAITS - CANONICAL MODERNIZED**
//! Compute trait definitions for universal providers
// Compute and resource management traits for universal primal integration.
// Native async traits without async_trait overhead for optimal performance.

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal compute primal provider trait
/// **CANONICAL MODERNIZATION**: Native async trait without `async_trait` overhead
pub trait ComputePrimalProvider: Send + Sync {
    /// Allocate compute resources
    fn allocate_resources(
        &self,
        spec: &ResourceSpec,
    ) -> impl std::future::Future<Output = Result<ResourceAllocation>> + Send;
    /// Deallocate compute resources
    fn deallocate_resources(
        &self,
        allocation_id: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Execute workload on allocated resources
    fn execute_workload(
        &self,
        workload: &WorkloadSpec,
    ) -> impl std::future::Future<Output = Result<WorkloadResult>> + Send;

    /// Monitor resource utilization
    fn get_resource_utilization(
        &self,
        allocation_id: &str,
    ) -> impl std::future::Future<Output = Result<ResourceUtilization>> + Send;

    /// Scale resources dynamically
    fn scale_resources(
        &self,
        allocation_id: &str,
        target: &ScalingTarget,
    ) -> impl std::future::Future<Output = Result<ResourceAllocation>> + Send;

    /// Get performance metrics
    fn get_performance_metrics(
        &self,
        allocation_id: &str,
    ) -> impl std::future::Future<Output = Result<PerformanceMetrics>> + Send;

    /// Optimize resource configuration
    fn optimize_configuration(
        &self,
        allocation_id: &str,
    ) -> impl std::future::Future<Output = Result<OptimizationRecommendation>> + Send;

    /// Get platform capabilities
    fn get_platform_capabilities(
        &self,
    ) -> impl std::future::Future<Output = Result<PlatformCapabilities>> + Send;
}

/// Resource specification for allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourcespec
pub struct ResourceSpec {
    /// Cpu Cores
    pub cpu_cores: u32,
    /// Memory in gigabytes
    pub memory_gb: u32,
    /// Storage in gigabytes
    pub storage_gb: Option<u32>,
    /// Count of gpu
    pub gpu_count: Option<u32>,
    /// Network Bandwidth Mbps
    pub network_bandwidth_mbps: Option<u32>,
    /// Requirements
    pub requirements: HashMap<String, String>,
}
/// Resource allocation result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourceallocation
pub struct ResourceAllocation {
    /// Allocation identifier
    pub allocation_id: String,
    /// Allocated Resources
    pub allocated_resources: ResourceSpec,
    /// Endpoint
    pub endpoint: String,
    /// Expires At
    pub expires_at: Option<std::time::SystemTime>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Workload specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workloadspec
pub struct WorkloadSpec {
    /// Name
    pub name: String,
    /// Image
    pub image: String,
    /// Command
    pub command: Vec<String>,
    /// Resource Requirements
    pub resource_requirements: ResourceSpec,
    /// Timeout Seconds
    pub timeout_seconds: Option<u64>,
}
/// Workload execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Workloadresult
pub struct WorkloadResult {
    /// Performance Metrics
    pub performance_metrics: PerformanceMetrics,
    /// Workload identifier
    pub workload_id: String,
    /// Exit Code
    pub exit_code: i32,
    /// Stdout
    pub stdout: String,
    /// Stderr
    pub stderr: String,
    /// Execution Time Ms
    pub execution_time_ms: u64,
    /// Resource Usage
    pub resource_usage: ResourceUtilization,
}
/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancemetrics
pub struct PerformanceMetrics {
    /// Cpu Utilization Percent
    pub cpu_utilization_percent: f64,
    /// Memory Utilization Percent
    pub memory_utilization_percent: f64,
    /// Network Io Mbps
    pub network_io_mbps: f64,
    /// Disk Io Mbps
    pub disk_io_mbps: f64,
    /// Latency Ms
    pub latency_ms: f64,
    /// Throughput Ops Per Sec
    pub throughput_ops_per_sec: f64,
}
/// Scaling target specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Scalingtarget
pub struct ScalingTarget {
    /// Target Cpu Cores
    pub target_cpu_cores: Option<u32>,
    /// Target Memory in gigabytes
    pub target_memory_gb: Option<u32>,
    /// Target Instances
    pub target_instances: Option<u32>,
    /// Scaling Policy
    pub scaling_policy: ScalingPolicy,
}
/// Scaling policy enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Scalingpolicy
pub enum ScalingPolicy {
    /// Manual
    Manual,
    /// Autoscale
    AutoScale {
        min_instances: u32,
        max_instances: u32,
    },
    /// Loadbased
    LoadBased { target_utilization: f64 },
}
/// Resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Resourceutilization
pub struct ResourceUtilization {
    /// Cpu Percent
    pub cpu_percent: f64,
    /// Memory Percent
    pub memory_percent: f64,
    /// Storage Percent
    pub storage_percent: Option<f64>,
    /// Network Percent
    pub network_percent: Option<f64>,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}
/// Platform capabilities information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Platformcapabilities
pub struct PlatformCapabilities {
    /// Max Cpu Cores
    pub max_cpu_cores: u32,
    /// Max Memory in gigabytes
    pub max_memory_gb: u32,
    /// Max Storage in gigabytes
    pub max_storage_gb: u32,
    /// Gpu Types
    pub gpu_types: Vec<String>,
    /// Supported Architectures
    pub supported_architectures: Vec<String>,
    /// Features
    pub features: Vec<String>,
}
/// Optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Optimizationrecommendation
pub struct OptimizationRecommendation {
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Estimated Improvement Percent
    pub estimated_improvement_percent: f64,
    /// Estimated Cost Savings Percent
    pub estimated_cost_savings_percent: Option<f64>,
    /// Implementation Complexity
    pub implementation_complexity: ComplexityLevel,
}
/// Implementation complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Complexitylevel
pub enum ComplexityLevel {
    /// Low
    Low,
    /// Medium
    Medium,
    /// High
    High,
    /// Critical
    Critical,
}
