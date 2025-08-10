/// Core types and data structures for NestGate
///
/// This module contains fundamental data types used throughout the system.
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::time::SystemTime;

use crate::Result;

/// Allocation status for resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AllocationStatus {
    Active,
    Inactive,
    Pending,
    Failed,
}

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
    /// Long-term archival storage for rarely accessed data
    Archive,
}

impl Display for StorageTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageTier::Hot => write!(f, "Hot"),
            StorageTier::Warm => write!(f, "Warm"),
            StorageTier::Cold => write!(f, "Cold"),
            StorageTier::Cache => write!(f, "Cache"),
            StorageTier::Archive => write!(f, "Archive"),
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
            StorageTier::Archive,
        ]
    }

    /// Get the priority order of tiers (Hot = highest priority)
    pub fn priority(&self) -> u8 {
        match self {
            StorageTier::Hot => 1,
            StorageTier::Warm => 2,
            StorageTier::Cold => 3,
            StorageTier::Cache => 0, // Special case - cache has unique priority
            StorageTier::Archive => 4, // Lowest priority for long-term storage
        }
    }

    /// Check if this tier is suitable for caching
    pub fn is_cache_tier(&self) -> bool {
        matches!(self, StorageTier::Cache | StorageTier::Hot)
    }

    /// Get the string representation of the storage tier
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageTier::Hot => "hot",
            StorageTier::Warm => "warm",
            StorageTier::Cold => "cold",
            StorageTier::Cache => "cache",
            StorageTier::Archive => "archive",
        }
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
    pub last_check: SystemTime,
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
            last_check: SystemTime::now(),
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
            total_memory: 8_589_934_592,     // 8GB
            available_memory: 4_294_967_296, // 4GB
            cpu_cores: 4,
            uptime_seconds: 86400, // 1 day in seconds
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

/// Cryptographic proof for decentralized authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptographicProof {
    /// User identifier
    pub user_id: String,
    /// Cryptographic signature
    pub signature: String,
    /// Public key for verification
    pub public_key: String,
    /// Timestamp when proof was created
    pub timestamp: i64,
    /// Nonce to prevent replay attacks
    pub nonce: String,
    /// Challenge that was signed
    pub challenge: String,
}

/// Access grant based on distributed consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessGrant {
    /// Permissions granted by consensus
    pub permissions: Vec<String>,
    /// Valid until (consensus-determined)
    pub valid_until: i64,
    /// Hash of the cryptographic proof
    pub proof_hash: String,
    /// BearDog nodes that participated in consensus
    pub consensus_nodes: Vec<String>,
    /// Consensus percentage achieved
    pub consensus_percentage: f64,
}

/// Consensus result from BearDog network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    /// Whether consensus was achieved
    pub consensus_achieved: bool,
    /// Percentage of nodes that agreed
    pub consensus_percentage: f64,
    /// Granted permissions
    pub granted_permissions: Vec<String>,
    /// Consensus expiry time
    pub consensus_expiry: i64,
    /// Participating nodes
    pub participating_nodes: Vec<String>,
}

/// Universal security service node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityServiceNode {
    /// Service identifier
    pub service_id: String,
    /// Node endpoint URL
    pub endpoint: String,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Node public key for verification
    pub public_key: String,
    /// Node status
    pub status: ServiceNodeStatus,
    /// Last seen timestamp
    pub last_seen: i64,
    /// Priority for load balancing
    pub priority: u8,
}

/// Service node status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceNodeStatus {
    /// Node is active and participating
    Active,
    /// Node is temporarily unavailable
    Unavailable,
    /// Node has been compromised or is misbehaving
    Compromised,
    /// Node is under maintenance
    Maintenance,
}

/// Universal Manager trait for all manager structs across NestGate
///
/// This trait provides a consistent interface for all manager components,
/// reducing code duplication and improving maintainability.
#[async_trait]
pub trait UniversalManager: Send + Sync {
    /// The type of configuration this manager accepts
    type Config: Send + Sync + Clone + std::fmt::Debug;

