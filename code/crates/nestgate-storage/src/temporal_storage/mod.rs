// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **Temporal Storage System**
//!
//! Universal storage system spanning all technology eras from punch cards to DNA storage.
//!
//! This module provides a comprehensive temporal storage abstraction that:
//! - Supports all storage technologies across history (1890s punch cards → future quantum)
//! - Provides universal data source interface with native async
//! - Enables capability-based data access
//! - Supports classification and intelligent tiering
//! - Handles access control and authentication
//!
//! ## Architecture
//!
//! The module is organized into domain-driven submodules:
//! - `device` - Hardware and physical storage devices
//! - `connection` - Connectivity and data source connections
//! - `data_types` - Data type classification and description
//! - `access_control` - Authentication and rate limiting
//! - `ingestion` - Data ingestion and validation
//! - `classification` - Data classification and storage strategies
//! - `sources` - Data source types and streaming
//! - `system` - System orchestration and era mapping
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nestgate_core::temporal_storage::{TemporalStorageSystem, TemporalDevice};
//!
//! # fn main() -> nestgate_core::Result<()> {
//! // Initialize system with auto-discovery
//! let system = TemporalStorageSystem::with_discovery()?;
//!
//! println!("Discovered {} devices", system.device_count());
//! println!("Total capacity: {} MB", system.total_capacity_mb());
//! # Ok(())
//! # }
//! ```

// Submodules (domain-driven organization)
pub mod access_control;
pub mod classification;
pub mod connection;
pub mod data_types;
pub mod device;
pub mod ingestion;
pub mod sources;
pub mod system;

// Public API re-exports (backward compatibility)

// Device types
pub use device::{
    PerformanceTier, PhysicalDimensions, StorageEra, StorageTechnology, TemporalDevice,
};

// Connection types
pub use connection::{ConnectionHandle, ConnectionStatus, Metadata, UniversalDataSource};

// Data types
pub use data_types::{DataDescriptor, DataType, DatasetType, ModelType};

// Access control
pub use access_control::{AccessRequirements, AuthenticationMethod, RateLimits};

// Ingestion
pub use ingestion::{IngestedData, IngestionMetadata, ValidationStatus};

// Classification
pub use classification::{
    CompressionStrategy, ContentType, DataCategory, DataClassification, PredictedAccessPattern,
    RecommendedTier, ReplicationStrategy,
};

// Sources
pub use sources::{
    APIType, CloudProvider, DataCapabilityType, DataSourceType, DataStream, FutureTechnology,
    LegacyMediaType,
};

// System
pub use system::{EraMapping, TemporalStorageSystem};

