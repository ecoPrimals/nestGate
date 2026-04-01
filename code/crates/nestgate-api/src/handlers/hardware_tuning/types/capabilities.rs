// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// **SYSTEM CAPABILITIES**
///
/// Hardware capabilities and specifications of the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Systemcapabilities
pub struct SystemCapabilities {
    /// Number of CPU cores available
    pub cpu_cores: usize,
    /// CPU model identifier
    pub cpu_model: String,
    /// Total system memory in gigabytes
    pub memory_gb: u64,
    /// Whether GPU acceleration is available
    pub gpu_available: bool,
    /// GPU information if available
    pub gpu_info: Option<GpuInfo>,
}

/// **CPU INFORMATION**
///
/// Detailed CPU specifications and capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cpuinfo
pub struct CpuInfo {
    /// Number of CPU cores
    pub cores: usize,
    /// CPU model name and identifier
    pub model: String,
}

/// **MEMORY INFORMATION**
///
/// System memory specifications and availability.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Memoryinfo
pub struct MemoryInfo {
    /// Total system memory in gigabytes
    pub total_gb: u64,
}

/// **GPU INFORMATION**
///
/// Graphics processing unit specifications and capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Gpuinfo
pub struct GpuInfo {
    /// GPU device name
    pub name: String,
    /// GPU memory in megabytes
    pub memory_mb: u64,
}
