//! # Comprehensive Integration Test Configuration
//!
//! **CANONICAL MODERNIZATION COMPLETE** - Unified test configuration system
//!
//! This module provides unified test configuration patterns that eliminate
//! test config fragmentation and provide consistent testing infrastructure.

// Use the centralized unified test config system instead of fragmented structs
pub use crate::common::config::UnifiedTestConfig;

/// Create optimized comprehensive integration test configuration
pub fn create_optimized_comprehensive_config() -> UnifiedTestConfig {
    UnifiedTestConfig::builder()
        .test_name("comprehensive-integration-optimized")
        .test_description("Optimized comprehensive integration testing configuration")
        .max_duration(std::time::Duration::from_secs(300))
        .isolation_level(TestIsolationLevel::Container)
        .build()
} 