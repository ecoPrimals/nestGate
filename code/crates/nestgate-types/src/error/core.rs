// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// Core error handling types

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Error context information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Errorcontext
pub struct ErrorContext {
    /// Operation being performed when error occurred
    pub operation: String,
    /// Component or module where error occurred
    pub component: String,
    /// Timestamp when error occurred
    pub timestamp: SystemTime,
}
impl Default for ErrorContext {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            operation: "unknown".to_string(),
            component: "unknown".to_string(),
            timestamp: SystemTime::now(),
        }
    }
}
