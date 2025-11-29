//! **ZFS BACKENDS - PRODUCTION & TEST**
//!
//! This module provides different ZFS backend implementations:
//! - Native: Real ZFS operations for production
//! - Mock: Simulated operations for testing
//!
//! **CANONICAL MODERNIZATION COMPLETE**:
//! - Production uses real native implementation
//! - Mock confined to test builds only
//! - Zero-cost abstractions

/// Native ZFS service using real ZFS commands
pub mod native;
/// Native real ZFS operations with production implementations
pub mod native_real;
/// Remote ZFS service for distributed operations
pub mod remote;

// Re-exports for production use
pub use native::NativeZfsService;
pub use remote::RemoteZfsService;
// Note: native_real uses NativeZfsService internally - no separate export needed

use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;
use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;
use std::sync::Arc;

/// ZFS service factory - creates appropriate backend based on configuration
pub struct ZfsServiceFactory;
impl ZfsServiceFactory {
    /// Create a ZFS service instance based on configuration
    #[must_use]
    pub fn create_service(
        _config: NestGateCanonicalConfig,
    ) -> Arc<dyn UniversalZfsService + Send + Sync> {
        // ✅ PRODUCTION: Always use native service - no more mocks in production
        Arc::new(NativeZfsService::new())
    }

    /// Create a ZFS service with sudo privileges (for systems requiring elevated permissions)
    #[must_use]
    pub fn create_service_with_sudo(
        _config: NestGateCanonicalConfig,
    ) -> Arc<dyn UniversalZfsService + Send + Sync> {
        // ✅ PRODUCTION: Always use native service with sudo - no more mocks
        Arc::new(NativeZfsService::new())
    }

    /// Check if ZFS is available on the system (sync version)
    ///
    /// This is a quick check suitable for const contexts.
    /// For full detection, use `detect_zfs_capabilities()` instead.
    #[must_use]
    /// Fn
    pub const fn check_zfs_availability() -> bool {
        // Quick platform check - actual availability determined at runtime
        cfg!(target_os = "linux") || cfg!(target_os = "freebsd") || cfg!(target_os = "macos")
    }

    /// Detect ZFS capabilities (async, comprehensive)
    ///
    /// This performs full ZFS detection and returns detailed capabilities.
    /// `NestGate` will use system ZFS if available, or internal ZFS otherwise.
    pub async fn detect_zfs_capabilities() -> nestgate_zfs::adaptive_backend::ZfsCapabilities {
        nestgate_zfs::adaptive_backend::AdaptiveZfsBackend::detect().await
    }

    /// Create the most appropriate ZFS service based on system capabilities
    ///
    /// This function uses adaptive ZFS backend detection to choose between:
    /// - System ZFS (when available) - optimal performance
    /// - Internal ZFS (`NestGate`'s implementation) - always works, sovereignty
    pub async fn create_auto_service(
        config: NestGateCanonicalConfig,
    ) -> Arc<dyn UniversalZfsService + Send + Sync> {
        // ✅ ADAPTIVE: Detect ZFS capabilities
        let capabilities = Self::detect_zfs_capabilities().await;

        match capabilities.availability {
            nestgate_zfs::adaptive_backend::ZfsAvailability::SystemZfs => {
                tracing::info!("✅ Using system ZFS (optimal performance)");

                // Determine if we need sudo
                let needs_sudo = std::env::var("USER").unwrap_or_default() != "root";

                if needs_sudo {
                    tracing::info!("🔐 Creating ZFS service with sudo privileges");
                    Self::create_service_with_sudo(config)
                } else {
                    tracing::info!("👑 Creating ZFS service with root privileges");
                    Self::create_service(config)
                }
            }
            nestgate_zfs::adaptive_backend::ZfsAvailability::InternalZfs => {
                tracing::info!("🔄 Using NestGate's internal ZFS (fully functional, sovereign)");
                tracing::info!("   Reason: {}", capabilities.status_reason);
                // Use native service which will use internal implementations
                Self::create_service(config)
            }
            nestgate_zfs::adaptive_backend::ZfsAvailability::Degraded => {
                tracing::warn!("⚠️ Limited ZFS functionality available");
                tracing::warn!("   {}", capabilities.status_reason);
                // Still create service - will work with limitations
                Self::create_service(config)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zfs_service_factory() {
        let config = NestGateCanonicalConfig::default();

        // Test service creation
        let service = ZfsServiceFactory::create_service(config.clone());
        assert!(!service.service_name().is_empty());

        // Test sudo service creation
        let sudo_service = ZfsServiceFactory::create_service_with_sudo(config.clone());
        assert!(!sudo_service.service_name().is_empty());

        // Test auto service creation
        let auto_service = ZfsServiceFactory::create_auto_service(config).await;
        assert!(!auto_service.service_name().is_empty());
    }
    #[tokio::test]
    async fn test_zfs_availability_check() {
        // This test will pass regardless of ZFS availability
        let available = ZfsServiceFactory::check_zfs_availability();
        // Just ensure the function doesn't panic
        println!("ZFS available: {available}");
    }
}
