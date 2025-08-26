//
// Comprehensive snapshot lifecycle management with automated policies,
// retention rules, and backup integration for production-ready ZFS systems.

mod events;
mod manager;
mod operations;
mod policy;
mod scheduler;
mod types;

#[cfg(test)]
mod tests;

// Re-export public API
pub use events::{PolicyStats, SnapshotAutomationStatus, SnapshotEvent, SnapshotEventType};
pub use manager::ZfsSnapshotManager;
pub use operations::SnapshotOperationType;
pub use policy::{RetentionPolicy, ScheduleFrequency, SnapshotPolicy};
pub use types::{SnapshotInfo, SnapshotOperation, SnapshotOperationStatus, SnapshotStatistics};
