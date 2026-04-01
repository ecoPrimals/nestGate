// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Audit events and audit result types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Event ID
    pub id: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: AuditEventType,
    /// User ID
    pub user_id: Option<String>,
    /// Resource accessed
    pub path: String,
    /// Action performed
    pub action: String,
    /// Result status
    pub result: AuditResult,
    /// Additional details
    pub details: HashMap<String, String>,
    /// Source IP address
    pub source_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
}

/// Audit event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    /// Data access
    DataAccess,
    /// Data modification
    DataModification,
    /// Data deletion
    DataDeletion,
    /// Policy change
    PolicyChange,
    /// Authentication
    Authentication,
    /// Authorization
    Authorization,
    /// System configuration
    SystemConfiguration,
    /// Compliance violation
    ComplianceViolation,
}

impl std::fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataAccess => write!(f, "Data Access"),
            Self::DataModification => write!(f, "Data Modification"),
            Self::DataDeletion => write!(f, "Data Deletion"),
            Self::PolicyChange => write!(f, "Policy Change"),
            Self::Authentication => write!(f, "Authentication"),
            Self::Authorization => write!(f, "Authorization"),
            Self::SystemConfiguration => write!(f, "System Configuration"),
            Self::ComplianceViolation => write!(f, "Compliance Violation"),
        }
    }
}

/// Audit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditResult {
    /// Success
    Success,
    /// Failure
    Failure,
    /// Unauthorized
    Unauthorized,
    /// Forbidden
    Forbidden,
    /// Error
    Error,
}
