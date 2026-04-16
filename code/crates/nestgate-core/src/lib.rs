// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Core library providing unified types, configurations, constants, and interfaces
//! for all NestGate components.
//!
//! As of the crate decomposition, foundational modules live in dedicated crates
//! and are re-exported here for backward compatibility:
//!
//! - `nestgate-types`: error, result_types, unified_enums
//! - `nestgate-config`: config, constants, canonical_modernization, defaults
//! - `nestgate-storage`: universal_storage, temporal_storage
//! - `nestgate-rpc`: rpc (JSON-RPC + tarpc)
//! - `nestgate-discovery`: discovery, capabilities, service_discovery

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![expect(
    deprecated,
    reason = "Crate re-exports and legacy modules still surface deprecated aliases; migrate call sites to canonical_primary (e.g. CanonicalNetworkConfig) per type deprecation notes."
)]
// Workspace enables pedantic/nursery clippy; child modules in separate files do not inherit a parent
// module's `#!allow`, so these stay crate-level until fixed or replaced with per-file allows.
// Lint hygiene (nestgate-core#lints): shrink this allow list as modules gain targeted attributes.
#![expect(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::must_use_candidate,
    clippy::option_if_let_else,
    clippy::needless_pass_by_value,
    clippy::unused_self,
    clippy::implicit_hasher,
    clippy::doc_markdown,
    clippy::float_cmp,
    clippy::suboptimal_flops,
    clippy::manual_midpoint,
    clippy::inline_always,
    clippy::collapsible_if,
    clippy::redundant_closure_for_method_calls,
    clippy::single_char_pattern,
    clippy::cast_possible_wrap
)]
// `#[cfg(test)]` modules use permissive test patterns; `cargo clippy -p nestgate-core --lib` does not set `cfg(test)`.
#![cfg_attr(
    test,
    allow(
        dead_code,
        unused_imports,
        unused_variables,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines,
        clippy::cognitive_complexity,
        clippy::bool_assert_comparison,
        clippy::assertions_on_constants,
        clippy::no_effect_underscore_binding,
        clippy::field_reassign_with_default,
    )
)]

// ==================== FOUNDATION RE-EXPORTS (nestgate-types) ====================

pub use nestgate_types::config_error;
pub use nestgate_types::error;
pub use nestgate_types::internal_error;
pub use nestgate_types::result_types;
pub use nestgate_types::unified_enums;
pub use nestgate_types::validation_error;

// ==================== CONFIG RE-EXPORTS (nestgate-config) ====================

pub use nestgate_config::canonical_modernization;
pub use nestgate_config::capability_based_config;
pub use nestgate_config::capability_config;
pub use nestgate_config::config;
pub use nestgate_config::constants;
pub use nestgate_config::defaults;
pub use nestgate_config::defaults_v2_config;
pub use nestgate_config::environment;
pub use nestgate_config::environment_config;
pub use nestgate_config::sovereignty_config;

// ==================== STORAGE RE-EXPORTS (nestgate-storage) ====================

pub use nestgate_storage::temporal_storage;
pub use nestgate_storage::universal_storage;

// ==================== RPC RE-EXPORTS (nestgate-rpc) ====================

pub use nestgate_rpc::rpc;

// ==================== DISCOVERY RE-EXPORTS (nestgate-discovery) ====================

pub use nestgate_discovery::capabilities;
pub use nestgate_discovery::capability_discovery;
pub use nestgate_discovery::infant_discovery;
pub use nestgate_discovery::primal_discovery;
pub use nestgate_discovery::primal_self_knowledge;
#[deprecated(
    since = "4.7.0",
    note = "Service registry and orchestration discovery are delegated to the orchestration \
            capability provider via IPC. Use capability-based peer lookup instead."
)]
pub use nestgate_discovery::service_discovery;
pub use nestgate_discovery::universal_primal_discovery;

// ==================== CORE-ONLY MODULES ====================

/// Canonical types system
pub mod canonical_types;
/// Data source integrations
pub mod data_sources;
// ==================== PLATFORM RE-EXPORTS (nestgate-platform) ====================

pub use nestgate_platform::env_process;
pub use nestgate_platform::linux_proc;
pub use nestgate_platform::platform;
/// NAT traversal persistence types and beacon records.
pub mod nat_traversal;
/// Error recovery and resilience patterns
pub mod recovery;
/// Service metadata storage for Universal IPC Architecture
pub mod service_metadata;
/// Canonical trait system with native async
pub mod traits;
/// Core types module
pub mod types;
/// Universal adapter for primal integration
pub mod universal_adapter;
// ==================== CACHE RE-EXPORTS (nestgate-cache) ====================

pub use nestgate_cache::uuid_cache;

/// HTTP client stub (pure-Rust evolution path). **Opt-in:** `dev-stubs` feature only.
#[cfg(feature = "dev-stubs")]
pub mod http_client_stub;
pub use nestgate_cache::cache;
// ==================== SECURITY RE-EXPORTS (nestgate-security) ====================

pub use nestgate_security::cert;
pub use nestgate_security::crypto;
// ==================== OBSERVE RE-EXPORTS (nestgate-observe) ====================

pub use nestgate_observe::diagnostics;
/// Mathematical utilities
pub mod math;
pub use nestgate_observe::observability;
/// Response handling
pub mod response;
/// Return builders for standardized response construction
pub mod return_builders;
/// Safe operations utilities
pub mod safe_operations;

#[cfg(any(test, feature = "dev-stubs"))]
pub mod dev_stubs;

pub use nestgate_security::jwt_validation;
pub use nestgate_security::zero_cost;
pub use nestgate_security::zero_cost_security_provider;
/// Canonical type definitions
pub mod canonical;
/// Memory layout optimizations
pub mod memory_layout;
/// Network services and protocols
pub mod network;
/// Performance optimization system
pub mod performance;
/// Service implementations with native async
pub mod services;
/// SIMD optimizations
pub mod simd;
/// Universal providers with zero-cost patterns
pub mod universal_providers_zero_cost;
/// Universal traits system
pub mod universal_traits;

pub use nestgate_cache::cache_math;
/// Consensus algorithm mathematics
pub mod consensus_math;
/// Validation predicate functions (extracted for mutation testing and integration tests)
pub mod validation_predicates;

// ==================== CANONICAL RE-EXPORTS ====================

pub use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
pub use nestgate_config::constants::*;
pub use nestgate_discovery::universal_primal_discovery::service_registry::ServiceRegistry;
pub use nestgate_types::error::{CanonicalResult, TestResult};
pub use nestgate_types::error::{NestGateError, Result};
pub use nestgate_types::result_types::{ConnectionFactory, HealthCheckFn, ValidatorFn, VoidResult};
pub use nestgate_types::unified_enums::*;
pub use traits::{
    CanonicalAutomation, CanonicalMcp, CanonicalNetwork, CanonicalProvider, CanonicalSecurity,
    CanonicalService, CanonicalStorage,
};

// ==================== BACKWARD COMPATIBILITY ====================

pub use nestgate_config::config::canonical_primary::NestGateCanonicalConfig as CanonicalConfig;
pub use nestgate_types::error::NestGateError as Error;

#[cfg(test)]
mod edge_case_tests;

#[cfg(test)]
mod result_types_comprehensive_tests;

#[cfg(test)]
mod core_coverage_tests;
