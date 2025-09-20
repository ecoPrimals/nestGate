//! REST API Models - Modular Structure
//!
//! This module provides a clean, organized structure for all REST API models,
//! broken down into logical groupings for better maintainability and compliance
//! with the 1000-line file size limit.

use serde::{Deserialize, Serialize};

// ==================== CORE MODULES ====================

/// Core types and enums used across the API
pub mod types;

/// Dataset-related request and response models
pub mod datasets;

/// Snapshot-related models
pub mod snapshots;

/// Performance and metrics models
pub mod performance;

/// Cost estimation and analysis models
pub mod costs;

/// ZFS-specific models and statistics
pub mod zfs;

/// Hardware analysis and auto-configuration models
pub mod hardware;

/// Validation utilities and custom deserializers
pub mod validation;

/// Storage configuration and benchmarking models
pub mod storage;

// ==================== RE-EXPORTS ====================

// Re-export all types for backward compatibility
pub use datasets::*;
pub use hardware::*;
pub use snapshots::*;
pub use storage::*;
pub use types::{
    Alert, AlertCondition, AlertSeverity, AlertStatus, AlertType, ChecksumType,
    CircuitBreakerConfig, ComparisonOperator, CompressionType, DashboardAlert, DatasetStatus,
    DatasetType, DiskIoMetrics, NetworkIoMetrics, RetryPolicy, SnapshotStatus, StorageBackendType,
    StorageMetrics, SystemMetrics, TimeoutConfig, ValidationContext,
};
// Note: ZfsMetrics from types module excluded to avoid conflict
pub use validation::*;
pub use zfs::*; // This includes ZfsMetrics from zfs module

// ==================== UTILITY TYPES ====================

/// **CACHED RESPONSE**
///
/// A response wrapper that includes caching metadata for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResponse<T> {
    /// The actual response data
    pub data: T,
    /// Timestamp when the data was cached
    pub cached_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the cached data expires
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// **API RESPONSE**
///
/// Standard API response wrapper with success/error handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Whether the request was successful
    pub success: bool,
    /// Response data (present on success)
    pub data: Option<T>,
    /// Error message (present on failure)
    pub error: Option<String>,
    /// Response timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    /// Create a successful API response with data
    pub const fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create an error API response with error message
    pub const fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now(),
        }
    }
}
