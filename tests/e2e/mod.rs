//! E2E Test Organization Module
//! 
//! This module organizes the comprehensive e2e workflows
//! to comply with 1000 lines per file limit.

pub mod framework;
pub mod workflow_runner;
pub mod chaos_testing;
pub mod performance_testing;
pub mod critical_workflows;
pub mod fault_tolerance_scenarios;
pub mod advanced_scenarios;
pub mod expanded_scenarios;
pub mod security_tests;
pub mod service_discovery_workflows;
pub mod storage_migration_workflows;
pub mod workflows;

// Re-export the main framework for backward compatibility
pub use framework::E2ETestingFramework; 