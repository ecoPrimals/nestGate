// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **ORCHESTRATOR TYPES**
//!
//! Orchestrator routing, service registration, and load balancing types.

use serde::{Deserialize, Serialize};

use super::services::ServiceInfo;

/// Orchestrator Route Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorRoutePayload {
    /// Route type
    pub route_type: RouteType,
    /// Target node
    pub target_node: Option<String>,
}

/// Route Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RouteType {
    Direct,
    Broadcast,
    Multicast(Vec<String>),
}

/// Service Registration Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationPayload {
    /// Service info
    pub service: ServiceInfo,
}

/// Service Discovery Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryPayload {
    /// Service type
    pub service_type: Option<String>,
}

/// Load Balancing Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingPayload {
    /// Service type
    pub service_type: String,
    /// Load balancing info
    pub info: LoadBalancingInfo,
}

/// Load Balancing Algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
}

/// Load Balancing Info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingInfo {
    /// Algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Node weights
    pub node_weights: std::collections::HashMap<String, f64>,
}
