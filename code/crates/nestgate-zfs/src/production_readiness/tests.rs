// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;

#[tokio::test]
async fn test_production_readiness_assessment()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let validator = ProductionReadinessValidator::new();
    let result = validator.assess_production_readiness();

    // Should always produce a report
    assert!(result.is_ok());
    let report = result?;

    // Should have findings or be ready
    assert!(!report.findings.is_empty() || report.ready_for_production);

    // Should have recommendations
    assert!(!report.recommendations.is_empty());

    println!("Production Readiness Report generated");
    Ok(())
}

#[tokio::test]
async fn test_mock_detection() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Test mock mode detection
    nestgate_core::env_process::set_var("NESTGATE_MOCK_MODE", "true");
    let validator = ProductionReadinessValidator::new();
    let mocks = validator.identify_mock_dependencies()?;
    assert!(!mocks.is_empty());

    nestgate_core::env_process::set_var("NESTGATE_MOCK_MODE", "false");
    let mocks = validator.identify_mock_dependencies()?;
    assert!(mocks.is_empty());

    nestgate_core::env_process::remove_var("NESTGATE_MOCK_MODE");
    let mocks = validator.identify_mock_dependencies()?;
    assert!(mocks.is_empty()); // Should default to false

    Ok(())
}

#[test]
fn production_readiness_report_serde_roundtrip() {
    let report = ProductionReadinessReport {
        ready_for_production: true,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: true,
        security_validated: true,
        configuration_validated: true,
        findings: vec![ReadinessFinding {
            category: "c".to_string(),
            description: "d".to_string(),
            severity: FindingSeverity::Info,
            blocking: false,
        }],
        recommendations: vec!["r".to_string()],
    };
    let json = serde_json::to_string(&report).expect("serialize");
    let back: ProductionReadinessReport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.ready_for_production, report.ready_for_production);
    assert_eq!(back.findings.len(), 1);
}

#[test]
fn production_readiness_validator_default_matches_new() {
    let a = ProductionReadinessValidator::new();
    let b = ProductionReadinessValidator::default();
    assert_eq!(format!("{:?}", a.real_ops), format!("{:?}", b.real_ops));
}

#[test]
fn finding_severity_serde_roundtrip() {
    for sev in [
        FindingSeverity::Info,
        FindingSeverity::Warning,
        FindingSeverity::Error,
        FindingSeverity::Critical,
    ] {
        let j = serde_json::to_string(&sev).unwrap();
        let back: FindingSeverity = serde_json::from_str(&j).unwrap();
        assert_eq!(format!("{sev:?}"), format!("{back:?}"));
    }
}

#[test]
fn check_production_readiness_returns_report() {
    let r = check_production_readiness().expect("check");
    assert!(r.recommendations.len() >= 3);
}

#[test]
fn readiness_finding_serde_roundtrip() {
    let f = ReadinessFinding {
        category: "cat".into(),
        description: "desc".into(),
        severity: FindingSeverity::Warning,
        blocking: true,
    };
    let j = serde_json::to_string(&f).expect("serialize");
    let back: ReadinessFinding = serde_json::from_str(&j).expect("deserialize");
    assert_eq!(back.category, f.category);
    assert_eq!(back.blocking, f.blocking);
}

#[test]
fn real_zfs_operations_default_debug() {
    let o = RealZfsOperations::default();
    let _ = format!("{o:?}");
}

#[test]
fn validate_performance_fails_when_min_memory_below_threshold() {
    temp_env::with_var("NESTGATE_MIN_MEMORY_MB", Some("256"), || {
        let v = ProductionReadinessValidator::new();
        assert!(!v.validate_performance().expect("result"));
    });
}

#[test]
fn validate_performance_passes_when_min_memory_at_default_level() {
    temp_env::with_var("NESTGATE_MIN_MEMORY_MB", None::<&str>, || {
        let v = ProductionReadinessValidator::new();
        assert!(v.validate_performance().expect("result"));
    });
}

#[test]
fn validate_performance_passes_when_min_memory_explicitly_high() {
    temp_env::with_var("NESTGATE_MIN_MEMORY_MB", Some("2048"), || {
        let v = ProductionReadinessValidator::new();
        assert!(v.validate_performance().expect("result"));
    });
}

#[test]
fn detect_real_hardware_false_when_mock_mode_enabled() {
    temp_env::with_var("NESTGATE_MOCK_MODE", Some("true"), || {
        let v = ProductionReadinessValidator::new();
        assert!(!v.detect_real_hardware().expect("result"));
    });
}

#[test]
fn validate_security_passes_when_secure_mode_unset_or_false() {
    temp_env::with_var("NESTGATE_SECURE_MODE", None::<&str>, || {
        let v = ProductionReadinessValidator::new();
        assert!(v.validate_security().expect("result"));
    });
    temp_env::with_var("NESTGATE_SECURE_MODE", Some("false"), || {
        let v = ProductionReadinessValidator::new();
        assert!(v.validate_security().expect("result"));
    });
}

