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
pub struct ResourceSpec {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub storage_gb: Option<u32>,
    pub gpu_count: Option<u32>,
    pub network_bandwidth_mbps: Option<u32>,
    pub requirements: HashMap<String, String>,
}
/// Resource allocation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub allocation_id: String,
    pub allocated_resources: ResourceSpec,
    pub endpoint: String,
    pub expires_at: Option<std::time::SystemTime>,
    pub metadata: HashMap<String, String>,
}
/// Workload specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadSpec {
    pub name: String,
    pub image: String,
    pub command: Vec<String>,
    pub resource_requirements: ResourceSpec,
    pub timeout_seconds: Option<u64>,
}
/// Workload execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadResult {
    pub performance_metrics: PerformanceMetrics,
    pub workload_id: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub execution_time_ms: u64,
    pub resource_usage: ResourceUtilization,
}
/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_utilization_percent: f64,
    pub memory_utilization_percent: f64,
    pub network_io_mbps: f64,
    pub disk_io_mbps: f64,
    pub latency_ms: f64,
    pub throughput_ops_per_sec: f64,
}
/// Scaling target specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingTarget {
    pub target_cpu_cores: Option<u32>,
    pub target_memory_gb: Option<u32>,
    pub target_instances: Option<u32>,
    pub scaling_policy: ScalingPolicy,
}
/// Scaling policy enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingPolicy {
    Manual,
    AutoScale {
        min_instances: u32,
        max_instances: u32,
    },
    LoadBased {
        target_utilization: f64,
    },
}
/// Resource utilization metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub storage_percent: Option<f64>,
    pub network_percent: Option<f64>,
    pub timestamp: std::time::SystemTime,
}
/// Platform capabilities information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformCapabilities {
    pub max_cpu_cores: u32,
    pub max_memory_gb: u32,
    pub max_storage_gb: u32,
    pub gpu_types: Vec<String>,
    pub supported_architectures: Vec<String>,
    pub features: Vec<String>,
}
/// Optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub recommendations: Vec<String>,
    pub estimated_improvement_percent: f64,
    pub estimated_cost_savings_percent: Option<f64>,
    pub implementation_complexity: ComplexityLevel,
}
/// Implementation complexity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    Critical,
}
