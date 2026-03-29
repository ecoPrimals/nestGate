// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Self-Knowledge Pattern
//!
//! **Philosophy**: Each primal knows ONLY itself, discovers others at runtime.
//!
//! ## Core Principles
//!
//! 1. **Self-Awareness**: Each primal maintains complete knowledge of its own:
//!    - Capabilities it provides
//!    - Endpoints it exposes
//!    - Resources it has available
//!    - Current health status
//!
//! 2. **Runtime Discovery**: Each primal discovers other primals dynamically:
//!    - No hardcoded URLs or addresses
//!    - No compile-time primal dependencies
//!    - Capability-based queries
//!    - Graceful degradation when primals unavailable
//!
//! 3. **Sovereignty**: No primal has privileged knowledge of another:
//!    - All relationships discovered at runtime
//!    - Configuration comes from capabilities, not assumptions
//!    - Network topology can change without code changes
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │ NestGate Primal                                              │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                              │
//! │  ┌────────────────────┐      ┌──────────────────────────┐  │
//! │  │  Self-Knowledge    │      │  Discovery Service       │  │
//! │  ├────────────────────┤      ├──────────────────────────┤  │
//! │  │ • My Capabilities  │      │ • mDNS / Consul / k8s    │  │
//! │  │ • My Endpoints     │◄────►│ • Capability Queries     │  │
//! │  │ • My Resources     │      │ • Health Checks          │  │
//! │  │ • My Health        │      │ • Cache Management       │  │
//! │  └────────────────────┘      └──────────────────────────┘  │
//! │           │                            │                    │
//! │           └────────┬───────────────────┘                    │
//! │                    ▼                                        │
//! │         ┌─────────────────────────┐                        │
//! │         │ Discovered Primals       │                        │
//! │         ├─────────────────────────┤                        │
//! │         │ Orchestrator: ...        │                        │
//! │         │ AI: ...                  │                        │
//! │         │ Security: ...            │                        │
//! │         └─────────────────────────┘                        │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! // Requires SocketAddr for endpoints; see self_knowledge::discovery for full example
//! use nestgate_core::self_knowledge::{SelfKnowledge, PrimalDiscovery};
//! let self_knowledge = SelfKnowledge::builder()
//!     .with_id("nestgate")
//!     .with_capability("storage")
//!     .build()?;
//! let discovery = PrimalDiscovery::new(self_knowledge);
//! ```
//!
//! ## Evolution from Old Pattern
//!
//! ### Old: Hardcoded Dependencies
//! ```rust,ignore
//! // ❌ Hardcoded - violates sovereignty
//! const ORCHESTRATOR_URL: &str = "http://orchestrator:8080";
//! const AI_SERVICE_URL: &str = "http://ai:9000";
//!
//! async fn connect_to_orchestrator() -> Result<Connection> {
//!     client.connect(ORCHESTRATOR_URL).await
//! }
//! ```
//!
//! ### New: Capability-Based Discovery
//! ```rust,ignore
//! // ✅ Discovered - sovereign and flexible
//! async fn connect_to_capability(
//!     discovery: &PrimalDiscovery,
//!     cap: Capability,
//! ) -> Result<Connection> {
//!     let primals = discovery
//!         .find_capability(cap)
//!         .await
//!         .context("Failed to discover capability")?;
//!     
//!     let best = select_best_primal(&primals)?;
//!     client.connect(&best.endpoint).await
//! }
//! ```

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

pub mod announcement;
pub mod builder;
pub mod discovery;

// Re-exports for convenience
pub use announcement::Announcement;
pub use builder::SelfKnowledgeBuilder;
pub use discovery::PrimalDiscovery;

/// Unique identifier for a primal in the ecosystem
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrimalId(String);

impl PrimalId {
    /// Create a new primal ID
    ///
    /// **Convention**: Use the primal's name (e.g., "nestgate", "orchestrator", "beardog")
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the string representation
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PrimalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// What this primal knows about itself
///
/// **Philosophy**: Complete self-awareness, no assumptions about others.
///
/// This struct contains everything a primal knows about its own:
/// - Identity and role in ecosystem
/// - Capabilities it can provide
/// - Endpoints where it's accessible
/// - Current resource availability
/// - Health and operational status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfKnowledge {
    /// My unique identifier in the ecosystem
    pub id: PrimalId,

