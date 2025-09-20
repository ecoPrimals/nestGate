// **UNIFIED DYNAMIC DISCOVERY CONFIGURATION**
//! Unified Dynamic Config functionality and utilities.
// This configuration has been consolidated into the canonical configuration system.
// All dynamic discovery configurations are now available through the canonical domains.

/// Re-export canonical configurations for backward compatibility
pub use crate::config::canonical_master::domains::{
    CanonicalNetworkConfig,
    CanonicalStorageConfig,
    CanonicalSecurityConfig,
    CanonicalPerformanceConfig,
};
/// Backward compatibility type alias
pub type UnifiedDynamicConfig = CanonicalNetworkConfig;
/// Default unified dynamic configuration
pub const fn default_unified_config() -> CanonicalNetworkConfig {
    CanonicalNetworkConfig::default()
}
