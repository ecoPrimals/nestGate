//! Event types for clustering
//!
//! This module contains event definitions for cluster state changes,
//! node status updates, and other cluster-related events.

use serde::{Deserialize, Serialize};

use super::enums::ClusterHealthStatus;

// ==================== CLUSTER EVENTS ====================

/// Cluster event notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterEvent {
    /// Node joined the cluster
    NodeJoined(String),
    /// Node left the cluster
    NodeLeft(String),
    /// Node failed
    NodeFailed(String),
    /// Leader elected
    LeaderElected(String),
    /// Leader lost
    LeaderLost,
    /// Network partition detected
    PartitionDetected(Vec<String>),
    /// Network partition healed
    PartitionHealed,
    /// Cluster health status changed
    ClusterHealthChanged(ClusterHealthStatus),
}
