/// Core types and data structures for NestGate
///
/// This module contains fundamental data types used throughout the system.
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::time::SystemTime;

use crate::Result;

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

/// Universal service trait for lightweight service components
#[async_trait]
pub trait UniversalService: Send + Sync {
    /// Service name
    fn service_name(&self) -> &str;

    /// Service version
    fn service_version(&self) -> &str;

    /// Start the service
    async fn start(&mut self) -> Result<()>;

    /// Stop the service
    async fn stop(&mut self) -> Result<()>;

    /// Check if service is healthy
    async fn is_healthy(&self) -> bool;
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
            let _deserialized: AccessPattern = serde_json::from_str(&serialized).unwrap();
            // Note: We can't use PartialEq on AccessPattern in the current setup
            // but serialization/deserialization should work
            assert!(!serialized.is_empty());
        }
    }
}
