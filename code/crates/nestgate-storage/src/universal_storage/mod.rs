// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **UNIVERSAL STORAGE SYSTEM**
//!
//! Unified storage abstraction layer for `NestGate`, consolidating all storage backends
//! into a single, consistent interface with zero-cost abstractions.
//!
//! # Architecture
//!
//! The universal storage system provides:
//! - **Unified Trait System**: Consistent interface for all storage operations
//! - **Factory Pattern**: Dynamic backend creation based on configuration
//! - **Comprehensive Monitoring**: Metrics and performance tracking
//! - **Zero-Cost Abstractions**: No runtime overhead for abstractions
//! - **Backend Agnostic**: Support for filesystem, object storage, block storage, ZFS
//!
//! # Supported Backends
//!
//! - **ZFS**: Advanced filesystem with snapshots, compression, deduplication
//! - **Filesystem**: Standard POSIX filesystem operations  
//! - **Object Storage**: S3-compatible object storage (planned)
//! - **Block Storage**: Raw block device access (planned)
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use nestgate_core::universal_storage::{
//!     ZeroCostStorageBackend, StorageDetector, StorageConfig
//! };
//!
//! // Auto-detect available storage backends
//! let detector = StorageDetector::new();
//! let backends = detector.detect_backends().await?;
//!
//! // Create storage backend
//! let config = StorageConfig::default();
//! let backend = ZeroCostStorageBackend::new(config)?;
//!
//! // Perform operations
//! backend.create_dataset("my-data").await?;
//! backend.create_snapshot("my-data@backup").await?;
//! ```
//!
//! # Status
//!
//! **Production Ready**: ✅ Core functionality complete\
//! **Test Coverage**: Comprehensive edge case and error tests\
//! **Performance**: Zero-cost abstractions with Arc for efficient sharing

// ==================== CANONICAL STORAGE SYSTEM ====================

/// Zero-cost storage backend implementations for optimal performance
pub mod types;
pub mod zero_cost_storage_backend;

/// Zero-cost storage trait definitions for type-safe, zero-overhead abstractions
pub mod zero_cost_storage_traits;

/// Consolidated type definitions for storage operations
pub mod consolidated_types;

// Test expansion for consolidated types (Nov 6, 2025)
#[cfg(test)]
mod consolidated_types_tests;

#[cfg(test)]
mod storage_edge_cases;
#[cfg(test)]
mod storage_error_tests; // Nov 23, 2025 - P1 test expansion // Nov 23, 2025 - P1-5 edge case tests
// Auto-configuration for storage backends
pub mod auto_configurator;
// Storage detection and discovery
pub mod storage_detector;
// Storage detector runtime configuration
pub mod storage_detector_config;
// Enterprise storage operations module does not exist on disk.
// Enterprise capabilities are provided via universal storage adapter.

// ==================== MODERN VENDOR-AGNOSTIC STORAGE ====================

/// **UNIVERSAL AGNOSTIC STORAGE** (December 15, 2025)
///
/// Zero vendor coupling. Protocol-based discovery.
/// Works with any storage system: AWS, MinIO, Wasabi, Azure, GCS, or future systems.
pub mod universal;

// ==================== RE-EXPORTS ====================

// Re-export zero-cost storage backend
pub use zero_cost_storage_backend::ZeroCostStorageBackend;
// Re-export zero-cost storage traits
pub use zero_cost_storage_traits::{
    ZeroCostStorageBackend as ZeroCostStorageBackendTrait, ZeroCostStorageProvider,
};
// Re-export storage detector config
pub use storage_detector_config::{SharedStorageDetectorConfig, StorageDetectorConfig};

// Re-export universal agnostic storage
pub use universal::{
    AuthenticationPattern, DiscoveredProtocol, DiscoveredStorage, StorageFeature,
    StorageOperationPattern, TransportProtocol, UniversalStorageAdapter, UniversalStorageDiscovery,
};

// Enterprise storage capabilities provided by universal storage adapter.
// The enterprise module was planned but never created.
// Enterprise features are accessible via UniversalStorageAdapter::discover().

// Auto-configuration utilities
pub use auto_configurator::AutoConfigurator;

// Storage detection utilities
pub use storage_detector::{DetectedStorage, StorageDetector};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        // Test with canonical storage configuration
        let config = auto_configurator::AutoConfigurator::new(vec![]);
        assert!(config.is_auto_tuning_enabled()); // Auto-tuning should be enabled by default

        // Test with custom settings that disable auto-tuning
        let custom_settings = auto_configurator::ConfiguratorSettings {
            enable_auto_tuning: false,
            ..Default::default()
        };
        let config_disabled =
            auto_configurator::AutoConfigurator::with_settings(vec![], custom_settings);
        assert!(!config_disabled.is_auto_tuning_enabled());
    }

    #[tokio::test]
    async fn test_storage_detection() {
        let mut detector = storage_detector::StorageDetector::new();
        let result = detector.scan_available_storage().await;
        assert!(result.is_ok());
    }
}
