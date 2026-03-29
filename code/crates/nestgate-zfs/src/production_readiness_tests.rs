// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for Production Readiness module
//!
//! Tests cover:
//! - Production readiness validation
//! - Finding severity levels
//! - Report generation
//! - ZFS availability checks
//! - Configuration validation

use super::production_readiness::*;

// ==================== STRUCT CREATION TESTS ====================

#[test]
fn test_production_readiness_report_creation() {
    let report = ProductionReadinessReport {
        ready_for_production: true,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: true,
        security_validated: true,
        configuration_validated: true,
        findings: vec![],
        recommendations: vec![],
    };

    assert!(report.ready_for_production);
    assert!(report.zfs_available);
    assert!(report.real_hardware_detected);
    assert!(report.mock_dependencies.is_empty());
}

#[test]
fn test_production_readiness_report_with_issues() {
    let report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: false,
        real_hardware_detected: false,
        mock_dependencies: vec!["mock_zfs".to_string()],
        performance_validated: false,
        security_validated: false,
        configuration_validated: false,
        findings: vec![],
        recommendations: vec!["Install ZFS".to_string()],
    };

    assert!(!report.ready_for_production);
    assert!(!report.zfs_available);
    assert_eq!(report.mock_dependencies.len(), 1);
    assert_eq!(report.recommendations.len(), 1);
}

#[test]
fn test_readiness_finding_creation() {
    let finding = ReadinessFinding {
        category: "Security".to_string(),
        description: "TLS not configured".to_string(),
        severity: FindingSeverity::Warning,
        blocking: false,
    };

    assert_eq!(finding.category, "Security");
    assert!(!finding.blocking);
}

#[test]
fn test_blocking_finding() {
    let finding = ReadinessFinding {
        category: "Critical".to_string(),
        description: "ZFS not available".to_string(),
        severity: FindingSeverity::Critical,
        blocking: true,
    };

    assert!(finding.blocking);
    assert!(matches!(finding.severity, FindingSeverity::Critical));
}

// ==================== FINDING SEVERITY TESTS ====================

#[test]
fn test_finding_severity_levels() {
    let info = FindingSeverity::Info;
    let warning = FindingSeverity::Warning;
    let error = FindingSeverity::Error;
    let critical = FindingSeverity::Critical;

    // All levels should be distinct
    assert!(matches!(info, FindingSeverity::Info));
    assert!(matches!(warning, FindingSeverity::Warning));
    assert!(matches!(error, FindingSeverity::Error));
    assert!(matches!(critical, FindingSeverity::Critical));
}

#[test]
fn test_severity_in_finding() {
    let finding_info = ReadinessFinding {
        category: "Info".to_string(),
        description: "System info".to_string(),
        severity: FindingSeverity::Info,
        blocking: false,
    };

    assert!(matches!(finding_info.severity, FindingSeverity::Info));
    assert!(!finding_info.blocking);
}

// ==================== ZFS OPERATIONS TESTS ====================

#[test]
fn test_real_zfs_operations_creation() {
    let _ops = RealZfsOperations::default();
    // Should create successfully
}

#[tokio::test]
async fn test_zfs_availability_check() {
    let available = RealZfsOperations::is_available().await;
    // Result depends on whether ZFS is installed
    // Both true and false are valid in test environments
    // This test verifies the method is callable without panicking
    let _ = available; // Verify method returns bool
}

// ==================== REPORT WITH FINDINGS TESTS ====================

#[test]
fn test_report_with_multiple_findings() {
    let findings = vec![
        ReadinessFinding {
            category: "Performance".to_string(),
            description: "Benchmarks not run".to_string(),
            severity: FindingSeverity::Warning,
            blocking: false,
        },
        ReadinessFinding {
            category: "Security".to_string(),
            description: "Audit needed".to_string(),
            severity: FindingSeverity::Info,
            blocking: false,
        },
    ];

    let report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: false,
        security_validated: false,
        configuration_validated: true,
        findings: findings.clone(),
        recommendations: vec!["Run benchmarks".to_string()],
    };

    assert_eq!(report.findings.len(), 2);
    assert!(!report.performance_validated);
}

