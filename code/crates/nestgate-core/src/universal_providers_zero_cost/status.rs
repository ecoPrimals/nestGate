// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents the current status of a service including health and availability information.
pub struct ServiceStatus {
    /// Running
    pub running: bool,
    /// Replicas
    pub replicas: u32,
    /// The health status description of the service
    pub health: String,
    /// Last Updated
    pub last_updated: std::time::SystemTime,
}
impl Default for ServiceStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            running: false,
            replicas: 0,
            health: "unknown".to_string(),
            last_updated: std::time::SystemTime::now(),
        }
    }
}

/// Compute resources information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Computeresources
pub struct ComputeResources {
    /// Available Cpu
    pub available_cpu: f64,
    /// Available Memory in gigabytes
    pub available_memory_gb: f64,
    /// Active Tasks
    pub active_tasks: u32,
    /// Max Tasks
    pub max_tasks: u32,
}
