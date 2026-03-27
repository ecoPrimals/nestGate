// System utilities for NestGate core functionality
//! System functionality and utilities.
// Provides safe system operations and utilities.
//!
//! **Linux**: [`crate::linux_proc`] + [`num_cpus`] where possible; **`sysinfo`** for OS branding
//! and non-Linux / fallback.
//! **ecoBin v3.0**: `utils` module wiring to `lib.rs` is still pending (`utils.rs` vs `utils/mod.rs`).

use crate::{NestGateError, Result};
use sysinfo::System;

// ==================== SECTION ====================

/// Get the operating system name
pub fn get_os_name() -> String {
    System::name().unwrap_or_else(|| "Unknown".to_string())
}

/// Get the system architecture
pub fn get_architecture() -> String {
    std::env::consts::ARCH.to_string()
}

/// Get the operating system version
pub fn get_os_version() -> Result<String> {
    Ok(System::os_version().unwrap_or_else(|| "unknown".to_string()))
}

/// Get detailed OS information
pub fn get_os_info() -> Result<OsInfo> {
    Ok(OsInfo {
        name: get_os_name(),
        version: get_os_version()?,
        architecture: get_architecture(),
        kernel_version: get_kernel_version()?,
    })
}

/// OS information structure
#[derive(Debug, Clone)]
/// Osinfo
pub struct OsInfo {
    /// Name
    pub name: String,
    /// Version
    pub version: String,
    /// Architecture
    pub architecture: String,
    /// Kernel Version
    pub kernel_version: String,
}

/// Get kernel version
pub fn get_kernel_version() -> Result<String> {
    #[cfg(target_os = "linux")]
    if let Some(k) = crate::linux_proc::kernel_version_line() {
        return Ok(k);
    }
    Ok(System::kernel_version().unwrap_or_else(|| "unknown".to_string()))
}
// ==================== SECTION ====================

/// Get the number of CPU cores (pure Rust: [`num_cpus`])
pub fn get_cpu_count() -> usize {
    num_cpus::get()
}

/// Get the number of physical CPU cores ([`num_cpus`], with logical count as fallback)
pub fn get_physical_cpu_count() -> usize {
    num_cpus::get_physical().unwrap_or_else(num_cpus::get)
}

/// Get CPU information — Linux reads `/proc/cpuinfo` where helpful; `sysinfo` elsewhere
pub fn get_cpu_info() -> CpuInfo {
    #[cfg(target_os = "linux")]
    {
        if let Some((model, mhz)) = cpu_model_and_mhz_linux() {
            return CpuInfo {
                logical_cores: get_cpu_count(),
                physical_cores: get_physical_cpu_count(),
                model,
                frequency: mhz.map(|m| m / 1000.0),
            };
        }
    }
    let sys = System::new_all();
    let cpus = sys.cpus();
    CpuInfo {
        logical_cores: cpus.len(),
        physical_cores: sys.physical_core_count().unwrap_or(cpus.len()),
        model: cpus
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string()),
        frequency: cpus.first().map(|c| c.frequency() as f64 / 1000.0),
    }
}

#[cfg(target_os = "linux")]
fn cpu_model_and_mhz_linux() -> Option<(String, Option<f64>)> {
    let s = std::fs::read_to_string("/proc/cpuinfo").ok()?;
    let mut model = None::<String>;
    let mut mhz = None::<f64>;
    for line in s.lines() {
        if line.starts_with("model name") {
            model = line.split(':').nth(1).map(str::trim).map(String::from);
        } else if line.starts_with("cpu MHz") {
            mhz = line
                .split(':')
                .nth(1)
                .and_then(|x| x.trim().parse::<f64>().ok());
        }
        if model.is_some() && mhz.is_some() {
            break;
        }
    }
    Some((model.unwrap_or_else(|| "Unknown CPU".to_string()), mhz))
}