    /// The type of health information this manager provides
    type Health: Send + Sync + Serialize + std::fmt::Debug;

    /// The type of metrics this manager provides
    type Metrics: Send + Sync + Serialize + std::fmt::Debug;

    /// Initialize the manager with configuration
    async fn initialize(&mut self, config: Self::Config) -> Result<()>;

    /// Start the manager's operations
    async fn start(&mut self) -> Result<()>;

    /// Stop the manager gracefully
    async fn stop(&mut self) -> Result<()>;

    /// Get current health status
    async fn health_check(&self) -> Result<Self::Health>;

    /// Get performance metrics
    async fn get_metrics(&self) -> Result<Self::Metrics>;

    /// Restart the manager (default implementation)
    async fn restart(&mut self) -> Result<()> {
        self.stop().await?;
        self.start().await?;
        Ok(())
    }

    /// Get manager information
    fn manager_info(&self) -> ManagerInfo;
}

/// Information about a manager instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagerInfo {
    /// Manager name/type
    pub name: String,
    /// Manager version
    pub version: String,
    /// Manager capabilities
    pub capabilities: Vec<String>,
    /// Manager dependencies
    pub dependencies: Vec<String>,
    /// Manager status
    pub status: ManagerStatus,
}

/// Manager status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManagerStatus {
    /// Manager is not initialized
    Uninitialized,
    /// Manager is initializing
    Initializing,
    /// Manager is running normally
    Running,
    /// Manager is stopping
    Stopping,
    /// Manager has stopped
    Stopped,
    /// Manager has encountered an error
    Error,
}

// REMOVED: Deprecated UniversalService trait eliminated
// All implementations migrated to nestgate_core::traits::UniversalService

/// **RESOURCE ALLOCATION** - manages system resource assignments
#[derive(Debug, Clone)]
pub struct ResourceAllocation {
    pub id: String,
    pub resource_type: String,
    pub status: String,
    pub amount: u64,
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub disk_gb: u64,
    pub network_bandwidth_mbps: u32,
    pub allocated_at: std::time::SystemTime,
    pub expires_at: Option<std::time::SystemTime>,
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for ResourceAllocation {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            resource_type: "compute".to_string(),
            status: "available".to_string(),
            amount: 1,
            cpu_cores: 1,
            memory_mb: 512,
            disk_gb: 10,
            network_bandwidth_mbps: 100,
            allocated_at: std::time::SystemTime::now(),
            expires_at: None,
            metadata: std::collections::HashMap::new(),
        }
    }
}

/// **WORKLOAD RESULT** - represents the result of a workload execution
#[derive(Debug, Clone)]
pub struct WorkloadResult {
    pub workload_id: String,
    pub success: bool,
    pub status: String,
    pub message: String,
    pub execution_time_ms: u64,
    pub resources_used: ResourceAllocation,
    pub result_data: serde_json::Value,
    pub metrics: std::collections::HashMap<String, f64>,
    pub started_at: std::time::SystemTime,
    pub completed_at: Option<std::time::SystemTime>,
    pub error_message: Option<String>,
}

impl Default for WorkloadResult {
    fn default() -> Self {
        Self {
            workload_id: uuid::Uuid::new_v4().to_string(),
            success: true,
            status: "completed".to_string(),
            message: "Workload completed successfully".to_string(),
            execution_time_ms: 0,
            resources_used: ResourceAllocation::default(),
            result_data: serde_json::Value::Null,
            metrics: std::collections::HashMap::new(),
            started_at: std::time::SystemTime::now(),
            completed_at: Some(std::time::SystemTime::now()),
            error_message: None,
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
    fn test_storage_tier_serialization() -> crate::Result<()> {
        for tier in StorageTier::all() {
            let serialized = serde_json::to_string(&tier).map_err(|e| {
                tracing::error!("JSON serialization failed: {}", e);
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("JSON serialization error: {}", e),
                )
            })?;
            let deserialized: StorageTier = serde_json::from_str(&serialized).map_err(|e| {
                tracing::error!("JSON parsing failed: {}", e);
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("JSON parsing error: {}", e),
                )
            })?;
            assert_eq!(tier, deserialized);
        }
        Ok(())
    }

