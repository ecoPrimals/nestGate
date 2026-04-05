// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Universal Service Manager Detection
//!
//! **UNIVERSAL ARCHITECTURE** - Runtime capability-based service manager detection
//! **EVOLUTION**: Phase 2 Task 2 - Deep Debt Evolution (Jan 31, 2026)
//!
//! Provides trait-based abstraction for detecting service managers with runtime
//! capability detection instead of compile-time OS checks.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │   UniversalServiceDetector          │
//! │   (Runtime Capability Detection)    │
//! └──────────────┬──────────────────────┘
//!                │
//!       ┌────────┴────────┐
//!       │                 │
//! ┌─────▼─────┐    ┌─────▼──────┐
//! │ Systemd   │    │  Launchd   │
//! │ Detector  │    │  Detector  │
//! └───────────┘    └────────────┘
//!       │                 │
//!       │          ┌──────▼──────┐
//!       │          │   Windows   │
//!       │          │   Service   │
//!       │          └─────────────┘
//!       │
//!       └──────► Runtime Detection!
//! ```
//!
//! ## Key Features
//!
//! - **Runtime Detection**: Checks for actual capability, not just OS
//! - **Container-Friendly**: Works in Docker/Kubernetes without systemd
//! - **Graceful Degradation**: Falls back to manual service if no manager
//! - **Future-Proof**: Easy to add new service managers
//!
//! ## Usage
//!
//! ```rust
//! use nestgate_installer::platform::{UniversalServiceDetector, ServiceManager};
//!
//! fn detect_service_manager() -> ServiceManager {
//!     let detector = UniversalServiceDetector::new();
//!     detector.detect()
//! }
//! ```

use std::path::Path;

/// Service manager types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceManager {
    /// Systemd (Linux)
    Systemd,
    /// Launchd (macOS)
    Launchd,
    /// Windows Service Manager
    WindowsService,
    /// Manual service (no manager detected)
    Manual,
}

impl ServiceManager {
    /// Get human-readable name
    #[must_use]
    pub const fn name(&self) -> &str {
        match self {
            Self::Systemd => "systemd",
            Self::Launchd => "launchd",
            Self::WindowsService => "Windows Service",
            Self::Manual => "manual",
        }
    }

    /// Check if this service manager supports automatic startup
    #[must_use]
    pub const fn supports_auto_start(&self) -> bool {
        !matches!(self, Self::Manual)
    }
}

/// Universal trait for service manager detection
///
/// **CAPABILITY-BASED**: Checks for actual capability, not just OS type
pub trait ServiceDetector: Send + Sync {
    /// Detect if this service manager is available
    ///
    /// **RUNTIME CHECK**: Actually verifies the service manager exists and is functional
    fn detect(&self) -> bool;

    /// Get service manager type
    fn manager_type(&self) -> ServiceManager;

    /// Get detector name for logging
    fn name(&self) -> &str;
}

/// Systemd detector
///
/// **CAPABILITY-BASED**: Checks for /run/systemd/system, not just OS type
/// This ensures we detect systemd only when it's actually running.
pub struct SystemdDetector;

impl ServiceDetector for SystemdDetector {
    fn detect(&self) -> bool {
        // Check for systemd runtime directory (indicates systemd is running)
        if Path::new("/run/systemd/system").exists() {
            tracing::debug!("✅ Detected systemd (runtime directory exists)");
            return true;
        }

        // Alternative check: systemctl command available
        if std::process::Command::new("systemctl")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            tracing::debug!("✅ Detected systemd (systemctl available)");
            return true;
        }

        tracing::debug!("❌ Systemd not detected");
        false
    }

    fn manager_type(&self) -> ServiceManager {
        ServiceManager::Systemd
    }

    fn name(&self) -> &'static str {
        "systemd-detector"
    }
}

/// Launchd detector (macOS)
///
/// **CAPABILITY-BASED**: Checks for launchd socket, not just macOS
pub struct LaunchdDetector;

impl ServiceDetector for LaunchdDetector {
    fn detect(&self) -> bool {
        // Check for launchd socket
        if Path::new("/var/run/launchd.socket").exists() {
            tracing::debug!("✅ Detected launchd (socket exists)");
            return true;
        }

        // Alternative: check for launchctl command
        if std::process::Command::new("launchctl")
            .arg("version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            tracing::debug!("✅ Detected launchd (launchctl available)");
            return true;
        }

        tracing::debug!("❌ Launchd not detected");
        false
    }

