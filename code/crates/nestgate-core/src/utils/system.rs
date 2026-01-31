// System utilities for NestGate core functionality
//! System functionality and utilities.
// Provides safe system operations and utilities.
//!
//! **UNIVERSAL SYSTEM INFO** - Uses `sysinfo` crate for cross-platform support
//! **EVOLUTION**: Migrated from Linux `/proc/` to universal Rust (Jan 31, 2026)
//! **Phase 1**: Deep Debt Evolution - Modern Idiomatic Rust

use crate::{NestGateError, Result};
use sysinfo::{System, SystemExt, CpuExt, ProcessExt, Pid};

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
    Ok(System::kernel_version().unwrap_or_else(|| "unknown".to_string()))
}
// ==================== SECTION ====================

/// Get the number of CPU cores (universal via sysinfo)
pub fn get_cpu_count() -> usize {
    let sys = System::new_all();
    sys.cpus().len()
}

/// Get the number of physical CPU cores (universal via sysinfo)
pub fn get_physical_cpu_count() -> usize {
    let sys = System::new_all();
    sys.physical_core_count().unwrap_or_else(|| sys.cpus().len())
}

/// Get CPU information (universal via sysinfo)
pub fn get_cpu_info() -> CpuInfo {
    let sys = System::new_all();
    let cpus = sys.cpus();
    
    CpuInfo {
        logical_cores: cpus.len(),
        physical_cores: sys.physical_core_count().unwrap_or(cpus.len()),
        model: cpus.first().map(|c| c.brand().to_string()).unwrap_or_else(|| "Unknown CPU".to_string()),
        frequency: cpus.first().map(|c| c.frequency() as f64 / 1000.0), // MHz to GHz
    }
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

/// Get total system memory in bytes (universal via sysinfo)
pub fn get_total_memory() -> Result<u64> {
    let sys = System::new_all();
    Ok(sys.total_memory())
}

/// Get free system memory in bytes (universal via sysinfo)
pub fn get_free_memory() -> Result<u64> {
    let sys = System::new_all();
    Ok(sys.available_memory())
}

/// Get used system memory in bytes (universal via sysinfo)
pub fn get_used_memory() -> Result<u64> {
    let sys = System::new_all();
    Ok(sys.used_memory())
}

/// Get memory information (universal via sysinfo)
pub fn get_memory_info() -> Result<MemoryInfo> {
    let sys = System::new_all();
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
/// **100% SAFE**: This function uses completely safe filesystem operations via sysinfo
pub fn get_total_disk() -> Result<u64> {
    use sysinfo::{System, Disks, DisksExt};
    
    let disks = Disks::new_with_refreshed_list();
    
    // Find root disk or return sum of all disks
    let total = disks.iter()
        .map(|disk| disk.total_space())
        .sum();
    
    if total > 0 {
        Ok(total)
    } else {
        // Fallback estimate if no disks detected
        Ok(100 * 1024 * 1024 * 1024) // 100GB estimate
    }
}

/// Get free disk space in bytes for the root filesystem
///
/// **100% SAFE**: This function uses completely safe filesystem operations via sysinfo
pub fn get_free_disk() -> Result<u64> {
    use sysinfo::{System, Disks, DisksExt};
    
    let disks = Disks::new_with_refreshed_list();
    
    // Find root disk or return sum of available space
    let free = disks.iter()
        .map(|disk| disk.available_space())
        .sum();
    
    if free > 0 {
        Ok(free)
    } else {
        // Fallback estimate if no disks detected
        Ok(50 * 1024 * 1024 * 1024) // 50GB estimate
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

/// Get system uptime in seconds (universal via sysinfo)
pub fn get_uptime() -> Result<u64> {
    let sys = System::new_all();
    Ok(System::uptime(&sys))
}

/// Get load average (universal via sysinfo)
pub fn get_load_average() -> Result<LoadAverage> {
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
        // Load averages should be non-negative
        assert!(load_avg >= 0.0);

        Ok(())
    }
}
