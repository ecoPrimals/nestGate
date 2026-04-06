// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Determines what storage capabilities are available in the current environment
// and selects the appropriate backend (native ZFS, remote ZFS, or development abstraction).

use nestgate_types::{EnvSource, ProcessEnv};
use tokio::process::Command;
use tracing::{debug, info, warn};

/// Hardware capabilities detected in the current environment
#[derive(Debug, Clone, PartialEq, Eq)]
/// Hardwarecapabilities
pub enum HardwareCapabilities {
    /// Native ZFS is available with pools
    NativeZfs,
    /// Development environment (laptop, VM without ZFS)
    DevelopmentEnvironment,
    /// Container environment (Docker, Podman, etc.)
    ContainerEnvironment,
}
/// Hardware environment detector with caching for performance
pub struct HardwareEnvironmentDetector;
impl HardwareEnvironmentDetector {
    /// Detect the current hardware capabilities (cached)
    pub async fn detect_capabilities() -> HardwareCapabilities {
        // For now, just perform detection directly without caching in async context
        Self::detect_capabilities_from_env(&ProcessEnv).await
    }

    /// Detect capabilities using an injectable environment source
    pub async fn detect_capabilities_from_env(env: &dyn EnvSource) -> HardwareCapabilities {
        Self::perform_detection_from_env(env).await
    }

    /// Check if we're in a development environment
    #[must_use]
    pub fn is_development_environment() -> bool {
        Self::is_development_environment_from_env(&ProcessEnv)
    }

    /// Check development mode using an injectable environment source
    pub fn is_development_environment_from_env(env: &dyn EnvSource) -> bool {
        // Check explicit environment variable first
        if env.get_or("NESTGATE_DEV_ENVIRONMENT", "") == "true" {
            debug!("Development environment explicitly enabled via NESTGATE_DEV_ENVIRONMENT");
            return true;
        }

        // Check for common development indicators
        if Self::is_likely_dev_machine_from_env(env) || Self::is_container_environment_from_env(env)
        {
            return true;
        }

        // Check if ZFS hardware is unavailable
        if !Self::is_zfs_available_sync() {
            debug!("Development environment detected: ZFS not available");
            return true;
        }

        false
    }

    /// Perform comprehensive hardware detection
    async fn perform_detection_from_env(env: &dyn EnvSource) -> HardwareCapabilities {
        info!("🔍 Detecting hardware environment capabilities...");

        // Check explicit development mode
        if env.get_or("NESTGATE_DEV_ENVIRONMENT", "") == "true" {
            info!("💻 Development environment (explicit via NESTGATE_DEV_ENVIRONMENT)");
            return HardwareCapabilities::DevelopmentEnvironment;
        }

        // Check for container environment
        if Self::is_container_environment_from_env(env) {
            info!("🐳 Container environment detected");
            return HardwareCapabilities::ContainerEnvironment;
        }

        // Check for ZFS availability
        if Self::is_zfs_available().await {
            info!("🔧 Native ZFS hardware detected");
            return HardwareCapabilities::NativeZfs;
        }

        // Check development machine indicators
        if Self::is_likely_dev_machine_from_env(env) {
            info!("💻 Development machine detected (no ZFS hardware)");
            return HardwareCapabilities::DevelopmentEnvironment;
        }

        // Default to development environment
        warn!("⚠️ Unable to detect hardware capabilities, defaulting to development environment");
        HardwareCapabilities::DevelopmentEnvironment
    }

    /// Check if ZFS is available (async version)
    async fn is_zfs_available() -> bool {
        match Command::new("zfs").arg("version").output().await {
            Ok(output) => {
                let success = output.status.success();
                if success {
                    debug!("✅ ZFS command available");
                } else {
                    debug!("❌ ZFS command failed");
                }
                success
            }
            Err(e) => {
                debug!("❌ ZFS command not found: {}", e);
                false
            }
        }
    }

    /// Check if ZFS is available (sync version for non-async callers)
    fn is_zfs_available_sync() -> bool {
        std::process::Command::new("zfs")
            .arg("version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Detect if we're running in a container
    fn is_container_environment_from_env(env: &dyn EnvSource) -> bool {
        // Check for container indicators
        std::path::Path::exists(std::path::Path::new("/.dockerenv"))
            || env.get("container").is_some()
            || env.get("KUBERNETES_SERVICE_HOST").is_some()
            || std::fs::read_to_string("/proc/1/cgroup")
                .map(|contents| {
                    contents.contains("container_runtime") || contents.contains("kubepods")
                })
                .unwrap_or(false)
    }

    /// Detect if we're likely on a development machine
    fn is_likely_dev_machine_from_env(env: &dyn EnvSource) -> bool {
        // Check for common development indicators
        env.get("HOME").is_some()
            && (
                env.get("SSH_CLIENT").is_none() && // Not SSH session
            env.get("DISPLAY").is_some()
                // Has display (desktop)
            )
            || env
                .get("USER")
                .is_some_and(|u| u == "developer" || u.contains("dev"))
            || env
                .get("HOSTNAME")
                .is_some_and(|h| h.contains("dev") || h.contains("laptop"))
    }

    /// Get a detailed environment report for debugging
    pub async fn get_environment_report() -> String {
        Self::get_environment_report_from_env(&ProcessEnv).await
    }

    /// Environment report using an injectable environment source
    pub async fn get_environment_report_from_env(env: &dyn EnvSource) -> String {
        let capabilities = Self::detect_capabilities_from_env(env).await;
        let zfs_available = Self::is_zfs_available().await;
        let is_container = Self::is_container_environment_from_env(env);
        let is_dev_machine = Self::is_likely_dev_machine_from_env(env);

        format!(
            "Hardware Environment Report:\n\
             - Detected Capabilities: {:?}\n\
             - ZFS Available: {}\n\
             - Container Environment: {}\n\
             - Development Machine: {}\n\
             - Explicit Dev Mode: {}",
            capabilities,
            zfs_available,
            is_container,
            is_dev_machine,
            env.get_or("NESTGATE_DEV_ENVIRONMENT", "")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;

    #[tokio::test]
    async fn test_hardware_detection() {
        let capabilities = HardwareEnvironmentDetector::detect_capabilities().await;
        println!("Detected capabilities: {capabilities:?}");

        let report = HardwareEnvironmentDetector::get_environment_report().await;
        println!("Environment report:\n{report}");
    }

    #[test]
    fn test_container_detection() {
        let is_container =
            HardwareEnvironmentDetector::is_container_environment_from_env(&MapEnv::new());
        println!("Container environment: {is_container}");
    }

    #[test]
    fn test_explicit_dev_mode_from_map_env() {
        let env = MapEnv::from([("NESTGATE_DEV_ENVIRONMENT", "true")]);
        assert!(HardwareEnvironmentDetector::is_development_environment_from_env(&env));
    }
}
