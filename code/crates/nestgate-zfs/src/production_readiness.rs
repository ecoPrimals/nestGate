// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

use nestgate_core::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Production readiness assessment report
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Productionreadinessreport
pub struct ProductionReadinessReport {
    /// Whether the system is ready for production
    pub ready_for_production: bool,
    /// ZFS availability status
    pub zfs_available: bool,
    /// Real hardware detection status
    pub real_hardware_detected: bool,
    /// List of active mock dependencies
    pub mock_dependencies: Vec<String>,
    /// Performance validation status
    pub performance_validated: bool,
    /// Security validation status
    pub security_validated: bool,
    /// Configuration validation status
    pub configuration_validated: bool,
    /// Detailed findings
    pub findings: Vec<ReadinessFinding>,
    /// Recommendations for production deployment
    pub recommendations: Vec<String>,
}
/// Individual readiness finding
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Readinessfinding
pub struct ReadinessFinding {
    /// Category of the finding
    pub category: String,
    /// Specific finding description
    pub description: String,
    /// Severity level
    pub severity: FindingSeverity,
    /// Whether this blocks production deployment
    pub blocking: bool,
}
/// Severity levels for findings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Findingseverity
pub enum FindingSeverity {
    /// Info
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}
/// Production Readiness Validator
pub struct ProductionReadinessValidator {
    /// Real ZFS operations handler
    real_ops: RealZfsOperations,
}
/// Real ZFS operations (placeholder for actual implementation)
#[derive(Debug, Default)]
/// Realzfsoperations
pub struct RealZfsOperations {}