#[test]
fn validate_security_passes_when_secure_mode_unparseable() {
    temp_env::with_var("NESTGATE_SECURE_MODE", Some("not-a-bool"), || {
        let v = ProductionReadinessValidator::new();
        assert!(v.validate_security().expect("result"));
    });
}

#[test]
fn validate_min_memory_mb_invalid_env_falls_back_to_default() {
    temp_env::with_var("NESTGATE_MIN_MEMORY_MB", Some("not-a-number"), || {
        let v = ProductionReadinessValidator::new();
        assert!(v.validate_performance().expect("result"));
    });
}

#[test]
fn validate_configuration_succeeds_with_writable_temp_directories() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let data = tmp.path().join("data");
    let cfg = tmp.path().join("config");
    let data_s = data.to_str().expect("utf8");
    let cfg_s = cfg.to_str().expect("utf8");
    temp_env::with_vars(
        [
            ("NESTGATE_DATA_DIR", Some(data_s)),
            ("NESTGATE_CONFIG_DIR", Some(cfg_s)),
        ],
        || {
            let v = ProductionReadinessValidator::new();
            assert!(v.validate_configuration().expect("result"));
        },
    );
}

#[test]
fn assess_production_readiness_with_mock_mode_records_mock_dependency_finding() {
    temp_env::with_var("NESTGATE_MOCK_MODE", Some("true"), || {
        let v = ProductionReadinessValidator::new();
        let report = v.assess_production_readiness().expect("report");
        assert!(!report.mock_dependencies.is_empty());
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.category == "Mock Dependencies" && f.blocking)
        );
    });
}

#[tokio::test]
async fn real_zfs_operations_is_available_is_stable_across_calls() {
    let a = RealZfsOperations::is_available().await;
    let b = RealZfsOperations::is_available().await;
    assert_eq!(a, b);
}

#[test]
fn generate_findings_adds_zfs_availability_when_zfs_false() {
    let v = ProductionReadinessValidator::new();
    let mut report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: false,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: true,
        security_validated: true,
        configuration_validated: true,
        findings: vec![],
        recommendations: vec![],
    };
    v.generate_findings_and_recommendations(&mut report)
        .expect("findings");
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.category == "ZFS Availability" && f.blocking)
    );
}

#[test]
fn generate_findings_adds_hardware_warning_when_not_real() {
    let v = ProductionReadinessValidator::new();
    let mut report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: true,
        real_hardware_detected: false,
        mock_dependencies: vec![],
        performance_validated: true,
        security_validated: true,
        configuration_validated: true,
        findings: vec![],
        recommendations: vec![],
    };
    v.generate_findings_and_recommendations(&mut report)
        .expect("findings");
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.category == "Hardware Detection" && !f.blocking)
    );
}

#[test]
fn generate_findings_adds_performance_when_validation_fails() {
    let v = ProductionReadinessValidator::new();
    let mut report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: false,
        security_validated: true,
        configuration_validated: true,
        findings: vec![],
        recommendations: vec![],
    };
    v.generate_findings_and_recommendations(&mut report)
        .expect("findings");
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.category == "Performance" && !f.blocking)
    );
}

#[test]
fn generate_findings_adds_security_when_validation_fails() {
    let v = ProductionReadinessValidator::new();
    let mut report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: true,
        security_validated: false,
        configuration_validated: true,
        findings: vec![],
        recommendations: vec![],
    };
    v.generate_findings_and_recommendations(&mut report)
        .expect("findings");
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.category == "Security" && f.blocking)
    );
}

#[test]
fn generate_findings_adds_configuration_when_validation_fails() {
    let v = ProductionReadinessValidator::new();
    let mut report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec![],
        performance_validated: true,
        security_validated: true,
        configuration_validated: false,
        findings: vec![],
        recommendations: vec![],
    };
    v.generate_findings_and_recommendations(&mut report)
        .expect("findings");
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.category == "Configuration" && f.blocking)
    );
}

#[test]
fn check_zfs_availability_matches_proc_path() {
    let v = ProductionReadinessValidator::new();
    let expected = std::path::Path::new("/proc/spl/kstat/zfs").exists();
    assert_eq!(v.check_zfs_availability().expect("zfs"), expected);
}

#[test]
fn generate_findings_adds_mock_dependencies_when_mock_list_nonempty() {
    let v = ProductionReadinessValidator::new();
    let mut report = ProductionReadinessReport {
        ready_for_production: false,
        zfs_available: true,
        real_hardware_detected: true,
        mock_dependencies: vec!["Mock mode enabled".into()],
        performance_validated: true,
        security_validated: true,
        configuration_validated: true,
        findings: vec![],
        recommendations: vec![],
    };
    v.generate_findings_and_recommendations(&mut report)
        .expect("findings");
    assert!(
        report
            .findings
            .iter()
            .any(|f| f.category == "Mock Dependencies" && f.blocking)
    );
}