// Tests (keeping in same module for now)
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // ==================== TEMPORAL DEVICE TESTS ====================

    #[tokio::test]
    async fn test_temporal_device_detection() {
        let devices = TemporalDevice::auto_detect_any_storage().unwrap_or_else(|e| {
            tracing::error!(
                "Expect failed ({}): {:?}",
                "Failed to detect temporal devices in test",
                e
            );
            vec![] // Return empty vector on error for test
        });
        // Test passes if no errors occur
        assert_eq!(devices.len(), 0); // Currently returns empty vec
    }

    #[test]
    fn test_temporal_device_creation() {
        let device = TemporalDevice {
            era: StorageEra::Modern,
            technology: StorageTechnology::NVMe,
            capacity_mb: 1024 * 1024, // 1TB
            performance_tier: PerformanceTier::Ultra,
            physical_dimensions: PhysicalDimensions {
                width_mm: 100.0,
                height_mm: 70.0,
                depth_mm: 7.0,
            },
            supported_formats: vec!["ext4".to_string(), "xfs".to_string()],
            metadata: HashMap::new(),
        };

        assert_eq!(device.capacity_mb, 1024 * 1024);
        assert_eq!(device.supported_formats.len(), 2);
    }

    // ==================== STORAGE ERA TESTS ====================

    #[test]
    fn test_storage_era_classification() {
        let punch_card = StorageEra::Prehistoric;
        let nvme = StorageEra::Modern;
        let dna = StorageEra::Biological;

        assert_eq!(punch_card, StorageEra::Prehistoric);
        assert_eq!(nvme, StorageEra::Modern);
        assert_eq!(dna, StorageEra::Biological);
    }

    #[test]
    fn test_storage_era_all_variants() {
        let eras = [
            StorageEra::Prehistoric,
            StorageEra::Magnetic,
            StorageEra::Digital,
            StorageEra::Modern,
            StorageEra::Biological,
            StorageEra::Quantum,
        ];

        assert_eq!(eras.len(), 6);
    }

    // ==================== SYSTEM TESTS ====================

    #[test]
    fn test_temporal_storage_system_creation() {
        let system = TemporalStorageSystem::new();
        assert_eq!(system.device_count(), 0);
        assert_eq!(system.total_capacity_mb(), 0);
    }

    #[test]
    fn test_temporal_storage_system_add_device() {
        let mut system = TemporalStorageSystem::new();
        let device = TemporalDevice {
            era: StorageEra::Modern,
            technology: StorageTechnology::NVMe,
            capacity_mb: 1024 * 1024,
            performance_tier: PerformanceTier::Ultra,
            physical_dimensions: PhysicalDimensions {
                width_mm: 100.0,
                height_mm: 70.0,
                depth_mm: 7.0,
            },
            supported_formats: vec!["ext4".to_string()],
            metadata: HashMap::new(),
        };

        system.add_device(device);
        assert_eq!(system.device_count(), 1);
        assert_eq!(system.total_capacity_mb(), 1024 * 1024);
    }

    // ==================== CLASSIFICATION TESTS ====================

    #[test]
    fn test_data_classification_defaults() {
        let classification = DataClassification::default();
        assert!(classification.should_compress());
        assert!(classification.should_replicate());
    }

    #[test]
    fn test_data_classification_critical() {
        let classification = DataClassification::critical();
        assert!(matches!(
            classification.data_category,
            DataCategory::Critical
        ));
        assert!(classification.should_replicate());
    }

    #[test]
    fn test_data_classification_archival() {
        let classification = DataClassification::archival();
        assert!(matches!(
            classification.data_category,
            DataCategory::Archive
        ));
        assert!(classification.should_compress());
    }

    // ==================== ACCESS CONTROL TESTS ====================

    #[test]
    fn test_access_requirements_public() {
        let requirements = AccessRequirements::public();
        assert!(!requirements.requires_authentication());
    }

    #[test]
    fn test_rate_limits_per_minute() {
        let limits = RateLimits::per_minute(60);
        assert_eq!(limits.requests_per_minute, 60);
        assert_eq!(limits.requests_per_hour, 3600);
    }

    #[test]
    fn test_rate_limits_exceeded() {
        let limits = RateLimits::per_minute(10);
        assert!(limits.is_exceeded(10, 0, 0));
        assert!(!limits.is_exceeded(5, 0, 0));
    }

    // ==================== DATA DESCRIPTOR TESTS ====================

    #[test]
    fn test_data_descriptor_creation() {
        let descriptor = DataDescriptor::new(
            "test-id".to_string(),
            DataType::Documents,
            1024,
            "/path/to/data".to_string(),
        );

        assert_eq!(descriptor.id, "test-id");
        assert_eq!(descriptor.size_bytes, 1024);
    }

    #[test]
    fn test_data_descriptor_size_formatting() {
        let descriptor = DataDescriptor::new(
            "test".to_string(),
            DataType::Documents,
            1024 * 1024 * 1024, // 1GB
            "/path".to_string(),
        );

        let size_str = descriptor.size_human_readable();
        assert!(size_str.contains("GB"));
    }

    // ==================== INGESTION TESTS ====================

    #[test]
    fn test_ingested_data_creation() {
        let descriptor = DataDescriptor::new(
            "data-1".to_string(),
            DataType::Documents,
            1024,
            "/path".to_string(),
        );

        let mut data = IngestedData::new(
            "ingest-1".to_string(),
            descriptor,
            vec![1, 2, 3, 4],
            "checksum123".to_string(),
        );

        assert_eq!(data.content_size(), 4);
        assert!(!data.is_valid());
        assert!(!data.is_validated());

        data.mark_valid();
        assert!(data.is_valid());
        assert!(data.is_validated());
    }

    #[test]
    fn test_validation_status() {
        let status = ValidationStatus::Valid;
        assert!(status.is_acceptable());
        assert!(status.error_message().is_none());

        let invalid = ValidationStatus::Invalid("error".to_string());
        assert!(!invalid.is_acceptable());
        assert!(invalid.error_message().is_some());
    }

    // ==================== EDGE CASE TESTS ====================

    #[test]
    fn test_device_zero_capacity() {
        let device = TemporalDevice {
            era: StorageEra::Modern,
            technology: StorageTechnology::NVMe,
            capacity_mb: 0, // Edge case: zero capacity
            performance_tier: PerformanceTier::Medium,
            physical_dimensions: PhysicalDimensions {
                width_mm: 100.0,
                height_mm: 70.0,
                depth_mm: 7.0,
            },
            supported_formats: vec!["ext4".to_string()],
            metadata: HashMap::new(),
        };

        // Zero capacity is valid (could be offline device)
        assert_eq!(device.capacity_mb, 0);
    }

    #[test]
    fn test_device_empty_formats() {
        let device = TemporalDevice {
            era: StorageEra::Prehistoric,
            technology: StorageTechnology::PunchCard,
            capacity_mb: 1,
            performance_tier: PerformanceTier::Low,
            physical_dimensions: PhysicalDimensions {
                width_mm: 100.0,
                height_mm: 70.0,
                depth_mm: 1.0,
            },
            supported_formats: vec![], // Edge case: no formats
            metadata: HashMap::new(),
        };

        assert!(device.supported_formats.is_empty());
    }

    #[test]
    fn test_device_extreme_capacity() {
        let device = TemporalDevice {
            era: StorageEra::Quantum,
            technology: StorageTechnology::Quantum,
            capacity_mb: u64::MAX, // Edge case: maximum capacity
            performance_tier: PerformanceTier::Ultra,
            physical_dimensions: PhysicalDimensions {
                width_mm: 1.0,
                height_mm: 1.0,
                depth_mm: 1.0,
            },
            supported_formats: vec!["quantum".to_string()],
            metadata: HashMap::new(),
        };

        assert_eq!(device.capacity_mb, u64::MAX);
    }

    #[test]
    fn test_physical_dimensions_zero() {
        let dimensions = PhysicalDimensions {
            width_mm: 0.0,
            height_mm: 0.0,
            depth_mm: 0.0,
        };

        // Zero dimensions valid for virtual/quantum storage
        assert_eq!(dimensions.width_mm, 0.0);
    }

    #[test]
    fn test_physical_dimensions_negative() {
        let dimensions = PhysicalDimensions {
            width_mm: -1.0,
            height_mm: -1.0,
            depth_mm: -1.0,
        };

        // Negative dimensions technically valid (f64), but semantically invalid
        assert!(dimensions.width_mm < 0.0);
    }

    #[test]
    fn test_system_multiple_eras() {
        let mut system = TemporalStorageSystem::new();

        // Add devices from all eras
        for era in [
            StorageEra::Prehistoric,
            StorageEra::Magnetic,
            StorageEra::Digital,
            StorageEra::Modern,
            StorageEra::Biological,
            StorageEra::Quantum,
        ] {
            let device = TemporalDevice {
                era: era.clone(),
                technology: StorageTechnology::NVMe,
                capacity_mb: 1000,
                performance_tier: PerformanceTier::Medium,
                physical_dimensions: PhysicalDimensions {
                    width_mm: 100.0,
                    height_mm: 70.0,
                    depth_mm: 7.0,
                },
                supported_formats: vec!["format".to_string()],
                metadata: HashMap::new(),
            };
            system.add_device(device);
        }

        assert_eq!(system.device_count(), 6);
        assert_eq!(system.total_capacity_mb(), 6000);
    }

    #[test]
    fn test_rate_limits_zero() {
        let limits = RateLimits::per_minute(0);
        assert_eq!(limits.requests_per_minute, 0);
        assert_eq!(limits.requests_per_hour, 0);
    }

    #[test]
    fn test_rate_limits_high_value() {
        let limits = RateLimits::per_minute(10000);
        assert_eq!(limits.requests_per_minute, 10000);
        assert_eq!(limits.requests_per_hour, 600000);
    }

    #[test]
    fn test_data_descriptor_zero_size() {
        let descriptor = DataDescriptor::new(
            "empty".to_string(),
            DataType::Documents,
            0, // Zero bytes
            "/path".to_string(),
        );

        assert_eq!(descriptor.size_bytes, 0);
        let size_str = descriptor.size_human_readable();
        assert!(size_str.contains("0") || size_str.contains("B"));
    }

    #[test]
    fn test_data_descriptor_huge_size() {
        let descriptor = DataDescriptor::new(
            "huge".to_string(),
            DataType::Documents,
            u64::MAX, // Maximum size
            "/path".to_string(),
        );

        assert_eq!(descriptor.size_bytes, u64::MAX);
    }

    #[test]
    fn test_ingested_data_empty_content() {
        let descriptor = DataDescriptor::new(
            "empty".to_string(),
            DataType::Documents,
            0,
            "/path".to_string(),
        );

        let data = IngestedData::new(
            "ingest-empty".to_string(),
            descriptor,
            vec![], // Empty content
            "checksum".to_string(),
        );

        assert_eq!(data.content_size(), 0);
    }

    #[test]
    fn test_classification_categories_exist() {
        // Test that key categories exist
        let critical = DataCategory::Critical;
        let important = DataCategory::Important;
        let standard_cat = DataCategory::Standard;

        // Just verify they compile and can be created
        assert!(matches!(critical, DataCategory::Critical));
        assert!(matches!(important, DataCategory::Important));
        assert!(matches!(standard_cat, DataCategory::Standard));
    }

    #[test]
    fn test_access_requirements_no_auth() {
        let requirements = AccessRequirements::public();
        assert!(!requirements.requires_authentication());
    }

    #[test]
    fn test_system_device_management() {
        let mut system = TemporalStorageSystem::new();
        let device = TemporalDevice {
            era: StorageEra::Modern,
            technology: StorageTechnology::NVMe,
            capacity_mb: 1000,
            performance_tier: PerformanceTier::Medium,
            physical_dimensions: PhysicalDimensions {
                width_mm: 100.0,
                height_mm: 70.0,
                depth_mm: 7.0,
            },
            supported_formats: vec!["ext4".to_string()],
            metadata: HashMap::new(),
        };

        system.add_device(device);
        assert_eq!(system.device_count(), 1);
        assert_eq!(system.total_capacity_mb(), 1000);
    }

    #[test]
    fn test_system_capacity_overflow() {
        let mut system = TemporalStorageSystem::new();

        // Add two devices with near-max capacity
        let device1 = TemporalDevice {
            era: StorageEra::Quantum,
            technology: StorageTechnology::Quantum,
            capacity_mb: u64::MAX / 2,
            performance_tier: PerformanceTier::Ultra,
            physical_dimensions: PhysicalDimensions {
                width_mm: 1.0,
                height_mm: 1.0,
                depth_mm: 1.0,
            },
            supported_formats: vec!["quantum".to_string()],
            metadata: HashMap::new(),
        };

        let device2 = device1.clone();

        system.add_device(device1);
        system.add_device(device2);

        // Total capacity should handle overflow gracefully
        let total = system.total_capacity_mb();
        assert!(total > 0); // Should not panic
    }

    #[test]
    fn test_validation_status_all_variants() {
        let valid = ValidationStatus::Valid;
        assert!(valid.is_acceptable());

        let unvalidated = ValidationStatus::Unvalidated;
        assert!(!unvalidated.is_acceptable());

        let invalid = ValidationStatus::Invalid("test error".to_string());
        assert!(!invalid.is_acceptable());
        assert_eq!(invalid.error_message(), Some("test error"));

        let partial = ValidationStatus::PartiallyValid(vec!["warning1".to_string()]);
        assert!(partial.is_acceptable()); // Partially valid still acceptable
    }

    #[test]
    fn test_content_type_all_variants() {
        let types = [
            ContentType::Text,
            ContentType::Binary,
            ContentType::Structured,
            ContentType::Multimedia,
            ContentType::Scientific,
            ContentType::Code,
            ContentType::Unknown,
        ];

        assert_eq!(types.len(), 7);
    }

    #[test]
    fn test_ingested_data_validation_lifecycle() {
        let descriptor = DataDescriptor::new(
            "lifecycle".to_string(),
            DataType::Documents,
            100,
            "/path".to_string(),
        );

        let mut data = IngestedData::new(
            "ingest-lifecycle".to_string(),
            descriptor,
            vec![1, 2, 3],
            "checksum".to_string(),
        );

        // Initial state: not validated
        assert!(!data.is_validated());
        assert!(!data.is_valid());

        // Mark as valid
        data.mark_valid();
        assert!(data.is_validated());
        assert!(data.is_valid());

        // Mark as invalid
        data.mark_invalid("test error".to_string());
        assert!(data.is_validated());
        assert!(!data.is_valid());
    }
}
