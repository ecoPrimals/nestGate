//! **MIGRATED FILE SYSTEM MONITOR MODULE**
//!
//! This module now uses the canonical configuration system instead of
//! scattered FSMonitor-specific configuration structures.

// Re-export from canonical configuration system
pub use nestgate_core::config::canonical_primary::{FsMonitorConfig, NestGateCanonicalConfig};

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
// Removed unused import for pedantic perfection

use nestgate_core::NestGateError as FsMonitorError;

// ==================== CORE TYPES ====================

/// File system event types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FsEventType {
    /// File was created
    Create,
    /// File was modified
    Modify,
    /// File was deleted
    Delete,
    /// File was moved/renamed
    Move,
    /// Directory was created
    CreateDir,
    /// Directory was deleted
    DeleteDir,
    /// File attributes changed
    Chmod,
    /// Other event type
    Other(String),
}
/// File system event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsEvent {
    /// Event type
    pub event_type: FsEventType,
    /// Path that triggered the event
    pub path: PathBuf,
    /// Timestamp when event occurred
    pub timestamp: std::time::SystemTime,
    /// Optional additional metadata
    pub metadata: Option<serde_json::Value>,
}
/// File system event handler trait
pub trait FsEventHandler: Send + Sync + std::fmt::Debug {
    /// Handle a file system event
    fn handle_event(&self, event: &FsEvent) -> Result<(), FsMonitorError>;

    /// Get handler name for debugging
    fn name(&self) -> &str;

    /// Check if handler is interested in this event type
    fn handles_event_type(&self, event_type: &FsEventType) -> bool;
}
/// File system monitor service
#[derive(Debug)]
pub struct FsMonitor {
    #[allow(dead_code)]
    config: FsMonitorConfig,
    handlers: Vec<Box<dyn FsEventHandler>>,
}
impl FsMonitor {
    /// Create a new file system monitor with canonical configuration
    #[must_use]
    pub fn new(config: FsMonitorConfig) -> Self {
        Self {
            config,
            handlers: Vec::new(),
        }
    }

    /// Add an event handler
    pub fn add_handler(&mut self, handler: Box<dyn FsEventHandler>) {
        self.handlers.push(handler);
    }

    /// Start monitoring file system events
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn start(&self) -> Result<(), FsMonitorError> {
        // Implementation would go here
        Ok(())
    }

    /// Stop monitoring
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn stop(&self) -> Result<(), FsMonitorError> {
        // Implementation would go here
        Ok(())
    }

    /// Process a file system event
    #[allow(dead_code)]
    fn process_event(&self, event: &FsEvent) -> Result<(), FsMonitorError> {
        for handler in &self.handlers {
            if handler.handles_event_type(&event.event_type) {
                handler.handle_event(event)?;
            }
        }
        Ok(())
    }
}

// ==================== MIGRATION COMPLETE ====================
//
// All deprecated FSMonitor configuration structures have been removed.
// Use the canonical configuration system instead:
//
// ```rust
// use nestgate_core::config::canonical_primary::{NestGateCanonicalConfig, FsMonitorConfig};
//
// let config = NestGateCanonicalConfig::default();
// let fsmonitor_config = config.services.fsmonitor;
// ```

// ==================== CONVENIENCE FUNCTIONS ====================

