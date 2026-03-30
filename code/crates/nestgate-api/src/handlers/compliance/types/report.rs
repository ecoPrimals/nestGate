// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Aggregated compliance reporting types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::regulatory::RegulatoryFramework;
use super::violations::ComplianceViolation;

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Report timestamp
    pub timestamp: DateTime<Utc>,
    /// Total policies
    pub total_policies: usize,
    /// Total violations
    pub total_violations: usize,
    /// Critical violations
    pub critical_violations: usize,
    /// Compliance score (0-100)
    pub compliance_score: f32,
    /// Regulatory frameworks
    pub frameworks: Vec<RegulatoryFramework>,
    /// Recent violations
    pub recent_violations: Vec<ComplianceViolation>,
}
