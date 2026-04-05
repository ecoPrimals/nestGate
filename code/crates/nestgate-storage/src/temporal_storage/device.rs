// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! **Hardware & Physical Storage Devices**
//!
//! Domain: Physical storage devices across all technology eras
//!
//! This module handles:
//! - Storage device abstraction (from punch cards to quantum storage)
//! - Era classification (Prehistoric → Quantum)
//! - Technology types (`PunchCard` → Quantum)
//! - Performance tiers and physical dimensions
//! - Device detection and discovery

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core temporal device abstraction
///
/// Represents any storage device across all technology eras,
/// from 1890s punch cards to future quantum storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalDevice {
    /// Technology era this device belongs to
    pub era: StorageEra,
    /// Specific storage technology
    pub technology: StorageTechnology,
    /// Capacity in megabytes
    pub capacity_mb: u64,
    /// Performance tier classification
    pub performance_tier: PerformanceTier,
    /// Physical dimensions of the device
    pub physical_dimensions: PhysicalDimensions,
    /// File systems or formats supported
    pub supported_formats: Vec<String>,
    /// Additional device metadata
    pub metadata: HashMap<String, String>,
}

/// Storage technology eras
///
/// Classification of storage devices by historical era.
/// This enables temporal-aware storage management.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageEra {
    /// 1890s-1960s: Punch card era
    Prehistoric,
    /// 1960s-1990s: Magnetic tape/floppy era
    Magnetic,
    /// 1990s-2010s: Digital era (HDD/SSD)
    Digital,
    /// 2010s-present: Modern `NVMe` era
    Modern,
    /// 2020s+: Biological/DNA storage era
    Biological,
    /// Future: Quantum storage era
    Quantum,
}

/// Storage technology types
///
/// Specific storage technology implementations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageTechnology {
    /// Punch card technology (1890s-1960s)
    PunchCard,
    /// Floppy disk technology (1970s-1990s)
    Floppy,
    /// Magnetic tape technology (1950s-present)
    MagneticTape,
    /// Hard disk drive technology (1950s-present)
    HardDisk,
    /// Solid state drive technology (2000s-present)
    SolidState,
    /// `NVMe` technology (2010s-present)
    NVMe,
    /// DNA storage technology (experimental/future)
    Dna,
    /// Quantum storage technology (future)
    Quantum,
}

/// Performance tiers
///
/// Classification of storage performance characteristics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceTier {
    /// Low performance (archival, cold storage)
    Low,
    /// Medium performance (standard storage)
    Medium,
    /// High performance (hot storage, databases)
    High,
    /// Ultra performance (`NVMe`, in-memory)
    Ultra,
}

/// Physical dimensions of a storage device
///
/// Represents the physical form factor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalDimensions {
    /// Width in millimeters
    pub width_mm: f64,
    /// Height in millimeters
    pub height_mm: f64,
    /// Depth in millimeters
    pub depth_mm: f64,
}

impl TemporalDevice {
    /// Auto-detect any storage devices
    ///
    /// Scans the system for storage devices across all eras.
    ///
    /// # Returns
    ///
    /// Vector of detected storage devices
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - System resources are unavailable
    /// - Detection fails due to permissions
    pub fn auto_detect_any_storage() -> Result<Vec<Self>> {
        let mut devices = Vec::new();

        // Detect legacy devices
        devices.extend(Self::detect_legacy_devices()?);

        // Detect modern devices
        devices.extend(Self::detect_modern_devices()?);

        // Detect future devices
        devices.extend(Self::detect_future_devices()?);

        Ok(devices)
    }

    /// Detect legacy storage devices
    ///
    /// Scans for prehistoric and magnetic era devices.
    const fn detect_legacy_devices() -> Result<Vec<Self>> {
        // FUTURE: Implement legacy device detection if hardware support needed
        // - Punch card readers (if any still exist)
        // - Floppy disk drives
        // - Magnetic tape drives
        Ok(vec![])
    }

    /// Detect modern storage devices
    ///
    /// Scans for digital and modern era devices (HDD, SSD, `NVMe`).
    const fn detect_modern_devices() -> Result<Vec<Self>> {
        // FUTURE: Implement modern device detection when physical hardware support needed
        // - Hard disk drives (HDD)
        // - Solid state drives (SSD)
        // - NVMe devices
        Ok(vec![])
    }

    /// Detect future/experimental storage devices
    ///
    /// Scans for biological and quantum storage devices.
    const fn detect_future_devices() -> Result<Vec<Self>> {
        // FUTURE: Implement future storage technologies when available
        // - DNA storage systems
        // - Quantum storage (when available)
        Ok(vec![])
    }
}
