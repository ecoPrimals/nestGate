// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Tests for ZFS health monitoring functionality

use super::health::*;
use crate::{config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager};
use std::sync::Arc;
use std::time::SystemTime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_is_critical() {
        assert!(HealthStatus::Critical.is_critical());
        assert!(!HealthStatus::Healthy.is_critical());
        assert!(!HealthStatus::Warning.is_critical());
        assert!(!HealthStatus::Unknown.is_critical());
    }

    #[test]
    fn test_health_status_is_healthy() {
        assert!(HealthStatus::Healthy.is_healthy());
        assert!(!HealthStatus::Critical.is_healthy());
        assert!(!HealthStatus::Warning.is_healthy());
        assert!(!HealthStatus::Unknown.is_healthy());
    }

    #[test]
    fn test_health_status_display() {
        assert_eq!(format!("{}", HealthStatus::Healthy), "Healthy");
        assert_eq!(format!("{}", HealthStatus::Warning), "Warning");
        assert_eq!(format!("{}", HealthStatus::Critical), "Critical");
        assert_eq!(format!("{}", HealthStatus::Unknown), "Unknown");
    }

    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_eq!(HealthStatus::Warning, HealthStatus::Warning);
        assert_eq!(HealthStatus::Critical, HealthStatus::Critical);
        assert_eq!(HealthStatus::Unknown, HealthStatus::Unknown);

        assert_ne!(HealthStatus::Healthy, HealthStatus::Warning);
        assert_ne!(HealthStatus::Critical, HealthStatus::Unknown);
    }

    #[test]
    fn test_health_report_creation() {
        let report = HealthReport {
            component_type: "pool".to_string(),
            component_name: "test-pool".to_string(),
            status: HealthStatus::Healthy,
            last_check: SystemTime::now(),
            details: "All systems operational".to_string(),
        };

        assert_eq!(report.component_type, "pool");
        assert_eq!(report.component_name, "test-pool");
        assert_eq!(report.status, HealthStatus::Healthy);
        assert_eq!(report.details, "All systems operational");
    }

    #[test]
    fn test_alert_level_variants() {
        let info = Alert {
            id: "test-1".to_string(),
            level: AlertLevel::Info,
            message: "Info message".to_string(),
            timestamp: SystemTime::now(),
            component: "test".to_string(),
        };

        let warning = Alert {
            id: "test-2".to_string(),
            level: AlertLevel::Warning,
            message: "Warning message".to_string(),
            timestamp: SystemTime::now(),
            component: "test".to_string(),
        };

        let critical = Alert {
            id: "test-3".to_string(),
            level: AlertLevel::Critical,
            message: "Critical message".to_string(),
            timestamp: SystemTime::now(),
            component: "test".to_string(),
        };

        assert_eq!(info.id, "test-1");
        assert_eq!(warning.id, "test-2");
        assert_eq!(critical.id, "test-3");
    }

    #[test]
    fn test_health_monitor_creation() {
        let config = ZfsConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
        let dataset_manager =
            Arc::new(ZfsDatasetManager::new(config.clone(), pool_manager.clone()));

        let result = ZfsHealthMonitor::new(pool_manager, dataset_manager);

        // Just verify it was created without panicking
        assert!(result.is_ok());
    }

    #[test]
    fn test_health_report_clone() {
        let report = HealthReport {
            component_type: "dataset".to_string(),
            component_name: "test-dataset".to_string(),
            status: HealthStatus::Warning,
            last_check: SystemTime::now(),
            details: "Low disk space".to_string(),
        };

        let cloned = report.clone();
        assert_eq!(report.component_type, cloned.component_type);
        assert_eq!(report.component_name, cloned.component_name);
        assert_eq!(report.status, cloned.status);
        assert_eq!(report.details, cloned.details);
    }

    #[test]
    fn test_alert_clone() {
        let alert = Alert {
            id: "alert-1".to_string(),
            level: AlertLevel::Critical,
            message: "Disk failure detected".to_string(),
            timestamp: SystemTime::now(),
            component: "pool-1".to_string(),
        };

        let cloned = alert.clone();
        assert_eq!(alert.id, cloned.id);
        assert_eq!(alert.message, cloned.message);
        assert_eq!(alert.component, cloned.component);
    }

    #[test]
    fn test_health_status_pattern_matching() {
        let status = HealthStatus::Critical;

        let message = match status {
            HealthStatus::Healthy => "OK",
            HealthStatus::Warning => "Warning",
            HealthStatus::Critical => "Critical",
            HealthStatus::Unknown => "Unknown",
        };

        assert_eq!(message, "Critical");
    }

    #[test]
    fn test_alert_level_pattern_matching() {
        let level = AlertLevel::Warning;

        let priority = match level {
            AlertLevel::Info => 1,
            AlertLevel::Warning => 2,
            AlertLevel::Critical => 3,
        };

        assert_eq!(priority, 2);
    }

    #[test]
    fn test_health_monitor_creation_and_drop() {
        let config = ZfsConfig::default();
        let pool_manager = Arc::new(ZfsPoolManager::new_production(config.clone()));
        let dataset_manager =
            Arc::new(ZfsDatasetManager::new(config.clone(), pool_manager.clone()));

        // Create
        let result = ZfsHealthMonitor::new(pool_manager, dataset_manager);
        assert!(result.is_ok());

        // Drop (cleanup)
        if let Ok(monitor) = result {
            drop(monitor);
        }

        // Test passed without panic - no assertion needed
    }

    #[test]
    fn test_health_report_with_different_statuses() {
        let statuses = vec![
            HealthStatus::Healthy,
            HealthStatus::Warning,
            HealthStatus::Critical,
            HealthStatus::Unknown,
        ];

        for status in statuses {
            let report = HealthReport {
                component_type: "test".to_string(),
                component_name: "component".to_string(),
                status: status.clone(),
                last_check: SystemTime::now(),
                details: "Test".to_string(),
            };

            assert_eq!(report.status, status);
        }
    }

    #[test]
    fn test_alert_with_different_levels() {
        let levels = vec![AlertLevel::Info, AlertLevel::Warning, AlertLevel::Critical];

        for (i, level) in levels.into_iter().enumerate() {
            let alert = Alert {
                id: format!("alert-{}", i),
                /// Level
                level,
                message: "Test message".to_string(),
                timestamp: SystemTime::now(),
                component: "test".to_string(),
            };

            assert_eq!(alert.id, format!("alert-{}", i));
        }
    }

    #[test]
    fn health_report_serde_roundtrip() {
        let report = HealthReport {
            component_type: "pool".into(),
            component_name: "tank".into(),
            status: HealthStatus::Healthy,
            last_check: SystemTime::UNIX_EPOCH,
            details: "ok".into(),
        };
        let json = serde_json::to_string(&report).expect("serialize HealthReport");
        let back: HealthReport = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.component_name, report.component_name);
        assert_eq!(back.status, report.status);
    }

    #[test]
    fn degradation_detection_via_status_ordering() {
        let worst = [
            HealthStatus::Healthy,
            HealthStatus::Warning,
            HealthStatus::Critical,
        ]
        .into_iter()
        .max_by_key(|s| match s {
            HealthStatus::Critical => 3,
            HealthStatus::Warning => 2,
            HealthStatus::Healthy => 1,
            HealthStatus::Unknown => 0,
        });
        assert_eq!(worst, Some(HealthStatus::Critical));
    }
}
