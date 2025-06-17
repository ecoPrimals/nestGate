//! Utility functions for the NestGate system
//!
//! This module provides various utility functions and helpers for use across
//! the NestGate system.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

use crate::error::{NestGateError, Result};

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
        // A valid hostname:
        // - Contains only letters, numbers, and hyphens
        // - Does not start or end with a hyphen
        // - Is between 1 and 63 characters long
        // - Does not contain consecutive hyphens
        
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
        // Try to bind to the port
        let addr = format!("127.0.0.1:{}", port);
        match tokio::net::TcpListener::bind(&addr).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

/// File system-related utilities
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
    
    /// Check if a path is readable
    pub fn is_readable(path: &Path) -> bool {
        match fs::metadata(path) {
            Ok(metadata) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let permissions = metadata.permissions();
                    return (permissions.mode() & 0o444) != 0;
                }
                
                #[cfg(not(unix))]
                {
                    // On non-Unix systems, just check if the file exists
                    true
                }
            }
            Err(_) => false,
        }
    }
    
    /// Check if a path is writable
    pub fn is_writable(path: &Path) -> bool {
        // If the path doesn't exist, check the parent directory
        if !path.exists() {
            if let Some(parent) = path.parent() {
                return is_writable(parent);
            }
            return false;
        }
        
        match fs::metadata(path) {
            Ok(metadata) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let permissions = metadata.permissions();
                    return (permissions.mode() & 0o222) != 0;
                }
                
                #[cfg(not(unix))]
                {
                    // On non-Unix systems, try to open the file for writing
                    if path.is_dir() {
                        let test_file = path.join(".write_test");
                        let result = fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(&test_file);
                        if result.is_ok() {
                            let _ = fs::remove_file(test_file);
                            return true;
                        }
                        false
                    } else {
                        fs::OpenOptions::new()
                            .write(true)
                            .open(path)
                            .is_ok()
                    }
                }
            }
            Err(_) => false,
        }
    }
    
    /// Get file size in bytes
    pub fn file_size(path: &Path) -> Result<u64> {
        let metadata = fs::metadata(path)?;
        Ok(metadata.len())
    }
    
    /// Create a directory and all parent directories
    pub fn create_dir_all(path: &Path) -> Result<()> {
        fs::create_dir_all(path)?;
        Ok(())
    }
    
    /// Remove a file
    pub fn remove_file(path: &Path) -> Result<()> {
        fs::remove_file(path)?;
        Ok(())
    }
    
    /// Remove a directory and all its contents
    pub fn remove_dir_all(path: &Path) -> Result<()> {
        fs::remove_dir_all(path)?;
        Ok(())
    }
    
    /// Get a temporary directory path
    pub fn temp_dir() -> PathBuf {
        std::env::temp_dir().join("nestgate")
    }
    
    /// Create a temporary directory
    pub fn create_temp_dir() -> Result<PathBuf> {
        let temp_dir = temp_dir();
        create_dir_all(&temp_dir)?;
        Ok(temp_dir)
    }
    
    /// Calculate directory size recursively
    pub fn directory_size(path: &Path) -> Result<u64> {
        if !path.is_dir() {
            return Err(NestGateError::InvalidInput(format!("Path is not a directory: {:?}", path)));
        }
        
        let mut total_size = 0;
        
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                total_size += file_size(&path)?;
            } else if path.is_dir() {
                total_size += directory_size(&path)?;
            }
        }
        
        Ok(total_size)
    }
}

/// String-related utilities
pub mod string {
    use super::*;
    
    /// Generate a random alphanumeric string of the specified length
    pub fn random_string(length: usize) -> String {
        use rand::{Rng, thread_rng};
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        
        let mut rng = thread_rng();
        let string: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        
        string
    }
    
    /// Truncate a string to a maximum length with ellipsis
    pub fn truncate(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            let mut truncated = String::with_capacity(max_len);
            truncated.push_str(&s[..max_len - 3]);
            truncated.push_str("...");
            truncated
        }
    }
    
    /// Check if a string contains only ASCII alphanumeric characters
    pub fn is_alphanumeric(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_alphanumeric())
    }
    
    /// Check if a string contains only ASCII alphabetic characters
    pub fn is_alphabetic(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_alphabetic())
    }
    
    /// Check if a string contains only ASCII digits
    pub fn is_numeric(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_digit())
    }
    
    /// Convert a string to snake_case
    pub fn to_snake_case(s: &str) -> String {
        let mut result = String::new();
        let mut prev_is_upper = false;
        
        for (i, c) in s.char_indices() {
            if c.is_uppercase() {
                if i > 0 && !prev_is_upper {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
                prev_is_upper = true;
            } else {
                result.push(c);
                prev_is_upper = false;
            }
        }
        
        result
    }
    
    /// Convert a string to camelCase
    pub fn to_camel_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;
        
        for c in s.chars() {
            if c == '_' || c == '-' || c == ' ' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        
        result
    }
    
    /// Convert a string to PascalCase
    pub fn to_pascal_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;
        
        for c in s.chars() {
            if c == '_' || c == '-' || c == ' ' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        
        result
    }
    
    /// Convert a string to kebab-case
    pub fn to_kebab_case(s: &str) -> String {
        let snake = to_snake_case(s);
        snake.replace('_', "-")
    }
}

