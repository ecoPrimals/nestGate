//! **CANONICAL TEST CONFIGURATION - CLEAN MIGRATION**
//!
//! This module has been fully migrated to the canonical configuration system.
//! All deprecated code has been removed for clean modernization.
//!
//! **CANONICAL USAGE**:
//! ```rust
//! use nestgate_core::config::canonical_primary::test_config::CanonicalTestConfigs;
//! let config = CanonicalTestConfigs::default();
//! ```

// Clean canonical re-exports - no deprecated code
#[cfg(feature = "dev-stubs")]
pub use nestgate_core::config::canonical_primary::test_config::*;

// Environment support
pub use nestgate_core::config::canonical_primary::phase2c_types::Environment;

// For tests without dev-stubs feature, provide basic types
#[cfg(not(feature = "dev-stubs"))]
pub use nestgate_core::config::NestGateCanonicalConfig as CanonicalTestConfig;

#[cfg(not(feature = "dev-stubs"))]
pub use nestgate_core::config::NestGateCanonicalConfig as TestDomainConfig;
