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

// For tests without dev-stubs feature, provide basic types
