// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Core types and data structures for the file system monitor.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

/// File system event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FsEventType {
    /// File was created
    Created,
    /// File was modified
    Modified,
    /// File was deleted
    Deleted,
    /// File was moved/renamed
    Moved,
    /// Directory was created
    DirectoryCreated,
    /// Directory was deleted
    DirectoryDeleted,
    /// Access time updated
    Accessed,
    /// Metadata changed
    MetadataChanged,
}
impl Default for FsEventType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Modified
    }
}

/// File system event details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsEvent {
    /// Event type
    pub event_type: FsEventType,
    /// Path that triggered the event
    pub path: PathBuf,
    /// Optional destination path for move events
    pub dest_path: Option<PathBuf>,
    /// Timestamp when event occurred
    pub timestamp: SystemTime,
    /// File size if available
    pub file_size: Option<u64>,
    /// Whether this is a directory
    pub is_directory: bool,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}
impl Default for FsEvent {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            event_type: FsEventType::default(),
            path: PathBuf::new(),
            dest_path: None,
            timestamp: SystemTime::now(),
            file_size: None,
            is_directory: false,
            metadata: HashMap::default(),
        }
    }
}

/// File system monitor statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsMonitorStats {
    /// Total events processed
    pub total_events: u64,
    /// Events by type
    pub events_by_type: HashMap<String, u64>,
    /// Events in last minute
    pub events_last_minute: u64,
    /// Events in last hour
    pub events_last_hour: u64,
    /// Number of watched paths
    pub watched_paths: usize,
    /// Monitor uptime
    pub uptime_seconds: u64,
    /// Buffer utilization percentage
    pub buffer_utilization: f64,
    /// Last event timestamp
    pub last_event_time: Option<SystemTime>,
    /// Performance metrics
    pub performance_metrics: HashMap<String, f64>,
}
impl Default for FsMonitorStats {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            total_events: 0,
            events_by_type: HashMap::default(),
            events_last_minute: 0,
            events_last_hour: 0,
            watched_paths: 0,
            uptime_seconds: 0,
            buffer_utilization: 0.0,
            last_event_time: None,
            performance_metrics: HashMap::default(),
        }
    }
}

/// Filter configuration for file system events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilter {
    /// Event types to include
    pub include_types: Vec<FsEventType>,
    /// Event types to exclude
    pub exclude_types: Vec<FsEventType>,
    /// File extensions to monitor (empty = all)
    pub file_extensions: Vec<String>,
    /// Paths to ignore
    pub ignore_paths: Vec<PathBuf>,
    /// Minimum file size threshold
    pub min_file_size: Option<u64>,
    /// Maximum file size threshold
    pub max_file_size: Option<u64>,
}
impl Default for EventFilter {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            include_types: vec![
                FsEventType::Created,
                FsEventType::Modified,
                FsEventType::Deleted,
                FsEventType::Moved,
            ],
            exclude_types: Vec::default(),
            file_extensions: Vec::default(),
            ignore_paths: vec![
                PathBuf::from(".snapshots"),
                PathBuf::from(".zfs"),
                PathBuf::from(".git"),
            ],
            min_file_size: None,
            max_file_size: None,
        }
    }
}

/// Performance tuning settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    /// Debounce interval to avoid duplicate events
    pub debounce_duration: Duration,
    /// Maximum events to buffer
    pub max_buffer_size: usize,
    /// Event processing batch size
    pub batch_size: usize,
    /// Worker thread count
    pub worker_threads: usize,
    /// Enable high-performance mode
    pub high_performance_mode: bool,
}
impl Default for PerformanceSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            debounce_duration: Duration::from_millis(100),
            max_buffer_size: 10_000,
            batch_size: 100,
            worker_threads: nestgate_core::linux_proc::logical_cpu_count().min(8),
            high_performance_mode: false,
        }
    }
}

/// Notification channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Channel name
    pub name: String,
    /// Channel type (email, webhook, etc.)
    pub channel_type: String,
    /// Channel configuration
    pub config: HashMap<String, serde_json::Value>,
    /// Whether channel is enabled
    pub enabled: bool,
}
impl Default for NotificationChannel {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            channel_type: "log".to_string(),
            config: HashMap::default(),
            enabled: true,
        }
    }
}

/// Watch configuration for a specific path
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::WatchConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::WatchConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct WatchConfig {
    /// Path to watch
    pub path: PathBuf,
    /// Whether to watch recursively
    pub recursive: bool,
    /// Event filter for this path
    pub filter: EventFilter,
    /// Performance settings for this path
    pub performance: PerformanceSettings,
    /// Whether this watch is enabled
    pub enabled: bool,
}
impl Default for WatchConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            path: PathBuf::from("/mnt/storage"),
            recursive: true,
            filter: EventFilter::default(),
            performance: PerformanceSettings::default(),
            enabled: true,
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[expect(deprecated)]
pub type WatchConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using WatchConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

