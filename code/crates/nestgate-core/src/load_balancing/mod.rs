//! Load Balancing Module - Capability Discovery Integration
//!
//! **ARCHITECTURE NOTE**: Load balancing is Songbird's domain, not NestGate's.
//! NestGate is a **storage primal** focused on ZFS, datasets, and data management.
//!
//! # Migration from Mocks
//!
//! **Old Approach (DELETED)**:
//! ```rust,ignore
//! // DON'T: Hardcoded implementations that duplicate Songbird
//! let balancer = MockLoadBalancer::new();
//! balancer.distribute_request(req).await?;
//! ```
//!
//! **Modern Approach (USE THIS)**:
//! ```rust,ignore
//! use nestgate_core::universal_adapter::capability_discovery::*;
//!
//! // Discover networking primal (Songbird) dynamically
//! let discovery = CapabilityDiscovery::new();
//! let networking_providers = discovery
//!     .discover(CapabilityType::Networking)
//!     .await?;
//!
//! if let Some(provider) = networking_providers.first() {
//!     // Use discovered primal for load balancing
//!     provider.handle_load_balanced_request(req).await?;
//! } else {
//!     // Fallback: direct connection (no load balancing)
//!     direct_connection(req).await?;
//! }
//! ```
//!
//! # Why This Architecture?
//!
//! 1. **Sovereignty**: Each primal knows only itself - no hardcoded primal names
//! 2. **Modularity**: Primals can be added/removed dynamically
//! 3. **Zero Duplication**: No mock implementations that duplicate other primals
//! 4. **Production Ready**: Real discovery, real delegation, no stubs
//!
//! # Trait Definitions
//!
//! These traits define the interface for capability discovery.
//! **Do NOT implement these** - discover them via capability system.

use serde::{Deserialize, Serialize};

/// Load balancing strategy hint (for capability discovery)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    WeightedRandom,
    IpHash,
}

/// Load balancing configuration (for capability requests)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    pub strategy: LoadBalancingStrategy,
    pub health_check_interval_secs: u64,
    pub max_retries: u32,
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            strategy: LoadBalancingStrategy::RoundRobin,
            health_check_interval_secs: 30,
            max_retries: 3,
        }
    }
}

/// Backend server information (for capability requests)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendServer {
    pub id: String,
    pub address: String,
    pub weight: u32,
    pub healthy: bool,
}

// NOTE: No implementations here - use capability discovery to find Songbird
// See module documentation for examples
