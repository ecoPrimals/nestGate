//! **CANONICAL TEST CONFIGURATION - CLEAN MIGRATION**
//!
//! This module has been fully migrated to the canonical configuration system.
//! All deprecated code has been removed for clean modernization.
//!
//! **CANONICAL USAGE**:
//! ```rust
//! use nestgate_core::config::canonical_master::test_config::CanonicalTestConfigs;
//! let config = CanonicalTestConfigs::default();
//! ```

// Clean canonical re-exports - no deprecated code
pub use nestgate_core::config::canonical_master::test_config::*;

// Environment support
pub use nestgate_core::config::defaults::Environment;
