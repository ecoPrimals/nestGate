// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Retention policies, access policies, and data classification.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Data retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Policy ID
    pub id: String,
    /// Policy name
    pub name: String,
    /// Data classification
    pub data_classification: DataClassification,
    /// Retention period in days
    pub retention_days: u32,
    /// Archive after days
    pub archive_after_days: Option<u32>,
    /// Auto-delete after retention
    pub auto_delete: bool,
    /// Legal hold override
    pub legal_hold: bool,
    /// Applicable data types
    pub data_types: Vec<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Access control policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Policy ID
    pub id: String,
    /// Policy name
    pub name: String,
    /// Required permissions
    pub required_permissions: Vec<String>,
    /// Minimum clearance level
    pub min_clearance_level: u8,
    /// Access time restrictions
    pub time_restrictions: Vec<TimeRestriction>,
    /// Location restrictions
    pub location_restrictions: Vec<String>,
    /// MFA required
    pub mfa_required: bool,
    /// Audit access
    pub audit_access: bool,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

/// Time restriction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestriction {
    /// Day of week (0-6, Sunday = 0)
    pub day_of_week: u8,
    /// Start time (24-hour format)
    pub start_time: String,
    /// End time (24-hour format)
    pub end_time: String,
    /// Timezone
    pub timezone: String,
}

/// Data classification levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataClassification {
    /// Public data
    Public,
    /// Internal data
    Internal,
    /// Confidential data
    Confidential,
    /// Restricted data
    Restricted,
    /// Top secret data
    TopSecret,
}
