// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Development-environment ZFS fallbacks (`dev-stubs` / tests only).
//!
//! Provides filesystem-based storage abstractions for laptops, containers, and CI
//! where native ZFS pools are unavailable. **Never enabled in release production builds.**

#![cfg(any(test, feature = "dev-stubs"))]

/// Hardware detection for development environments
pub mod hardware_detector;
pub mod storage_abstraction;
pub mod zfs_compatibility;

pub use hardware_detector::{HardwareCapabilities, HardwareEnvironmentDetector};
pub use storage_abstraction::DevEnvironmentStorageService;
pub use zfs_compatibility::DevEnvironmentZfsService;

use nestgate_core::error::NestGateError;

/// Check if we're running in a development environment without storage hardware
#[must_use]
pub fn is_dev_environment() -> bool {
    HardwareEnvironmentDetector::is_development_environment()
}

/// Get the appropriate storage service for the current environment
pub async fn create_storage_service()
-> nestgate_core::error::CanonicalResult<std::sync::Arc<DevEnvironmentStorageService>> {
    match HardwareEnvironmentDetector::detect_capabilities().await {
        HardwareCapabilities::NativeZfs => {
            tracing::info!("Native ZFS hardware detected");
            Err(NestGateError::not_implemented(
                "native ZFS storage service (use real ZFS manager, not dev-stubs fallback)",
            ))
        }
        HardwareCapabilities::DevelopmentEnvironment => {
            tracing::info!("Development environment - using storage abstraction");
            Ok(std::sync::Arc::new(DevEnvironmentStorageService::new()))
        }
        HardwareCapabilities::ContainerEnvironment => {
            tracing::info!("Container environment - using abstraction layer");
            Ok(std::sync::Arc::new(DevEnvironmentStorageService::new()))
        }
    }
}

/// Runtime information about feature availability
#[must_use]
pub const fn feature_info() -> FeatureInfo {
    FeatureInfo {
        dev_stubs: cfg!(any(test, feature = "dev-stubs")),
        hardware_detection: cfg!(feature = "hardware-detection"),
        container_support: cfg!(feature = "container"),
        dev_verbose_logging: cfg!(feature = "dev-verbose-logging"),
    }
}

/// Information about enabled features
#[derive(Debug, Clone)]
pub struct FeatureInfo {
    /// Whether dev-stubs fallbacks are compiled in
    pub dev_stubs: bool,
    /// Hardware Detection
    pub hardware_detection: bool,
    /// Container Support
    pub container_support: bool,
    /// Dev Verbose Logging
    pub dev_verbose_logging: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature_info_reflects_compile_time_flags() {
        let fi = feature_info();
        assert!(fi.dev_stubs);
    }

    #[test]
    fn feature_info_clone_and_debug() {
        let fi = feature_info();
        let _ = format!("{fi:?}");
        let _ = fi.clone();
    }
}
