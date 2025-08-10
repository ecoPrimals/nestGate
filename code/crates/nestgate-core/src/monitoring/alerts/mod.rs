//! Alert Management System
//!
//! Comprehensive alerting system for NestGate monitoring including rule evaluation,
//! notification channels, escalation policies, and alert suppression.

pub mod channels;
pub mod manager;
pub mod rules;
pub mod types;

// Re-export main types for backwards compatibility
pub use channels::{AlertChannel, NotificationRecord, SmtpConfig};
pub use manager::AlertManager;
pub use rules::{AlertRule, SuppressionRule, TimeWindow};
pub use types::{Alert, AlertCondition, AlertSeverity, AlertStatus, ThresholdOperator};
