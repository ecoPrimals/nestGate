// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Canonical Traits System for NestGate
//!
//! This module provides the canonical trait definitions for all service
//! interfaces in NestGate.
//!
//! ## Trait Hierarchy
//!
//! ```text
//! CanonicalService (base trait for all services)
//! ├── CanonicalProvider<T> (generic provider pattern)
//! ├── CanonicalStorage (storage services)
//! ├── CanonicalNetwork (network services)
//! ├── CanonicalSecurity (security services)
//! ├── CanonicalMcp (MCP services)
//! └── CanonicalAutomation (automation services)
//!
//! UnifiedStorage (canonical storage interface)
//! ├── Core operations (read, write, delete, list, exists)
//! ├── Metadata / Batch / Streaming operations
//! └── Health & monitoring (health_check, get_metrics)
//! ```
//!
//! All traits use `impl Future` for zero-cost async abstractions.

// ==================== MODULES ====================

pub mod async_migration_system;
pub mod canonical;
pub mod communication;
pub mod config_provider;
pub mod domain_extensions;
pub mod health_checks;
pub mod load_balancing;
pub mod native_async;
pub mod service_discovery;
pub mod service_trait;
pub mod unified_storage;
pub mod universal;
pub mod universal_service_zero_cost;

// ==================== RE-EXPORTS ====================

pub use canonical::{
    CanonicalAutomation, CanonicalMcp, CanonicalNetwork, CanonicalProvider,
    CanonicalProviderFactory, CanonicalSecurity, CanonicalService, CanonicalServiceFactory,
    CanonicalStorage, ZeroCostService,
};
pub use communication::{
    CommunicationLayer, CommunicationResponse, CommunicationStats, MessageType, ServiceAddress,
    ServiceMessage,
};
pub use config_provider::{ConfigProvider, ConfigProviderInfo, FederationConfig};
pub use domain_extensions::{StorageServiceExtension, ZfsServiceExtension};
pub use health_checks::{HealthCheck, HealthMonitor, HealthState, HealthStatus};
pub use load_balancing::{
    HealthAwareLoadBalancer, LeastConnectionsLoadBalancer, LoadBalancer, LoadBalancerStats,
    LoadBalancingAlgorithm, RandomLoadBalancer, RoundRobinLoadBalancer, ServiceStats,
    WeightedRandomLoadBalancer, WeightedRoundRobinLoadBalancer,
};
#[expect(deprecated, reason = "migration in progress")]
pub use native_async::{
    NativeAsyncApiHandler, NativeAsyncAutomationService, NativeAsyncMcpService,
    NativeAsyncMonitoringService, NativeAsyncNetworkService, NativeAsyncSecurityProvider,
    NativeAsyncService, NativeAsyncStorage, NativeAsyncUniversalProvider,
};
pub use service_discovery::{ServiceDiscovery, ServiceEvent, ServiceQuery};
pub use service_trait::Service;
pub use unified_storage::{
    StorageCapability, StorageMetadata, StorageStream, StorageTransaction, UnifiedStorage,
};

// ==================== VALIDATION ====================

/// Validate that a service implements the canonical interface.
pub const fn validate_canonical_service<S>(_service: &S) -> bool
where
    S: CanonicalService,
{
    true
}

/// Validate that a provider implements the canonical interface.
pub const fn validate_canonical_provider<T, P>(_provider: &P) -> bool
where
    P: CanonicalProvider<T>,
{
    true
}

#[cfg(test)]
mod traits_tests;
