//! Component types for clustering
//!
//! This module contains internal component structs used by ClusterManager,
//! including leader election, node discovery, and heartbeat management.

use std::collections::{HashMap, HashSet};
use std::time::{Duration, SystemTime};

use super::enums::ElectionState;
use super::types::{DiscoveredNode, HeartbeatInfo};

// ==================== CLUSTER COMPONENTS ====================

/// Leader election manager
pub struct LeaderElection {
    pub(super) current_term: u64,
    pub(super) voted_for: Option<String>,
    pub(super) election_timeout: Duration,
    pub(super) last_election: Option<SystemTime>,
    pub(super) votes_received: HashSet<String>,
    pub(super) election_state: ElectionState,
}

/// Node discovery manager
pub struct NodeDiscovery {
    pub(super) discovery_enabled: bool,
    pub(super) multicast_endpoint: String,
    pub(super) discovery_port: u16,
    pub(super) discovered_nodes: HashMap<String, DiscoveredNode>,
    pub(super) last_discovery: SystemTime,
}

/// Heartbeat manager
pub struct HeartbeatManager {
    pub(super) heartbeat_interval: Duration,
    pub(super) max_missed_heartbeats: u32,
    pub(super) node_heartbeats: HashMap<String, HeartbeatInfo>,
    pub(super) last_heartbeat_sent: Option<SystemTime>,
}
