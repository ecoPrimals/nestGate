/// Storage Types Module  
/// Storage resource and metrics definitions
/// **PROBLEM SOLVED**: Standardized storage resource management
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Storage resource representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResource {
    /// Unique resource identifier
    pub resource_id: String,
    /// Human-readable name
    pub name: String,
    /// Resource type (dataset, pool, volume, etc.)
    pub resource_type: String,
    /// Storage path or location
    pub path: String,
    /// Resource size in bytes
    pub size_bytes: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    /// Resource metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Resource tags
    pub tags: Vec<String>,
    /// Access permissions
    pub permissions: Vec<String>,
}

/// Configuration for creating storage resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceConfig {
    /// Resource name
    pub name: String,
    /// Resource type
    pub resource_type: String,
    /// Initial size (if applicable)
    pub initial_size: Option<u64>,
    /// Storage tier preference
    pub tier: Option<String>,
    /// Configuration options
    pub options: HashMap<String, serde_json::Value>,
}

/// Storage metrics and usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    /// Total storage capacity in bytes
    pub total_capacity_bytes: u64,
    /// Used storage in bytes
    pub used_bytes: u64,
    /// Available storage in bytes
    pub available_bytes: u64,
    /// Number of stored items/files
    pub item_count: u64,
    /// Storage utilization percentage
    pub utilization_percent: f32,
    /// Read operations per second
    pub read_ops_per_sec: f32,
    /// Write operations per second
    pub write_ops_per_sec: f32,
    /// Average read latency in milliseconds
    pub avg_read_latency_ms: f32,
    /// Average write latency in milliseconds
    pub avg_write_latency_ms: f32,
    /// Storage-specific metrics
    pub custom_metrics: HashMap<String, serde_json::Value>,
}

impl Default for StorageResource {
    fn default() -> Self {
        Self {
            resource_id: uuid::Uuid::new_v4().to_string(),
            name: "Unnamed Resource".to_string(),
            resource_type: "unknown".to_string(),
            path: "/".to_string(),
            size_bytes: 0,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            metadata: HashMap::new(),
            tags: Vec::new(),
            permissions: vec!["read".to_string()],
        }
    }
}

impl Default for StorageResourceConfig {
    fn default() -> Self {
        Self {
            name: "New Resource".to_string(),
            resource_type: "generic".to_string(),
            initial_size: None,
            tier: None,
            options: HashMap::new(),
        }
    }
}

impl Default for StorageMetrics {
    fn default() -> Self {
        Self {
            total_capacity_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            item_count: 0,
            utilization_percent: 0.0,
            read_ops_per_sec: 0.0,
            write_ops_per_sec: 0.0,
            avg_read_latency_ms: 0.0,
            avg_write_latency_ms: 0.0,
            custom_metrics: HashMap::new(),
        }
    }
}

impl StorageMetrics {
    /// Calculate available space percentage
    pub fn available_percent(&self) -> f32 {
        if self.total_capacity_bytes == 0 {
            0.0
        } else {
            (self.available_bytes as f32 / self.total_capacity_bytes as f32) * 100.0
        }
    }

    /// Check if storage is nearly full (>90% used)
    pub fn is_nearly_full(&self) -> bool {
        self.utilization_percent > 90.0
    }

    /// Get total operations per second
    pub fn total_ops_per_sec(&self) -> f32 {
        self.read_ops_per_sec + self.write_ops_per_sec
    }

    /// Add custom metric
    pub fn with_custom_metric(mut self, key: &str, value: serde_json::Value) -> Self {
        self.custom_metrics.insert(key.to_string(), value);
        self
    }
}

impl StorageResource {
    /// Create a new storage resource
    pub fn new(name: &str, resource_type: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            resource_type: resource_type.to_string(),
            path: path.to_string(),
            ..Default::default()
        }
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata.insert(key.to_string(), value);
        self.modified_at = Utc::now();
        self
    }

    /// Add tag
    pub fn with_tag(mut self, tag: &str) -> Self {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
            self.modified_at = Utc::now();
        }
        self
    }

    /// Set size
    pub fn with_size(mut self, size_bytes: u64) -> Self {
        self.size_bytes = size_bytes;
        self.modified_at = Utc::now();
        self
    }

    /// Check if user has permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }
}
