// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Health and metrics types for the consolidated canonical adapter
//!
//! This module contains structures for tracking adapter health, performance
//! statistics, and resource requirements.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

// ==================== RESOURCE TYPES ====================

/// Resource requirements for a capability
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceRequirements {
    /// CPU cores required
    pub cpu_cores: Option<u32>,
    /// Memory in megabytes
    pub memory_mb: Option<u64>,
    /// Storage in gigabytes
    pub storage_gb: Option<u64>,
    /// Network bandwidth
    pub network_bandwidth: Option<u64>,
}

// ==================== HEALTH MONITORING ====================

/// Health status of the adapter
#[derive(Debug, Clone)]
pub struct AdapterHealthStatus {
    /// Whether the adapter is healthy
    pub healthy: bool,
    /// Last health check timestamp
    pub last_check: SystemTime,
    /// Health check details
    pub details: HashMap<String, String>,
    /// Count of successful operations
    pub successful_operations: u64,
    /// Count of failed operations
    pub failed_operations: u64,
    /// Average response time
    pub response_time_avg: Duration,
}

impl Default for AdapterHealthStatus {
    fn default() -> Self {
        Self {
            healthy: true,
            last_check: SystemTime::now(),
            details: HashMap::new(),
            successful_operations: 0,
            failed_operations: 0,
            response_time_avg: Duration::from_millis(0),
        }
    }
}

// ==================== PERFORMANCE STATISTICS ====================

/// Performance statistics for the adapter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterStats {
    /// Service identifier
    pub service_id: String,
    /// Active connections
    pub active_connections: u32,
    /// Total requests processed
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Average response time
    pub average_response_time: Duration,
    /// Uptime duration
    pub uptime: Duration,
    /// Last updated timestamp
    pub last_updated: SystemTime,
}

impl Default for AdapterStats {
    fn default() -> Self {
        Self {
            service_id: Uuid::new_v4().to_string(),
            active_connections: 0,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_response_time: Duration::from_millis(0),
            uptime: Duration::from_secs(0),
            last_updated: SystemTime::now(),
        }
    }
}
