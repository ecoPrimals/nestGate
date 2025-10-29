//
// Core types and data structures for the file system monitor.

// TEMPORARY: SmartDefault disabled pending native async compatibility fixes
// use nestgate_core::smart_abstractions::smart_default::SmartDefault;
// SmartDefault temporarily disabled for compilation
// use nestgate_core::error::idiomatic_evolution::SmartDefault;
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
    fn default() -> Self {
        Self {
            debounce_duration: Duration::from_millis(100),
            max_buffer_size: 10_000,
            batch_size: 100,
            worker_threads: num_cpus::get().min(8),
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
