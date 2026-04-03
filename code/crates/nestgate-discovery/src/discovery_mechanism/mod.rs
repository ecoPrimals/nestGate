// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Discovery mechanism abstraction
//!
//! **Vendor-agnostic service discovery for the infant discovery pattern.**
//!
//! Per the PRIMAL responsibility matrix, **runtime peer discovery is not NestGate’s domain**:
//! production deployments delegate discovery to the **ecosystem platform** and **orchestration provider**. The types here are
//! optional building blocks for standalone binaries, tests, and local development only.
//!
//! Each backend (`mdns`, `consul`, `kubernetes`) is behind a crate feature so default builds do not
//! pull in unused discovery code.
//!
//! ## Philosophy
//!
//! Each primal starts with **zero knowledge** and discovers its ecosystem
//! at runtime using whatever discovery mechanism is available:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │ Primal Startup (Infant Discovery)                   │
//! ├─────────────────────────────────────────────────────┤
//! │                                                      │
//! │  1. Self-Awareness: "I am NestGate, I provide      │
//! │     Storage and ZFS capabilities"                   │
//! │                                                      │
//! │  2. Detect Discovery: Auto-detect available         │
//! │     mechanism (k8s? consul? mdns?)                  │
//! │                                                      │
//! │  3. Announce: Register my capabilities               │
//! │                                                      │
//! │  4. Discover: Find primals by capability             │
//! │     (not by name!)                                   │
//! │                                                      │
//! └─────────────────────────────────────────────────────┘
//! ```
//!
//! ## Example
//!
//! ```rust,ignore
//! // Full example requires DiscoveryBuilder, SelfKnowledge with SocketAddr endpoints
//! use nestgate_discovery::discovery_mechanism::{DiscoveryBuilder, MdnsDiscovery};
//! use nestgate_discovery::self_knowledge::SelfKnowledge;
//! let discovery = DiscoveryBuilder::default().detect()?;
//! let self_knowledge = SelfKnowledge::builder().with_id("nestgate").build()?;
//! ```

use crate::self_knowledge::SelfKnowledge;
use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

// Submodules
pub mod builder;
/// Minimal pure-Rust HTTP client for discovery bootstrap (no external deps)
pub mod http;
#[cfg(feature = "mdns")]
pub mod mdns;
#[cfg(any(test, feature = "dev-stubs"))]
pub mod testing;

#[cfg(feature = "consul")]
pub mod consul;

#[cfg(feature = "kubernetes")]
pub mod k8s;

#[cfg(test)]
mod tests;

// Re-exports for backward compatibility
pub use builder::DiscoveryBuilder;
#[cfg(feature = "mdns")]
pub use mdns::MdnsDiscovery;

#[cfg(feature = "consul")]
pub use consul::ConsulDiscovery;

#[cfg(feature = "kubernetes")]
pub use k8s::KubernetesDiscovery;

/// Simplified capability type for service discovery
///
/// Represents a capability that a service can provide or request.
/// This is a simplified string-based representation that will eventually
/// use the proper capability taxonomy from `crate::capabilities::taxonomy`.
///
/// # Examples
/// - "storage"
/// - "compute"
/// - "orchestration"
/// - "security"
pub type Capability = String;

/// Service discovery information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Unique service ID
    pub id: String,
    /// Service name (for logging/debugging only, not for discovery!)
    pub name: String,
    /// Capabilities this service provides
    pub capabilities: Vec<Capability>,
    /// Primary endpoint
    pub endpoint: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Health check endpoint
    pub health_endpoint: Option<String>,
}

/// Discovery mechanism trait
///
/// Implemented by different discovery backends (mDNS, Consul, Kubernetes, etc.)
pub trait DiscoveryMechanism: Send + Sync {
    /// Announce this primal's presence
    fn announce(
        &self,
        self_knowledge: &SelfKnowledge,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;

    /// Find services by capability
    fn find_by_capability(
        &self,
        capability: Capability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ServiceInfo>>> + Send + '_>>;

    /// Find a specific service by ID (for re-connection)
    fn find_by_id(
        &self,
        id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ServiceInfo>>> + Send + '_>>;

    /// Health check: is this service still available?
    fn health_check(
        &self,
        service_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>>;

    /// Deregister this primal (graceful shutdown)
    fn deregister(&self, service_id: &str)
    -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;

    /// Get mechanism name (for logging)
    fn mechanism_name(&self) -> &'static str;
}
