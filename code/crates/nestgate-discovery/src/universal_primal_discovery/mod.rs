// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Universal Primal Discovery module
//!
//! **Integration (dev-stubs):** When `nestgate-core` exposes stable `dev_stubs::primal_discovery`
//! types, enable `#[cfg(feature = "dev-stubs")]` re-exports of `NetworkConfigAdapter` and
//! `StandaloneNetworkAdapter` from this module for tests and local development.

/// Discovery backends for different service discovery mechanisms
pub mod backends;

/// Discovery result caching and persistence
pub mod cache;

/// **NEW**: Capability-based discovery with self-knowledge (Dec 4, 2025)
///
/// Pure capability-based discovery with zero hardcoding. Primals discover themselves
/// and find peers through runtime capability queries.
pub mod capability_based_discovery;

/// **NEW**: Service Registry - High-level API for capability discovery (Dec 10, 2025)
///
/// Provides ergonomic interface for finding services by capability without hardcoded URLs.
/// This is the recommended API for application code.
pub mod service_registry;

/// **NEW**: Production capability bridge (Dec 4, 2025)
///
/// Bridges old environment-based discovery with new capability-based discovery.
/// Provides backward compatibility while enabling migration to modern patterns.
pub mod production_capability_bridge;
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
#[cfg(test)]
mod capability_discovery_tests;
/// Main discovery orchestrator and public API
pub mod core;

#[cfg(test)]
mod discovery_edge_cases_tests; // Dec 10, 2025 - Edge case tests  // Dec 10, 2025 - Comprehensive capability discovery tests
#[cfg(test)]
mod error_path_tests;
/// Fallback strategies for discovery failures
pub mod fallbacks;
/// System introspection and auto-detection
pub mod introspection;
/// Introspection configuration
pub mod introspection_config;
// migration_to_self_knowledge removed — discovery delegated to orchestration-provider IPC
/// Network discovery (addresses, ports, interfaces)
pub mod network;
/// Network discovery configuration
pub mod network_discovery_config;
/// Timeout and performance optimization discovery
pub mod performance;
/// Production discovery implementation
pub mod production_discovery;
/// Production discovery configuration
pub mod production_discovery_config;
#[cfg(test)]
mod production_discovery_core_tests;
#[cfg(test)]
mod production_discovery_extended_tests;
/// Service registry and external discovery methods
pub mod registry;
/// Registry configuration
pub mod registry_config;
#[cfg(test)]
mod tests;

#[cfg(test)]
mod capability_based_discovery_tests;

#[cfg(test)]
mod cache_tests;

#[cfg(test)]
mod fallbacks_tests;

#[cfg(test)]
mod performance_tests;

pub use core::UniversalPrimalDiscovery;

// Re-export commonly used types
pub use cache::{CacheEntry, DiscoveryCache};
pub use fallbacks::get_fallback_port;
pub use introspection::{HardwareProfile, SystemCapabilities};
pub use introspection_config::{IntrospectionConfig, SharedIntrospectionConfig};
#[expect(deprecated)]
pub use network::{InterfaceInfo, NetworkDiscoveryConfig};
pub use network_discovery_config::{NetworkRuntimeConfig, SharedNetworkRuntimeConfig};
pub use performance::{OptimalTimeout, PerformanceTestConfig};
#[expect(deprecated)]
pub use production_discovery::{ProductionServiceDiscovery, ServiceDiscoveryConfig};
pub use production_discovery_config::{ProductionDiscoveryConfig, SharedProductionDiscoveryConfig};
pub use registry::{DiscoveryQuery, ServiceRegistryClient};
pub use registry_config::{RegistryConfig, SharedRegistryConfig};
