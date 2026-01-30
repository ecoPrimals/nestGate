//! Enum definitions for the consolidated canonical adapter
//!
//! This module contains all enum types used throughout the adapter system,
//! including capability categories, data types, discovery methods, and more.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== CAPABILITY ENUMS ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Capability category classification
pub enum CapabilityCategory {
    /// Storage
    Storage,
    /// Security
    Security,
    /// AI
    AI,
    /// Network
    Network,
    /// Orchestration
    Orchestration,
    /// Monitoring
    Monitoring,
    /// Custom category
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Types of data that can be handled
pub enum DataType {
    /// JSON data
    Json,
    /// Binary data
    Binary,
    /// Text data
    Text,
    /// Database data
    Database,
    /// Time series data
    TimeSeries,
    /// Stream data
    Stream,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Scalability rating for a capability
pub enum ScalabilityRating {
    /// Low scalability
    Low,
    /// Medium scalability
    Medium,
    /// High scalability
    High,
    /// Very high scalability
    VeryHigh,
}

// ==================== DISCOVERY ENUMS ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Method for discovering services
pub enum DiscoveryMethod {
    /// Environment variables
    Environment,
    /// Service registry lookup
    ServiceRegistry,
    /// Network scanning
    NetworkScan,
    /// Configuration files
    Configuration,
    /// DNS discovery
    DNS,
    /// Multicast discovery
    Multicast,
}

// ==================== REQUEST ENUMS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Retry backoff strategy
pub enum RetryBackoff {
    /// Linear backoff with fixed increment
    Linear {
        /// Increment duration
        increment: Duration
    },
    /// Exponential backoff with base and maximum
    Exponential {
        /// Base duration
        base: Duration,
        /// Maximum duration
        max: Duration
    },
    /// Fixed delay between retries
    Fixed {
        /// Fixed delay duration
        delay: Duration
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Priority level for requests
pub enum RequestPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

// ==================== RESPONSE ENUMS ====================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Status values for responses
pub enum ResponseStatus {
    /// Successful operation
    Success,
    /// Partially successful operation
    PartialSuccess,
    /// Failed operation
    Failed,
    /// Operation timed out
    Timeout,
    /// Resource not found
    NotFound,
}
