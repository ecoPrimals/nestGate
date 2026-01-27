//! Common test utilities and types

pub mod concurrent_test_framework;
pub mod config;
pub mod env_isolation;
pub mod isolated_context;
pub mod isolated_test_runner;
pub mod test_resource_manager;
pub mod test_types;
// Disabled: test_config - requires dev-stubs feature
// pub mod test_config;

// Modern concurrent test synchronization - REPLACES SLEEP-BASED PATTERNS
pub mod modern_sync;
pub mod sync_utils;

pub mod test_doubles;
