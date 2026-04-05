// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// **COMPUTE ALLOCATION**
///
/// Resource allocation specification for compute workloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Computeallocation
pub struct ComputeAllocation {
    /// Number of CPU cores allocated
    pub cpu_cores: u32,
    /// Amount of memory in gigabytes allocated
    pub memory_gb: u32,
    /// Number of GPU units allocated
    pub gpu_count: u32,
}

/// **COMPUTE RESOURCES**
///
/// Available compute resources in the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Computeresources
pub struct ComputeResources {
    /// Available CPU cores
    pub available_cpu: u32,
    /// Available memory in gigabytes
    pub available_memory_gb: u32,
    /// Available GPU units
    pub available_gpu: u32,
}

/// **COMPUTE RESOURCE REQUEST**
///
/// Request for compute resource allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for `ComputeResource` operation
pub struct ComputeResourceRequest {
    /// Number of CPU cores requested
    pub cpu_cores: u32,
    /// Amount of memory in gigabytes requested
    pub memory_gb: u32,
    /// Number of GPU units requested
    pub gpu_count: u32,
}

/// **AVAILABLE RESOURCES**
///
/// Currently available system resources for allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Availableresources
pub struct AvailableResources {
    /// Available CPU cores
    pub available_cpu: u32,
    /// Available memory in gigabytes
    pub available_memory_gb: u32,
    /// Available GPU units
    pub available_gpu: u32,
}

/// **GPU ALLOCATION**
///
/// GPU resource allocation specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Gpuallocation
pub struct GpuAllocation {
    /// GPU device identifier
    pub gpu_id: String,
    /// GPU memory allocation in gigabytes
    pub memory_gb: u32,
}

/// **TUNING SERVICE REGISTRATION**
///
/// Registration information for hardware tuning services.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tuningserviceregistration
pub struct TuningServiceRegistration {
    /// Name of the tuning service
    pub service_name: String,
    /// Service endpoint URL
    pub endpoint: String,
}

/// **COMPUTE ADAPTER**
///
/// Adapter for interfacing with compute services.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Computeadapter
pub struct ComputeAdapter {
    /// Name of the associated service
    pub service_name: String,
}

impl ComputeAdapter {
    /// Create a new compute adapter for the specified service
    #[must_use]
    pub const fn new(service_name: String) -> Self {
        Self { service_name }
    }
}
