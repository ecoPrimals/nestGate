//! Linux `/proc`-based hardware resource discovery (no `sysinfo`).
//!
//! On non-Linux hosts, values are best-effort from `std::thread::available_parallelism`
//! where applicable; memory may be unavailable without `/proc/meminfo`.

use nestgate_core::{NestGateError, Result};

use super::types::ComputeResources;

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
pub fn gpu_count_best_effort() -> u32 {
    #[cfg(target_os = "linux")]
    {
        if std::path::Path::new("/proc/driver/nvidia/gpus").exists() {
            if let Ok(entries) = std::fs::read_dir("/proc/driver/nvidia/gpus") {
                return u32::try_from(entries.count()).unwrap_or(0);
            }
        }
    }
    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .args(["-L"])
        .output()
    {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            return u32::try_from(s.lines().filter(|l| !l.trim().is_empty()).count()).unwrap_or(0);
        }
    }
    0
}

/// Build [`ComputeResources`] from `/proc` (and optional GPU tools).
pub fn compute_resources_from_proc() -> Result<ComputeResources> {
    let available_cpu = logical_cpu_count()?.max(1);
    let available_memory_gb = {
        let g = mem_total_gib()?;
        if g == 0 {
            1
        } else {
            g
        }
    };
    let available_gpu = gpu_count_best_effort();
    Ok(ComputeResources {
        available_cpu,
        available_memory_gb,
        available_gpu,
    })
}
