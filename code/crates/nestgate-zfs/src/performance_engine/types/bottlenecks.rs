// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// This module contains all the data structures, enums, and types used by the
// performance optimization engine.

use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;

use crate::types::StorageTier;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Performance engine configuration
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::PerformanceEngineConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::PerformanceEngineConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct PerformanceEngineConfig {
    pub monitoring_interval: std::time::Duration,
    pub optimization_interval: std::time::Duration,
    pub bottleneck_detection_interval: std::time::Duration,
    pub max_concurrent_optimizations: usize,
    pub enable_ai_guidance: bool,
}
impl Default for PerformanceEngineConfig {
    fn default() -> Self {
        Self {
            monitoring_interval: std::time::Duration::from_secs(5),
            optimization_interval: std::time::Duration::from_secs(30),
            bottleneck_detection_interval: std::time::Duration::from_secs(10),
            max_concurrent_optimizations: 3,
            enable_ai_guidance: true,
        }
    }
}
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type PerformanceEngineConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using PerformanceEngineConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
