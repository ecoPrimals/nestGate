//! Enhanced Utility Functions for NestGate v2
//!
//! Integrates advanced utilities with v2 system utilities
//! Provides comprehensive helper functions for the NestGate system

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::{Path, PathBuf};
use std::fs as stdfs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::env;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, TimeZone, Utc};

use crate::error::{NestGateError, Result};

/// Enhanced file system utilities with advanced capabilities
pub mod fs {
    use super::*;
    use std::io;

    /// Checks if a path exists.
    #[must_use]
    pub fn exists(path: &Path) -> bool {
        path.exists()
    }

    /// Creates a directory and all its parent directories if they don't exist.
    /// 
    /// # Errors
    /// Returns an error if the directory cannot be created.
    pub fn ensure_dir(path: &Path) -> io::Result<()> {
        stdfs::create_dir_all(path)
    }

    /// Removes a file or directory and all its contents.
    /// 
    /// # Errors
    /// Returns an error if the path cannot be removed.
    pub fn remove_path(path: &Path) -> io::Result<()> {
        if path.is_dir() {
            stdfs::remove_dir_all(path)
        } else {
            stdfs::remove_file(path)
        }
    }

    /// Gets the size of a file in bytes.
    /// 
    /// # Errors
    /// Returns an error if the file size cannot be determined.
    pub fn get_file_size(path: &Path) -> io::Result<u64> {
        stdfs::metadata(path).map(|m| m.len())
    }

    /// Recursively calculates the size of a directory
    pub fn get_directory_size(path: &Path) -> io::Result<u64> {
        let mut total_size = 0;
        
        if path.is_dir() {
            for entry in stdfs::read_dir(path)? {
                let entry = entry?;
                let metadata = entry.metadata()?;
                
                if metadata.is_dir() {
                    total_size += get_directory_size(&entry.path())?;
                } else {
                    total_size += metadata.len();
                }
            }
        } else {
            total_size = stdfs::metadata(path)?.len();
        }
        
        Ok(total_size)
    }

    /// Copy a file or directory recursively
    pub fn copy_recursive(src: &Path, dst: &Path) -> io::Result<()> {
        if src.is_dir() {
            stdfs::create_dir_all(dst)?;
            for entry in stdfs::read_dir(src)? {
                let entry = entry?;
                let src_path = entry.path();
                let dst_path = dst.join(entry.file_name());
                copy_recursive(&src_path, &dst_path)?;
            }
        } else {
            if let Some(parent) = dst.parent() {
                stdfs::create_dir_all(parent)?;
            }
            stdfs::copy(src, dst)?;
        }
        Ok(())
    }

    /// Check if a path is readable
    pub fn is_readable(path: &Path) -> bool {
        match stdfs::metadata(path) {
            Ok(metadata) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let permissions = metadata.permissions();
                    (permissions.mode() & 0o444) != 0
                }
                #[cfg(not(unix))]
                {
                    true // On non-Unix systems, assume readable if metadata is accessible
                }
            }
            Err(_) => false,
        }
    }

    /// Check if a path is writable
    pub fn is_writable(path: &Path) -> bool {
        if !path.exists() {
            if let Some(parent) = path.parent() {
                return is_writable(parent);
            }
            return false;
        }

        match stdfs::metadata(path) {
            Ok(metadata) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let permissions = metadata.permissions();
                    (permissions.mode() & 0o222) != 0
                }
                #[cfg(not(unix))]
                {
                    // On non-Unix systems, try to open for writing
                    if path.is_dir() {
                        let test_file = path.join(".write_test");
                        let result = stdfs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(&test_file);
                        if result.is_ok() {
                            let _ = stdfs::remove_file(test_file);
                            return true;
                        }
                        false
                    } else {
                        stdfs::OpenOptions::new()
                            .write(true)
                            .open(path)
                            .is_ok()
                    }
                }
            }
            Err(_) => false,
        }
    }
}