    /// Human-readable name
    pub name: String,

    /// My version (for compatibility checking)
    pub version: String,

    /// Capabilities I provide to the ecosystem
    pub capabilities: Vec<String>,

    /// Endpoints where I'm accessible
    ///
    /// Key: endpoint name ("api", "metrics", "websocket", etc.)
    /// Value: socket address
    pub endpoints: HashMap<String, SocketAddr>,

    /// Current resource limits and availability
    pub resources: ResourceInfo,

    /// Current health status
    pub health: HealthStatus,

    /// When this knowledge was last updated
    pub last_updated: SystemTime,
}

impl SelfKnowledge {
    /// Create a builder for constructing self-knowledge
    #[must_use]
    pub fn builder() -> SelfKnowledgeBuilder {
        SelfKnowledgeBuilder::new()
    }

    /// Update health status
    pub fn update_health(&mut self, status: HealthStatus) {
        self.health = status;
        self.last_updated = SystemTime::now();
    }

    /// Update resource information
    pub fn update_resources(&mut self, resources: ResourceInfo) {
        self.resources = resources;
        self.last_updated = SystemTime::now();
    }

    /// Get a specific endpoint by name
    #[must_use]
    pub fn get_endpoint(&self, name: &str) -> Option<&SocketAddr> {
        self.endpoints.get(name)
    }

    /// Check if this primal provides a capability
    #[must_use]
    pub fn has_capability(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }
}

/// Resource information about this primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInfo {
    /// Maximum concurrent connections this primal can handle
    pub max_connections: usize,

    /// Current number of active connections
    pub active_connections: usize,

    /// Available memory in bytes
    pub available_memory_bytes: u64,

    /// CPU load (0.0 to 1.0)
    pub cpu_load: f32,
}

impl Default for ResourceInfo {
    fn default() -> Self {
        Self {
            max_connections: 1000,
            active_connections: 0,
            available_memory_bytes: 1024 * 1024 * 1024, // 1GB
            cpu_load: 0.0,
        }
    }
}

/// Health status of this primal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HealthStatus {
    /// Fully operational, accepting requests
    Healthy,

    /// Degraded but operational (e.g., high load, some services down)
    Degraded,

    /// Unhealthy, should not receive new requests
    Unhealthy,

    /// Starting up, not ready yet
    #[default]
    Starting,

    /// Shutting down gracefully
    Stopping,
}

/// Information about a discovered primal
///
/// **Source**: Obtained through discovery service, not hardcoded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalInfo {
    /// The primal's unique identifier
    pub id: PrimalId,

    /// Human-readable name
    pub name: String,

    /// Version (for compatibility)
    pub version: String,

    /// Capabilities this primal provides
    pub capabilities: Vec<String>,

    /// Endpoints where this primal is accessible
    pub endpoints: HashMap<String, SocketAddr>,

    /// Current health status
    pub health: HealthStatus,

    /// When we discovered this primal
    pub discovered_at: SystemTime,

    /// Last time we checked this primal's health
    pub last_health_check: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primal_id_creation() {
        let id = PrimalId::new("nestgate");
        assert_eq!(id.as_str(), "nestgate");
        assert_eq!(id.to_string(), "nestgate");
    }

    #[test]
    fn test_self_knowledge_builder() {
        let knowledge = SelfKnowledge::builder()
            .with_id("nestgate")
            .with_name("NestGate")
            .with_version("0.1.0")
            .with_capability("storage")
            .with_capability("zfs")
            .build()
            .expect("Should build successfully");

        assert_eq!(knowledge.id.as_str(), "nestgate");
        assert_eq!(knowledge.name, "NestGate");
        assert_eq!(knowledge.capabilities.len(), 2);
        assert!(knowledge.has_capability("storage"));
        assert!(knowledge.has_capability("zfs"));
    }

    #[test]
    fn test_health_status() {
        let mut knowledge = SelfKnowledge::builder().with_id("test").build().unwrap();

        assert_eq!(knowledge.health, HealthStatus::Starting);

        knowledge.update_health(HealthStatus::Healthy);
        assert_eq!(knowledge.health, HealthStatus::Healthy);
    }
}
