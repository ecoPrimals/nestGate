/// **E2E COMPREHENSIVE WORKFLOW TESTS - REFACTORED**
///
/// This file has been refactored from 1027 lines into focused modules for better
/// maintainability and separation of concerns.
///
/// **New Architecture**:
/// - tests/e2e/workflows/mod.rs - Main orchestrator and shared types
/// - tests/e2e/workflows/nas_setup.rs - NAS setup workflow tests
/// - tests/e2e/workflows/file_management.rs - File management workflow tests
/// - tests/e2e/workflows/tier_management.rs - Tier management workflow tests
/// - tests/e2e/workflows/concurrent_users.rs - Concurrent user workflow tests
/// - tests/e2e/workflows/administration.rs - System administration workflow tests
/// - tests/e2e/workflows/backup_recovery.rs - Backup and recovery workflow tests
/// - tests/e2e/workflows/performance_optimization.rs - Performance optimization workflow tests
/// - tests/e2e/workflows/network_protocols.rs - Network protocol workflow tests
///
/// **Migration Guide**:
/// ```rust
/// // OLD: use tests::e2e_comprehensive_workflows::*;
/// // NEW: use tests::e2e::workflows::*;
/// ```

pub mod e2e {
    pub mod workflows;
}

// Re-export for backwards compatibility
pub use e2e::workflows::{
    run_comprehensive_workflow_test, E2EWorkflowOrchestrator, TestResults, WorkflowResults,
};

// Re-export common config for convenience
pub use crate::common::config::UnifiedTestConfig;

// USAGE EXAMPLES
//
// The refactored workflow system provides better organization and maintainability.
//
// # Basic Usage
//
// ```rust
// use crate::e2e_comprehensive_workflows::{run_comprehensive_workflow_test, UnifiedTestConfig};
//
// // Create test configuration
// let config = UnifiedTestConfig::development();
//
// // Run comprehensive workflow tests
// let results = run_comprehensive_workflow_test(&config).await?;
// ```
//
// # Individual Workflow Testing
//
// ```rust
// use crate::e2e::workflows::{E2EWorkflowOrchestrator, UnifiedTestConfig};
//
// // Create orchestrator
// let config = UnifiedTestConfig::production_like();
// let orchestrator = E2EWorkflowOrchestrator::new(config);
//
// // Run specific workflows
// let workflow_results = orchestrator.run_comprehensive_workflow_tests().await?;
// ```

// This completes the refactoring of the original 1027-line file into manageable modules.
