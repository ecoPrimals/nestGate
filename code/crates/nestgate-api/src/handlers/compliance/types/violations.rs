// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Compliance violations and resolution workflow types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Compliance violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    /// Violation ID
    pub id: String,
    /// Violation timestamp
    pub timestamp: DateTime<Utc>,
    /// Violation type
    pub violation_type: ViolationType,
    /// Severity level
    pub severity: ViolationSeverity,
    /// Description
    pub description: String,
    /// Affected resource
    pub path: String,
    /// Regulatory framework
    pub framework: String,
    /// Resolution status
    pub resolution_status: ResolutionStatus,
    /// Resolution deadline
    pub resolution_deadline: Option<DateTime<Utc>>,
    /// Assigned to
    pub assigned_to: Option<String>,
}

/// Violation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    /// Data retention violation
    DataRetention,
    /// Access control violation
    AccessControl,
    /// Encryption violation
    Encryption,
    /// Audit logging violation
    AuditLogging,
    /// Data residency violation
    DataResidency,
    /// Backup violation
    Backup,
    /// Documentation violation
    Documentation,
}

impl std::fmt::Display for ViolationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataRetention => write!(f, "Data Retention"),
            Self::AccessControl => write!(f, "Access Control"),
            Self::Encryption => write!(f, "Encryption"),
            Self::AuditLogging => write!(f, "Audit Logging"),
            Self::DataResidency => write!(f, "Data Residency"),
            Self::Backup => write!(f, "Backup"),
            Self::Documentation => write!(f, "Documentation"),
        }
    }
}

/// Violation severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Resolution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStatus {
    /// Open
    Open,
    /// In progress
    InProgress,
    /// Resolved
    Resolved,
    /// Closed
    Closed,
    /// Escalated
    Escalated,
}
