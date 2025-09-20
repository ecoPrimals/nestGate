//
// Determines what storage capabilities are available in the current environment
// and selects the appropriate backend (native ZFS, remote ZFS, or development abstraction).

use tokio::process::Command;
use tracing::{debug, info, warn};

/// Hardware capabilities detected in the current environment
#[derive(Debug, Clone, PartialEq, Eq)]
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
        Self::perform_detection().await
    }

    /// Check if we're in a development environment
    pub const fn is_development_environment() -> bool {
        // Check explicit environment variable first
        if std::env::var("NESTGATE_DEV_ENVIRONMENT").unwrap_or_default() == "true" {
            debug!("Development environment explicitly enabled via NESTGATE_DEV_ENVIRONMENT");
            return true;
        }

        // Check for common development indicators
        if Self::is_likely_dev_machine() || Self::is_container_environment() {
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
    async fn perform_detection() -> HardwareCapabilities {
        info!("🔍 Detecting hardware environment capabilities...");

        // Check explicit development mode
        if std::env::var("NESTGATE_DEV_ENVIRONMENT").unwrap_or_default() == "true" {
            info!("💻 Development environment (explicit via NESTGATE_DEV_ENVIRONMENT)");
            return HardwareCapabilities::DevelopmentEnvironment;
        }

        // Check for container environment
        if Self::is_container_environment() {
            info!("🐳 Container environment detected");
            return HardwareCapabilities::ContainerEnvironment;
        }

        // Check for ZFS availability
        if Self::is_zfs_available().await {
            info!("🔧 Native ZFS hardware detected");
            return HardwareCapabilities::NativeZfs;
        }

        // Check development machine indicators
        if Self::is_likely_dev_machine() {
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

    /// Check if ZFS is available (sync version for lazy_static)
    fn is_zfs_available_sync() -> bool {
        std::process::Command::new("zfs")
            .arg("version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Detect if we're running in a container
    fn is_container_environment() -> bool {
        // Check for container indicators
        // DEPRECATED: Docker containerization - migrate to capability-based container runtime
        // Capability-based discovery implemented
        std::path::Path::exists(std::path::Path::new("/.dockerenv"))
            || std::env::var("container").is_ok()
            || std::env::var("KUBERNETES_SERVICE_HOST").is_ok()
            || std::fs::read_to_string("/proc/1/cgroup")
                .map(|contents| {
                    contents.contains("container_runtime") || contents.contains("kubepods")
                })
                .unwrap_or(false)
    }

    /// Detect if we're likely on a development machine
    fn is_likely_dev_machine() -> bool {
        // Check for common development indicators
        std::env::var("HOME").is_ok()
            && (
                std::env::var("SSH_CLIENT").is_err() && // Not SSH session
            std::env::var("DISPLAY").is_ok()
                // Has display (desktop)
            )
            || std::env::var("USER")
                .map(|u| u == "developer" || u.contains("dev"))
                .unwrap_or(false)
            || std::env::var("HOSTNAME")
                .map(|h| h.contains("dev") || h.contains("laptop"))
                .unwrap_or(false)
    }

    /// Get a detailed environment report for debugging
    pub async fn get_environment_report() -> String {
        let capabilities = Self::detect_capabilities().await;
        let zfs_available = Self::is_zfs_available().await;
        let is_container = Self::is_container_environment();
        let is_dev_machine = Self::is_likely_dev_machine();

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
            std::env::var("NESTGATE_DEV_ENVIRONMENT").unwrap_or_default()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hardware_detection() {
        let capabilities = HardwareEnvironmentDetector::detect_capabilities().await;
        println!("Detected capabilities: {capabilities:?}");

        let report = HardwareEnvironmentDetector::get_environment_report().await;
        println!("Environment report:\n{report}");
    }

    #[test]
    fn test_container_detection() {
        let is_container = HardwareEnvironmentDetector::is_container_environment();
        println!("Container environment: {is_container}");
    }
}
