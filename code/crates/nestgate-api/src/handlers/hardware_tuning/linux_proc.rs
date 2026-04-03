// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Linux `/proc`-based hardware resource discovery (no `sysinfo`).
//!
//! On non-Linux hosts, values are best-effort from `std::thread::available_parallelism`
//! where applicable; memory may be unavailable without `/proc/meminfo`.

use nestgate_core::{NestGateError, Result};

use super::types::{ComputeResources, GpuInfo, SystemCapabilities};

/// Read logical CPU count from `/proc/cpuinfo` (Linux), else [`std::thread::available_parallelism`].
pub fn logical_cpu_count() -> Result<u32> {
    #[cfg(target_os = "linux")]
    {
        let cpu_info = std::fs::read_to_string("/proc/cpuinfo").map_err(|e| {
            NestGateError::system(
                "cpu_detection",
                format!("Failed to read /proc/cpuinfo: {e}"),
            )
        })?;
        let n = cpu_info
            .lines()
            .filter(|l| l.starts_with("processor"))
            .count();
        Ok(u32::try_from(n.max(1)).unwrap_or(1))
    }
    #[cfg(not(target_os = "linux"))]
    {
        std::thread::available_parallelism()
            .map(|n| n.get() as u32)
            .map_err(|e| {
                NestGateError::system("cpu_detection", format!("available_parallelism: {e}"))
            })
    }
}

/// Total RAM in GiB from `/proc/meminfo` `MemTotal` (Linux). On other OSes returns `Ok(0)` (unknown).
pub fn mem_total_gib() -> Result<u32> {
    #[cfg(target_os = "linux")]
    {
        let meminfo = std::fs::read_to_string("/proc/meminfo").map_err(|e| {
            NestGateError::system(
                "memory_detection",
                format!("Failed to read /proc/meminfo: {e}"),
            )
        })?;
        let total_kb = meminfo
            .lines()
            .find(|l| l.starts_with("MemTotal:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let gib = (total_kb / 1024 / 1024).max(1);
        Ok(u32::try_from(gib).unwrap_or(u32::MAX))
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok(0)
    }
}

/// Best-effort GPU count: NVIDIA via `nvidia-smi -L` line count, else 0.
#[must_use]
pub fn gpu_count_best_effort() -> u32 {
    #[cfg(target_os = "linux")]
    {
        if std::path::Path::new("/proc/driver/nvidia/gpus").exists()
            && let Ok(entries) = std::fs::read_dir("/proc/driver/nvidia/gpus")
        {
            return u32::try_from(entries.count()).unwrap_or(0);
        }
    }
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(["-L"])
        .output()
        && output.status.success()
    {
        let s = String::from_utf8_lossy(&output.stdout);
        return u32::try_from(s.lines().filter(|l| !l.trim().is_empty()).count()).unwrap_or(0);
    }
    0
}

/// Build [`ComputeResources`] from `/proc` (and optional GPU tools).
pub fn compute_resources_from_proc() -> Result<ComputeResources> {
    let available_cpu = logical_cpu_count()?.max(1);
    let available_memory_gb = {
        let g = mem_total_gib()?;
        if g == 0 { 1 } else { g }
    };
    let available_gpu = gpu_count_best_effort();
    Ok(ComputeResources {
        available_cpu,
        available_memory_gb,
        available_gpu,
    })
}

/// Human-readable CPU model from `/proc/cpuinfo` (Linux), else `"unknown"`.
pub fn cpu_model_best_effort() -> Result<String> {
    #[cfg(target_os = "linux")]
    {
        let cpu_info = std::fs::read_to_string("/proc/cpuinfo").map_err(|e| {
            NestGateError::system(
                "cpu_detection",
                format!("Failed to read /proc/cpuinfo: {e}"),
            )
        })?;
        let model = cpu_info
            .lines()
            .find(|line| line.starts_with("model name"))
            .and_then(|line| line.split(':').nth(1))
            .map_or_else(|| "Unknown CPU".to_string(), |s| s.trim().to_string());
        Ok(model)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok("unknown".to_string())
    }
}

/// First NVIDIA GPU via synchronous `nvidia-smi` (best-effort).
#[must_use]
pub fn nvidia_gpu_info_best_effort() -> Option<GpuInfo> {
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,memory.total",
            "--format=csv,noheader,nounits",
        ])
        .output()
        && output.status.success()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = output_str.lines().next() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                return Some(GpuInfo {
                    name: parts[0].trim().to_string(),
                    memory_mb: parts[1].trim().parse().unwrap_or(0),
                });
            }
        }
    }
    None
}

/// CPU, memory, and optional GPU capabilities from `/proc` and `nvidia-smi`.
pub fn system_capabilities_from_proc() -> Result<SystemCapabilities> {
    let cpu_cores = usize::try_from(logical_cpu_count()?.max(1)).unwrap_or(1);
    let memory_gb = u64::from(mem_total_gib()?.max(1));
    let cpu_model = cpu_model_best_effort()?;
    let gpu_info = nvidia_gpu_info_best_effort();
    Ok(SystemCapabilities {
        cpu_cores,
        cpu_model,
        memory_gb,
        gpu_available: gpu_info.is_some(),
        gpu_info,
    })
}
