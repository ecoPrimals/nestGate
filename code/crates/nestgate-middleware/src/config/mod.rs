// Simplified, unified middleware configuration using canonical patterns

use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;

/// Middleware configuration type alias
pub type MiddlewareConfig = NestGateCanonicalConfig;
/// Create default middleware configuration
#[must_use]
pub fn create_default_config() -> MiddlewareConfig {
    NestGateCanonicalConfig::default()
}
