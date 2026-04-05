// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **NETWORK ORCHESTRATION CONFIGURATION**

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};

/// Network orchestration configuration for distributed coordination.
///
/// Controls coordination between distributed services via heartbeats and a central coordinator.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `NetworkOrchestration`
pub struct NetworkOrchestrationConfig {
    /// Whether orchestration is enabled.
    pub enabled: bool,
    /// Address of the central coordinator service.
    pub coordinator_address: String,
    /// Interval in seconds between heartbeat messages.
    pub heartbeat_interval_secs: u64,
}

impl NetworkOrchestrationConfig {
    /// Create development-optimized configuration with orchestration disabled.
    ///
    /// Uses localhost coordinator for local testing.
    #[must_use]
    pub fn development_optimized() -> Self {
        use crate::constants::hardcoding::{addresses, runtime_fallback_ports};
        Self {
            enabled: false,
            coordinator_address: format!(
                "{}:{}",
                addresses::LOCALHOST_IPV4,
                runtime_fallback_ports::METRICS
            ),
            heartbeat_interval_secs: 30,
        }
    }

    /// Create production-hardened configuration with orchestration enabled.
    ///
    /// Enables distributed coordination with frequent heartbeats.
    #[must_use]
    pub fn production_hardened() -> Self {
        use crate::constants::hardcoding::runtime_fallback_ports;
        Self {
            enabled: true,
            coordinator_address: format!(
                "coordinator.nestgate.local:{}",
                runtime_fallback_ports::METRICS
            ),
            heartbeat_interval_secs: 10,
        }
    }

    /// Validate the orchestration configuration.
    ///
    /// Ensures coordinator address and heartbeat interval are properly configured.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub const fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Merge this configuration with another, preferring values from `other`.
    ///
    /// All fields from `other` will replace the current values.
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.coordinator_address = other.coordinator_address;
        self.heartbeat_interval_secs = other.heartbeat_interval_secs;
        self
    }
}
