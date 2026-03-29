// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Local stubs for types that lived in `nestgate-core` when this code was copied here.

/// Core service lifecycle trait (matches `nestgate_core::traits::Service` shape).
pub mod traits {
    use nestgate_types::Result;

    /// Core service trait for `NestGate` services — native async (RPITIT).
    pub trait Service: Send + Sync {
        /// Service name identifier
        fn name(&self) -> &str;

        /// Initialize the service
        fn initialize(&self) -> impl std::future::Future<Output = Result<()>> + Send;

        /// Start the service
        fn start(&self) -> impl std::future::Future<Output = Result<()>> + Send;

        /// Stop the service
        fn stop(&self) -> impl std::future::Future<Output = Result<()>> + Send;

        /// Shutdown the service (alias for `stop`)
        fn shutdown(&self) -> impl std::future::Future<Output = Result<()>> + Send {
            self.stop()
        }

        /// Returns `true` if the service is healthy
        fn health_check(&self) -> impl std::future::Future<Output = Result<bool>> + Send;
    }
}

/// Minimal canonical event types for tests and compatibility.
pub mod canonical_types {
    /// Event types aligned with the former `nestgate_core::canonical_types::events` module.
    pub mod events {
        use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
        use serde::{Deserialize, Deserializer, Serialize, Serializer};
        use std::collections::HashMap;
        use std::time::{Duration, SystemTime, UNIX_EPOCH};

        mod system_time_serde {
            use super::{
                Deserialize, Deserializer, Duration, Serialize, Serializer, SystemTime, UNIX_EPOCH,
            };

            #[derive(Serialize, Deserialize)]
            struct SystemTimeWire {
                secs_since_epoch: u64,
                nanos_since_epoch: u32,
            }

            pub fn serialize<S>(t: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let dur = t
                    .duration_since(UNIX_EPOCH)
                    .map_err(serde::ser::Error::custom)?;
                SystemTimeWire {
                    secs_since_epoch: dur.as_secs(),
                    nanos_since_epoch: dur.subsec_nanos(),
                }
                .serialize(serializer)
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
            where
                D: Deserializer<'de>,
            {
                let w = SystemTimeWire::deserialize(deserializer)?;
                UNIX_EPOCH
                    .checked_add(Duration::new(w.secs_since_epoch, w.nanos_since_epoch))
                    .ok_or_else(|| serde::de::Error::custom("invalid SystemTime"))
            }
        }

        /// Event severity levels
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
        pub enum EventSeverity {
            /// Debug-level events
            Debug,
            /// Informational events
            Info,
            /// Warning events
            Warning,
            /// Error events
            Error,
            /// Critical events
            Critical,
        }

        /// Event categories
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
        pub struct Event {
            /// Unique event identifier
            pub id: String,
            /// Timestamp when the event occurred
            #[serde(with = "system_time_serde")]
            pub timestamp: SystemTime,
            /// Category of the event
            pub category: EventCategory,
            /// Severity level of the event
            pub severity: EventSeverity,
            /// Human-readable event message
            pub message: String,
            /// Source that generated the event
            pub source: String,
            /// Additional structured data
            pub data: HashMap<String, serde_json::Value>,
            /// Tags for filtering
            pub tags: Vec<String>,
        }

        impl Default for Event {
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
    }
}