/// Create a new canonical `FSMonitor` configuration
#[must_use]
pub fn new_fsmonitor_config() -> FsMonitorConfig {
    FsMonitorConfig::default()
}
/// Create a development-optimized `FSMonitor` configuration
#[must_use]
pub fn dev_fsmonitor_config() -> FsMonitorConfig {
    // Development-specific optimizations would go here
    FsMonitorConfig::default()
}
/// Create a production-optimized `FSMonitor` configuration
#[must_use]
pub fn prod_fsmonitor_config() -> FsMonitorConfig {
    // Production-specific optimizations would go here
    FsMonitorConfig::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_fs_event_type_variants() {
        let types = [
            FsEventType::Create,
            FsEventType::Modify,
            FsEventType::Delete,
            FsEventType::Move,
            FsEventType::CreateDir,
            FsEventType::DeleteDir,
            FsEventType::Chmod,
        ];
        assert_eq!(types.len(), 7);
    }

    #[test]
    fn test_fs_event_type_other() {
        let event_type = FsEventType::Other("custom".to_string());
        assert!(matches!(event_type, FsEventType::Other(_)));
    }

    #[test]
    fn test_fs_event_type_equality() {
        assert_eq!(FsEventType::Create, FsEventType::Create);
        assert_ne!(FsEventType::Create, FsEventType::Modify);
    }

    #[test]
    fn test_fs_event_type_clone() {
        let event_type = FsEventType::Create;
        let cloned = event_type.clone();
        assert_eq!(event_type, cloned);
    }

    #[test]
    fn test_fs_event_creation() {
        let event = FsEvent {
            event_type: FsEventType::Create,
            path: PathBuf::from("/test/path"),
            timestamp: SystemTime::now(),
            metadata: None,
        };
        assert_eq!(event.event_type, FsEventType::Create);
        assert_eq!(event.path, PathBuf::from("/test/path"));
        assert!(event.metadata.is_none());
    }

    #[test]
    fn test_fs_event_with_metadata() {
        let metadata = serde_json::json!({"key": "value"});
        let event = FsEvent {
            event_type: FsEventType::Modify,
            path: PathBuf::from("/test/file.txt"),
            timestamp: SystemTime::now(),
            metadata: Some(metadata.clone()),
        };
        assert!(event.metadata.is_some());
        assert_eq!(event.metadata.expect("Operation failed"), metadata);
    }

    #[test]
    fn test_fs_event_clone() {
        let event = FsEvent {
            event_type: FsEventType::Delete,
            path: PathBuf::from("/test/deleted.txt"),
            timestamp: SystemTime::now(),
            metadata: None,
        };
        let cloned = event.clone();
        assert_eq!(cloned.event_type, event.event_type);
        assert_eq!(cloned.path, event.path);
    }

    #[test]
    fn test_fs_monitor_creation() {
        let config = FsMonitorConfig::default();
        let monitor = FsMonitor::new(config);
        assert_eq!(monitor.handlers.len(), 0);
    }

    #[test]
    fn test_fs_monitor_add_handler() {
        #[derive(Debug)]
        struct TestHandler;

        impl FsEventHandler for TestHandler {
            fn handle_event(&self, _event: &FsEvent) -> Result<(), FsMonitorError> {
                Ok(())
            }

            fn name(&self) -> &str {
                "test_handler"
            }

            fn handles_event_type(&self, _event_type: &FsEventType) -> bool {
                true
            }
        }

        let config = FsMonitorConfig::default();
        let mut monitor = FsMonitor::new(config);
        monitor.add_handler(Box::new(TestHandler));
        assert_eq!(monitor.handlers.len(), 1);
    }

    #[test]
    fn test_fs_monitor_start() {
        let config = FsMonitorConfig::default();
        let monitor = FsMonitor::new(config);
        let result = monitor.start();
        assert!(result.is_ok());
    }

    #[test]
    fn test_fs_monitor_stop() {
        let config = FsMonitorConfig::default();
        let monitor = FsMonitor::new(config);
        let result = monitor.stop();
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_fsmonitor_config() {
        let config = new_fsmonitor_config();
        // Just verify it creates without panic
        let _monitor = FsMonitor::new(config);
    }

    #[test]
    fn test_dev_fsmonitor_config() {
        let config = dev_fsmonitor_config();
        let _monitor = FsMonitor::new(config);
    }

    #[test]
    fn test_prod_fsmonitor_config() {
        let config = prod_fsmonitor_config();
        let _monitor = FsMonitor::new(config);
    }

    #[test]
    fn test_fs_event_handler_trait() {
        #[derive(Debug)]
        struct SelectiveHandler;

        impl FsEventHandler for SelectiveHandler {
            fn handle_event(&self, _event: &FsEvent) -> Result<(), FsMonitorError> {
                Ok(())
            }

            fn name(&self) -> &str {
                "selective"
            }

            fn handles_event_type(&self, event_type: &FsEventType) -> bool {
                matches!(event_type, FsEventType::Create | FsEventType::Delete)
            }
        }

        let handler = SelectiveHandler;
        assert_eq!(handler.name(), "selective");
        assert!(handler.handles_event_type(&FsEventType::Create));
        assert!(handler.handles_event_type(&FsEventType::Delete));
        assert!(!handler.handles_event_type(&FsEventType::Modify));
    }

    #[test]
    fn test_fs_event_serialization() {
        let event = FsEvent {
            event_type: FsEventType::Create,
            path: PathBuf::from("/test/path"),
            timestamp: SystemTime::now(),
            metadata: Some(serde_json::json!({"test": "data"})),
        };

        let serialized = serde_json::to_string(&event).expect("String operation failed");
        assert!(serialized.contains("Create"));
        assert!(serialized.contains("/test/path"));
    }

    #[test]
    fn test_fs_event_type_serialization() {
        let event_type = FsEventType::Modify;
        let serialized = serde_json::to_string(&event_type).expect("String operation failed");
        let deserialized: FsEventType =
            serde_json::from_str(&serialized).expect("Failed to convert from string");
        assert_eq!(event_type, deserialized);
    }

    #[test]
    fn test_fs_event_type_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(FsEventType::Create);
        set.insert(FsEventType::Modify);
        set.insert(FsEventType::Create); // Duplicate

        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_multiple_handlers() {
        #[derive(Debug)]
        struct Handler1;
        #[derive(Debug)]
        struct Handler2;

        impl FsEventHandler for Handler1 {
            fn handle_event(&self, _event: &FsEvent) -> Result<(), FsMonitorError> {
                Ok(())
            }
            fn name(&self) -> &str {
                "handler1"
            }
            fn handles_event_type(&self, _: &FsEventType) -> bool {
                true
            }
        }

        impl FsEventHandler for Handler2 {
            fn handle_event(&self, _event: &FsEvent) -> Result<(), FsMonitorError> {
                Ok(())
            }
            fn name(&self) -> &str {
                "handler2"
            }
            fn handles_event_type(&self, _: &FsEventType) -> bool {
                true
            }
        }

        let config = FsMonitorConfig::default();
        let mut monitor = FsMonitor::new(config);
        monitor.add_handler(Box::new(Handler1));
        monitor.add_handler(Box::new(Handler2));
        assert_eq!(monitor.handlers.len(), 2);
    }

    #[test]
    fn test_fs_event_different_paths() {
        let event1 = FsEvent {
            event_type: FsEventType::Create,
            path: PathBuf::from("/path1"),
            timestamp: SystemTime::now(),
            metadata: None,
        };

        let event2 = FsEvent {
            event_type: FsEventType::Create,
            path: PathBuf::from("/path2"),
            timestamp: SystemTime::now(),
            metadata: None,
        };

        assert_ne!(event1.path, event2.path);
    }

    #[test]
    fn test_fs_event_type_other_custom_values() {
        let custom1 = FsEventType::Other("symlink".to_string());
        let custom2 = FsEventType::Other("hardlink".to_string());

        if let FsEventType::Other(val) = custom1 {
            assert_eq!(val, "symlink");
        } else {
            panic!("Expected Other variant");
        }

        if let FsEventType::Other(val) = custom2 {
            assert_eq!(val, "hardlink");
        } else {
            panic!("Expected Other variant");
        }
    }

    #[test]
    fn test_fs_monitor_start_stop_sequence() {
        let config = FsMonitorConfig::default();
        let monitor = FsMonitor::new(config);

        assert!(monitor.start().is_ok());
        assert!(monitor.stop().is_ok());
    }

    #[test]
    fn test_fs_event_timestamp_is_recent() {
        let event = FsEvent {
            event_type: FsEventType::Modify,
            path: PathBuf::from("/test"),
            timestamp: SystemTime::now(),
            metadata: None,
        };

        let duration = event.timestamp.elapsed().expect("Operation failed");
        assert!(duration.as_secs() < 1); // Should be very recent
    }

    #[test]
    fn test_fs_event_type_debug_format() {
        let event_type = FsEventType::CreateDir;
        let debug_str = format!("{:?}", event_type);
        assert!(debug_str.contains("CreateDir"));
    }

    #[test]
    fn test_fs_monitor_debug_format() {
        let config = FsMonitorConfig::default();
        let monitor = FsMonitor::new(config);
        let debug_str = format!("{:?}", monitor);
        assert!(debug_str.contains("FsMonitor"));
    }

    #[test]
    fn test_fs_event_debug_format() {
        let event = FsEvent {
            event_type: FsEventType::Delete,
            path: PathBuf::from("/test/file"),
            timestamp: SystemTime::now(),
            metadata: None,
        };
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("FsEvent"));
        assert!(debug_str.contains("Delete"));
    }
}
