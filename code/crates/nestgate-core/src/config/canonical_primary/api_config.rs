//! **⚠️ DEPRECATED - USE CANONICAL INSTEAD**
//!
//! This module is deprecated. Use the canonical API configuration types instead.
//!
//! **Migration**: Use `nestgate_core::config::canonical_primary::domains::network::ApiConfig`

/// **API CONFIGURATION**
///
/// API and handler configuration types.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// API configuration (DEPRECATED)
///
/// **Migration Path**: Use `crate::config::canonical_primary::domains::network::ApiConfig` instead.
#[deprecated(
    since = "0.2.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::ApiConfig"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Enable API
    pub enabled: bool,
    /// API version
    pub version: String,
    /// API settings
    pub api_settings: HashMap<String, serde_json::Value>,
}
impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            version: "v1".to_string(),
            api_settings: HashMap::new(),
        }
    }
}
