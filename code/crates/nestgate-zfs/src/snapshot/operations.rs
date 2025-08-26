//
// Implementation of snapshot operations including create, delete, clone,
// rollback, send, and receive operations.

use serde::{Deserialize, Serialize};

/// Types of snapshot operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnapshotOperationType {
    /// Create a new snapshot
    Create,
    /// Delete an existing snapshot
    Delete,
    /// Clone a snapshot
    Clone,
    /// Rollback to a snapshot
    Rollback,
    /// Send snapshot to another location
    Send,
    /// Receive snapshot from another location
    Receive,
}
