// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Event handling and processing configuration — batching, queuing, error handling.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use super::fsmonitor_data_dir;
/// Event processing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventProcessingSettings {
    /// Enable event processing
    pub enabled: bool,
    /// Event batching configuration
    pub batching: EventBatchingSettings,
    /// Event queue configuration
    pub queue: EventQueueSettings,
    /// Event pipeline configuration
    pub pipeline: EventPipelineSettings,
    /// Error handling configuration
    pub error_handling: EventErrorHandlingSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBatchingSettings {
    /// Enable event batching
    pub enabled: bool,
    /// Maximum events per batch
    pub max_events_per_batch: u32,
    /// Batch timeout duration
    pub batch_timeout: Duration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventQueueSettings {
    /// Maximum queue size
    pub max_queue_size: u32,
    /// Queue overflow strategy
    pub overflow_strategy: QueueOverflowStrategy,
    /// Queue persistence settings
    pub persistence: QueuePersistenceSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueueOverflowStrategy {
    /// Drop oldest events
    DropOldest,
    /// Drop newest events
    DropNewest,
    /// Block until space available
    Block,
    /// Reject new events
    Reject,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuePersistenceSettings {
    /// Enable queue persistence
    pub enabled: bool,
    /// Persistence file path
    pub file_path: std::path::PathBuf,
    /// Sync frequency
    pub sync_frequency: Duration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPipelineSettings {
    /// Enable event pipeline
    pub enabled: bool,
    /// Pipeline stages
    pub stages: Vec<PipelineStage>,
    /// Parallel processing
    pub parallel_processing: bool,
    /// Worker thread count
    pub worker_threads: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    /// Stage name
    pub name: String,
    /// Stage enabled
    pub enabled: bool,
    /// Stage configuration
    pub config: HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventErrorHandlingSettings {
    /// Enable error handling
    pub enabled: bool,
    /// Retry policy
    pub retry_policy: RetryPolicy,
    /// Dead letter queue settings
    pub dead_letter_queue: DeadLetterQueueSettings,
    /// Error logging settings
    pub error_logging: ErrorLoggingSettings,
    /// Error notification settings
    pub error_notifications: ErrorNotificationSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Enable retries
    pub enabled: bool,
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Initial retry delay
    pub initial_delay: Duration,
    /// Maximum retry delay
    pub max_delay: Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterQueueSettings {
    /// Enable dead letter queue
    pub enabled: bool,
    /// Queue file path
    pub file_path: std::path::PathBuf,
    /// Maximum queue size
    pub max_size: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLoggingSettings {
    /// Enable error logging
    pub enabled: bool,
    /// Log level for errors
    pub log_level: String,
    /// Enable structured logging
    pub structured_logging: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorNotificationSettings {
    /// Enable error notifications
    pub enabled: bool,
    /// Notification channels
    pub channels: Vec<String>,
    /// Error threshold for notifications
    pub error_threshold: u32,
    /// Notification frequency limit
    pub frequency_limit: Duration,
}
impl Default for EventProcessingSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            batching: EventBatchingSettings::default(),
            queue: EventQueueSettings::default(),
            pipeline: EventPipelineSettings::default(),
            error_handling: EventErrorHandlingSettings::default(),
        }
    }
}

impl Default for EventBatchingSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            max_events_per_batch: 100,
            batch_timeout: Duration::from_millis(500),
        }
    }
}

impl Default for EventQueueSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_queue_size: 10_000,
            overflow_strategy: QueueOverflowStrategy::DropOldest,
            persistence: QueuePersistenceSettings::default(),
        }
    }
}

impl Default for QueuePersistenceSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: false,
            file_path: fsmonitor_data_dir().join("queue.dat"),
            sync_frequency: Duration::from_secs(30),
        }
    }
}

impl Default for EventPipelineSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            stages: Vec::new(),
            parallel_processing: true,
            worker_threads: nestgate_core::linux_proc::logical_cpu_count() as u32,
        }
    }
}

impl Default for EventErrorHandlingSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            retry_policy: RetryPolicy::default(),
            dead_letter_queue: DeadLetterQueueSettings::default(),
            error_logging: ErrorLoggingSettings::default(),
            error_notifications: ErrorNotificationSettings::default(),
        }
    }
}

impl Default for RetryPolicy {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

impl Default for DeadLetterQueueSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            file_path: fsmonitor_data_dir().join("dlq.dat"),
            max_size: 1000,
        }
    }
}

impl Default for ErrorLoggingSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: String::from("error"),
            structured_logging: true,
        }
    }
}

impl Default for ErrorNotificationSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            channels: Vec::new(),
            error_threshold: 10,
            frequency_limit: Duration::from_secs(5 * 60),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_processing_default_is_enabled() {
        let settings = EventProcessingSettings::default();
        assert!(settings.enabled);
        assert!(settings.batching.enabled);
        assert!(settings.pipeline.enabled);
    }

    #[test]
    fn default_batch_size_is_reasonable() {
        let settings = EventBatchingSettings::default();
        assert!(settings.max_events_per_batch >= 10);
        assert!(settings.max_events_per_batch <= 10_000);
    }

    #[test]
    fn default_queue_size_is_reasonable() {
        let settings = EventQueueSettings::default();
        assert!(settings.max_queue_size >= 100);
    }

    #[test]
    fn queue_persistence_disabled_by_default() {
        let settings = QueuePersistenceSettings::default();
        assert!(!settings.enabled);
    }

    #[test]
    fn queue_persistence_path_is_not_tmp() {
        let settings = QueuePersistenceSettings::default();
        assert!(
            !settings.file_path.starts_with("/tmp"),
            "must not default to /tmp — got {:?}",
            settings.file_path
        );
    }

    #[test]
    fn dlq_path_is_not_tmp() {
        let settings = DeadLetterQueueSettings::default();
        assert!(
            !settings.file_path.starts_with("/tmp"),
            "must not default to /tmp — got {:?}",
            settings.file_path
        );
    }

    #[test]
    fn retry_policy_default_limits() {
        let policy = RetryPolicy::default();
        assert!(policy.enabled);
        assert!(policy.max_attempts >= 1 && policy.max_attempts <= 10);
    }

    #[test]
    fn error_notification_disabled_by_default() {
        let settings = ErrorNotificationSettings::default();
        assert!(!settings.enabled);
    }

    #[test]
    fn settings_serialization_roundtrip() {
        let original = EventProcessingSettings::default();
        let json = serde_json::to_string(&original).expect("serialize");
        let deserialized: EventProcessingSettings =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(original.enabled, deserialized.enabled);
        assert_eq!(
            original.batching.max_events_per_batch,
            deserialized.batching.max_events_per_batch
        );
    }
}
