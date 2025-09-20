// Removed unused error imports
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Storage configuration settings with enhanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Cache size in bytes
    pub cache_size: u64,
    /// Maximum file size in bytes
    pub max_file_size: u64,

    /// Storage tier configurations
    pub tiers: Vec<StorageTierConfig>,

    /// Storage protocols configuration
    pub protocols: StorageProtocolsConfig,
}

/// Storage tier configuration with enhanced capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageTierConfig {
    /// Tier name
    pub name: String,
    /// Tier type (hot, warm, cold, archive)
    pub tier_type: String,

    /// Storage path

    /// Maximum capacity in bytes
    pub capacity: u64,

    /// Performance configuration
    pub performance: TierPerformanceConfig,
}

/// Tier performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPerformanceConfig {
    /// Maximum IOPS
    pub max_iops: u64,
    /// Maximum throughput in MB/s
    pub max_throughput: u64,

    /// Latency target in milliseconds
    pub latency_target: f64,
}

/// Storage protocols configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageProtocolsConfig {
    /// NFS configuration
    pub nfs: Option<NfsConfig>,
    /// SMB configuration
    pub smb: Option<SmbConfig>,

    /// iSCSI configuration
    pub iscsi: Option<IscsiConfig>,

    /// S3 configuration
    pub s3: Option<S3Config>,
}

/// NFS protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfsConfig {
    /// NFS version (3, 4, 4.1, 4.2)
    pub version: String,
    /// Export path

    /// Allowed clients
    pub allowed_clients: Vec<String>,

    /// Mount options
    pub mount_options: HashMap<String, String>,
}

/// SMB protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbConfig {
    /// SMB version (2, 3, 3.1)
    pub version: String,
    /// Share name
    pub share_name: String,

    /// Workgroup
    pub workgroup: String,

    /// Authentication method
    pub auth_method: String,
}

/// iSCSI protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IscsiConfig {
    /// Target name
    pub target_name: String,
    /// Portal address
    pub portal: String,

    /// Authentication settings
    pub auth: IscsiAuthConfig,
}

/// iSCSI authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IscsiAuthConfig {
    /// CHAP username
    pub username: Option<String>,
    /// CHAP secret
    pub secret: Option<String>,

    /// Mutual CHAP
    pub mutual_chap: bool,
}

/// S3 protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3Config {
    /// S3 endpoint
    pub endpoint: String,
    /// Bucket name
    pub bucket: String,

    /// Region
    pub region: String,

    /// Access key ID
    pub access_key_id: String,

    /// Secret access key
    pub secret_access_key: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            cache_size: 1024 * 1024 * 1024,          // 1GB
            max_file_size: 1024 * 1024 * 1024 * 100, // 100GB
            tiers: vec![],
            protocols: StorageProtocolsConfig::default(),
        }
    }
}

impl Default for TierPerformanceConfig {
    fn default() -> Self {
        Self {
            max_iops: 10000,
            max_throughput: 1000, // MB/s
            latency_target: 10.0, // ms
        }
    }
}

impl StorageConfig {
    /// Get a storage tier configuration by name
    pub const fn get_tier(&self, name: &str) -> Option<&StorageTierConfig> {
        self.tiers.iter().find(|tier| tier.name == name)
    }

    /// Get a mutable storage tier configuration by name
    #[must_use]
    pub fn get_tier_mut(&mut self, name: &str) -> Option<&mut StorageTierConfig> {
        self.tiers.iter_mut().find(|tier| tier.name == name)
    }

    /// Add a storage tier configuration
    pub fn add_tier(&mut self, tier: StorageTierConfig) {
        self.tiers.push(tier);
    }

    /// Remove a storage tier configuration by name
    #[must_use]
    pub fn remove_tier(&mut self, name: &str) -> Option<StorageTierConfig> {
        if let Some(pos) = self.tiers.iter().position(|tier| tier.name == name) {
            Some(self.tiers.remove(pos))
        } else {
            None
        }
    }

    /// Get all tier names
    pub const fn tier_names(&self) -> Vec<&str> {
        self.tiers.iter().map(|tier| tier.name.as_str()).collect()
    }

