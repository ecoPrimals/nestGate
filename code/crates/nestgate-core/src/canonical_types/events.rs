// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **EVENT TYPES** — Event handling and processing

use crate::canonical_modernization::canonical_constants::system::DEFAULT_SERVICE_NAME;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Event severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
/// Eventseverity
pub enum EventSeverity {
    /// Debug-level events for detailed troubleshooting
    Debug,
    /// Informational events for general logging
    Info,
    /// Warning events for potential issues
    Warning,
    /// Error events for failures
    Error,
    /// Critical events requiring immediate attention
    Critical,
}

/// Event categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Eventcategory
pub enum EventCategory {
    /// System-level events
    System,
    /// Security-related events
    Security,
    /// Network events
    Network,
    /// Storage events
    Storage,
    /// User action events
    User,
    /// Application-level events
    Application,
    /// Performance-related events
    Performance,
    /// Custom event category
    Custom(String),
}

/// Event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Event
pub struct Event {
    /// Unique event identifier
    pub id: String,
    /// Timestamp when the event occurred
    pub timestamp: SystemTime,
    /// Category of the event
    pub category: EventCategory,
    /// Severity level of the event
    pub severity: EventSeverity,
    /// Human-readable event message
    pub message: String,
    /// Source that generated the event
    pub source: String,
    /// Additional structured data for the event
    pub data: HashMap<String, serde_json::Value>,
    /// Tags for event categorization and filtering
    pub tags: Vec<String>,
}

impl Default for Event {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: SystemTime::now(),
            category: EventCategory::System,
            severity: EventSeverity::Info,
            message: "Default event".to_string(),
            source: DEFAULT_SERVICE_NAME.to_string(),
            data: HashMap::new(),
            tags: Vec::new(),
        }
    }
}
