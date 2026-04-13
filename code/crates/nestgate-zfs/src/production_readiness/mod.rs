// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Readiness APIs use Result for forward-compatible error propagation"
)]

mod readiness_env;
mod reporting;

use nestgate_core::Result;
use nestgate_types::{EnvSource, ProcessEnv};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
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
    /// Mock-mode notices (environment flags and missing Linux procfs signals)
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
    /// Injectable environment (use [`ProcessEnv`] in production).
    env: Arc<dyn EnvSource>,
}
/// Real ZFS operations dispatcher: runs `zfs` / `zpool` (and related) commands and maps output
/// into [`crate::handlers::ZfsResponse`].
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
        Self::new_with_env(Arc::new(ProcessEnv))
    }

    /// Create a validator that reads configuration from an injectable environment source
    /// (e.g. [`nestgate_types::MapEnv`] in tests).
    #[must_use]
    pub fn new_with_env(env: Arc<dyn EnvSource>) -> Self {
        Self {
            real_ops: RealZfsOperations::default(),
            env,
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
        info!("Starting comprehensive production readiness assessment...");
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

    /// Validates  Performance
    pub(crate) fn validate_performance(&self) -> Result<bool> {
        let min_mb = self
            .env
            .get("NESTGATE_MIN_MEMORY_MB")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(1024);

        if min_mb < 512 {
            return Ok(false);
        }

        #[cfg(target_os = "linux")]
        if let Some(total_mb) = mem_total_mb_from_proc() {
            return Ok(total_mb >= min_mb);
        }

        Ok(true)
    }

    /// Validates  Security
    pub(crate) fn validate_security(&self) -> Result<bool> {
        // Validate security configurations
        // Check for secure ZFS configuration
        let secure_mode = self
            .env
            .get("NESTGATE_SECURE_MODE")
            .unwrap_or_else(|| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        // Basic security validation - ensure encryption is available if required
        Ok(!secure_mode || self.check_encryption_support())
    }

    /// Validates  Configuration
    pub(crate) fn validate_configuration(&self) -> Result<bool> {
        // Validate system configuration
        // Check for required environment variables and configuration
        let data_dir = self
            .env
            .get("NESTGATE_DATA_DIR")
            .unwrap_or_else(|| "./data".to_string());
        let config_dir = self
            .env
            .get("NESTGATE_CONFIG_DIR")
            .unwrap_or_else(|| "./config".to_string());

        // Verify directories exist or can be created
        let result = std::fs::create_dir_all(&data_dir).is_ok()
            && std::fs::create_dir_all(&config_dir).is_ok();
        Ok(result)
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

#[cfg(target_os = "linux")]
fn mem_total_mb_from_proc() -> Option<u64> {
    let content = std::fs::read_to_string("/proc/meminfo").ok()?;
    let line = content.lines().find(|l| l.starts_with("MemTotal:"))?;
    let kb = line.split_whitespace().nth(1)?.parse::<u64>().ok()?;
    Some(kb / 1024)
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
mod real_zfs_operations_tests;
#[cfg(test)]
mod tests;
