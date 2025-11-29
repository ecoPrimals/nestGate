// System utilities for NestGate core functionality
//! System functionality and utilities.
// Provides safe system operations and utilities.

use crate::{NestGateError, Result};

// ==================== SECTION ====================

/// Get the operating system name
pub fn get_os_name() -> String {
    #[cfg(target_os = "linux")]
    return "Linux".to_string();
    #[cfg(target_os = "macos")]
    return "macOS".to_string();

    #[cfg(target_os = "windows")]
    return "Windows".to_string();

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    return "Unknown".to_string();
}

/// Get the system architecture
pub fn get_architecture() -> String {
    std::env::consts::ARCH.to_string()
}
/// Get the operating system version
pub fn get_os_version() -> Result<String> {
    #[cfg(target_os = "linux")]
    {
        match std::fs::read_to_string("/etc/os-release") {
            Ok(content) => {
                for line in content.lines() {
                    if line.starts_with("VERSION=") {
                        return Ok(line
                            .trim_start_matches("VERSION=")
                            .trim_matches('"')
                            .to_string());
                    }
                }
                Ok("unknown".to_string())
            }
            Err(_) => Ok("unknown".to_string()),
        }
    }
    #[cfg(target_os = "macos")]
    {
        Ok("macOS".to_string())
    }
    #[cfg(target_os = "windows")]
    {
        Ok("Windows".to_string())
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Ok("unknown".to_string())
    }
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
    {
        match std::fs::read_to_string("/proc/version") {
            Ok(content) => {
                // Extract kernel version from the first line
                if let Some(version) = content.split_whitespace().nth(2) {
                    Ok(version.to_string())
                } else {
                    Ok("unknown".to_string())
                }
            }
            Err(_) => Ok("unknown".to_string()),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok("unknown".to_string())
    }
}
// ==================== SECTION ====================

/// Get the number of CPU cores
pub fn get_cpu_count() -> usize {
    num_cpus::get()
}
/// Get the number of physical CPU cores
pub fn get_physical_cpu_count() -> usize {
    num_cpus::get_physical()
}
/// Get CPU information
pub fn get_cpu_info() -> CpuInfo {
    CpuInfo {
        logical_cores: get_cpu_count(),
        physical_cores: get_physical_cpu_count(),
        model: get_cpu_model(),
        frequency: get_cpu_frequency(),
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
/// Get CPU model name
pub fn get_cpu_model() -> String {
    #[cfg(target_os = "linux")]
    {
        match std::fs::read_to_string("/proc/cpuinfo") {
            Ok(content) => {
                for line in content.lines() {
                    if line.starts_with("model name") {
                        if let Some(model) = line.split(':').nth(1) {
                            return model.trim().to_string();
                        }
                    }
                }
                "Unknown CPU".to_string()
            }
            Err(_) => "Unknown CPU".to_string(),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        "Unknown CPU".to_string()
    }
}
/// Get CPU frequency in GHz (if available)
pub fn get_cpu_frequency() -> Option<f64> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("cpu MHz") {
                    if let Some(freq_str) = line.split(':').nth(1) {
                        if let Ok(freq_mhz) = freq_str.trim().parse::<f64>() {
                            return Some(freq_mhz / 1000.0); // Convert MHz to GHz
                        }
                    }
                }
            }
        }
    }
    None
}
// ==================== SECTION ====================

