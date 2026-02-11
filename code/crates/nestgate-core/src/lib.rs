// **NESTGATE CORE LIBRARY**
//! Lib functionality and utilities.
// The canonical core library for the NestGate ecosystem, providing unified
//! types, configurations, constants, and interfaces across all components.
//! Lib functionality and utilities.
// Core library providing unified types, configurations, constants, and interfaces.

// Enable documentation warnings for better API documentation
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
// Temporary allow deprecated during canonical config migration
#![allow(deprecated)]

// ==================== CORE CANONICAL MODULES ====================

/// **PRIMARY**: Canonical modernization infrastructure
pub mod canonical_modernization;
/// **PRIMARY**: Canonical types system
pub mod canonical_types;
// traits_root removed in v0.11.0 — use traits::canonical_unified_traits
// ==================== DOMAIN-SPECIFIC MODULES ====================
/// Capability-based service discovery
pub mod capabilities;
pub mod capability_config;
/// **NEW**: Capability-based primal discovery (Jan 26, 2026)
/// Deep debt solution: Replace hardcoded primal names with runtime capability discovery
pub mod capability_discovery;
/// **UNIFIED**: Capability resolver - bridge for all registry systems
/// Provides common interface for capability-based service discovery
pub mod capability_resolver;
/// Compile-time assertions for constant validation
pub mod compile_time_assertions;
/// **PRIMARY**: Unified configuration system - single source of truth
pub mod config;

// Re-export test configuration when dev-stubs feature is enabled
#[cfg(feature = "dev-stubs")]
pub use config::canonical_primary::CanonicalTestConfigs;
/// **PRIMARY**: Unified constants system
pub mod constants;
/// Data source integrations (Steam, NCBI, etc.)
pub mod data_sources;
/// **PRIMARY**: Default values and environment helpers
pub mod defaults;
/// Defaults configuration - thread-safe config for defaults module
pub mod defaults_v2_config;
/// Runtime capability discovery system (Infant Discovery Architecture)
pub mod discovery;
/// Ecosystem integration patterns
pub mod ecosystem_integration;
/// Environment configuration utilities
pub mod environment;
/// Environment configuration - thread-safe config for environment detection
pub mod environment_config;
/// **PRIMARY**: Unified error handling system
pub mod error;
#[cfg(test)]
mod error_path_coverage_tests;
/// Infant Discovery Architecture implementation
pub mod infant_discovery;
#[cfg(test)]
mod orchestration_tests; // Nov 23, 2025 - P1 test expansion
/// **NEW**: Primal discovery framework (Dec 6, 2025)
///
/// Platform-specific utilities with safe abstractions
pub mod platform;
/// **Core Principle**: Primals have only self-knowledge and discover others at runtime.
/// Enables zero-configuration, capability-based primal integration.
pub mod primal_discovery;
/// Error recovery and resilience patterns
pub mod recovery;
/// **PRIMARY**: Canonical Result type aliases (Nov 10, 2025 consolidation)
pub mod result_types;
/// **RPC MODULE**: tarpc + JSON-RPC for primal-to-primal communication
pub mod rpc;
/// **NEW**: Self-knowledge pattern for primal sovereignty (Dec 3, 2025)
///
/// **Philosophy**: Each primal knows only itself, discovers others at runtime.
/// Replaces hardcoded primal dependencies with capability-based discovery.
pub mod self_knowledge;
/// Service discovery and registry
pub mod service_discovery;
/// **PRIMARY**: Canonical trait system with native async
pub mod traits;
/// Core types module - basic data structures
pub mod types;
/// **UNIFIED**: Unified capability types across all systems
/// Single capability enum used by all discovery mechanisms
pub mod unified_capabilities;

/// NAT traversal persistence types and beacon records.
/// Stores NAT info, port patterns, relay preferences, and known peers
/// for the relay-assisted coordinated punch protocol.
pub mod nat_traversal;
/// Service metadata storage for Universal IPC Architecture
/// Stores persistent service registry (metadata only, NOT connections!)
/// Connection logic is delegated to Songbird
pub mod service_metadata;
/// Universal adapter for primal integration
pub mod universal_adapter;
/// Universal primal discovery system
pub mod universal_primal_discovery;
/// Universal storage system
pub mod universal_storage;
/// UUID caching for performance optimization
pub mod uuid_cache;
// ==================== SPECIALIZED SYSTEMS ====================

/// HTTP client stub (BiomeOS Pure Rust Evolution - replaces reqwest)
pub mod http_client_stub;

/// AI-first refactored components
pub mod ai_first_refactored;
/// Management integration
/// Caching system
pub mod cache;
/// Certificate management
pub mod cert;
/// Cryptographic operations (Pure Rust - RustCrypto)
pub mod crypto;
/// Diagnostic tools
pub mod diagnostics;
/// Events system for event bus, routing, and pubsub
pub mod events;
/// Hardware tuning and optimization
pub mod hardware_tuning;
/// Mathematical utilities (float comparison, etc.)
pub mod math;
/// Observability and monitoring
pub mod observability;
/// Response handling
pub mod response;
/// Return builders for standardized response construction
pub mod return_builders;
/// Safe operations utilities
pub mod safe_operations;