    /// Get total storage capacity across all tiers
    pub const fn total_capacity(&self) -> u64 {
        self.tiers.iter().map(|tier| tier.capacity).sum()
    }

    /// Check if a protocol is enabled
    pub const fn is_protocol_enabled(&self, protocol: &str) -> bool {
        match protocol {
            "nfs" => self.protocols.nfs.is_some(),
            "smb" => self.protocols.smb.is_some(),
            "iscsi" => self.protocols.iscsi.is_some(),
            "s3" => self.protocols.s3.is_some(),
            _ => false,
        }
    }
}

impl StorageTierConfig {
    /// Create a new storage tier configuration
        Self {
            name,
            tier_type,
            path,
            capacity,
            performance: TierPerformanceConfig::default(),
        }
    }

    /// Get the tier utilization percentage (0-100)
    pub const fn utilization_percent(&self, used_bytes: u64) -> f64 {
        if self.capacity == 0 {
            0.0
        } else {
            (f64::from(used_bytes) / self.f64::from(capacity)) * 100.0
        }
    }

    /// Get available capacity in bytes
    pub const fn available_capacity(&self, used_bytes: u64) -> u64 {
        self.capacity.saturating_sub(used_bytes)
    }

    /// Check if the tier is full (>95% utilization)
    pub const fn is_full(&self, used_bytes: u64) -> bool {
        self.utilization_percent(used_bytes) > 95.0
    }

    /// Check if the tier is nearly full (>90% utilization)
    pub const fn is_nearly_full(&self, used_bytes: u64) -> bool {
        self.utilization_percent(used_bytes) > 90.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_config_default() {
        let config = StorageConfig::default();
        assert_eq!(config.cache_size, 1024 * 1024 * 1024);
        assert_eq!(config.max_file_size, 1024 * 1024 * 1024 * 100);
        assert!(config.tiers.is_empty());
        assert!(config.protocols.nfs.is_none());
    }

    #[test]
    fn test_storage_tier_operations() {
        let mut config = StorageConfig::default();

        let tier = StorageTierConfig::new(
            "hot".to_string(),
            "hot".to_string(),
            "/mnt/hot".to_string(),
            1024 * 1024 * 1024 * 100, // 100GB
        );

        config.add_tier(tier);
        assert_eq!(config.tiers.len(), 1);
        assert!(config.get_tier("hot").is_some());
        assert!(config.get_tier("cold").is_none());

        let tier_names = config.tier_names();
        assert_eq!(tier_names, vec!["hot"]);

        let total_capacity = config.total_capacity();
        assert_eq!(total_capacity, 1024 * 1024 * 1024 * 100);

        let removed = config.remove_tier("hot");
        assert!(removed.is_some());
        assert_eq!(config.tiers.len(), 0);
    }

    #[test]
    fn test_tier_utilization() {
        let tier = StorageTierConfig::new(
            "test".to_string(),
            "hot".to_string(),
            "/mnt/test".to_string(),
            1000, // 1000 bytes total
        );

        assert_eq!(tier.utilization_percent(0), 0.0);
        assert_eq!(tier.utilization_percent(500), 50.0);
        assert_eq!(tier.utilization_percent(1000), 100.0);

        assert_eq!(tier.available_capacity(500), 500);
        assert_eq!(tier.available_capacity(1000), 0);

        assert!(!tier.is_full(900));
        assert!(tier.is_full(960));
        assert!(tier.is_nearly_full(910));
    }

    #[test]
    fn test_protocol_enabled() {
        let mut config = StorageConfig::default();

        assert!(!config.is_protocol_enabled("nfs"));
        assert!(!config.is_protocol_enabled("smb"));
        assert!(!config.is_protocol_enabled("iscsi"));
        assert!(!config.is_protocol_enabled("s3"));

        config.protocols.nfs = Some(NfsConfig {
            version: "4".to_string(),
            allowed_clients: vec!["*".to_string()],
            mount_options: HashMap::new(),
        );

        assert!(config.is_protocol_enabled("nfs"));
        assert!(!config.is_protocol_enabled("smb"));
    }
}
