/// **STORAGE RESOURCES AND CAPACITY TYPES**
///
/// This module contains core storage resource definitions, capacity management,
/// and health monitoring types. Split from consolidated_storage_types.rs for
/// better maintainability and 2000-line compliance.
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;
// Import unified enums
use crate::canonical_modernization::{UnifiedHealthStatus, UnifiedStorageTier, UnifiedStorageType};

// Import other storage modules
use super::access::{StorageAccessControl, StorageResourceMetadata};
use super::config::StorageResourceConfig;
use super::metrics::StorageMetrics;

// ==================== SECTION ====================

/// **THE** unified storage resource - consolidates all resource definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedStorageResource {
    /// Unique resource identifier
    pub resource_id: String,
    /// Human-readable name
    pub name: String,

    /// Storage type classification
    pub storage_type: UnifiedStorageType,

    /// Resource tier (hot, warm, cold, cache)
    pub tier: UnifiedStorageTier,

    /// Storage path or location

    /// Resource capacity information
    pub capacity: StorageCapacity,

    /// Health and status information
    pub health: StorageHealthInfo,

    /// Performance metrics
    pub metrics: StorageMetrics,

    /// Configuration settings
    pub config: StorageResourceConfig,

    /// Access control and permissions
    pub access: StorageAccessControl,

    /// Timestamps and metadata
    pub metadata: StorageResourceMetadata,

    /// Resource tags for organization
    pub tags: Vec<String>,
}

/// Storage capacity and usage information
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageCapacity {
    /// Total capacity in bytes
    pub total_bytes: u64,
    /// Currently used capacity in bytes
    pub used_bytes: u64,

    /// Available capacity in bytes
    pub available_bytes: u64,

    /// Reserved capacity in bytes
    pub reserved_bytes: u64,

    /// Compression ratio (if applicable)
    pub compression_ratio: Option<f64>,

    /// Deduplication ratio (if applicable)
    pub deduplication_ratio: Option<f64>,
}

impl StorageCapacity {
    /// Calculate usage percentage
    pub const fn usage_percentage(&self) -> f64 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.f64::from(used_bytes) / self.f64::from(total_bytes)) * 100.0
        }
    }

    /// Check if storage is near capacity
    pub const fn is_near_capacity(&self, threshold_percent: f64) -> bool {
        self.usage_percentage() >= threshold_percent
    }

    /// Get effective capacity after compression and deduplication
    pub fn effective_capacity(&self) -> u64 {
        let mut effective = self.total_bytes;

        if let Some(compression) = self.compression_ratio {
            effective = (f64::from(effective) * compression) as u64;
        }

        if let Some(dedup) = self.deduplication_ratio {
            effective = (f64::from(effective) * dedup) as u64;
        }

        effective
    }
}

/// Storage health and status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageHealthInfo {
    /// Overall health status
    pub status: UnifiedHealthStatus,
    /// Detailed health state
    pub state: StorageHealthState,

    /// Last health check timestamp
    pub last_check: DateTime<Utc>,

    /// Health check interval
    pub check_interval: Duration,

    /// Error count in recent period
    pub error_count: u32,

    /// Warning count in recent period
    pub warning_count: u32,

    /// Health score (0.0 - 1.0)
    pub health_score: f64,

    /// Detailed health messages
    pub messages: Vec<String>,
}

/// Detailed storage health states
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageHealthState {
    /// Storage is fully operational
    Online,
    /// Storage is operational but degraded
    Degraded,
    /// Storage has critical issues
    Faulted,
    /// Storage is temporarily offline
    Offline,
    /// Storage is unavailable
    Unavailable,
    /// Storage has been removed
    Removed,
    /// Storage is in maintenance mode
    Maintenance,
    /// Storage is being initialized
    Initializing,
}
// ==================== SECTION ====================

impl Default for UnifiedStorageResource {
    fn default() -> Self {
        Self {
            resource_id: Uuid::new_v4().to_string(),
            name: "default-storage".to_string(),
            storage_type: UnifiedStorageType::Local,
            tier: UnifiedStorageTier::Hot,
            capacity: StorageCapacity::default(),
            health: StorageHealthInfo::default(),
            metrics: StorageMetrics::default(),
            config: StorageResourceConfig::default(),
            access: StorageAccessControl::default(),
            metadata: StorageResourceMetadata::default(),
            tags: vec![],
        }
    }
}

impl Default for StorageHealthInfo {
    fn default() -> Self {
        Self {
            status: UnifiedHealthStatus::Unknown,
            state: StorageHealthState::Initializing,
            last_check: Utc::now(),
            check_interval: Duration::from_secs(300), // 5 minutes
            error_count: 0,
            warning_count: 0,
            health_score: 1.0,
            messages: vec![],
        }
    }
}

// ==================== SECTION ====================

impl UnifiedStorageResource {
    /// Create a new storage resource with default values
    pub const fn new(name: String, storage_type: UnifiedStorageType) -> Self {
        Self {
            name: name.clone(),
            storage_type: storage_type.clone(),
            config: StorageResourceConfig {
                name,
                storage_type,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Check if the resource is healthy
    pub const fn is_healthy(&self) -> bool {
        matches!(self.health.status, UnifiedHealthStatus::Healthy)
            && matches!(self.health.state, StorageHealthState::Online)
    }

    /// Get usage percentage
    pub const fn usage_percentage(&self) -> f64 {
        self.capacity.usage_percentage()
    }

    /// Check if resource needs attention
    pub const fn needs_attention(&self) -> bool {
        self.health.error_count > 0
            || self.health.warning_count > 5
            || self.health.health_score < 0.8
            || self.usage_percentage() > 90.0
    }
}

impl StorageHealthInfo {
    /// Update health status based on current conditions
    pub fn update_health(&mut self) {
        self.last_check = Utc::now();

        // Calculate health score based on various factors
        let mut score = 1.0;

        if self.error_count > 0 {
            score -= (self.f64::from(error_count) * 0.1).min(0.5);
        }

        if self.warning_count > 0 {
            score -= (self.f64::from(warning_count) * 0.05).min(0.3);
        }

        self.health_score = score.max(0.0);

        // Update status based on health score
        self.status = if self.health_score >= 0.8 {
            UnifiedHealthStatus::Healthy
        } else if self.health_score >= 0.5 {
            UnifiedHealthStatus::Degraded
        } else {
            UnifiedHealthStatus::Unhealthy
        };
    }
}