/// **DEV STUBS MODULE** (Feature-gated: `dev-stubs`)
///
/// Development stub implementations for testing and local development.
/// ⚠️ **NOT FOR PRODUCTION** - Only available with `dev-stubs` feature flag.
///
/// **Consolidated**: November 10, 2025
/// - Replaces: `universal_primal_discovery/stubs.rs`
/// - Replaces: `return_builders/mock_builders.rs`
/// - Replaces: `config/canonical_primary/domains/test_canonical/mocking.rs`
#[cfg(feature = "dev-stubs")]
pub mod dev_stubs;
// ⚠️ Security module temporarily disabled - has integration errors
// ✅ JWT validation extracted to standalone module below
// Re-enable after integration fixes are complete
// pub mod security;

/// **JWT VALIDATION MODULE** (Added November 13, 2025)
/// Standalone JWT secret validation for production security compliance
/// Does not depend on broken security module components
pub mod jwt_validation;
/// Security provider system
/// **CANONICAL SECURITY PROVIDER** (November 10, 2025)
/// Primary security provider using canonical SecurityProvider trait
pub mod security_provider_canonical;

/// **DEPRECATED**: Old security provider implementation
/// Use `security_provider_canonical` instead
#[deprecated(
    since = "0.11.3",
    note = "Use security_provider_canonical - migrated to canonical SecurityProvider trait"
)]
pub mod security_provider;
/// Sovereignty configuration helpers
pub mod sovereignty_config;
/// Temporal storage management
pub mod temporal_storage;
// ==================== ZERO-COST ARCHITECTURE ====================

/// Advanced optimization techniques and cutting-edge performance patterns
pub mod advanced_optimizations;
/// Zero-cost optimization patterns
#[doc = "Zero-cost optimization patterns"]
pub mod zero_cost;
/// Zero-cost architecture implementation
pub mod zero_cost_architecture;
/// Zero-cost evolution patterns
pub mod zero_cost_evolution;
/// Zero-cost security provider
pub mod zero_cost_security_provider;
// ==================== CANONICAL INFRASTRUCTURE ====================

/// Canonical type definitions
pub mod canonical;

/// Capability-based configuration (evolution from hardcoded values)
pub mod capability_based_config;

/// Interface definitions
pub mod interface;
/// Memory layout optimizations
pub mod memory_layout;
/// Performance optimizations
pub mod optimized;
/// Performance optimization system (connection pooling, advanced optimizations)
pub mod performance;
/// Primal self-knowledge and discovery system
pub mod primal_self_knowledge;
/// SIMD optimizations
pub mod simd;
/// Unified enumerations
pub mod unified_enums;
/// Universal providers with zero-cost patterns
pub mod universal_providers_zero_cost;
/// Universal traits system
pub mod universal_traits;
// ==================== SPECIALIZED SERVICES ====================

/// Network services and protocols
pub mod network;
/// Service implementations with native async
pub mod services;
// ==================== UNIFIED TYPES ====================

// unified_config_consolidation, unified_types removed in v0.11.0 — use config::canonical_primary
// ==================== CANONICAL RE-EXPORTS ====================
/// **THE CANONICAL CONFIGURATION** - Single source of truth
pub use config::canonical_primary::NestGateCanonicalConfig;
/// **CANONICAL CONSTANTS** - Single source for all constants
pub use constants::*;
/// **THE CANONICAL ERROR SYSTEM** - Single error type for all operations
pub use error::{NestGateError, Result};
/// **CANONICAL RESULT TYPES** - Consolidated Nov 10, 2025 (was 54 → 6 types)
/// All domain-specific aliases have been removed. Use `Result<T>` or `CanonicalResult<T>` instead.
pub use result_types::{
    CanonicalResult, ConnectionFactory, HealthCheckFn, TestResult, ValidatorFn, VoidResult,
};
/// **SERVICE REGISTRY** - Capability-based service discovery (Dec 10, 2025)
pub use universal_primal_discovery::service_registry::ServiceRegistry;
// Note: ZfsResult remains in error::unified_result_system for backwards compatibility
/// **THE CANONICAL TRAITS** - Single source for all service interfaces
pub use traits::{
    CanonicalAutomation, CanonicalMcp, CanonicalNetwork, CanonicalProvider, CanonicalSecurity,
    CanonicalService, CanonicalStorage,
};
/// **UNIFIED ENUMS** - Standardized enumerations
pub use unified_enums::*;
// ==================== BACKWARD COMPATIBILITY ====================

/// **BACKWARD COMPATIBILITY**: Legacy configuration access
pub use config::NestGateCanonicalConfig as CanonicalConfig;
/// **BACKWARD COMPATIBILITY**: Legacy error access
pub use error::NestGateError as Error;

// ==================== PERFORMANCE MODULES ====================

// memory_optimization, string_pool disabled — import issues pending resolution

// ==================== UTILITY MODULES ====================

/// Cache-related mathematical utilities
pub mod cache_math;
/// Consensus algorithm mathematics  
pub mod consensus_math;
/// Validation predicate functions
pub mod validation_predicates;
// zero_copy_optimization disabled — syntax errors pending resolution

/// Simple, working memory pool implementation
pub mod simple_memory_pool;

#[cfg(test)]
mod edge_case_tests;

#[cfg(test)]
mod result_types_comprehensive_tests;

#[cfg(test)]
mod core_coverage_boost;
pub mod discovery_mechanism;