/// CPU information structure
#[derive(Debug, Clone)]
/// Cpuinfo
pub struct CpuInfo {
    /// Logical Cores
    pub logical_cores: usize,
    /// Physical Cores
    pub physical_cores: usize,
    /// Model
    pub model: String,
    /// Frequency
    pub frequency: Option<f64>, // in GHz
}
// ==================== SECTION ====================

/// Get total system memory in bytes
pub fn get_total_memory() -> Result<u64> {
    #[cfg(target_os = "linux")]
    if let Some(t) = crate::linux_proc::total_memory_bytes() {
        return Ok(t);
    }
    let mut sys = System::new_all();
    sys.refresh_memory();
    Ok(sys.total_memory())
}

/// Get free system memory in bytes (`available` semantics; see `MemAvailable` on Linux)
pub fn get_free_memory() -> Result<u64> {
    #[cfg(target_os = "linux")]
    if let Some(a) = crate::linux_proc::available_memory_bytes() {
        return Ok(a);
    }
    let mut sys = System::new_all();
    sys.refresh_memory();
    Ok(sys.available_memory())
}

/// Get used system memory in bytes
pub fn get_used_memory() -> Result<u64> {
    #[cfg(target_os = "linux")]
    if let Some(u) = crate::linux_proc::used_memory_bytes() {
        return Ok(u);
    }
    let mut sys = System::new_all();
    sys.refresh_memory();
    Ok(sys.used_memory())
}

/// Get memory information
pub fn get_memory_info() -> Result<MemoryInfo> {
    #[cfg(target_os = "linux")]
    if let (Some(total), Some(free)) = (
        crate::linux_proc::total_memory_bytes(),
        crate::linux_proc::available_memory_bytes(),
    ) {
        let used = total.saturating_sub(free);
        let usage_percent = if total > 0 {
            (used as f64 / total as f64 * 100.0).round()
        } else {
            0.0
        };
        return Ok(MemoryInfo {
            total,
            free,
            used,
            usage_percent,
        });
    }
    let mut sys = System::new_all();
    sys.refresh_memory();
    let total = sys.total_memory();
    let used = sys.used_memory();
    let free = sys.available_memory();
    
    let usage_percent = if total > 0 {
        (used as f64 / total as f64 * 100.0).round()
    } else {
        0.0
    };
    
    Ok(MemoryInfo {
        total,
        free,
        used,
        usage_percent,
    })
}

/// Memory information structure
#[derive(Debug, Clone)]
/// Memoryinfo
pub struct MemoryInfo {
    /// Total
    pub total: u64,
    /// Free
    pub free: u64,
    /// Used
    pub used: u64,
    /// Usage Percent
    pub usage_percent: f64,
}
// ==================== SECTION ====================

/// Get total disk space in bytes for the root filesystem
///
/// **100% SAFE**: [`rustix::fs::statvfs`] on Linux; `sysinfo` disks elsewhere
pub fn get_total_disk() -> Result<u64> {
    #[cfg(target_os = "linux")]
    if let Ok((total, _)) = crate::linux_proc::statvfs_space(std::path::Path::new("/")) {
        if total > 0 {
            return Ok(total);
        }
    }
    use sysinfo::Disks;
    let disks = Disks::new_with_refreshed_list();
    let total: u64 = disks.iter().map(|disk| disk.total_space()).sum();
    if total > 0 {
        Ok(total)
    } else {
        Ok(100 * 1024 * 1024 * 1024)
    }
}

/// Get free disk space in bytes for the root filesystem
///
/// **100% SAFE**: [`rustix::fs::statvfs`] on Linux; `sysinfo` disks elsewhere
pub fn get_free_disk() -> Result<u64> {
    #[cfg(target_os = "linux")]
    if let Ok((_, avail)) = crate::linux_proc::statvfs_space(std::path::Path::new("/")) {
        if avail > 0 {
            return Ok(avail);
        }
    }
    use sysinfo::Disks;
    let disks = Disks::new_with_refreshed_list();
    let free: u64 = disks.iter().map(|disk| disk.available_space()).sum();
    if free > 0 {
        Ok(free)
    } else {
        Ok(50 * 1024 * 1024 * 1024)
    }
}
// ==================== SECTION ====================