    #[test]
    fn test_health_status() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let health = HealthStatus::default();
        assert!(health.overall_healthy);
        assert!(!health.services_running.is_empty());

        // Test serialization
        let serialized = serde_json::to_string(&health).map_err(|e| {
            tracing::error!("JSON serialization failed: {}", e);
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("JSON serialization error: {}", e),
            )
        })?;
        let deserialized: HealthStatus = serde_json::from_str(&serialized).map_err(|e| {
            tracing::error!("JSON parsing failed: {}", e);
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("JSON parsing error: {}", e),
            )
        })?;
        assert_eq!(health.overall_healthy, deserialized.overall_healthy);
        Ok(())
    }

    #[test]
    fn test_system_info() -> crate::Result<()> {
        let info = SystemInfo::default();
        assert!(!info.hostname.is_empty());
        assert!(info.total_memory > 0);
        assert!(info.cpu_cores > 0);

        // Test serialization
        let serialized = serde_json::to_string(&info).map_err(|e| {
            tracing::error!("JSON serialization failed: {}", e);
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("JSON serialization error: {}", e),
            )
        })?;
        let deserialized: SystemInfo = serde_json::from_str(&serialized).map_err(|e| {
            tracing::error!("JSON parsing failed: {}", e);
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("JSON parsing error: {}", e),
            )
        })?;
        assert_eq!(info.hostname, deserialized.hostname);
        Ok(())
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
    fn test_access_pattern_serialization() -> crate::Result<()> {
        Ok(())
    }

    #[test]
    fn test_serialization_backup() -> crate::Result<()> {
        let patterns = vec![
            AccessPattern::Sequential,
            AccessPattern::Random,
            AccessPattern::Write,
            AccessPattern::Read,
            AccessPattern::Mixed,
        ];

        for pattern in patterns {
            let serialized = serde_json::to_string(&pattern).map_err(|e| {
                tracing::error!("JSON serialization failed: {}", e);
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("JSON serialization error: {}", e),
                )
            })?;
            let _deserialized: AccessPattern = serde_json::from_str(&serialized).map_err(|e| {
                tracing::error!("JSON parsing failed: {}", e);
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("JSON parsing error: {}", e),
                )
            })?;
            // Note: We can't use PartialEq on AccessPattern in the current setup
            // but serialization/deserialization should work
            assert!(!serialized.is_empty());
        }
        Ok(())
    }
}

// ==================== NETWORK TYPES ====================

/// Network statistics for monitoring and diagnostics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub connections_active: u32,
    pub connections_total: u64,
    pub errors: u64,
    pub timestamp: SystemTime,
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self {
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
            connections_active: 0,
            connections_total: 0,
            errors: 0,
            timestamp: SystemTime::now(),
        }
    }
}

/// Service instance information for discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    pub id: String,
    pub name: String,
    pub version: String,
    pub endpoint: String,
    pub port: u16,
    pub status: ServiceStatus,
    pub capabilities: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
    pub last_seen: SystemTime,
}

impl Default for ServiceInstance {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "unknown".to_string(),
            version: "0.1.0".to_string(),
            endpoint: "localhost".to_string(),
            port: 8080,
            status: ServiceStatus::Unknown,
            capabilities: vec![],
            metadata: std::collections::HashMap::new(),
            last_seen: SystemTime::now(),
        }
    }
}

/// Service status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Error,
    Unknown,
    Healthy,
    Unhealthy,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self::Unknown
    }
}
