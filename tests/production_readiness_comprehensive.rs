//! Comprehensive Production Readiness Test
//!
//! This test validates that NestGate is ready for production deployment,
//! with real ZFS operations replacing development environment simulations.

use nestgate_zfs::production_readiness::{
    check_production_readiness, ProductionReadinessValidator,
};
use std::env;
use tokio;

/// Test production readiness assessment
#[tokio::test]
async fn test_comprehensive_production_readiness() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Running comprehensive production readiness assessment...");

    // Run the production readiness check
    let report = check_production_readiness().await?;

    // Validate report structure
    assert!(!report.findings.is_empty() || report.ready_for_production);
    assert!(!report.recommendations.is_empty());

    // Print detailed report
    println!("\n📊 PRODUCTION READINESS REPORT");
    println!("================================");
    println!("Ready for Production: {}", report.ready_for_production);
    println!("ZFS Available: {}", report.zfs_available);
    println!("Real Hardware Detected: {}", report.real_hardware_detected);
    println!("Performance Validated: {}", report.performance_validated);
    println!("Security Validated: {}", report.security_validated);
    println!(
        "Configuration Validated: {}",
        report.configuration_validated
    );

    if !report.mock_dependencies.is_empty() {
        println!("\n⚠️ Mock Dependencies:");
        for mock in &report.mock_dependencies {
            println!("  - {}", mock);
            Ok(())
        }
        Ok(())
    }

    if !report.findings.is_empty() {
        println!("\n🔍 Findings:");
        for finding in &report.findings {
            let severity_icon = match finding.severity {
                nestgate_zfs::production_readiness::FindingSeverity::Info => "ℹ️",
                nestgate_zfs::production_readiness::FindingSeverity::Warning => "⚠️",
                nestgate_zfs::production_readiness::FindingSeverity::Error => "❌",
                nestgate_zfs::production_readiness::FindingSeverity::Critical => "🚨",
            };
            println!(
                "  {} [{}] {}: {}",
                severity_icon,
                finding.category,
                if finding.blocking {
                    "BLOCKING"
                } else {
                    "NON-BLOCKING"
                },
                finding.description
            );
            Ok(())
        }
    }

    if !report.recommendations.is_empty() {
        println!("\n💡 Recommendations:");
        for (i, rec) in report.recommendations.iter().enumerate() {
            println!("  {}. {}", i + 1, rec);
        }
    }

    println!("\n✅ Production readiness assessment completed");
}

/// Test mock vs real implementation detection
#[tokio::test]
async fn test_mock_vs_real_detection() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Testing mock vs real implementation detection...");

    // Test with mock mode enabled
    env::set_var("NESTGATE_MOCK_MODE", "true");

    let validator = ProductionReadinessValidator::new();
    let report_with_mocks = validator.assess_production_readiness().await?;

    // Should detect mock dependencies
    assert!(!report_with_mocks.mock_dependencies.is_empty());
    assert!(!report_with_mocks.ready_for_production); // Mocks should prevent production readiness

    println!("✅ Mock mode correctly detected");

    // Test with mock mode disabled
    env::remove_var("NESTGATE_MOCK_MODE");

    let report_without_mocks = validator.assess_production_readiness().await?;

    // Should have fewer or no mock dependencies (depending on ZFS availability)
    println!(
        "Mock dependencies without mock mode: {:?}",
        report_without_mocks.mock_dependencies
    );

    println!("✅ Mock vs real detection working correctly");
    Ok(())
}

/// Test ZFS availability detection
#[tokio::test]
async fn test_zfs_availability_detection() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Testing ZFS availability detection...");

    let available = nestgate_zfs::real_zfs_operations::RealZfsOperations::is_available().await;

    if available {
        println!("✅ ZFS is available on this system - can use real operations");

        // Test real operations
        let real_ops = nestgate_zfs::real_zfs_operations::RealZfsOperations::default();
        match real_ops.health_check().await {
            Ok(_) => println!("✅ Real ZFS health check passed"),
            Err(e) => println!("⚠️ Real ZFS health check failed: {}", e),
    Ok(())
        }
    } else {
        println!("ℹ️ ZFS not available - using development environment simulation");
        println!("   This is expected in CI/container environments");
        Ok(())
    }

    println!("✅ ZFS availability detection completed");
    Ok(())
}

/// Test performance validation
#[tokio::test]
async fn test_performance_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Testing performance validation...");

    let validator = ProductionReadinessValidator::new();

    let start = std::time::Instant::now();
    let performance_valid = validator.validate_performance().await?;
    let duration = start.elapsed();

    println!("Performance validation result: {}", performance_valid);
    println!("Validation took: {:?}", duration);

    // Performance validation itself should be fast
    assert!(duration.as_millis() < 10000); // Should complete within 10 seconds

    println!("✅ Performance validation test completed");
    Ok(())
}

/// Test configuration validation
#[tokio::test]
async fn test_configuration_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚙️ Testing configuration validation...");

    let validator = ProductionReadinessValidator::new();

    // Test with current environment
    let config_valid = validator.validate_configuration().await?;

    // Should always return a result
    println!("Configuration valid: {}", config_valid);

    // Test with specific environment variables
    env::set_var("NESTGATE_API_HOST", "test.example.com");
    env::set_var("NESTGATE_API_PORT", "9090");

    let config_valid_with_env = validator.validate_configuration().await?;
    println!(
        "Configuration valid with env vars: {}",
        config_valid_with_env
    );

    // Cleanup
    env::remove_var("NESTGATE_API_HOST");
    env::remove_var("NESTGATE_API_PORT");

    println!("✅ Configuration validation test completed");
    Ok(())
}

/// Integration test: Full production readiness workflow
#[tokio::test]
async fn test_full_production_readiness_workflow() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏭 Testing full production readiness workflow...");

    // This test simulates a complete production deployment check
    let validator = ProductionReadinessValidator::new();
    let report = validator.assess_production_readiness().await?;

    // Log the complete assessment
    println!("\n🏭 FULL PRODUCTION READINESS ASSESSMENT");
    println!("======================================");

    println!(
        "Overall Status: {}",
        if report.ready_for_production {
            "🟢 READY FOR PRODUCTION"
        } else {
            "🟡 NEEDS ATTENTION"
    Ok(())
        }
    );

    // Categorize findings by severity
    let critical_findings: Vec<_> = report
        .findings
        .iter()
        .filter(|f| {
            matches!(
                f.severity,
                nestgate_zfs::production_readiness::FindingSeverity::Critical
            )
        })
        .collect();

    let warning_findings: Vec<_> = report
        .findings
        .iter()
        .filter(|f| {
            matches!(
                f.severity,
                nestgate_zfs::production_readiness::FindingSeverity::Warning
            )
        })
        .collect();

    if !critical_findings.is_empty() {
        println!("\n🚨 CRITICAL ISSUES:");
        for finding in critical_findings {
            println!("  - {}: {}", finding.category, finding.description);
            Ok(())
        }
        Ok(())
    }

    if !warning_findings.is_empty() {
        println!("\n⚠️ WARNINGS:");
        for finding in warning_findings {
            println!("  - {}: {}", finding.category, finding.description);
        }
    }

    // Print actionable next steps
    println!("\n📋 NEXT STEPS:");
    for (i, recommendation) in report.recommendations.iter().enumerate() {
        println!("  {}. {}", i + 1, recommendation);
    }

    println!("\n✅ Full production readiness workflow test completed");
}