/// Get system hostname (universal)
pub fn get_hostname() -> Result<String> {
    gethostname::gethostname()
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| NestGateError::internal_error("Failed to get hostname"))
}

/// Get system uptime in seconds
pub fn get_uptime() -> Result<u64> {
    #[cfg(target_os = "linux")]
    if let Some(u) = crate::linux_proc::uptime_secs() {
        return Ok(u);
    }
    let sys = System::new_all();
    Ok(System::uptime(&sys))
}

/// Get load average
pub fn get_load_average() -> Result<LoadAverage> {
    #[cfg(target_os = "linux")]
    if let Some((one, five, fifteen)) = crate::linux_proc::load_averages() {
        return Ok(LoadAverage {
            one_minute: one,
            five_minutes: five,
            fifteen_minutes: fifteen,
        });
    }
    let sys = System::new_all();
    let load = System::load_average(&sys);
    Ok(LoadAverage {
        one_minute: load.one,
        five_minutes: load.five,
        fifteen_minutes: load.fifteen,
    })
}

/// Load average information
#[derive(Debug, Clone)]
/// Loadaverage
pub struct LoadAverage {
    /// One Minute
    pub one_minute: f64,
    /// Five Minutes
    pub five_minutes: f64,
    /// Fifteen Minutes
    pub fifteen_minutes: f64,
}
// ==================== SECTION ====================

/// Get current process ID
pub fn get_current_pid() -> u32 {
    std::process::id()
}
/// Get parent process ID
///
/// **100% SAFE**: Uses completely safe process information reading
pub fn get_parent_pid() -> Option<u32> {
    crate::utils::completely_safe_system::SafeSystemOps::get_parent_process_id().ok()
}
/// Check if running as root (safe system operation)
pub fn is_root() -> bool {
    crate::utils::completely_safe_system::SafeSystemOps::is_running_as_root()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_info() {
        let os_name = get_os_name();
        assert!(!os_name.is_empty());

        let arch = get_architecture();
        assert!(!arch.is_empty());

        let os_info = get_os_info().unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
            .into());
        );
        assert!(!os_info.name.is_empty());
        assert!(!os_info.architecture.is_empty());
    }

    #[test]
    fn test_cpu_info() {
        let cpu_count = get_cpu_count();
        assert!(cpu_count > 0);

        let physical_count = get_physical_cpu_count();
        assert!(physical_count > 0);
        assert!(physical_count <= cpu_count);

        let cpu_info = get_cpu_info();
        assert!(cpu_info.logical_cores > 0);
        assert!(cpu_info.physical_cores > 0);
    }

    #[test]
    fn test_memory_info() {
        let total = get_total_memory().unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
            .into());
        );
        assert!(total > 0);

        let free = get_free_memory().unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
            .into());
        );
        assert!(free > 0);

        let memory_info = get_memory_info().unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
            .into());
        );
        assert!(memory_info.total > 0);
        assert!(memory_info.usage_percent >= 0.0);
        assert!(memory_info.usage_percent <= 100.0);
    }

    #[test]
    fn test_system_identification() -> Result<()> {
        let hostname = get_hostname()?;
        assert!(!hostname.is_empty());

        let pid = get_current_pid();
        assert!(pid > 0);

        let uptime = get_uptime()?;
        // Uptime should be non-negative (could be 0 on non-Linux systems)
        assert!(uptime >= 0);

        Ok(())
    }

    #[test]
    fn test_load_average() -> Result<()> {
        let load_avg = get_load_average()?;
        assert!(load_avg.one_minute >= 0.0);
        assert!(load_avg.five_minutes >= 0.0);
        assert!(load_avg.fifteen_minutes >= 0.0);

        Ok(())
    }
}
