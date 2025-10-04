use crate::universal_adapter::{PrimalAgnosticAdapter, CapabilityCategory, CapabilityRequest};
/// Compute Capabilities (Compute Primal Integration)
///
/// Defines capability interfaces for hardware optimization, resource allocation,
/// and performance tuning through the Compute compute primal.
use super::{CapabilityRequest, CapabilityResponse, UniversalCapability};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Hardware optimization request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareOptimizationRequest {
    pub optimization_level: u8, // 1-10 scale
    pub constraints: Vec<String>,
    pub timeout_seconds: Option<u64>,
}
/// Hardware optimization response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareOptimizationResponse {
    pub optimization_applied: bool,
    pub performance_gain: f64, // Percentage improvement
    pub recommendations: Vec<String>,
    pub metrics: HashMap<String, f64>,
}
/// Resource allocation request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocationRequest {
    pub resource_type: String,
    pub requested_amount: u64,
    pub priority: u8,
    pub duration_seconds: Option<u64>,
}
/// Resource allocation response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocationResponse {
    pub allocated: bool,
    pub allocation_id: String,
    pub actual_amount: u64,
    pub expires_at: Option<String>,
}
/// Performance tuning request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTuningRequest {
    pub target_service: String,
    pub tuning_profile: String,
    pub custom_parameters: HashMap<String, serde_json::Value>,
}
/// Performance tuning response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTuningResponse {
    pub metrics: std::collections::HashMap<String, f64>,
    pub tuning_applied: bool,
    pub profile_used: String,
    pub warnings: Vec<String>,
}
/// Compute capability trait for Compute integration
/// **MODERNIZED**: Native async patterns for zero-cost compute operations
pub trait ComputeCapability: UniversalCapability {
    /// Optimize hardware resources for better performance - native async, no Future boxing
    fn optimize_hardware(
        &self,
        request: HardwareOptimizationRequest,
    ) -> impl std::future::Future<Output = Result<HardwareOptimizationResponse, Box<dyn std::error::Error + Send + Sync>>> + Send;
    /// Allocate compute resources - native async
    fn allocate_resources(
        &self,
        request: ResourceAllocationRequest,
    ) -> impl std::future::Future<Output = Result<ResourceAllocationResponse, Box<dyn std::error::Error + Send + Sync>>> + Send;

    /// Apply performance tuning - native async
    fn tune_performance(
        &self,
        request: PerformanceTuningRequest,
    ) -> impl std::future::Future<Output = Result<PerformanceTuningResponse, Box<dyn std::error::Error + Send + Sync>>> + Send;
}

/// Mock implementation for testing and development
pub struct MockComputeCapability {
    enabled: bool,
}
impl MockComputeCapability {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl Default for MockComputeCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl UniversalCapability for MockComputeCapability {
    async fn execute(
        &self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, Box<dyn std::error::Error + Send + Sync>> {
        if !self.enabled {
            return Ok(CapabilityResponse::error(
                "Mock compute capability is disabled",
            ));
        }

        match request.capability_id.as_str() {
            "compute.hardware_optimization" => {
                let response_data = serde_json::to_value(HardwareOptimizationResponse {
                    optimization_applied: true,
                    performance_gain: 15.0,
                    recommendations: vec!["Increase memory allocation".to_string()],
                    metrics: HashMap::from([
                        ("cpu_utilization".to_string(), 75.0),
                        ("memory_usage".to_string(), 60.0),
                    ]),
                })?;
                Ok(CapabilityResponse::success(response_data))
            }
            "compute.path_allocation" => {
                let response_data = serde_json::to_value(ResourceAllocationResponse {
                    allocated: true,
                    allocation_id: "mock-allocation-123".to_string(),
                    actual_amount: 1024,
                    expires_at: Some("2024-12-31T23:59:59Z".to_string()),
                })?;
                Ok(CapabilityResponse::success(response_data))
            }
            "compute.performance_tuning" => {
                let response_data = serde_json::to_value(PerformanceTuningResponse {
                    tuning_applied: true,
                    profile_used: "high_performance".to_string(),
                    metrics: HashMap::from([
                        ("throughput".to_string(), 1250.0),
                        ("latency_ms".to_string(), 12.5),
                    ]),
                    warnings: vec![],
                })?;
                Ok(CapabilityResponse::success(response_data))
            }
            _ => Ok(CapabilityResponse::error(format!(
                "Unknown capability: {}",
                request.capability_id
            ))),
        }
    }

    fn get_metadata(&self) -> HashMap<String, serde_json::Value> {
        HashMap::from([
            (
                "name".to_string(),
                serde_json::Value::String("Mock Compute Capability".to_string()),
            ),
            (
                "version".to_string(),
                serde_json::Value::String("1.0.0".to_string()),
            ),
            (
                "capabilities".to_string(),
                serde_json::json!([
                    "compute.hardware_optimization",
                    "compute.path_allocation",
                    "compute.performance_tuning"
                ]),
            ),
        ])
    }

    async fn health_check(&self) -> bool {
        self.enabled
    }
}

impl ComputeCapability for MockComputeCapability {
    async fn optimize_hardware(
        &self,
        _request: HardwareOptimizationRequest,
    ) -> Result<HardwareOptimizationResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(HardwareOptimizationResponse {
            optimization_applied: true,
            performance_gain: 15.0,
            recommendations: vec!["Mock optimization applied".to_string()],
            metrics: HashMap::new(),
        })
    }

    async fn allocate_resources(
        &self,
        _request: ResourceAllocationRequest,
    ) -> Result<ResourceAllocationResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(ResourceAllocationResponse {
            allocated: true,
            allocation_id: "mock-123".to_string(),
            actual_amount: 1024,
            expires_at: None,
        })
    }

    async fn tune_performance(
        &self,
        _request: PerformanceTuningRequest,
    ) -> Result<PerformanceTuningResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(PerformanceTuningResponse {
                metrics: std::collections::HashMap::new(),
            tuning_applied: true,
            profile_used: "mock_profile".to_string(),
            warnings: vec![],
        })
    }
}
