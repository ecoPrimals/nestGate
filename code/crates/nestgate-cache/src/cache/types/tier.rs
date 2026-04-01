// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! [`StorageTier`] extension for the cache layer.

use std::time::Duration;

/// Storage tier for caching - use unified definition from `nestgate-types`
pub use nestgate_types::unified_enums::StorageTier;

/// Extension methods for [`StorageTier`] used by the cache layer (cannot use inherent impl on a foreign type).
pub trait CacheStorageTierExt {
    /// Get tier priority (lower number = higher priority)
    #[must_use]
    fn priority(&self) -> u8;

    /// Get typical access time for this tier
    #[must_use]
    fn typical_access_time(&self) -> Duration;
}

impl CacheStorageTierExt for StorageTier {
    fn priority(&self) -> u8 {
        match self {
            Self::Hot => 0,
            Self::Warm => 1,
            Self::Cool => 2,
            Self::Cold => 3,
            Self::Frozen => 4,
            Self::Custom(_) => 5,
        }
    }

    fn typical_access_time(&self) -> Duration {
        match self {
            Self::Hot => Duration::from_micros(100),
            Self::Warm => Duration::from_millis(1),
            Self::Cool => Duration::from_millis(10),
            Self::Cold => Duration::from_millis(100),
            Self::Frozen => Duration::from_secs(10),
            Self::Custom(_) => Duration::from_millis(50),
        }
    }
}
