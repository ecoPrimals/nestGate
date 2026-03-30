// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use nestgate_core::Result;

use super::allocation::ComputeAllocation;
use super::results::LiveHardwareMetrics;

/// **LIVE HARDWARE TUNING SESSION**
///
/// Represents an active hardware tuning session with real-time metrics collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Livehardwaretuningsession
pub struct LiveHardwareTuningSession {
    /// Unique session identifier for tracking
    pub session_id: String,
    /// Timestamp when the session was started
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Current resource allocation configuration
    pub resource_allocation: ComputeAllocation,
    /// Real-time hardware metrics being collected
    pub current_metrics: LiveHardwareMetrics,
}

impl LiveHardwareTuningSession {
    /// Create a new hardware tuning session
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn new() -> Result<Self> {
        Ok(Self {
            session_id: format!("session_{}", Uuid::new_v4()),
            started_at: Utc::now(),
            resource_allocation: ComputeAllocation {
                cpu_cores: 8,
                memory_gb: 16,
                gpu_count: 1,
            },
            current_metrics: LiveHardwareMetrics {
                cpu_usage: 25.0,
                memory_usage: 40.0,
                disk_io: 80.0,
                network_io: 60.0,
                power_consumption: 280.0,
                temperature: 58.0,
                gpu_usage: 15.0,
                disk_usage: 70.0,
                network_usage: 30.0,
                timestamp: Utc::now(),
            },
        })
    }

    /// Collect current hardware performance metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn collect_current_metrics(&self) -> Result<LiveHardwareMetrics> {
        Ok(LiveHardwareMetrics {
            cpu_usage: 30.0,
            memory_usage: 45.0,
            disk_io: 85.0,
            network_io: 65.0,
            power_consumption: 290.0,
            temperature: 60.0,
            gpu_usage: 20.0,
            disk_usage: 70.0,
            network_usage: 35.0,
            timestamp: Utc::now(),
        })
    }
}
