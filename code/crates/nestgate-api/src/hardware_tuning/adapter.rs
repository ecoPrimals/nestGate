//! Hardware Tuning Adapter
//!
//! This module provides the adapter-based implementation for hardware tuning operations,
//! replacing the hardcoded ToadstoolComputeClient with the universal adapter pattern.

use super::types::{
    ComputeAllocation, ComputeResourceRequest, LiveHardwareMetrics, TuningServiceRegistration,
};
use nestgate_core::ecosystem_integration::universal_adapter::{
    CapabilityRequest, CapabilityResponse,
};
use nestgate_core::ecosystem_integration::{
    ComputeCapability, HardwareOptimizationRequest, HardwareOptimizationResponse,
    MockComputeCapability, PerformanceTuningRequest, PerformanceTuningResponse,
    ResourceAllocationRequest, ResourceAllocationResponse, UniversalAdapter,
};
use nestgate_core::NestGateError;
use std::sync::Arc;
use tracing::{error, info, warn};

type Result<T> = std::result::Result<T, NestGateError>;

/// Hardware tuning adapter using universal adapter pattern
#[derive(Debug, Clone)]
pub struct HardwareTuningAdapter {
    /// Universal adapter for external primal communication
    adapter: Arc<UniversalAdapter>,
    /// Service name for registration
    service_name: String,
}

impl HardwareTuningAdapter {
    /// Create new hardware tuning adapter
    pub fn new(adapter: Arc<UniversalAdapter>, service_name: String) -> Self {
        info!("🔧 Creating Hardware Tuning Adapter via Universal Adapter");
        info!("🔧 Service: {}", service_name);

        Self {
            adapter,
            service_name,
        }
    }

    /// Create adapter with mock capabilities for testing
    pub fn new_with_mock() -> Result<Self> {
        // For now, return an error since we need the actual adapter infrastructure
        // This will be implemented when the universal adapter is fully available
        Err(NestGateError::Internal {
            message: "Mock adapter not yet implemented - use real adapter".to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            debug_info: None,
            is_bug: false,
        })
    }

