//! # Cluster Configuration Types
//!
//! Configuration structures for enterprise clustering.
//!
//! **Phase 3: Smart Refactoring** - Extracted from monolithic `clustering.rs` (Jan 30, 2026)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;

/// Cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Cluster name
    pub cluster_name: String,
    /// Node identifier
    pub node_id: String,
    /// Bind Endpoint
    pub bind_endpoint: SocketAddr,
    /// Nodes
    pub nodes: Vec<ClusterNodeConfig>,
    /// Election Timeout Ms
    pub election_timeout_ms: u64,
    /// Heartbeat Interval Ms
    pub heartbeat_interval_ms: u64,
    /// Max Missed Heartbeats
    pub max_missed_heartbeats: u32,
    /// Discovery Enabled
    pub discovery_enabled: bool,
    /// Discovery Multicast Endpoint
    pub discovery_multicast_endpoint: String,
    /// Discovery Port
    pub discovery_port: u16,
    /// Encryption Enabled
    pub encryption_enabled: bool,
    /// Cluster Secret
    pub cluster_secret: Option<String>,
}

/// Individual cluster node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNodeConfig {
    /// Node identifier
    pub node_id: String,
    /// Endpoint
    pub endpoint: SocketAddr,
    /// Region
    pub region: Option<String>,
    /// Zone
    pub zone: Option<String>,
    /// Weight
    pub weight: u32,
    /// Tags
    pub tags: HashMap<String, String>,
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            cluster_name: "nestgate-cluster".to_string(),
            node_id: uuid::Uuid::new_v4().to_string(),
            bind_endpoint: "127.0.0.1:9000".parse().expect("Valid bind endpoint"),
            nodes: vec![],
            election_timeout_ms: 5000,
            heartbeat_interval_ms: 1000,
            max_missed_heartbeats: 3,
            discovery_enabled: true,
            discovery_multicast_endpoint: "239.255.255.250:9001".to_string(),
            discovery_port: 9001,
            encryption_enabled: false,
            cluster_secret: None,
        }
    }
}
