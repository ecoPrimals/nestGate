// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **SYSTEM TYPES** — Core system status and resource management

use serde::{Deserialize, Serialize};

/// Allocation status for resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Status values for Allocation
pub enum AllocationStatus {
    /// Resource is actively allocated
    Active,
    /// Resource is not allocated
    Inactive,
    /// Resource allocation is pending
    Pending,
    /// Resource allocation has failed
    Failed,
}

impl Default for AllocationStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self::Inactive
    }
}
