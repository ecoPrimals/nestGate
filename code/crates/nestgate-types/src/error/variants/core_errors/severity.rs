// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Severity levels for unified errors.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Errorseverity
pub enum ErrorSeverity {
    /// Low
    Low,
    /// Medium
    Medium,
    /// High
    High,
    /// Critical
    Critical,
}