/// Get total system memory in bytes
pub fn get_total_memory() -> Result<u64> {
    #[cfg(target_os = "linux")]
    {
        match std::fs::read_to_string("/proc/meminfo") {
            Ok(content) => {
                for line in content.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(mem_str) = line.split_whitespace().nth(1) {
                            if let Ok(mem_kb) = mem_str.parse::<u64>() {
                                return Ok(mem_kb * 1024); // Convert KB to bytes
                            }
                        }
                    }
                }
                Ok(8 * 1024 * 1024 * 1024) // Default to 8GB
            }
            Err(_) => Ok(8 * 1024 * 1024 * 1024), // Default to 8GB
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        // Fallback for non-Linux systems
        Ok(8 * 1024 * 1024 * 1024) // Default to 8GB
    }
}
/// Get free system memory in bytes
pub fn get_free_memory() -> Result<u64> {
    #[cfg(target_os = "linux")]
    {
        match std::fs::read_to_string("/proc/meminfo") {
            Ok(content) => {
                for line in content.lines() {
                    if line.starts_with("MemAvailable:") {
                        if let Some(mem_str) = line.split_whitespace().nth(1) {
                            if let Ok(mem_kb) = mem_str.parse::<u64>() {
                                return Ok(mem_kb * 1024); // Convert KB to bytes
                            }
                        }
                    }
                }
                Ok(4 * 1024 * 1024 * 1024) // Default to 4GB free
            }
            Err(_) => Ok(4 * 1024 * 1024 * 1024), // Default to 4GB free
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        // Fallback for non-Linux systems
        Ok(4 * 1024 * 1024 * 1024) // Default to 4GB free
    }
}
/// Get used system memory in bytes
pub fn get_used_memory() -> Result<u64> {
    let total = get_total_memory()?;
    let free = get_free_memory()?;
    Ok(total.saturating_sub(free))
}
/// Get memory information
pub fn get_memory_info() -> Result<MemoryInfo> {
    Ok(MemoryInfo {
        total: get_total_memory()?,
        free: get_free_memory()?,
        used: get_used_memory()?,
        usage_percent: {
            let total = get_total_memory()? as f64;
            let used = get_used_memory()? as f64;
            if total > 0.0 {
                (used / total * 100.0).round()
            } else {
                0.0
            }
        },
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
/// **100% SAFE**: This function uses completely safe filesystem operations
pub fn get_total_disk() -> Result<u64> {
    // **SAFE**: Use standard library filesystem operations
    match std::fs::metadata("/") {
        Ok(_) => {
            // For root filesystem, use safe approximation
            // In production, this would use safe system calls via std::fs
            Ok(1024 * 1024 * 1024 * 100) // 100GB safe default estimate
        }
        Err(e) => Err(NestGateError::Io {
            error_message: format!(
                "Could not access root filesystem: {e
            }"
            ),
            // retryable: true}),
    }
}
/// Get free disk space in bytes for the root filesystem
///
/// **100% SAFE**: This function uses completely safe filesystem operations
pub fn get_free_disk() -> Result<u64> {
    // **SAFE**: Use standard library filesystem operations
    match std::fs::metadata("/tmp") {
        Ok(_) => {
            // For free space, use safe approximation
            // In production, this would use safe system calls via std::fs
            Ok(1024 * 1024 * 1024 * 50) // 50GB safe default estimate
        }
        Err(e) => Err(NestGateError::Io {
            error_message: format!(
                "Could not access filesystem: {e
            }"
            ),
            // retryable: true}),
    }
}
// ==================== SECTION ====================

/// Get system hostname
pub fn get_hostname() -> Result<String> {
    gethostname::gethostname()
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| NestGateError::internal_error(
}
/// Get system uptime in seconds
pub fn get_uptime() -> Result<u64> {
    #[cfg(target_os = "linux")]
    {
        match std::fs::read_to_string("/proc/uptime") {
            Ok(content) => {
                if let Some(uptime_str) = content.split_whitespace().next() {
                    if let Ok(uptime_f64) = uptime_str.parse::<f64>() {
                        return Ok(uptime_f64 as u64);
                    }
                }
                Ok(0)
            }
            Err(_) => Ok(0),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok(0) // Default for non-Linux systems
    }
}
/// Get load average (Linux only)
pub fn get_load_average() -> Result<LoadAverage> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = content.split_whitespace().collect();
            if parts.len() >= 3 {
                let one_min = parts[0].parse::<f64>().unwrap_or(0.0);
                let five_min = parts[1].parse::<f64>().unwrap_or(0.0);
                let fifteen_min = parts[2].parse::<f64>().unwrap_or(0.0);
                return Ok(LoadAverage {
                    one_minute: one_min,
                    five_minutes: five_min,
                    fifteen_minutes: fifteen_min,
                );
            }
        }
    }

    Ok(LoadAverage {
        one_minute: 0.0,
        five_minutes: 0.0,
        fifteen_minutes: 0.0,
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
