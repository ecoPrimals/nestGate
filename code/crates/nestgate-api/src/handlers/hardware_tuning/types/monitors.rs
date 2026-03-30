// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use chrono::{DateTime, Utc};

use super::allocation::ComputeAllocation;
use super::results::LiveHardwareMetrics;

/// **CPU MONITOR**
///
/// Hardware monitor for CPU performance and utilization.
#[derive(Debug, Clone)]
/// Cpumonitor
pub struct CpuMonitor;

/// **MEMORY MONITOR**
///
/// Hardware monitor for memory usage and availability.
#[derive(Debug, Clone)]
/// Memorymonitor
pub struct MemoryMonitor;

/// **GPU MONITOR**
///
/// Hardware monitor for GPU utilization and performance.
#[derive(Debug, Clone)]
/// Gpumonitor
pub struct GpuMonitor;

/// **TUNING SESSION**
///
/// Active tuning session
#[derive(Debug, Clone)]
/// Tuningsession
pub struct TuningSession {
    /// Unique identifier for the tuning session
    pub session_id: String,
    /// Timestamp when the session was started
    pub started_at: DateTime<Utc>,
    /// Resource allocation configuration for this session
    pub resource_allocation: ComputeAllocation,
    /// Current hardware metrics being monitored
    pub current_metrics: LiveHardwareMetrics,
}

/// **HARDWARE MONITORS**
///
/// Collection of hardware monitoring services.
#[derive(Debug, Clone)]
/// Hardwaremonitors
pub struct HardwareMonitors {
    /// CPU performance monitor
    pub cpu: CpuMonitor,
    /// Memory utilization monitor
    pub memory: MemoryMonitor,
    /// GPU performance monitor (if available)
    pub gpu: Option<GpuMonitor>,
}
