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
pub mod network;
pub mod performance;
pub mod registry;
pub mod stubs;
pub use core::UniversalPrimalDiscovery;

// Re-export commonly used types
pub use cache::{CacheEntry, DiscoveryCache};
pub use fallbacks::get_fallback_port;
pub use introspection::{HardwareProfile, SystemCapabilities};
pub use network::{InterfaceInfo, NetworkDiscoveryConfig};
pub use performance::{OptimalTimeout, PerformanceTestConfig};
pub use registry::{DiscoveryQuery, ServiceRegistryClient};
pub use stubs::{
    discover_bind_address, discover_endpoint, discover_limit, discover_port, discover_timeout,
    NetworkConfigAdapter, StandaloneNetworkAdapter,
};
