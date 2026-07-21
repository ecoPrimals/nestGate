// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use chrono::{DateTime, Utc};

use nestgate_core::Result;

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

/// **DISK MONITOR**
///
/// Best-effort root filesystem usage via `df` (see `DiskMonitor::usage_percent`).
#[derive(Debug, Clone)]
pub struct DiskMonitor;

/// **NETWORK MONITOR**
///
/// Best-effort aggregate activity from `/proc/net/dev` (see `NetworkMonitor::usage_percent`).
#[derive(Debug, Clone)]
pub struct NetworkMonitor;

impl CpuMonitor {
    /// Current aggregate CPU utilisation via `linux_proc` (best-effort, single sample).
    pub(crate) fn usage_percent(&self) -> Result<f64> {
        let _: &Self = self;
        Ok(nestgate_platform::linux_proc::globalcpu_usage_percent_from_stat().unwrap_or(0.0))
    }
}

impl MemoryMonitor {
    /// Resident memory pressure as a percentage of total RAM via `linux_proc`.
    pub(crate) fn usage_percent(&self) -> Result<f64> {
        let _: &Self = self;
        Ok(nestgate_platform::linux_proc::memory_usage_percent().unwrap_or(0.0))
    }
}

impl GpuMonitor {
    /// GPU utilisation via `nvidia-smi` when available.
    pub(crate) fn usage_percent(&self) -> Result<f64> {
        let _: &Self = self;
        if let Ok(output) = std::process::Command::new("nvidia-smi")
            .args([
                "--query-gpu=utilization.gpu",
                "--format=csv,noheader,nounits",
            ])
            .output()
            && output.status.success()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(usage) = stdout.trim().parse::<f64>() {
                return Ok(usage);
            }
        }
        Ok(0.0)
    }
}

impl DiskMonitor {
    /// Root mount utilisation percentage from `df` (best-effort).
    pub(crate) fn usage_percent(&self) -> Result<f64> {
        let _: &Self = self;
        if std::fs::metadata("/").is_err() {
            return Ok(0.0);
        }
        match std::process::Command::new("df")
            .args(["/", "--output=pcent"])
            .output()
        {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(line) = stdout.lines().nth(1) {
                    let percent_str = line.trim().trim_end_matches('%');
                    if let Ok(usage) = percent_str.parse::<f64>() {
                        return Ok(usage);
                    }
                }
                Ok(0.0)
            }
            _ => Ok(0.0),
        }
    }
}

impl NetworkMonitor {
    /// Heuristic activity level from `linux_proc` (not a true link utilisation %).
    pub(crate) fn usage_percent(&self) -> Result<f64> {
        let _: &Self = self;
        let (rx, tx) = nestgate_platform::linux_proc::network_rx_tx_bytes_sum().unwrap_or((0, 0));
        let total_bytes = rx.saturating_add(tx);
        Ok(if total_bytes > 0 { 10.0 } else { 0.0 })
    }
}

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
