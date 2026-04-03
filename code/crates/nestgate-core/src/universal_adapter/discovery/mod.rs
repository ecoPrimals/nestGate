// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// **CANONICAL ADAPTER DISCOVERY**
///
/// Consolidated discovery utilities for the universal adapter system.
mod capability_registry;
mod service;
mod types;

#[cfg(test)]
mod tests;

pub use capability_registry::CapabilityDiscovery;
pub use service::{discover_by_capability, discover_services, health_check_service};
pub use types::{
    DiscoveredService, DiscoveryConfig, DiscoveryConfigCanonical, DiscoveryMethod, DiscoveryResult,
};
