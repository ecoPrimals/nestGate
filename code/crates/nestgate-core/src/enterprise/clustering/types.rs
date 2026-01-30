//! Type definitions for clustering
//!
//! This module contains all struct definitions used in the clustering system,
//! including nodes, cluster state, health information, and status types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::SystemTime;

use super::enums::*;

// ==================== NODE TYPES ====================

/// Individual cluster node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNode {
    /// Node identifier
    pub node_id: String,
    /// Endpoint
    pub endpoint: SocketAddr,
    /// Status
    pub status: NodeStatus,
    /// Role
    pub role: NodeRole,
    /// Last Heartbeat
    pub last_heartbeat: SystemTime,
    /// Additional metadata key-value pairs
    pub metadata: NodeMetadata,
    /// Capabilities
    pub capabilities: Vec<NodeCapability>,
}

/// Node metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    /// Version
    pub version: String,
    /// Started At
    pub started_at: SystemTime,
    /// Region
    pub region: Option<String>,
    /// Zone
    pub zone: Option<String>,
    /// Weight
    pub weight: u32,
    /// Tags
    pub tags: HashMap<String, String>,
    /// Resources
    pub resources: NodeResources,
}

/// Node resource information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResources {
    /// Cpu Cores
    pub cpu_cores: u32,
    /// Memory in gigabytes
    pub memory_gb: u32,
    /// Storage in gigabytes
    pub storage_gb: u64,
    /// Network Bandwidth Mbps
    pub network_bandwidth_mbps: u32,
    /// Load Average
    pub load_average: f64,
    /// Memory Usage Percent
    pub memory_usage_percent: f64,
    /// Storage Usage Percent
    pub storage_usage_percent: f64,
}

/// Discovered node from discovery process
#[derive(Debug, Clone)]
pub struct DiscoveredNode {
    /// Node identifier
    pub node_id: String,
    /// Endpoint
    pub endpoint: SocketAddr,
    /// Discovered At
    pub discovered_at: SystemTime,
    /// Capabilities
    pub capabilities: Vec<NodeCapability>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

/// Heartbeat information
#[derive(Debug, Clone)]
pub struct HeartbeatInfo {
    /// Last Received
    pub last_received: SystemTime,
    /// Count of missed
    pub missed_count: u32,
    /// Rtt Ms
    pub rtt_ms: u64,
}

// ==================== CLUSTER STATE TYPES ====================

/// Overall cluster state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterState {
    /// Cluster identifier
    pub cluster_id: String,
    /// Nodes
    pub nodes: HashMap<String, ClusterNode>,
    /// Leader identifier
    pub leader_id: Option<String>,
    /// Election Term
    pub election_term: u64,
    /// Cluster Health
    pub cluster_health: ClusterHealth,
    /// Partition Info
    pub partition_info: PartitionInfo,
    /// Last Updated
    pub last_updated: SystemTime,
}

/// Cluster health assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterHealth {
    /// Overall Status
    pub overall_status: ClusterHealthStatus,
    /// Active Nodes
    pub active_nodes: u32,
    /// Failed Nodes
    pub failed_nodes: u32,
    /// Degraded Nodes
    pub degraded_nodes: u32,
    /// Quorum Available
    pub quorum_available: bool,
    /// Leader Available
    pub leader_available: bool,
    /// Data Consistency
    pub data_consistency: ConsistencyStatus,
}

/// Partition information for network splits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionInfo {
    /// Partitions
    pub partitions: Vec<Partition>,
    /// Majority Partition
    pub majority_partition: Option<String>,
    /// Split Brain Detected
    pub split_brain_detected: bool,
}

/// Network partition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partition {
    /// Partition identifier
    pub partition_id: String,
    /// Nodes
    pub nodes: Vec<String>,
    /// Whether this has leader
    pub has_leader: bool,
    /// Size of quorum
    pub quorum_size: u32,
}

// ==================== STATUS TYPES ====================

/// Cluster status for external reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterStatus {
    /// Cluster name
    pub cluster_name: String,
    /// Cluster identifier
    pub cluster_id: String,
    /// Total Nodes
    pub total_nodes: u32,
    /// Active Nodes
    pub active_nodes: u32,
    /// Leader identifier
    pub leader_id: Option<String>,
    /// Local Node identifier
    pub local_node_id: String,
    /// Local Node Role
    pub local_node_role: NodeRole,
    /// Cluster Health
    pub cluster_health: ClusterHealthStatus,
    /// Quorum Available
    pub quorum_available: bool,
    /// Last Updated
    pub last_updated: SystemTime,
}