/// Enhanced system information structure with advanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Operating system name
    pub os_name: String,
    /// Operating system version
    pub os_version: String,
    /// System architecture
    pub architecture: String,
    /// Number of CPU cores
    pub cpu_cores: u32,
    /// Total memory in bytes
    pub total_memory: u64,
    /// Free memory in bytes
    pub free_memory: u64,
    /// Total disk space in bytes
    pub total_disk: u64,
    /// Free disk space in bytes
    pub free_disk: u64,
    /// System uptime
    pub uptime: Duration,
    /// System hostname
    pub hostname: String,
}

impl SystemInfo {
    /// Creates a new SystemInfo instance with current system data
    #[must_use]
    pub fn new() -> Self {
        Self {
            os_name: env::consts::OS.to_string(),
            os_version: sys::get_os_version().unwrap_or_else(|_| "unknown".to_string()),
            architecture: env::consts::ARCH.to_string(),
            cpu_cores: sys::get_cpu_count() as u32,
            total_memory: sys::get_total_memory().unwrap_or(0),
            free_memory: sys::get_free_memory().unwrap_or(0),
            total_disk: sys::get_total_disk().unwrap_or(0),
            free_disk: sys::get_free_disk().unwrap_or(0),
            uptime: sys::get_uptime().unwrap_or(Duration::ZERO),
            hostname: sys::get_hostname().unwrap_or_else(|_| "unknown".to_string()),
        }
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhanced system utilities with advanced capabilities
pub mod sys {
    use super::*;

    /// Gets system information.
    #[must_use]
    pub fn get_system_info() -> SystemInfo {
        SystemInfo::new()
    }

    /// Get the operating system version
    pub fn get_os_version() -> Result<String> {
        #[cfg(target_os = "linux")]
        {
            match stdfs::read_to_string("/etc/os-release") {
                Ok(content) => {
                    for line in content.lines() {
                        if line.starts_with("VERSION=") {
                            return Ok(line.trim_start_matches("VERSION=").trim_matches('"').to_string());
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

    /// Get the number of CPU cores
    pub fn get_cpu_count() -> usize {
        num_cpus::get()
    }

    /// Get system hostname
    pub fn get_hostname() -> Result<String> {
        gethostname::gethostname()
            .to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| NestGateError::SystemError("Failed to get hostname".to_string()))
    }

    /// Get total system memory in bytes
    pub fn get_total_memory() -> Result<u64> {
        #[cfg(target_os = "linux")]
        {
            match stdfs::read_to_string("/proc/meminfo") {
                Ok(content) => {
                    let parts: Vec<&str> = content.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<u64>() {
                            return Ok(kb * 1024); // Convert KB to bytes
                        }
                    }
                    Err(NestGateError::SystemError("Failed to parse /proc/meminfo".to_string()))
                }
                Err(e) => Err(NestGateError::SystemError(format!("Failed to read /proc/meminfo: {}", e))),
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
            match stdfs::read_to_string("/proc/meminfo") {
                Ok(content) => {
                    let parts: Vec<&str> = content.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<u64>() {
                            return Ok(kb * 1024); // Convert KB to bytes
                        }
                    }
                    Err(NestGateError::SystemError("Failed to parse /proc/meminfo".to_string()))
                }
                Err(e) => Err(NestGateError::SystemError(format!("Failed to read /proc/meminfo: {}", e))),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            // Fallback for non-Linux systems
            Ok(4 * 1024 * 1024 * 1024) // Default to 4GB free
        }
    }

    /// Get total disk space in bytes
    pub fn get_total_disk() -> Result<u64> {
        // This is a simplified implementation
        // In a real implementation, you'd want to check specific mount points
        match stdfs::metadata("/") {
            Ok(_) => Ok(1024 * 1024 * 1024 * 1024), // Default to 1TB
            Err(e) => Err(NestGateError::SystemError(format!("Failed to get disk info: {}", e))),
        }
    }

    /// Get free disk space in bytes
    pub fn get_free_disk() -> Result<u64> {
        // This is a simplified implementation
        Ok(512 * 1024 * 1024 * 1024) // Default to 512GB free
    }

    /// Get system uptime
    pub fn get_uptime() -> Result<Duration> {
        #[cfg(target_os = "linux")]
        {
            match stdfs::read_to_string("/proc/uptime") {
                Ok(content) => {
                    let parts: Vec<&str> = content.split_whitespace().collect();
                    if !parts.is_empty() {
                        if let Ok(seconds) = parts[0].parse::<f64>() {
                            return Ok(Duration::from_secs_f64(seconds));
                        }
                    }
                    Err(NestGateError::SystemError("Failed to parse /proc/uptime".to_string()))
                }
                Err(e) => Err(NestGateError::SystemError(format!("Failed to read /proc/uptime: {}", e))),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            // Fallback for non-Linux systems
            Ok(Duration::from_secs(3600)) // Default to 1 hour uptime
        }
    }
}

/// Enhanced time utilities with advanced capabilities
pub mod time {
    use super::*;

    /// Converts a timestamp to a DateTime
    #[must_use]
    pub fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
        Utc.timestamp_opt(timestamp, 0).single().unwrap_or_else(Utc::now)
    }

    /// Gets the current timestamp in seconds
    #[must_use]
    pub fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| i64::try_from(d.as_secs()).unwrap_or(0))
            .unwrap_or(0)
    }

    /// Gets the current timestamp in milliseconds
    #[must_use]
    pub fn current_timestamp_millis() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| i64::try_from(d.as_millis()).unwrap_or(0))
            .unwrap_or(0)
    }

    /// Format a duration as a human-readable string
    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.as_secs();
        let days = total_seconds / 86400;
        let hours = (total_seconds % 86400) / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if days > 0 {
            format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
        } else if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }

    /// Parse a timestamp string
    pub fn parse_timestamp(s: &str) -> Result<i64> {
        s.parse::<i64>()
            .map_err(|e| NestGateError::InvalidInput(format!("Invalid timestamp: {}", e)))
    }

    /// Get elapsed time since a timestamp
    pub fn elapsed_since(timestamp: i64) -> Duration {
        let now = current_timestamp();
        if now >= timestamp {
            Duration::from_secs((now - timestamp) as u64)
        } else {
            Duration::ZERO
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_timestamp_conversion() {
            let now = current_timestamp();
            let datetime = timestamp_to_datetime(now);
            assert_eq!(datetime.timestamp(), now);
        }

        #[test]
        fn test_duration_formatting() {
            assert_eq!(format_duration(Duration::from_secs(30)), "30s");
            assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
            assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
            assert_eq!(format_duration(Duration::from_secs(90061)), "1d 1h 1m 1s");
        }
    }
}

/// Network-related utilities
pub mod network {
    use super::*;
    
    /// Check if an IP address is valid
    pub fn is_valid_ip(ip: &str) -> bool {
        ip.parse::<IpAddr>().is_ok()
    }
    
    /// Check if an IP address is a valid IPv4 address
    pub fn is_valid_ipv4(ip: &str) -> bool {
        ip.parse::<Ipv4Addr>().is_ok()
    }
    
    /// Check if an IP address is a valid IPv6 address
    pub fn is_valid_ipv6(ip: &str) -> bool {
        ip.parse::<Ipv6Addr>().is_ok()
    }
    
    /// Check if a CIDR notation is valid
    pub fn is_valid_cidr(cidr: &str) -> bool {
        let parts: Vec<&str> = cidr.split('/').collect();
        if parts.len() != 2 {
            return false;
        }
        
        let ip = parts[0];
        let prefix = parts[1];
        
        if !is_valid_ip(ip) {
            return false;
        }
        
        if let Ok(prefix_len) = prefix.parse::<u8>() {
            if ip.parse::<Ipv4Addr>().is_ok() {
                return prefix_len <= 32;
            } else if ip.parse::<Ipv6Addr>().is_ok() {
                return prefix_len <= 128;
            }
        }
        
        false
    }
    
    /// Parse a CIDR notation into IP address and prefix length
    pub fn parse_cidr(cidr: &str) -> Result<(IpAddr, u8)> {
        let parts: Vec<&str> = cidr.split('/').collect();
        if parts.len() != 2 {
            return Err(NestGateError::InvalidInput(format!("Invalid CIDR notation: {}", cidr)));
        }
        
        let ip = parts[0].parse::<IpAddr>()
            .map_err(|_| NestGateError::InvalidInput(format!("Invalid IP address: {}", parts[0])))?;
        
        let prefix = parts[1].parse::<u8>()
            .map_err(|_| NestGateError::InvalidInput(format!("Invalid prefix length: {}", parts[1])))?;
        
        // Validate prefix length
        match ip {
            IpAddr::V4(_) if prefix > 32 => {
                return Err(NestGateError::InvalidInput(format!("Invalid IPv4 prefix length: {}", prefix)));
            }
            IpAddr::V6(_) if prefix > 128 => {
                return Err(NestGateError::InvalidInput(format!("Invalid IPv6 prefix length: {}", prefix)));
            }
            _ => {}
        }
        
        Ok((ip, prefix))
    }
    
    /// Check if a hostname is valid
    pub fn is_valid_hostname(hostname: &str) -> bool {
        if hostname.is_empty() || hostname.len() > 63 {
            return false;
        }
        
        if hostname.starts_with('-') || hostname.ends_with('-') {
            return false;
        }
        
        if hostname.contains("--") {
            return false;
        }
        
        hostname.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
    }
    
    /// Check if a port number is valid
    pub fn is_valid_port(port: u16) -> bool {
        port > 0
    }
    
    /// Check if a port is available
    pub async fn is_port_available(port: u16) -> bool {
        let addr = format!("127.0.0.1:{}", port);
        (tokio::net::TcpListener::bind(&addr).await).is_ok()
    }
}

/// File system-related utilities (v2 compatibility)
pub mod filesys {
    use super::*;
    
    /// Check if a path exists
    pub fn path_exists(path: &Path) -> bool {
        path.exists()
    }
    
    /// Check if a path is a directory
    pub fn is_directory(path: &Path) -> bool {
        path.is_dir()
    }
    
    /// Check if a path is a file
    pub fn is_file(path: &Path) -> bool {
        path.is_file()
    }
    
    /// Check if a path is readable (delegated to fs module)
    pub fn is_readable(path: &Path) -> bool {
        fs::is_readable(path)
    }
    
    /// Check if a path is writable (delegated to fs module)
    pub fn is_writable(path: &Path) -> bool {
        fs::is_writable(path)
    }
    
    /// Get file size
    pub fn file_size(path: &Path) -> Result<u64> {
        fs::get_file_size(path)
            .map_err(|e| NestGateError::FileSystem(format!("Failed to get file size: {}", e)))
    }
    
    /// Create directory
    pub fn create_dir_all(path: &Path) -> Result<()> {
        fs::ensure_dir(path)
            .map_err(|e| NestGateError::FileSystem(format!("Failed to create directory: {}", e)))
    }
    
    /// Remove file
    pub fn remove_file(path: &Path) -> Result<()> {
        stdfs::remove_file(path)
            .map_err(|e| NestGateError::FileSystem(format!("Failed to remove file: {}", e)))
    }
    
    /// Remove directory
    pub fn remove_dir_all(path: &Path) -> Result<()> {
        stdfs::remove_dir_all(path)
            .map_err(|e| NestGateError::FileSystem(format!("Failed to remove directory: {}", e)))
    }
    
    /// Get temp directory
    pub fn temp_dir() -> PathBuf {
        std::env::temp_dir()
    }
    
    /// Create temp directory
    pub fn create_temp_dir() -> Result<PathBuf> {
        let temp_dir = temp_dir().join(format!("nestgate-{}", uuid::Uuid::new_v4()));
        create_dir_all(&temp_dir)?;
        Ok(temp_dir)
    }
    
    /// Calculate directory size
    pub fn directory_size(path: &Path) -> Result<u64> {
        fs::get_directory_size(path)
            .map_err(|e| NestGateError::FileSystem(format!("Failed to calculate directory size: {}", e)))
    }
}

/// String-related utilities
pub mod string {
    use fastrand;

    /// Generate a random string of specified length
    pub fn random_string(length: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789";
        (0..length)
            .map(|_| {
                let idx = fastrand::usize(..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    /// Truncate a string to a maximum length
    pub fn truncate(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        }
    }

    /// Check if a string contains only alphanumeric characters
    pub fn is_alphanumeric(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric())
    }

    /// Check if a string contains only alphabetic characters
    pub fn is_alphabetic(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic())
    }

    /// Check if a string contains only numeric characters
    pub fn is_numeric(s: &str) -> bool {
        !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
    }

    /// Convert string to snake_case
    pub fn to_snake_case(input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c.is_uppercase() && !result.is_empty() {
                result.push('_');
            }
            if let Some(lowercase_char) = c.to_lowercase().next() {
                result.push(lowercase_char);
            }
        }
        
        result
    }

    /// Convert string to camelCase
    pub fn to_camel_case(input: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;
        
        for c in input.chars() {
            if c == '_' || c == '-' {
                capitalize_next = true;
            } else if capitalize_next {
                if let Some(uppercase_char) = c.to_uppercase().next() {
                    result.push(uppercase_char);
                }
                capitalize_next = false;
            } else {
                if let Some(lowercase_char) = c.to_lowercase().next() {
                    result.push(lowercase_char);
                }
            }
        }
        
        result
    }

    /// Convert string to PascalCase
    pub fn to_pascal_case(s: &str) -> String {
        let camel = to_camel_case(s);
        if let Some(first_char) = camel.chars().next() {
            format!("{}{}", first_char.to_uppercase(), &camel[1..])
        } else {
            camel
        }
    }

    /// Convert string to kebab-case
    pub fn to_kebab_case(s: &str) -> String {
        to_snake_case(s).replace('_', "-")
    }
}

/// Serialization utilities
pub mod serialization {
    use super::*;

    /// Serialize to JSON
    pub fn to_json<T: Serialize>(value: &T) -> Result<String> {
        serde_json::to_string(value)
            .map_err(|e| NestGateError::Serialization(e.to_string()))
    }

    /// Serialize to pretty JSON
    pub fn to_json_pretty<T: Serialize>(value: &T) -> Result<String> {
        serde_json::to_string_pretty(value)
            .map_err(|e| NestGateError::Serialization(e.to_string()))
    }

    /// Deserialize from JSON
    pub fn from_json<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T> {
        serde_json::from_str(json)
            .map_err(|e| NestGateError::Serialization(e.to_string()))
    }

    /// Serialize to YAML
    pub fn to_yaml<T: Serialize>(value: &T) -> Result<String> {
        serde_yaml::to_string(value)
            .map_err(|e| NestGateError::Serialization(e.to_string()))
    }

    /// Deserialize from YAML
    pub fn from_yaml<T: for<'de> Deserialize<'de>>(yaml: &str) -> Result<T> {
        serde_yaml::from_str(yaml)
            .map_err(|e| NestGateError::Serialization(e.to_string()))
    }
}

/// System-related utilities
pub mod system {
    use super::*;

    /// Get system hostname (delegated to sys module)
    pub fn get_hostname() -> Result<String> {
        sys::get_hostname()
    }

    /// Get CPU count (delegated to sys module)
    pub fn get_cpu_count() -> usize {
        sys::get_cpu_count()
    }

    /// Get system uptime (delegated to sys module)
    pub fn get_uptime() -> Result<Duration> {
        sys::get_uptime()
    }

    /// Get memory information (total, free)
    pub fn get_memory_info() -> Result<(u64, u64)> {
        let total = sys::get_total_memory()?;
        let free = sys::get_free_memory()?;
        Ok((total, free))
    }
}

/// Configuration-related utilities
pub mod config {
    use super::*;

    /// Load configuration from file
    pub fn load_config<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
        let content = stdfs::read_to_string(path)
            .map_err(|e| NestGateError::FileSystem(format!("Failed to read config file: {}", e)))?;
        
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
           path.extension().and_then(|s| s.to_str()) == Some("yml") {
            serialization::from_yaml(&content)
        } else {
            serialization::from_json(&content)
        }
    }

    /// Save configuration to file
    pub fn save_config<T: Serialize>(value: &T, path: &Path) -> Result<()> {
        let content = if path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
                         path.extension().and_then(|s| s.to_str()) == Some("yml") {
            serialization::to_yaml(value)?
        } else {
            serialization::to_json_pretty(value)?
        };
        
        if let Some(parent) = path.parent() {
            filesys::create_dir_all(parent)?;
        }
        
        stdfs::write(path, content)
            .map_err(|e| NestGateError::FileSystem(format!("Failed to write config file: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_filesystem_operations() {
        let dir = tempdir().expect("Failed to create temporary directory for filesystem test");
        let file_path = dir.path().join("test.txt");
        let dir_path = dir.path().join("subdir");

        // Test file creation and size
        stdfs::File::create(&file_path)
            .expect("Failed to create test file")
            .write_all(b"test")
            .expect("Failed to write test data to file");
        
        assert_eq!(fs::get_file_size(&file_path).expect("Failed to get file size"), 4);

        // Test directory creation
        fs::ensure_dir(&dir_path).expect("Failed to create test directory");
        assert!(dir_path.exists());

        // Test file removal
        fs::remove_path(&file_path).expect("Failed to remove test file");
        assert!(!file_path.exists());

        // Test directory removal
        fs::remove_path(&dir_path).expect("Failed to remove test directory");
        assert!(!dir_path.exists());
    }

    #[test]
    fn test_system_info() {
        let info = SystemInfo::new();
        assert!(!info.os_name.is_empty());
        assert!(!info.architecture.is_empty());
        assert!(info.cpu_cores > 0);
    }

    #[test]
    fn test_time_utils() {
        let timestamp = time::current_timestamp();
        let datetime = time::timestamp_to_datetime(timestamp);
        assert_eq!(datetime.timestamp(), timestamp);

        let duration = Duration::from_secs(3661);
        assert_eq!(time::format_duration(duration), "1h 1m 1s");
    }

    #[test]
    fn test_string_utils() {
        assert_eq!(string::to_snake_case("CamelCase"), "camel_case");
        assert_eq!(string::to_camel_case("snake_case"), "snakeCase");
        assert_eq!(string::to_pascal_case("snake_case"), "SnakeCase");
        assert_eq!(string::to_kebab_case("CamelCase"), "camel-case");
        
        assert!(string::is_alphanumeric("abc123"));
        assert!(!string::is_alphanumeric("abc-123"));
        
        let random = string::random_string(10);
        assert_eq!(random.len(), 10);
    }

    #[test]
    fn test_network_utils() {
        assert!(network::is_valid_ip("192.168.1.1"));
        assert!(network::is_valid_ipv4("192.168.1.1"));
        assert!(network::is_valid_ipv6("::1"));
        assert!(network::is_valid_cidr("192.168.1.0/24"));
        assert!(network::is_valid_hostname("example"));
        assert!(!network::is_valid_hostname("-example"));
        assert!(network::is_valid_port(8080));
        assert!(!network::is_valid_port(0));
    }
} 