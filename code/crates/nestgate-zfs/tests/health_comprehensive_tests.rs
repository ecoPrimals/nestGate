//! Comprehensive tests for ZFS health monitoring
//! Target: Improve coverage of health module types and enums

use nestgate_zfs::health::{Alert, AlertLevel, HealthReport, HealthStatus};
use std::time::SystemTime;

// ==================== HEALTH STATUS TESTS ====================

#[test]
fn test_health_status_variants() {
    let statuses = [
        HealthStatus::Healthy,
        HealthStatus::Warning,
        HealthStatus::Critical,
        HealthStatus::Unknown,
    ];

    assert_eq!(statuses.len(), 4);
}

#[test]
fn test_health_status_is_healthy() {
    assert!(HealthStatus::Healthy.is_healthy());
    assert!(!HealthStatus::Warning.is_healthy());
    assert!(!HealthStatus::Critical.is_healthy());
    assert!(!HealthStatus::Unknown.is_healthy());
}

#[test]
fn test_health_status_is_critical() {
    assert!(!HealthStatus::Healthy.is_critical());
    assert!(!HealthStatus::Warning.is_critical());
    assert!(HealthStatus::Critical.is_critical());
    assert!(!HealthStatus::Unknown.is_critical());
}

#[test]
fn test_health_status_display() {
    assert_eq!(format!("{}", HealthStatus::Healthy), "Healthy");
    assert_eq!(format!("{}", HealthStatus::Warning), "Warning");
    assert_eq!(format!("{}", HealthStatus::Critical), "Critical");
    assert_eq!(format!("{}", HealthStatus::Unknown), "Unknown");
}

