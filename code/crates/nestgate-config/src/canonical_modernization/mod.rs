// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **CANONICAL MODERNIZATION SYSTEM**
//! Module definitions and exports.
// This module provides the complete canonical modernization framework for NestGate,
//! implementing unified systems that replace fragmented legacy patterns.

// Removed unused serde imports

// ==================== SECTION ====================

/// Canonical constants system providing unified constant definitions across the system
pub mod canonical_constants;

#[cfg(test)]
mod canonical_constants_tests;
// Constants consolidation system - disabled for compilation compatibility
/// Builder patterns for canonical configurations
pub mod builders;
/// Idiomatic evolution patterns and utilities for code modernization
pub mod idiomatic_evolution;
/// Unified enumerations providing standardized enum types
pub mod unified_enums;
/// REMOVED: unified_types was migrated to config::canonical_primary (November 2025)
/// Zero-cost trait implementations providing high-performance abstractions
pub mod zero_cost_traits;
// ==================== SECTION ====================

use crate::config::canonical_primary::NestGateCanonicalConfig;

/// **BACKWARD COMPATIBILITY**: Legacy configuration type alias for migration compatibility
pub type CanonicalModernizedConfig = NestGateCanonicalConfig;
// `service_metadata` may be reintroduced when ServiceRegistration / ServiceEndpoint / ServiceStatus
// are available from a crate below nestgate-core in the dependency graph.

// ==================== SECTION ====================

pub use canonical_constants::*;

// Re-export from canonical_primary (unified_types deprecated)
pub use crate::config::canonical_primary::service::ServiceConfig as UnifiedServiceConfig;
// Re-export from unified_enums - only enums that actually exist
pub use nestgate_types::unified_enums::service_types::{UnifiedServiceState, UnifiedServiceType};
