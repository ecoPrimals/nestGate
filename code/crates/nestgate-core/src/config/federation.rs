// Removed unused error imports
use serde::{Deserialize, Serialize};
use uuid;

/// MCP integration configuration (from Phase 1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// Enable MCP integration
    pub enabled: bool,

    /// Cluster endpoint
    pub cluster_endpoint: String,

    /// Node ID
    pub node_id: String,

    /// Enable federation
    pub federation_enabled: bool,

    /// MCP capabilities
    pub capabilities: McpCapabilitiesConfig,
}

/// MCP capabilities configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpCapabilitiesConfig {
    /// Supported storage protocols
    pub storage_protocols: Vec<String>,

    /// Supported storage tiers
    pub storage_tiers: Vec<String>,

    /// Maximum volume size
    pub max_volume_size: u64,

    /// Maximum number of volumes
    pub max_volumes: u32,
}

/// Federation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationConfig {
    /// Enable federation
    pub enabled: bool,

    /// Cluster name
    pub cluster_name: String,

    /// Federation mode (standalone, leader, follower)
    pub mode: String,

    /// Peer nodes
    pub peers: Vec<String>,

    /// Heartbeat interval in seconds
    pub heartbeat_interval: u64,
}

impl Default for McpConfig {
    fn default() -> Self {
        // Generate dynamic node ID instead of hardcoding
        let node_id = format!(
            "nestgate-{}",
            &uuid::Uuid::new_v4().simple().to_string()[..8]
        );

        Self {
            enabled: false, // Disabled by default - orchestration service manages MCP
            cluster_endpoint: std::env::var("NESTGATE_CLUSTER_ENDPOINT")
                // SOVEREIGNTY FIX: Use environment-based cluster endpoint discovery
            .unwrap_or_else(|_| std::env::var("NESTGATE_CLUSTER_ENDPOINT")
                .unwrap_or_else(|_| "dynamic://cluster-capability".to_string())),
            node_id,
            federation_enabled: false,
            capabilities: McpCapabilitiesConfig::default(),
        }
    }
}

impl Default for McpCapabilitiesConfig {
    fn default() -> Self {
        Self {
            storage_protocols: vec!["nfs".to_string(), "smb".to_string(), "s3".to_string()],
            storage_tiers: vec!["hot".to_string(), "warm".to_string(), "cold".to_string()],
            max_volume_size: 1024 * 1024 * 1024 * 1024, // 1TB
            max_volumes: 1000,
        }
    }
}

impl Default for FederationConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default - orchestration service manages federation
            cluster_name: "".to_string(), // Empty - orchestration service provides cluster name
            mode: "standalone".to_string(), // Default to standalone
            peers: vec![],  // Empty - orchestration service discovers peers
            heartbeat_interval: 30,
        }
    }
}

impl McpConfig {
    /// Check if MCP is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if federation is enabled
    pub fn is_federation_enabled(&self) -> bool {
        self.federation_enabled
    }

    /// Get cluster endpoint
    pub fn cluster_endpoint(&self) -> &str {
        &self.cluster_endpoint
    }

    /// Get node ID
    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Validate MCP configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled {
            if self.cluster_endpoint.is_empty() {
                return Err("Cluster endpoint cannot be empty when MCP is enabled".to_string());
            }
            if self.node_id.is_empty() {
                return Err("Node ID cannot be empty when MCP is enabled".to_string());
            }
            // Validate capabilities
            self.capabilities.validate()?;
        }
        Ok(())
    }
}

impl McpCapabilitiesConfig {
    /// Check if a storage protocol is supported
    pub fn supports_protocol(&self, protocol: &str) -> bool {
        self.storage_protocols.contains(&protocol.to_string())
    }

    /// Check if a storage tier is supported
    pub fn supports_tier(&self, tier: &str) -> bool {
        self.storage_tiers.contains(&tier.to_string())
    }

    /// Add a storage protocol
    pub fn add_protocol(&mut self, protocol: String) {
        if !self.storage_protocols.contains(&protocol) {
            self.storage_protocols.push(protocol);
        }
    }

    /// Remove a storage protocol
    pub fn remove_protocol(&mut self, protocol: &str) {
        self.storage_protocols.retain(|p| p != protocol);
    }

    /// Add a storage tier
    pub fn add_tier(&mut self, tier: String) {
        if !self.storage_tiers.contains(&tier) {
            self.storage_tiers.push(tier);
        }
    }

    /// Remove a storage tier
    pub fn remove_tier(&mut self, tier: &str) {
        self.storage_tiers.retain(|t| t != tier);
    }

    /// Validate capabilities configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.max_volume_size == 0 {
            return Err("Max volume size must be greater than 0".to_string());
        }

        if self.max_volumes == 0 {
            return Err("Max volumes must be greater than 0".to_string());
        }

        if self.storage_protocols.is_empty() {
            return Err("At least one storage protocol must be supported".to_string());
        }

        if self.storage_tiers.is_empty() {
            return Err("At least one storage tier must be supported".to_string());
        }
        Ok(())
    }
}

impl FederationConfig {
    /// Check if federation is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get cluster name
    pub fn cluster_name(&self) -> &str {
        &self.cluster_name
    }

    /// Get federation mode
    pub fn mode(&self) -> &str {
        &self.mode
    }

    /// Get peer nodes
    pub fn peers(&self) -> &[String] {
        &self.peers
    }

    /// Add a peer node
    pub fn add_peer(&mut self, peer: String) {
        if !self.peers.contains(&peer) {
            self.peers.push(peer);
        }
    }

