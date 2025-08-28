// Simplified, unified middleware configuration using canonical patterns

use nestgate_core::config::canonical_master::NestGateCanonicalConfig;
use serde::{Deserialize, Serialize};

/// Middleware configuration type alias
pub type MiddlewareConfig = NestGateCanonicalConfig;

/// Create default middleware configuration
pub fn create_default_config() -> MiddlewareConfig {
    NestGateCanonicalConfig::default()
}