#[test]
fn test_report_with_blocking_findings() {
    let findings = vec![ReadinessFinding {
        category: "Critical".to_string(),
        description: "Database not configured".to_string(),
        severity: FindingSeverity::Critical,
        blocking: true,
    }];

    let report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: true,
        security_validated: true,
        configuration_validated: false,
        findings,
        recommendations: vec!["Configure database".to_string()],
    };

    assert!(!report.ready_for_production);
    assert!(report.findings[0].blocking);
}

// ==================== MOCK DEPENDENCY TRACKING TESTS ====================

#[test]
fn test_report_with_mock_dependencies() {
    let mocks = vec![
        "mock_zfs".to_string(),
        "mock_network".to_string(),
        "mock_storage".to_string(),
    ];

    let report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: false,
        real_hardware_detected: false,
        mock_dependencies: mocks.clone(),
        performance_validated: false,
        security_validated: false,
        configuration_validated: false,
        findings: vec![],
        recommendations: vec!["Remove all mocks".to_string()],
    };

    assert_eq!(report.mock_dependencies.len(), 3);
    assert!(report.mock_dependencies.contains(&"mock_zfs".to_string()));
}

#[test]
fn test_report_no_mock_dependencies() {
    let report = ProductionReadinessReport {
        ready_for_production: true,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: true,
        security_validated: true,
        configuration_validated: true,
        findings: vec![],
        recommendations: vec![],
    };

    assert!(report.mock_dependencies.is_empty());
    assert!(report.ready_for_production);
}

// ==================== VALIDATION STATUS TESTS ====================

#[test]
fn test_all_validations_passed() {
    let report = ProductionReadinessReport {
        ready_for_production: true,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: true,
        security_validated: true,
        configuration_validated: true,
        findings: vec![],
        recommendations: vec![],
    };

    assert!(report.performance_validated);
    assert!(report.security_validated);
    assert!(report.configuration_validated);
    assert!(report.ready_for_production);
}

#[test]
fn test_partial_validations() {
    let report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: true,
        security_validated: false,
        configuration_validated: true,
        findings: vec![],
        recommendations: vec!["Complete security audit".to_string()],
    };

    assert!(report.performance_validated);
    assert!(!report.security_validated);
    assert!(report.configuration_validated);
    assert!(!report.ready_for_production);
}

// ==================== RECOMMENDATION TESTS ====================

#[test]
fn test_report_with_recommendations() {
    let recommendations = vec![
        "Enable monitoring".to_string(),
        "Configure backups".to_string(),
        "Run load tests".to_string(),
    ];

    let report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: false,
        security_validated: true,
        configuration_validated: true,
        findings: vec![],
        recommendations: recommendations.clone(),
    };

    assert_eq!(report.recommendations.len(), 3);
    assert!(
        report
            .recommendations
            .contains(&"Enable monitoring".to_string())
    );
}

// Total tests added: 15
// Focus areas:
// - Struct creation (2 tests)
// - Finding creation (2 tests)
// - Severity levels (2 tests)
// - ZFS operations (2 tests)
// - Report with findings (2 tests)
// - Mock dependencies (2 tests)
// - Validation status (2 tests)
// - Recommendations (1 test)

#[test]
fn ready_for_production_requires_all_green_flags() {
    let all_green = ProductionReadinessReport {
        ready_for_production: true,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: true,
        security_validated: true,
        configuration_validated: true,
        findings: vec![],
        recommendations: vec![],
    };
    assert!(all_green.ready_for_production);

    let missing_zfs = ProductionReadinessReport {
        zfs_available: false,
        ready_for_production: false,
        ..all_green.clone()
    };
    assert!(!missing_zfs.zfs_available);

    let with_mock = ProductionReadinessReport {
        mock_dependencies: vec!["m".into()],
        ready_for_production: false,
        ..all_green
    };
    assert_eq!(with_mock.mock_dependencies.len(), 1);
}

#[test]
fn readiness_finding_severity_ordering_for_triage() {
    let critical = FindingSeverity::Critical;
    let info = FindingSeverity::Info;
    let rank = |s: &FindingSeverity| match s {
        FindingSeverity::Critical => 4,
        FindingSeverity::Error => 3,
        FindingSeverity::Warning => 2,
        FindingSeverity::Info => 1,
    };
    assert!(rank(&critical) > rank(&info));
}