    /// Remove a peer node
    pub fn remove_peer(&mut self, peer: &str) {
        self.peers.retain(|p| p != peer);
    }

    /// Check if running in standalone mode
    pub fn is_standalone(&self) -> bool {
        self.mode == "standalone"
    }

    /// Check if running in leader mode
    pub fn is_leader(&self) -> bool {
        self.mode == "leader"
    }

    /// Check if running in follower mode
    pub fn is_follower(&self) -> bool {
        self.mode == "follower"
    }

    /// Validate federation configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled {
            if self.cluster_name.is_empty() {
                return Err("Cluster name cannot be empty when federation is enabled".to_string());
            }

            if !["standalone", "leader", "follower"].contains(&self.mode.as_str()) {
                return Err(
                    "Federation mode must be one of: standalone, leader, follower".to_string(),
                );
            }

            if self.heartbeat_interval == 0 {
                return Err("Heartbeat interval must be greater than 0".to_string());
            }

            if self.mode != "standalone" && self.peers.is_empty() {
                return Err("Peer nodes must be configured for non-standalone mode".to_string());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_config_default() {
        let config = McpConfig::default();
        assert!(!config.is_enabled());
        assert!(!config.is_federation_enabled());
        assert!(!config.node_id().is_empty());
        // SOVEREIGNTY FIX: Updated test to use dynamic endpoint pattern
        assert!(
            config.cluster_endpoint().contains("cluster-capability")
                || config.cluster_endpoint().contains("localhost")
        );
    }

    #[test]
    fn test_mcp_capabilities_default() {
        let capabilities = McpCapabilitiesConfig::default();
        assert!(capabilities.supports_protocol("nfs"));
        assert!(capabilities.supports_protocol("smb"));
        assert!(capabilities.supports_protocol("s3"));
        assert!(!capabilities.supports_protocol("unknown"));

        assert!(capabilities.supports_tier("hot"));
        assert!(capabilities.supports_tier("warm"));
        assert!(capabilities.supports_tier("cold"));
        assert!(!capabilities.supports_tier("unknown"));
    }

    #[test]
    fn test_federation_config_default() {
        let config = FederationConfig::default();
        assert!(!config.is_enabled());
        assert!(config.is_standalone());
        assert!(!config.is_leader());
        assert!(!config.is_follower());
        assert_eq!(config.cluster_name(), "");
        assert_eq!(config.peers().len(), 0);
    }

    #[test]
    fn test_mcp_capabilities_operations() {
        let mut capabilities = McpCapabilitiesConfig::default();

        // Test adding protocol
        capabilities.add_protocol("iscsi".to_string());
        assert!(capabilities.supports_protocol("iscsi"));

        // Test removing protocol
        capabilities.remove_protocol("nfs");
        assert!(!capabilities.supports_protocol("nfs"));

        // Test adding tier
        capabilities.add_tier("archive".to_string());
        assert!(capabilities.supports_tier("archive"));

        // Test removing tier
        capabilities.remove_tier("hot");
        assert!(!capabilities.supports_tier("hot"));
    }

    #[test]
    fn test_federation_config_operations() {
        let mut config = FederationConfig::default();

        // Test adding peer
        config.add_peer("node1.example.com".to_string());
        assert_eq!(config.peers().len(), 1);
        assert!(config.peers().contains(&"node1.example.com".to_string()));

        // Test removing peer
        config.remove_peer("node1.example.com");
        assert_eq!(config.peers().len(), 0);
    }

    #[test]
    fn test_mcp_config_validation() {
        let mut config = McpConfig::default();

        // Disabled config should validate
        assert!(config.validate().is_ok());

        // Enable MCP
        config.enabled = true;
        assert!(config.validate().is_ok());

        // Empty cluster endpoint should fail
        config.cluster_endpoint = "".to_string();
        assert!(config.validate().is_err());

        // Empty node ID should fail
        // SOVEREIGNTY FIX: Use environment-based endpoint in tests
        config.cluster_endpoint =
            std::env::var("TEST_CLUSTER_ENDPOINT").unwrap_or_else(|_| "localhost:8080".to_string());
        config.node_id = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_federation_config_validation() {
        let mut config = FederationConfig::default();

        // Disabled config should validate
        assert!(config.validate().is_ok());

        // Enable federation
        config.enabled = true;
        config.cluster_name = "test-cluster".to_string();
        config.mode = "standalone".to_string();
        assert!(config.validate().is_ok());

        // Empty cluster name should fail
        config.cluster_name = "".to_string();
        assert!(config.validate().is_err());

        // Invalid mode should fail
        config.cluster_name = "test-cluster".to_string();
        config.mode = "invalid".to_string();
        assert!(config.validate().is_err());

        // Leader mode without peers should fail
        config.mode = "leader".to_string();
        assert!(config.validate().is_err());

        // Add peer and it should validate
        config.add_peer("node1.example.com".to_string());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_capabilities_validation() {
        let mut capabilities = McpCapabilitiesConfig::default();

        // Valid configuration should pass
        assert!(capabilities.validate().is_ok());

        // Zero max volume size should fail
        capabilities.max_volume_size = 0;
        assert!(capabilities.validate().is_err());

        // Zero max volumes should fail
        capabilities.max_volume_size = 1024;
        capabilities.max_volumes = 0;
        assert!(capabilities.validate().is_err());

        // Empty protocols should fail
        capabilities.max_volumes = 1000;
        capabilities.storage_protocols.clear();
        assert!(capabilities.validate().is_err());

        // Empty tiers should fail
        capabilities.storage_protocols.push("nfs".to_string());
        capabilities.storage_tiers.clear();
        assert!(capabilities.validate().is_err());
    }
}
