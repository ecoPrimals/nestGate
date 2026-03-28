// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **CANONICAL TEST CONFIGURATION MODULE**
//! This module provides access to the canonical test configuration system.
//! All test configurations have been consolidated into the canonical domains system.
// This module provides access to the canonical test configuration system.
// All test configurations have been consolidated into the canonical domains system.
//!
//! **⚠️ DEV/TEST ONLY**: This module is only available with `dev-stubs` feature

/// Re-export the canonical test configuration from domains
pub use super::domains::test_canonical::{
    CanonicalTestConfigs, TestConfig, TestConfigs, UnifiedTestConfig,
};
/// Backward compatibility alias
pub type TestConfiguration = CanonicalTestConfigs;
/// Default test configuration
#[must_use]
pub fn default_test_config() -> CanonicalTestConfigs {
    CanonicalTestConfigs::default()
}