    fn manager_type(&self) -> ServiceManager {
        ServiceManager::Launchd
    }

    fn name(&self) -> &'static str {
        "launchd-detector"
    }
}

/// Windows Service detector
///
/// **CAPABILITY-BASED**: Checks for Windows service management capability
pub struct WindowsServiceDetector;

impl ServiceDetector for WindowsServiceDetector {
    fn detect(&self) -> bool {
        #[cfg(windows)]
        {
            // Check for sc.exe (Service Control Manager)
            if std::process::Command::new("sc.exe")
                .arg("query")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                tracing::debug!("✅ Detected Windows Service Manager");
                return true;
            }
        }

        tracing::debug!("❌ Windows Service Manager not detected");
        false
    }

    fn manager_type(&self) -> ServiceManager {
        ServiceManager::WindowsService
    }

    fn name(&self) -> &'static str {
        "windows-service-detector"
    }
}

/// Universal service manager detector with runtime capability detection
///
/// **ADAPTIVE**: Detects available service manager at runtime
pub struct UniversalServiceDetector {
    detectors: Vec<Box<dyn ServiceDetector>>,
}

impl Default for UniversalServiceDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl UniversalServiceDetector {
    /// Create new universal service detector
    ///
    /// **UNIVERSAL**: Works on all platforms, detects at runtime
    pub fn new() -> Self {
        tracing::info!("🔍 Initializing universal service manager detector");

        let detectors: Vec<Box<dyn ServiceDetector>> = vec![
            Box::new(SystemdDetector),
            Box::new(LaunchdDetector),
            Box::new(WindowsServiceDetector),
        ];

        Self { detectors }
    }

    /// Detect available service manager
    ///
    /// **RUNTIME DETECTION**: Checks actual capability, not compile-time OS
    pub fn detect(&self) -> ServiceManager {
        tracing::info!("🔍 Detecting available service manager...");

        // Try each detector in order
        for detector in &self.detectors {
            tracing::debug!("Trying detector: {}", detector.name());
            if detector.detect() {
                let manager = detector.manager_type();
                tracing::info!("✅ Detected service manager: {}", manager.name());
                return manager;
            }
        }

        tracing::warn!("⚠️  No service manager detected, will use manual service");
        ServiceManager::Manual
    }

    /// Check if any service manager is available
    #[must_use]
    pub fn has_service_manager(&self) -> bool {
        !matches!(self.detect(), ServiceManager::Manual)
    }

    /// Get detailed detection info for diagnostics
    #[must_use]
    pub fn detect_with_info(&self) -> (ServiceManager, Vec<String>) {
        let mut messages = Vec::new();

        for detector in &self.detectors {
            let detected = detector.detect();
            let msg = if detected {
                format!("✅ {}: Available", detector.name())
            } else {
                format!("❌ {}: Not available", detector.name())
            };
            messages.push(msg);
        }

        let manager = self.detect();
        (manager, messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universal_detector_creation() {
        let detector = UniversalServiceDetector::new();
        assert!(!detector.detectors.is_empty(), "Should have detectors");
    }

    #[test]
    fn test_detection() {
        let detector = UniversalServiceDetector::new();
        let manager = detector.detect();

        // Should detect something (or manual)
        println!("Detected service manager: {}", manager.name());

        // Should always return a valid value
        assert!(matches!(
            manager,
            ServiceManager::Systemd
                | ServiceManager::Launchd
                | ServiceManager::WindowsService
                | ServiceManager::Manual
        ));
    }

    #[test]
    fn test_detection_with_info() {
        let detector = UniversalServiceDetector::new();
        let (manager, messages) = detector.detect_with_info();

        println!("Service manager: {}", manager.name());
        for msg in &messages {
            println!("  {}", msg);
        }

        // Should have detection messages
        assert!(!messages.is_empty());
    }

    #[test]
    fn test_service_manager_properties() {
        let systemd = ServiceManager::Systemd;
        assert_eq!(systemd.name(), "systemd");
        assert!(systemd.supports_auto_start());

        let manual = ServiceManager::Manual;
        assert_eq!(manual.name(), "manual");
        assert!(!manual.supports_auto_start());
    }
}
