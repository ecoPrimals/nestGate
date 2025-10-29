/// File watching configuration and patterns - extracted from monolithic config
/// Handles all aspects of file system watching behavior
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
/// Watch configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchSettings {
    /// Enable file system monitoring
    pub enabled: bool,
    /// Paths to watch
    pub watch_paths: Vec<WatchPath>,
    /// Enable recursive watching
    pub recursive: bool,
    /// Follow symbolic links
    pub follow_symlinks: bool,
    /// Watch mode configuration
    pub watch_mode: WatchMode,
    /// Recursive watching settings
    pub recursive_settings: RecursiveWatchSettings,
    /// Debouncing settings to reduce noise
    pub debouncing: DebouncingSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPath {
    pub path: PathBuf,
    pub recursive: bool,
    pub enabled: bool,
    pub priority: u32,
    pub custom_settings: HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WatchMode {
    /// Use native file system events
    Native,
    /// Use polling-based monitoring
    Polling { interval: Duration },
    /// Hybrid approach (native with polling fallback)
    Hybrid {
        native_timeout: Duration,
        polling_interval: Duration,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveWatchSettings {
    /// Maximum depth for recursive watching (0 = unlimited)
    pub max_depth: u32,
    /// Enable cross-filesystem watching
    pub cross_filesystem: bool,
    /// Exclude hidden directories
    pub exclude_hidden: bool,
    /// Directory patterns to exclude
    pub exclude_patterns: Vec<String>,
    /// Maximum number of directories to watch
    pub max_directories: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebouncingSettings {
    /// Enable debouncing
    pub enabled: bool,
    /// Debounce timeout
    pub timeout: Duration,
}

impl Default for WatchSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            watch_paths: Vec::new(),
            recursive: true,
            follow_symlinks: false,
            watch_mode: WatchMode::Native,
            recursive_settings: RecursiveWatchSettings::default(),
            debouncing: DebouncingSettings::default(),
        }
    }
}

impl Default for RecursiveWatchSettings {
    fn default() -> Self {
        Self {
            max_depth: 0, // unlimited
            cross_filesystem: false,
            exclude_hidden: true,
            exclude_patterns: vec![
                ".git".to_string(),
                ".svn".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
            ],
            max_directories: Some(10_000),
        }
    }
}

impl Default for DebouncingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_millis(100),
        }
    }
}
