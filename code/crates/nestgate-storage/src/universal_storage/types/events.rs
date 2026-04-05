// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage event types
//!
//! Event types for monitoring and responding to storage system changes.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Storage event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageEventType {
    /// Resource created
    Created,
    /// Resource modified
    Modified,
    /// Resource deleted
    Deleted,
    /// Resource moved
    Moved,
    /// Resource accessed
    Accessed,
    /// Permissions changed
    PermissionsChanged,
    /// Health status changed
    HealthChanged,
    /// Capacity changed
    CapacityChanged,
    /// Performance alert
    PerformanceAlert,
    /// Error occurred
    Error,
}

/// Storage event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEvent {
    /// Unique event identifier
    pub event_id: String,
    /// Event type
    pub event_type: StorageEventType,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Related resource ID (if applicable)
    pub resource_id: Option<String>,
}
