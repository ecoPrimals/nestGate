// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Discovery configuration and result types for the universal adapter.

use crate::canonical_types::service::{ServiceState, ServiceType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::DiscoveryConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::DiscoveryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for Discovery
pub struct DiscoveryConfig {
    /// Discovery endpoint
    pub endpoint: String,
    /// Discovery timeout
    pub timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Discovery interval for periodic discovery
    pub discovery_interval: Duration,
    /// Enabled discovery methods
    pub methods: Vec<DiscoveryMethod>,
}
impl Default for DiscoveryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            endpoint: crate::constants::canonical_defaults::network::build_endpoint(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            discovery_interval: Duration::from_secs(60),
            methods: vec![
                DiscoveryMethod::Environment,
                DiscoveryMethod::ServiceRegistry,
            ],
        }
    }
}

/// Discovery methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Discoverymethod
pub enum DiscoveryMethod {
    /// Environment variable discovery
    Environment,
    /// Service registry lookup
    ServiceRegistry,
    /// Network scanning
    NetworkScan,
    /// Configuration file
    Configuration,
    /// DNS-based discovery
    Dns,
}
/// Discovered service information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Service implementation for Discovered
pub struct DiscoveredService {
    /// Service identifier
    pub id: String,
    /// Service name
    pub name: String,
    /// Service type
    pub service_type: ServiceType,
    /// Service state
    pub state: ServiceState,
    /// Service endpoint
    pub endpoint: String,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Discovery timestamp
    pub discovered_at: SystemTime,
    /// Last health check
    pub last_health_check: Option<SystemTime>,
}
/// Discovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Discoveryresult
pub struct DiscoveryResult {
    /// Discovered services
    pub services: Vec<DiscoveredService>,
    /// Discovery method used
    pub method: DiscoveryMethod,
    /// Discovery duration
    pub duration: Duration,
    /// Success status
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Discoveryconfigcanonical
pub type DiscoveryConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