    /// Register hardware tuning service with external compute primal
    pub async fn register_tuning_service(&self, service: &TuningServiceRegistration) -> Result<()> {
        info!(
            "🔧 Registering hardware tuning service via adapter: {}",
            service.name
        );

        // Convert service registration to capability request
        let payload = serde_json::to_vec(service).map_err(|e| NestGateError::Internal {
            message: format!("Failed to serialize service registration: {}", e),
            location: Some(format!("{}:{}", file!(), line!())),
            debug_info: None,
            is_bug: false,
        })?;

        let request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "compute.service_registration".to_string(),
            payload,
            metadata: std::collections::HashMap::new(),
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(30)),
            priority: 5,
            requires_encryption: false,
        };

        // Execute via universal adapter
        match self.adapter.execute_capability(request).await {
            Ok(response) => {
                if response.success {
                    info!(
                        "✅ Hardware tuning service registered via adapter: {}",
                        service.name
                    );
                    Ok(())
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{:?}", e))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    error!("❌ Failed to register via adapter: {}", error_msg);
                    Err(NestGateError::Internal {
                        message: format!("Adapter registration failed: {}", error_msg),
                        location: Some(format!("{}:{}", file!(), line!())),
                        debug_info: None,
                        is_bug: false,
                    })
                }
            }
            Err(e) => {
                error!("❌ Adapter communication failed: {}", e);
                Err(NestGateError::Internal {
                    message: format!("Adapter communication failed: {}", e),
                    location: Some(format!("{}:{}", file!(), line!())),
                    debug_info: None,
                    is_bug: false,
                })
            }
        }
    }

    /// Request compute resources via adapter
    pub async fn request_compute_resources(
        &self,
        request: &ComputeResourceRequest,
    ) -> Result<ComputeAllocation> {
        info!("🔧 Requesting compute resources via adapter");

        // Convert to adapter request format
        let adapter_request = ResourceAllocationRequest {
            resource_type: "compute".to_string(),
            requested_amount: request.cpu_cores as u64,
            priority: 5, // Medium priority by default
            duration_seconds: Some(request.duration_minutes.map(|m| m * 60).unwrap_or(3600) as u64),
        };

        let payload =
            serde_json::to_vec(&adapter_request).map_err(|e| NestGateError::Internal {
                message: format!("Failed to serialize resource request: {}", e),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: None,
                is_bug: false,
            })?;

        let capability_request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "compute.resource_allocation".to_string(),
            payload,
            metadata: std::collections::HashMap::new(),
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(30)),
            priority: 5,
            requires_encryption: false,
        };

        // Execute via universal adapter
        match self.adapter.execute_capability(capability_request).await {
            Ok(response) => {
                if response.success {
                    // Convert response back to our format
                    let adapter_response: ResourceAllocationResponse =
                        serde_json::from_slice(&response.payload).map_err(|e| {
                            NestGateError::Internal {
                                message: format!(
                                    "Failed to deserialize allocation response: {}",
                                    e
                                ),
                                location: Some(format!("{}:{}", file!(), line!())),
                                debug_info: None,
                                is_bug: false,
                            }
                        })?;

                    let allocation = ComputeAllocation {
                        allocation_id: adapter_response.allocation_id,
                        cpu_cores: adapter_response.actual_amount as u32,
                        memory_gb: request.memory_gb, // Use requested value for now
                        gpu_allocation: if request.gpu_required {
                            Some(crate::hardware_tuning::types::GpuAllocation {
                                gpu_count: 1,
                                gpu_type: "Tesla V100".to_string(),
                                memory_gb: 16,
                            })
                        } else {
                            None
                        },
                        expires_at: chrono::Utc::now() + chrono::Duration::hours(1), // Default 1 hour expiry
                        compute_node: "node-1".to_string(), // Default compute node
                    };

                    info!(
                        "✅ Compute resources allocated via adapter: {} cores",
                        allocation.cpu_cores
                    );
                    Ok(allocation)
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{:?}", e))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    error!("❌ Failed to allocate compute resources: {}", error_msg);
                    Err(NestGateError::Internal {
                        message: format!("Adapter allocation failed: {}", error_msg),
                        location: Some(format!("{}:{}", file!(), line!())),
                        debug_info: None,
                        is_bug: false,
                    })
                }
            }
            Err(e) => {
                error!("❌ Adapter communication failed: {}", e);
                Err(NestGateError::Internal {
                    message: format!("Adapter communication failed: {}", e),
                    location: Some(format!("{}:{}", file!(), line!())),
                    debug_info: None,
                    is_bug: false,
                })
            }
        }
    }

    /// Get live hardware metrics via adapter
    pub async fn get_live_hardware_metrics(&self) -> Result<LiveHardwareMetrics> {
        info!("🔧 Getting live hardware metrics via adapter");

        let metrics_request = serde_json::json!({
            "metric_types": ["cpu", "memory", "disk", "network"],
            "format": "live"
        });

        let payload =
            serde_json::to_vec(&metrics_request).map_err(|e| NestGateError::Internal {
                message: format!("Failed to serialize metrics request: {}", e),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: None,
                is_bug: false,
            })?;

        let request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "compute.hardware_metrics".to_string(),
            payload,
            metadata: std::collections::HashMap::new(),
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(15)),
            priority: 7, // Higher priority for live data
            requires_encryption: false,
        };

        match self.adapter.execute_capability(request).await {
            Ok(response) => {
                if response.success {
                    // Deserialize the response payload
                    let metrics_data: serde_json::Value = serde_json::from_slice(&response.payload)
                        .map_err(|e| NestGateError::Internal {
                            message: format!("Failed to deserialize metrics response: {}", e),
                            location: Some(format!("{}:{}", file!(), line!())),
                            debug_info: None,
                            is_bug: false,
                        })?;

                    // Convert adapter response to our metrics format
                    let metrics = LiveHardwareMetrics {
                        timestamp: chrono::Utc::now(),
                        _cpu_usage: metrics_data
                            .get("cpu_usage")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0),
                        memory_usage: metrics_data
                            .get("memory_usage")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0),
                        gpu_usage: metrics_data.get("gpu_usage").and_then(|v| v.as_f64()),
                        temperature: metrics_data
                            .get("temperature")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0),
                        power_consumption: metrics_data
                            .get("power_consumption")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0),
                        network_io: crate::hardware_tuning::types::NetworkIoMetrics {
                            bytes_sent: 0,
                            bytes_received: 0,
                            packets_sent: 0,
                            packets_received: 0,
                        },
                        disk_io: crate::hardware_tuning::types::DiskIoMetrics {
                            read_bytes: 0,
                            write_bytes: 0,
                            read_ops: 0,
                            write_ops: 0,
                        },
                    };

                    Ok(metrics)
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{:?}", e))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    Err(NestGateError::Internal {
                        message: format!("Failed to get live metrics: {}", error_msg),
                        location: Some(format!("{}:{}", file!(), line!())),
                        debug_info: None,
                        is_bug: false,
                    })
                }
            }
            Err(e) => {
                error!("❌ Adapter communication failed: {}", e);
                Err(NestGateError::Internal {
                    message: format!("Adapter communication failed: {}", e),
                    location: Some(format!("{}:{}", file!(), line!())),
                    debug_info: None,
                    is_bug: false,
                })
            }
        }
    }

    /// Optimize hardware performance via adapter
    pub async fn optimize_hardware_performance(
        &self,
        target_resource: &str,
        optimization_level: u8,
    ) -> Result<HardwareOptimizationResponse> {
        info!("🔧 Optimizing hardware performance via adapter");

        let optimization_request = HardwareOptimizationRequest {
            target_resource: target_resource.to_string(),
            optimization_level,
            constraints: vec![],        // No constraints by default
            timeout_seconds: Some(300), // 5 minute timeout
        };

        let payload =
            serde_json::to_vec(&optimization_request).map_err(|e| NestGateError::Internal {
                message: format!("Failed to serialize optimization request: {}", e),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: None,
                is_bug: false,
            })?;

        let capability_request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "compute.hardware_optimization".to_string(),
            payload,
            metadata: std::collections::HashMap::new(),
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(300)), // 5 minutes
            priority: 6,                                        // Higher priority for optimization
            requires_encryption: false,
        };

        match self.adapter.execute_capability(capability_request).await {
            Ok(response) => {
                if response.success {
                    let optimization_response: HardwareOptimizationResponse =
                        serde_json::from_slice(&response.payload).map_err(|e| {
                            NestGateError::Internal {
                                message: format!(
                                    "Failed to deserialize optimization response: {}",
                                    e
                                ),
                                location: Some(format!("{}:{}", file!(), line!())),
                                debug_info: None,
                                is_bug: false,
                            }
                        })?;

                    info!(
                        "✅ Hardware optimization completed: {}% performance gain",
                        optimization_response.performance_gain
                    );
                    Ok(optimization_response)
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{:?}", e))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    Err(NestGateError::Internal {
                        message: format!("Hardware optimization failed: {}", error_msg),
                        location: Some(format!("{}:{}", file!(), line!())),
                        debug_info: None,
                        is_bug: false,
                    })
                }
            }
            Err(e) => {
                error!("❌ Adapter communication failed: {}", e);
                Err(NestGateError::Internal {
                    message: format!("Adapter communication failed: {}", e),
                    location: Some(format!("{}:{}", file!(), line!())),
                    debug_info: None,
                    is_bug: false,
                })
            }
        }
    }
}
