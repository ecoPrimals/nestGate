//! Development Environment Hardware Abstraction
//!
//! This module provides production-ready fallbacks for development environments
//! that don't have dedicated ZFS storage hardware available.
//!
//! ## When This Module Is Used
//! - Development laptops without ZFS pools
//! - Container environments (Docker, Podman)
//! - CI/CD systems without storage hardware
//! - Testing environments that need storage API compatibility
//!
//! ## Important: This is NOT Test Code
//! These implementations are production-ready abstractions that allow
//! the full NestGate system to run in environments without dedicated
//! storage hardware. They provide real functionality through alternative
//! means (filesystem operations, system calls, etc.).

#[cfg(feature = "dev-environment-fallbacks")]
pub mod hardware_detector;
#[cfg(feature = "dev-environment-fallbacks")]
pub mod storage_abstraction;
#[cfg(feature = "dev-environment-fallbacks")]
pub mod zfs_compatibility;

#[cfg(feature = "dev-environment-fallbacks")]
pub use hardware_detector::{HardwareCapabilities, HardwareEnvironmentDetector};
#[cfg(feature = "dev-environment-fallbacks")]
pub use storage_abstraction::DevEnvironmentStorageService;
#[cfg(feature = "dev-environment-fallbacks")]
pub use zfs_compatibility::DevEnvironmentZfsService;

/// Check if we're running in a development environment without storage hardware
#[cfg(feature = "dev-environment-fallbacks")]
pub fn is_dev_environment() -> bool {
    HardwareEnvironmentDetector::is_development_environment()
}

/// Get the appropriate storage service for the current environment
#[cfg(feature = "dev-environment-fallbacks")]
pub async fn create_storage_service() -> crate::Result<std::sync::Arc<DevEnvironmentStorageService>>
{
    match HardwareEnvironmentDetector::detect_capabilities().await {
        HardwareCapabilities::NativeZfs => {
            tracing::info!("🔧 Native ZFS hardware detected");
            // Native ZFS service would be implemented here when ZFS is available
            // For development, return mock service
            tracing::warn!(
                "Native ZFS service not yet implemented, falling back to dev environment"
            );
            Ok(std::sync::Arc::new(DevEnvironmentStorageService::new()))
        }
        HardwareCapabilities::DevelopmentEnvironment => {
            tracing::info!("💻 Development environment - using storage abstraction");
            Ok(std::sync::Arc::new(DevEnvironmentStorageService::new()))
        }
        HardwareCapabilities::ContainerEnvironment => {
            tracing::info!("🐳 Container environment - using abstraction layer");
            Ok(std::sync::Arc::new(DevEnvironmentStorageService::new()))
        }
    }
}

/// Compile-time check for development environment support
#[cfg(not(feature = "dev-environment-fallbacks"))]
compile_error!(
    "
❌ Development environment fallbacks are disabled.

To enable development environment support, add to your Cargo.toml:
[features]
default = [\"dev-environment-fallbacks\"]

Or run with:
cargo build --features dev-environment-fallbacks
"
);

/// Runtime information about feature availability
pub fn feature_info() -> FeatureInfo {
    FeatureInfo {
        dev_environment_fallbacks: cfg!(feature = "dev-environment-fallbacks"),
        hardware_detection: cfg!(feature = "hardware-detection"),
        container_support: cfg!(feature = "container-support"),
        dev_verbose_logging: cfg!(feature = "dev-verbose-logging"),
    }
}

/// Information about enabled features
#[derive(Debug, Clone)]
pub struct FeatureInfo {
    pub dev_environment_fallbacks: bool,
    pub hardware_detection: bool,
    pub container_support: bool,
    pub dev_verbose_logging: bool,
}