/// Serialization utilities
pub mod serialization {
    use super::*;
    
    /// Serialize an object to JSON
    pub fn to_json<T: Serialize>(value: &T) -> Result<String> {
        serde_json::to_string(value)
            .map_err(|e| NestGateError::Json(e.to_string()))
    }
    
    /// Serialize an object to JSON with pretty formatting
    pub fn to_json_pretty<T: Serialize>(value: &T) -> Result<String> {
        serde_json::to_string_pretty(value)
            .map_err(|e| NestGateError::Json(e.to_string()))
    }
    
    /// Deserialize an object from JSON
    pub fn from_json<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T> {
        serde_json::from_str(json)
            .map_err(|e| NestGateError::Json(e.to_string()))
    }
    
    /// Serialize an object to YAML
    pub fn to_yaml<T: Serialize>(value: &T) -> Result<String> {
        serde_yaml::to_string(value)
            .map_err(|e| NestGateError::Yaml(e.to_string()))
    }
    
    /// Deserialize an object from YAML
    pub fn from_yaml<T: for<'de> Deserialize<'de>>(yaml: &str) -> Result<T> {
        serde_yaml::from_str(yaml)
            .map_err(|e| NestGateError::Yaml(e.to_string()))
    }
}

/// Time-related utilities
pub mod time {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use crate::error::{NestGateError, Result};
    
    /// Get the current Unix timestamp in seconds
    pub fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_secs(0))
            .as_secs()
    }
    
    /// Get the current Unix timestamp in milliseconds
    pub fn current_timestamp_millis() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| Duration::from_millis(0))
            .as_millis()
    }
    
    /// Format a duration in a human-readable way
    pub fn format_duration(duration: Duration) -> String {
        let seconds = duration.as_secs();
        
        if seconds < 60 {
            return format!("{} second{}", seconds, if seconds == 1 { "" } else { "s" });
        }
        
        let minutes = seconds / 60;
        if minutes < 60 {
            return format!("{} minute{}", minutes, if minutes == 1 { "" } else { "s" });
        }
        
        let hours = minutes / 60;
        if hours < 24 {
            return format!("{} hour{}", hours, if hours == 1 { "" } else { "s" });
        }
        
        let days = hours / 24;
        format!("{} day{}", days, if days == 1 { "" } else { "s" })
    }
    
    /// Parse a timestamp from a string
    pub fn parse_timestamp(s: &str) -> Result<u64> {
        s.parse::<u64>()
            .map_err(|_| NestGateError::InvalidInput(format!("Invalid timestamp: {}", s)))
    }
}

/// System-related utilities
pub mod system {
    use std::time::Duration;
    use procfs::Current;
    use crate::error::{NestGateError, Result};
    
    /// Get the hostname
    pub fn get_hostname() -> Result<String> {
        gethostname::gethostname()
            .into_string()
            .map_err(|_| NestGateError::Internal("Failed to get hostname".to_string()))
    }
    
    /// Get the number of CPU cores
    pub fn get_cpu_count() -> usize {
        num_cpus::get()
    }
    
    /// Get the system uptime
    #[cfg(target_os = "linux")]
    pub fn get_uptime() -> Result<Duration> {
        // Use procfs::Uptime::current() to get the system uptime
        let uptime = procfs::Uptime::current()
            .map_err(|e| NestGateError::Internal(format!("Failed to get uptime: {}", e)))?;
        
        Ok(Duration::from_secs_f64(uptime.uptime))
    }
    
    /// Get the system uptime
    #[cfg(not(target_os = "linux"))]
    pub fn get_uptime() -> Result<Duration> {
        Err(NestGateError::Internal("Uptime not supported on this platform".to_string()))
    }
    
    /// Get the system memory info
    #[cfg(target_os = "linux")]
    pub fn get_memory_info() -> Result<(u64, u64)> {
        // Use procfs::Meminfo::current() to get memory information
        let meminfo = procfs::Meminfo::current()
            .map_err(|e| NestGateError::Internal(format!("Failed to get memory info: {}", e)))?;
        
        let total = meminfo.mem_total;
        let free = meminfo.mem_free + meminfo.cached + meminfo.buffers;
        
        Ok((total, free))
    }
    
    /// Get the system memory info
    #[cfg(not(target_os = "linux"))]
    pub fn get_memory_info() -> Result<(u64, u64)> {
        Err(NestGateError::Internal("Memory info not supported on this platform".to_string()))
    }
}

/// Configuration-related utilities
pub mod config {
    use super::*;
    
    /// Load a configuration file
    pub fn load_config<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
        let contents = fs::read_to_string(path)?;
        
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            serialization::from_json(&contents)
        } else {
            serialization::from_yaml(&contents)
        }
    }
    
    /// Save a configuration file
    pub fn save_config<T: Serialize>(value: &T, path: &Path) -> Result<()> {
        let contents = if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            serialization::to_json_pretty(value)?
        } else {
            serialization::to_yaml(value)?
        };
        
        fs::write(path, contents)?;
        Ok(())
    }
} 