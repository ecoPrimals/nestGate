// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **FEDERATION TYPES**
//!
//! Federation join, leave, sync, and heartbeat payload types.

use serde::{Deserialize, Serialize};

use super::services::ClusterHealth;

/// Federation Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationStatus {
    /// Cluster ID
    pub cluster_id: String,
    /// Cluster health
    pub health: ClusterHealth,
    /// Member count
    pub member_count: usize,
}

/// Federation Join Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationJoinPayload {
    /// Node ID
    pub node_id: String,
    /// Node endpoint
    pub endpoint: String,
    /// Node capabilities
    pub capabilities: Vec<String>,
}

/// Federation Leave Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationLeavePayload {
    /// Node ID
    pub node_id: String,
}

/// Federation Sync Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationSyncPayload {
    /// Sync type
    pub sync_type: FederationSyncType,
}

/// Federation Sync Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FederationSyncType {
    Full,
    Incremental,
    Metadata,
}

/// Federation Heartbeat Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationHeartbeatPayload {
    /// Node ID
    pub node_id: String,
    /// Timestamp
    pub timestamp: std::time::SystemTime,
}
