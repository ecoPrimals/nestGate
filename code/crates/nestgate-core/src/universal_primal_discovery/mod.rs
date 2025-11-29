//! Universal Primal Discovery module

pub mod cache;
// Universal Primal Discovery - Modular Architecture
// This module implements the Universal Primal Principle: eliminating ALL hardcoded
// values through dynamic runtime discovery. The architecture is split into focused
// modules for maintainability and team collaboration.
// ## Module Organization
// - `core`: Main discovery orchestrator and public API
// - `network`: Network discovery (addresses, ports, interfaces)
// - `performance`: Timeout and performance optimization discovery
// - `registry`: Service registry and external discovery methods
// - `cache`: Discovery result caching and persistence
// - `introspection`: System introspection and auto-detection
pub mod core;
pub mod fallbacks;
pub mod introspection;
pub mod introspection_config;
pub mod network;
pub mod network_discovery_config;
pub mod performance;
pub mod production_discovery;
pub mod production_discovery_config;
pub mod registry;
pub mod registry_config;
#[cfg(feature = "dev-stubs")]
pub mod stubs;
#[cfg(test)]
mod tests;
pub use core::UniversalPrimalDiscovery;

// Re-export commonly used types
pub use cache::{CacheEntry, DiscoveryCache};
pub use fallbacks::get_fallback_port;
pub use introspection::{HardwareProfile, SystemCapabilities};
pub use introspection_config::{IntrospectionConfig, SharedIntrospectionConfig};
#[allow(deprecated)]
pub use network::{InterfaceInfo, NetworkDiscoveryConfig};
pub use network_discovery_config::{NetworkRuntimeConfig, SharedNetworkRuntimeConfig};
pub use performance::{OptimalTimeout, PerformanceTestConfig};
pub use production_discovery::{ProductionServiceDiscovery, ServiceDiscoveryConfig};
pub use production_discovery_config::{ProductionDiscoveryConfig, SharedProductionDiscoveryConfig};
pub use registry::{DiscoveryQuery, ServiceRegistryClient};
pub use registry_config::{RegistryConfig, SharedRegistryConfig};
#[cfg(feature = "dev-stubs")]
#[allow(deprecated)]
pub use stubs::{
    discover_bind_address, discover_endpoint, discover_limit, discover_port, discover_timeout,
    NetworkConfigAdapter, StandaloneNetworkAdapter,
};