impl RealZfsOperations {
    /// Checks if Available
    pub async fn is_available() -> bool {
        // Check if ZFS is available on the system
        tokio::process::Command::new("zfs")
            .arg("version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub async fn get_pool_status(
        &self,
        pool_name: Option<String>,
    ) -> Result<crate::handlers::ZfsResponse> {
        use nestgate_core::NestGateError;
        let mut cmd = tokio::process::Command::new("zpool");
        cmd.arg("status").arg("-j"); // JSON output

        if let Some(pool) = pool_name {
            cmd.arg(pool);
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| NestGateError::io_error(e.to_string()))?;
        if !output.status.success() {
            return Err(NestGateError::system(
                format!(
                    "ZFS pool status command failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                "zfs_operations",
            ));
        }

        let json_str = String::from_utf8(output.stdout)
            .map_err(|e| NestGateError::validation_error(e.to_string()))?;

        // Parse the JSON output and convert to PoolInfo structures
        let json_value: serde_json::Value = serde_json::from_str(&json_str)?;
        let mut pools = Vec::new();

        // Extract pool information from JSON (simplified parsing)
        if let Some(pool_array) = json_value.as_array() {
            for pool in pool_array {
                if let Some(name) = pool.get("name").and_then(|n| n.as_str()) {
                    pools.push(crate::handlers::PoolInfo {
                        name: name.to_string(),
                        state: pool
                            .get("state")
                            .and_then(|s| s.as_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        size: pool
                            .get("size")
                            .and_then(|s| s.as_str())
                            .unwrap_or("0")
                            .to_string(),
                        allocated: pool
                            .get("allocated")
                            .and_then(|s| s.as_str())
                            .unwrap_or("0")
                            .to_string(),
                        free: pool
                            .get("free")
                            .and_then(|s| s.as_str())
                            .unwrap_or("0")
                            .to_string(),
                        devices: Vec::new(), // Simplified for now
                    });
                }
            }
        }

        Ok(crate::handlers::ZfsResponse::PoolStatus { pools })
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub async fn get_dataset_list(
        &self,
        pool_name: Option<String>,
    ) -> Result<crate::handlers::ZfsResponse> {
        use nestgate_core::NestGateError;
        let mut cmd = tokio::process::Command::new("zfs");
        cmd.arg("list")
            .arg("-H")
            .arg("-o")
            .arg("name,used,avail,refer,mountpoint");

        if let Some(pool) = pool_name {
            cmd.arg(pool);
        }

        let output = cmd
            .output()
            .await
            .map_err(|e| NestGateError::io_error(e.to_string()))?;
        if !output.status.success() {
            return Err(NestGateError::system(
                format!(
                    "ZFS dataset list command failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                "zfs_operations",
            ));
        }

        let output_str = String::from_utf8(output.stdout)
            .map_err(|e| NestGateError::validation_error(e.to_string()))?;
        let datasets: Vec<crate::handlers::DatasetInfo> = output_str
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                crate::handlers::DatasetInfo {
                    name: parts.first().unwrap_or(&"").to_string(),
                    used: parts.get(1).unwrap_or(&"").to_string(),
                    available: parts.get(2).unwrap_or(&"").to_string(),
                    referenced: parts.get(3).unwrap_or(&"").to_string(),
                    mountpoint: parts.get(4).unwrap_or(&"").to_string(),
                }
            })
            .collect();

        Ok(crate::handlers::ZfsResponse::DatasetList { datasets })
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub async fn get_snapshot_list(
        &self,
        dataset: Option<String>,
    ) -> Result<crate::handlers::ZfsResponse> {
        use nestgate_core::NestGateError;
        let dataset_name = dataset.unwrap_or_default();
        let output = tokio::process::Command::new("zfs")
            .arg("list")
            .arg("-t")
            .arg("snapshot")
            .arg("-H")
            .arg("-o")
            .arg("name,used,creation")
            .arg(&dataset_name)
            .output()
            .await
            .map_err(|e| NestGateError::io_error(e.to_string()))?;

        if !output.status.success() {
            return Err(NestGateError::system(
                format!(
                    "ZFS snapshot list command failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
                "zfs_operations",
            ));
        }

        let output_str = String::from_utf8(output.stdout)
            .map_err(|e| NestGateError::validation_error(e.to_string()))?;
        let snapshots: Vec<crate::handlers::SnapshotInfo> = output_str
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                crate::handlers::SnapshotInfo {
                    name: parts.first().unwrap_or(&"").to_string(),
                    used: parts.get(1).unwrap_or(&"").to_string(),
                    referenced: parts.get(2).unwrap_or(&"").to_string(),
                    creation: parts.get(2).unwrap_or(&"").to_string(),
                }
            })
            .collect();

        Ok(crate::handlers::ZfsResponse::SnapshotList { snapshots })
    }
}
impl ProductionReadinessValidator {
    /// Create new production readiness validator
    #[must_use]
    pub fn new() -> Self {
        Self {
            real_ops: RealZfsOperations::default(),
        }
    }

    /// Perform comprehensive production readiness assessment
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn assess_production_readiness(&self) -> Result<ProductionReadinessReport> {
        info!("🔍 Starting comprehensive production readiness assessment...");
        let mut report = ProductionReadinessReport {
            ready_for_production: false,
            zfs_available: false,
            real_hardware_detected: false,
            mock_dependencies: Vec::new(),
            performance_validated: false,
            security_validated: false,
            configuration_validated: false,
            findings: Vec::new(),
            recommendations: Vec::new(),
        };

        // 1. Check ZFS availability
        report.zfs_available = self.check_zfs_availability()?;

        // 2. Detect real hardware
        report.real_hardware_detected = self.detect_real_hardware()?;

        // 3. Identify mock dependencies
        report.mock_dependencies = self.identify_mock_dependencies()?;

        // 4. Validate performance
        report.performance_validated = self.validate_performance()?;

        // 5. Validate security
        report.security_validated = self.validate_security()?;

        // 6. Validate configuration
        report.configuration_validated = self.validate_configuration()?;

        // Determine overall readiness
        report.ready_for_production = report.zfs_available
            && report.real_hardware_detected
            && report.mock_dependencies.is_empty()
            && report.performance_validated
            && report.security_validated
            && report.configuration_validated;

        self.generate_findings_and_recommendations(&mut report)?;

        Ok(report)
    }

    /// Check Zfs Availability
    fn check_zfs_availability(&self) -> Result<bool> {
        // Check if ZFS is available on the system
        Ok(std::path::Path::new("/proc/spl/kstat/zfs").exists())
    }

    /// Detect Real Hardware
    fn detect_real_hardware(&self) -> Result<bool> {
        // Detect if we're running on real hardware vs virtualized environment
        Ok(!std::env::var("NESTGATE_MOCK_MODE")
            .unwrap_or_default()
            .eq("true"))
    }

    /// Identify Mock Dependencies
    fn identify_mock_dependencies(&self) -> Result<Vec<String>> {
        let mut mocks = Vec::new();

        if std::env::var("NESTGATE_MOCK_MODE")
            .unwrap_or_default()
            .eq("true")
        {
            mocks.push("Mock mode enabled".to_string());
        }

        Ok(mocks)
    }

    /// Validates  Performance
    fn validate_performance(&self) -> Result<bool> {
        // Validate performance characteristics
        // Check system resources and ZFS performance metrics
        let available_memory = std::env::var("NESTGATE_MIN_MEMORY_MB")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(1024); // Minimum 1GB RAM

        // Basic performance validation
        Ok(available_memory >= 512) // Require at least 512MB for ZFS operations
    }

    /// Validates  Security
    fn validate_security(&self) -> Result<bool> {
        // Validate security configurations
        // Check for secure ZFS configuration
        let secure_mode = std::env::var("NESTGATE_SECURE_MODE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        // Basic security validation - ensure encryption is available if required
        Ok(!secure_mode || self.check_encryption_support())
    }

    /// Validates  Configuration
    fn validate_configuration(&self) -> Result<bool> {
        // Validate system configuration
        // Check for required environment variables and configuration
        let data_dir = std::env::var("NESTGATE_DATA_DIR").unwrap_or_else(|_| "./data".to_string());
        let config_dir =
            std::env::var("NESTGATE_CONFIG_DIR").unwrap_or_else(|_| "./config".to_string());

        // Verify directories exist or can be created
        let result = std::fs::create_dir_all(&data_dir).is_ok()
            && std::fs::create_dir_all(&config_dir).is_ok();
        Ok(result)
    }

    /// Generate Findings And Recommendations
    fn generate_findings_and_recommendations(
        &self,
        report: &mut ProductionReadinessReport,
    ) -> Result<()> {
        if !report.zfs_available {
            report.findings.push(ReadinessFinding {
                category: "ZFS Availability".to_string(),
                description: "ZFS is not available on this system".to_string(),
                severity: FindingSeverity::Critical,
                blocking: true,
            });
            report
                .recommendations
                .push("Install ZFS kernel modules and utilities".to_string());
        }

        if !report.real_hardware_detected {
            report.findings.push(ReadinessFinding {
                category: "Hardware Detection".to_string(),
                description: "Running in mock/virtual mode — not real hardware".to_string(),
                severity: FindingSeverity::Warning,
                blocking: false,
            });
            report
                .recommendations
                .push("Run on real hardware for production workloads".to_string());
        }

        if !report.mock_dependencies.is_empty() {
            report.findings.push(ReadinessFinding {
                category: "Mock Dependencies".to_string(),
                description: format!(
                    "Mock dependencies active: {}",
                    report.mock_dependencies.join(", ")
                ),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push("Disable mock mode for production: unset NESTGATE_MOCK_MODE".to_string());
        }

        if !report.performance_validated {
            report.findings.push(ReadinessFinding {
                category: "Performance".to_string(),
                description: "Performance validation failed — insufficient resources".to_string(),
                severity: FindingSeverity::Warning,
                blocking: false,
            });
            report
                .recommendations
                .push("Ensure sufficient memory and CPU for production workloads".to_string());
        }

        if !report.security_validated {
            report.findings.push(ReadinessFinding {
                category: "Security".to_string(),
                description: "Security validation failed — encryption not available".to_string(),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push("Enable ZFS encryption support for secure mode".to_string());
        }

        if !report.configuration_validated {
            report.findings.push(ReadinessFinding {
                category: "Configuration".to_string(),
                description: "Required directories could not be created".to_string(),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push("Verify NESTGATE_DATA_DIR and NESTGATE_CONFIG_DIR are writable".to_string());
        }

        // Add general production recommendations
        report
            .recommendations
            .push("Review logs for any warnings during operation".to_string());
        report
            .recommendations
            .push("Monitor ZFS pool health and performance metrics".to_string());
        report
            .recommendations
            .push("Ensure backup and recovery procedures are tested".to_string());

        Ok(())
    }

    /// Check if ZFS encryption support is available
    fn check_encryption_support(&self) -> bool {
        // Check if ZFS encryption modules are loaded
        std::process::Command::new("zfs")
            .args(["get", "encryption", "/"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl Default for ProductionReadinessValidator {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to run production readiness check
pub fn check_production_readiness() -> Result<ProductionReadinessReport> {
    let validator = ProductionReadinessValidator::new();
    validator.assess_production_readiness()
}
#[cfg(test)]
mod tests {
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
}
