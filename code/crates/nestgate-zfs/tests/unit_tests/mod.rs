//
// Organized unit tests for NestGate ZFS components split by functionality:
// - config_tests: ZFS configuration tests
// - performance_tests: Performance metrics and tier tests
// - heuristic_tests: Heuristic-based optimization tests
// - migration_tests: Migration functionality tests
// - snapshot_tests: Snapshot policy and retention tests
// - automation_tests: Automation and lifecycle management tests
// - lifecycle_tests: Phase 2 lifecycle management tests
// - comprehensive_tests: Comprehensive unit tests

mod config_tests;
mod performance_tests;
mod heuristic_tests;
mod migration_tests;
mod snapshot_tests;
mod automation_tests;
mod lifecycle_tests;
mod comprehensive_tests;

// Re-export all tests
pub use config_tests::*;
pub use performance_tests::*;
pub use heuristic_tests::*;
pub use migration_tests::*;
pub use snapshot_tests::*;
pub use automation_tests::*;
pub use lifecycle_tests::*;
pub use comprehensive_tests::*; 