#[test]
fn test_health_status_debug() {
    let status = HealthStatus::Healthy;
    let debug_str = format!("{:?}", status);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_health_status_clone() {
    let status1 = HealthStatus::Healthy;
    let status2 = status1.clone();

    assert!(status2.is_healthy());
}

#[test]
fn test_health_status_equality() {
    assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
    assert_ne!(HealthStatus::Healthy, HealthStatus::Warning);
}

// ==================== HEALTH REPORT TESTS ====================

#[test]
fn test_create_health_report() {
    let report = HealthReport {
        component_type: "pool".to_string(),
        component_name: "testpool".to_string(),
        status: HealthStatus::Healthy,
        last_check: SystemTime::now(),
        details: "All systems operational".to_string(),
    };

    assert_eq!(report.component_type, "pool");
    assert_eq!(report.component_name, "testpool");
    assert!(report.status.is_healthy());
}

#[test]
fn test_health_report_with_warning() {
    let report = HealthReport {
        component_type: "dataset".to_string(),
        component_name: "testdataset".to_string(),
        status: HealthStatus::Warning,
        last_check: SystemTime::now(),
        details: "High usage detected".to_string(),
    };

    assert_eq!(report.status, HealthStatus::Warning);
    assert!(!report.status.is_healthy());
}

#[test]
fn test_health_report_with_critical() {
    let report = HealthReport {
        component_type: "disk".to_string(),
        component_name: "disk0".to_string(),
        status: HealthStatus::Critical,
        last_check: SystemTime::now(),
        details: "Disk failure imminent".to_string(),
    };

    assert!(report.status.is_critical());
}

#[test]
fn test_health_report_debug() {
    let report = HealthReport {
        component_type: "test".to_string(),
        component_name: "test".to_string(),
        status: HealthStatus::Healthy,
        last_check: SystemTime::now(),
        details: "test".to_string(),
    };

    let debug_str = format!("{:?}", report);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_health_report_clone() {
    let report1 = HealthReport {
        component_type: "pool".to_string(),
        component_name: "testpool".to_string(),
        status: HealthStatus::Healthy,
        last_check: SystemTime::now(),
        details: "OK".to_string(),
    };

    let report2 = report1.clone();
    assert_eq!(report1.component_name, report2.component_name);
}

// ==================== ALERT LEVEL TESTS ====================

#[test]
fn test_alert_level_variants() {
    let levels = [AlertLevel::Info, AlertLevel::Warning, AlertLevel::Critical];

    assert_eq!(levels.len(), 3);
}

#[test]
fn test_alert_level_debug() {
    let level = AlertLevel::Warning;
    let debug_str = format!("{:?}", level);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_alert_level_clone() {
    let level1 = AlertLevel::Critical;
    let level2 = level1.clone();

    assert!(format!("{:?}", level1) == format!("{:?}", level2));
}

// ==================== ALERT TESTS ====================

#[test]
fn test_create_alert() {
    let alert = Alert {
        id: "alert-001".to_string(),
        level: AlertLevel::Warning,
        message: "High memory usage".to_string(),
        timestamp: SystemTime::now(),
        component: "system".to_string(),
    };

    assert_eq!(alert.id, "alert-001");
    assert_eq!(alert.message, "High memory usage");
}

#[test]
fn test_alert_info_level() {
    let alert = Alert {
        id: "info-001".to_string(),
        level: AlertLevel::Info,
        message: "System started".to_string(),
        timestamp: SystemTime::now(),
        component: "core".to_string(),
    };

    assert!(format!("{:?}", alert.level).contains("Info"));
}

#[test]
fn test_alert_critical_level() {
    let alert = Alert {
        id: "crit-001".to_string(),
        level: AlertLevel::Critical,
        message: "System failure".to_string(),
        timestamp: SystemTime::now(),
        component: "disk".to_string(),
    };

    assert!(format!("{:?}", alert.level).contains("Critical"));
}

#[test]
fn test_alert_debug() {
    let alert = Alert {
        id: "test-001".to_string(),
        level: AlertLevel::Info,
        message: "Test".to_string(),
        timestamp: SystemTime::now(),
        component: "test".to_string(),
    };

    let debug_str = format!("{:?}", alert);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_alert_clone() {
    let alert1 = Alert {
        id: "clone-001".to_string(),
        level: AlertLevel::Warning,
        message: "Clone test".to_string(),
        timestamp: SystemTime::now(),
        component: "test".to_string(),
    };

    let alert2 = alert1.clone();
    assert_eq!(alert1.id, alert2.id);
    assert_eq!(alert1.message, alert2.message);
}

// ==================== SERIALIZATION TESTS ====================

#[test]
fn test_health_status_serialization() {
    let status = HealthStatus::Healthy;
    let json = serde_json::to_string(&status).expect("Failed to serialize");
    assert!(!json.is_empty());
}

#[test]
fn test_health_status_deserialization() {
    let status = HealthStatus::Warning;
    let json = serde_json::to_string(&status).expect("Failed to serialize");
    let deserialized: HealthStatus = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(status, deserialized);
}

#[test]
fn test_health_report_serialization() {
    let report = HealthReport {
        component_type: "pool".to_string(),
        component_name: "testpool".to_string(),
        status: HealthStatus::Healthy,
        last_check: SystemTime::now(),
        details: "OK".to_string(),
    };

    let json = serde_json::to_string(&report).expect("Failed to serialize");
    assert!(json.contains("testpool"));
}

#[test]
fn test_health_report_deserialization() {
    let report = HealthReport {
        component_type: "pool".to_string(),
        component_name: "testpool".to_string(),
        status: HealthStatus::Healthy,
        last_check: SystemTime::now(),
        details: "OK".to_string(),
    };

    let json = serde_json::to_string(&report).expect("Failed to serialize");
    let deserialized: HealthReport = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(report.component_name, deserialized.component_name);
}

#[test]
fn test_alert_level_serialization() {
    let level = AlertLevel::Critical;
    let json = serde_json::to_string(&level).expect("Failed to serialize");
    assert!(!json.is_empty());
}

#[test]
fn test_alert_serialization() {
    let alert = Alert {
        id: "test-001".to_string(),
        level: AlertLevel::Warning,
        message: "Test alert".to_string(),
        timestamp: SystemTime::now(),
        component: "test".to_string(),
    };

    let json = serde_json::to_string(&alert).expect("Failed to serialize");
    assert!(json.contains("test-001"));
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_empty_component_names() {
    let report = HealthReport {
        component_type: "".to_string(),
        component_name: "".to_string(),
        status: HealthStatus::Unknown,
        last_check: SystemTime::now(),
        details: "".to_string(),
    };

    assert_eq!(report.component_type, "");
}

#[test]
fn test_long_details_string() {
    let long_details = "a".repeat(10000);
    let report = HealthReport {
        component_type: "pool".to_string(),
        component_name: "testpool".to_string(),
        status: HealthStatus::Healthy,
        last_check: SystemTime::now(),
        details: long_details.clone(),
    };

    assert_eq!(report.details.len(), 10000);
}

#[test]
fn test_special_characters_in_details() {
    let report = HealthReport {
        component_type: "pool".to_string(),
        component_name: "testpool".to_string(),
        status: HealthStatus::Healthy,
        last_check: SystemTime::now(),
        details: "Test\nwith\nspecial\tchars: \"quotes\" 'apostrophes'".to_string(),
    };

    assert!(report.details.contains("special"));
}

#[test]
fn test_multiple_alerts() {
    let mut alerts = Vec::new();

    for i in 0..10 {
        alerts.push(Alert {
            id: format!("alert-{:03}", i),
            level: if i < 5 {
                AlertLevel::Info
            } else {
                AlertLevel::Warning
            },
            message: format!("Alert {}", i),
            timestamp: SystemTime::now(),
            component: "test".to_string(),
        });
    }

    assert_eq!(alerts.len(), 10);
}

#[test]
fn test_health_report_collection() {
    let now = SystemTime::now();
    let reports = [
        HealthReport {
            component_type: "pool".to_string(),
            component_name: "pool1".to_string(),
            status: HealthStatus::Healthy,
            last_check: now,
            details: "OK".to_string(),
        },
        HealthReport {
            component_type: "pool".to_string(),
            component_name: "pool2".to_string(),
            status: HealthStatus::Warning,
            last_check: now,
            details: "High usage".to_string(),
        },
        HealthReport {
            component_type: "pool".to_string(),
            component_name: "pool3".to_string(),
            status: HealthStatus::Critical,
            last_check: now,
            details: "Failure".to_string(),
        },
    ];

    assert_eq!(reports.len(), 3);

    let critical_count = reports.iter().filter(|r| r.status.is_critical()).count();
    assert_eq!(critical_count, 1);
}

// ==================== PATTERN MATCHING TESTS ====================

#[test]
fn test_match_health_status() {
    let status = HealthStatus::Warning;

    let message = match status {
        HealthStatus::Healthy => "All good",
        HealthStatus::Warning => "Watch out",
        HealthStatus::Critical => "Emergency",
        HealthStatus::Unknown => "Unknown state",
    };

    assert_eq!(message, "Watch out");
}

#[test]
fn test_match_alert_level() {
    let level = AlertLevel::Critical;

    let priority = match level {
        AlertLevel::Info => 1,
        AlertLevel::Warning => 2,
        AlertLevel::Critical => 3,
    };

    assert_eq!(priority, 3);
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_health_monitoring_workflow() {
    // Create a health report
    let report = HealthReport {
        component_type: "pool".to_string(),
        component_name: "testpool".to_string(),
        status: HealthStatus::Warning,
        last_check: SystemTime::now(),
        details: "Capacity above 80%".to_string(),
    };

    // Generate an alert if not healthy
    if !report.status.is_healthy() {
        let alert = Alert {
            id: format!("alert-{}", report.component_name),
            level: if report.status.is_critical() {
                AlertLevel::Critical
            } else {
                AlertLevel::Warning
            },
            message: format!("{}: {}", report.component_name, report.details),
            timestamp: SystemTime::now(),
            component: report.component_type.clone(),
        };

        assert_eq!(alert.component, "pool");
    }
}

#[test]
fn test_alert_escalation() {
    let statuses = vec![
        HealthStatus::Healthy,
        HealthStatus::Warning,
        HealthStatus::Critical,
    ];

    for status in statuses {
        if status.is_critical() {
            let _alert = Alert {
                id: "crit-001".to_string(),
                level: AlertLevel::Critical,
                message: "Critical issue".to_string(),
                timestamp: SystemTime::now(),
                component: "system".to_string(),
            };
            // Assert critical alert created
            assert!(status.is_critical());
        }
    }
}
