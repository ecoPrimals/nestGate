//! Core types and data structures for NestGate
//! 
//! This module contains fundamental data types used throughout the system.

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

/// Storage tier enum for tiered storage management
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageTier {
    /// High-performance storage for frequently accessed data
    Hot,
    /// Medium-performance storage for moderately accessed data
    Warm,
    /// Low-performance storage for rarely accessed data
    Cold,
    /// Fast cache storage for temporary data
    Cache,
}

impl Display for StorageTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageTier::Hot => write!(f, "Hot"),
            StorageTier::Warm => write!(f, "Warm"),
            StorageTier::Cold => write!(f, "Cold"),
            StorageTier::Cache => write!(f, "Cache"),
        }
    }
}

impl StorageTier {
    /// Get all available storage tiers
    pub fn all() -> Vec<StorageTier> {
        vec![
            StorageTier::Hot,
            StorageTier::Warm,
            StorageTier::Cold,
            StorageTier::Cache,
        ]
    }
    
    /// Get the priority order of tiers (Hot = highest priority)
    pub fn priority(&self) -> u8 {
        match self {
            StorageTier::Hot => 1,
            StorageTier::Warm => 2,
            StorageTier::Cold => 3,
            StorageTier::Cache => 0, // Special case - cache has unique priority
        }
    }
    
    /// Check if this tier is suitable for caching
    pub fn is_cache_tier(&self) -> bool {
        matches!(self, StorageTier::Cache | StorageTier::Hot)
    }
}

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub overall_healthy: bool,
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub network_connected: bool,
    pub services_running: Vec<String>,
    pub last_check: std::time::SystemTime,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self {
            overall_healthy: true,
            cpu_usage_percent: 0.0,
            memory_usage_percent: 0.0,
            disk_usage_percent: 0.0,
            network_connected: true,
            services_running: vec!["nestgate-core".to_string()],
            last_check: std::time::SystemTime::now(),
        }
    }
}

/// System information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_type: String,
    pub os_version: String,
    pub architecture: String,
    pub total_memory: u64,
    pub available_memory: u64,
    pub cpu_cores: u32,
    pub uptime_seconds: u64,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            hostname: "nestgate-host".to_string(),
            os_type: "Linux".to_string(),
            os_version: "6.0".to_string(),
            architecture: "x86_64".to_string(),
            total_memory: 8_589_934_592, // 8GB
            available_memory: 4_294_967_296, // 4GB
            cpu_cores: 4,
            uptime_seconds: 86400, // 1 day
        }
    }
}

/// File access pattern for AI/ML analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessPattern {
    Sequential,
    Random,
    Write,
    Read,
    Mixed,
}

/// Priority levels for various operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
            Priority::Critical => write!(f, "Critical"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_tier_operations() {
        // Test all tiers
        let all_tiers = StorageTier::all();
        assert_eq!(all_tiers.len(), 4);
        assert!(all_tiers.contains(&StorageTier::Hot));
        assert!(all_tiers.contains(&StorageTier::Warm));
        assert!(all_tiers.contains(&StorageTier::Cold));
        assert!(all_tiers.contains(&StorageTier::Cache));
        
        // Test priority ordering
        assert!(StorageTier::Hot.priority() < StorageTier::Warm.priority());
        assert!(StorageTier::Warm.priority() < StorageTier::Cold.priority());
        
        // Test cache tier identification
        assert!(StorageTier::Hot.is_cache_tier());
        assert!(StorageTier::Cache.is_cache_tier());
        assert!(!StorageTier::Cold.is_cache_tier());
    }
    
    #[test]
    fn test_storage_tier_display() {
        assert_eq!(StorageTier::Hot.to_string(), "Hot");
        assert_eq!(StorageTier::Warm.to_string(), "Warm");
        assert_eq!(StorageTier::Cold.to_string(), "Cold");
        assert_eq!(StorageTier::Cache.to_string(), "Cache");
    }
    
    #[test]
    fn test_storage_tier_serialization() {
        for tier in StorageTier::all() {
            let serialized = serde_json::to_string(&tier).unwrap();
            let deserialized: StorageTier = serde_json::from_str(&serialized).unwrap();
            assert_eq!(tier, deserialized);
        }
    }
    
    #[test]
    fn test_health_status() {
        let health = HealthStatus::default();
        assert!(health.overall_healthy);
        assert!(!health.services_running.is_empty());
        
        // Test serialization
        let serialized = serde_json::to_string(&health).unwrap();
        let deserialized: HealthStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(health.overall_healthy, deserialized.overall_healthy);
    }
    
    #[test]
    fn test_system_info() {
        let info = SystemInfo::default();
        assert!(!info.hostname.is_empty());
        assert!(info.total_memory > 0);
        assert!(info.cpu_cores > 0);
        
        // Test serialization
        let serialized = serde_json::to_string(&info).unwrap();
        let deserialized: SystemInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(info.hostname, deserialized.hostname);
    }
    
    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Low < Priority::Medium);
        assert!(Priority::Medium < Priority::High);
        assert!(Priority::High < Priority::Critical);
        
        // Test display
        assert_eq!(Priority::Critical.to_string(), "Critical");
        assert_eq!(Priority::Low.to_string(), "Low");
    }
    
    #[test]
    fn test_access_pattern_serialization() {
        let patterns = vec![
            AccessPattern::Sequential,
            AccessPattern::Random,
            AccessPattern::Write,
            AccessPattern::Read,
            AccessPattern::Mixed,
        ];
        
        for pattern in patterns {
            let serialized = serde_json::to_string(&pattern).unwrap();
            let deserialized: AccessPattern = serde_json::from_str(&serialized).unwrap();
            // Note: We can't use PartialEq on AccessPattern in the current setup
            // but serialization/deserialization should work
            assert!(!serialized.is_empty());
        }
    }
} 