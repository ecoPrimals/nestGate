/// **REFACTORED TEST CONFIGURATION**
/// 
/// This file has been refactored from 1589 lines into focused modules for better
/// maintainability and separation of concerns.
/// 
/// **New Architecture**:
/// - tests/common/config/execution.rs - Test execution and resource management
/// - tests/common/config/mocking.rs - Mock service configuration
/// - tests/common/config/performance.rs - Performance and chaos testing
/// - tests/common/config/mod.rs - Main unified configuration
/// 
/// **Migration Guide**:
/// ```rust
/// // OLD: use tests::common::test_config::UnifiedTestConfig;
/// // NEW: use tests::common::config::UnifiedTestConfig;
/// ```

// Re-export the new modular configuration
pub use super::config::*; 