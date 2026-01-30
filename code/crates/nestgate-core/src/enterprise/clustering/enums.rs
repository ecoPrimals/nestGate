//! Enum definitions for clustering
//!
//! This module contains all enum types used in the clustering system,
//! including node status, roles, capabilities, and health status.

use serde::{Deserialize, Serialize};

// ==================== NODE ENUMS ====================

/// Node status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeStatus {
    /// Node is starting up
    Starting,
    /// Node is active and healthy
    Active,
    /// Node is degraded but functional
    Degraded,
    /// Node is unhealthy
    Unhealthy,
    /// Node is leaving the cluster
    Leaving,
    /// Node has failed
    Failed,
}

/// Node role in cluster
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeRole {
    /// Leader node
    Leader,
    /// Follower node
    Follower,
    /// Candidate for leadership
    Candidate,
    /// Observer (read-only)
    Observer,
}

/// Node capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeCapability {
    /// Storage capability
    Storage,
    /// Compute capability
    Compute,
    /// Gateway capability
    Gateway,
    /// Monitoring capability
    Monitoring,
    /// Analytics capability
    Analytics,
    /// Backup capability
    Backup,
}

// ==================== CLUSTER HEALTH ENUMS ====================

/// Cluster health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterHealthStatus {
    /// All nodes healthy
    Healthy,
    /// Some degradation
    Degraded,
    /// Critical issues
    Critical,
    /// Cluster has failed
    Failed,
}

/// Data consistency status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsistencyStatus {
    /// Data is consistent
    Consistent,
    /// Data is inconsistent
    Inconsistent,
    /// Repairing inconsistencies
    Repairing,
    /// Status unknown
    Unknown,
}

// ==================== ELECTION ENUMS ====================

/// Election state
#[derive(Debug, Clone, PartialEq)]
pub enum ElectionState {
    /// Following the leader
    Follower,
    /// Candidate for leadership
    Candidate,
    /// Current leader
    Leader,
}
