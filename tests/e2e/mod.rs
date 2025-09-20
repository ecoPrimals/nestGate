//! E2E Test Organization Module
//! 
//! This module organizes the comprehensive e2e workflows
//! to comply with 1000 lines per file limit.

pub mod framework;
pub mod workflow_runner;
pub mod chaos_testing;
pub mod performance_testing;

// Re-export the main framework for backward compatibility
pub use framework::E2ETestingFramework; 