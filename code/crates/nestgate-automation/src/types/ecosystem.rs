//
// This module defines types for ecosystem integration and service management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Use canonical ServiceInfo instead of local definition
pub use nestgate_core::canonical_types::service::ServiceInfo;

/// Ecosystem configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemConfig {
    pub ecosystem_id: String,
    pub name: String,
    pub version: String,
    pub services: Vec<ServiceInfo>,
    pub metadata: HashMap<String, String>,
}
// ServiceInfo definition removed - use canonical_types::service::ServiceInfo

/// Capability provider for ecosystem integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityProvider {
    pub id: String,
    pub name: String,
    pub capabilities: Vec<String>,
    pub endpoint: String,
    pub status: ProviderStatus,
    pub metadata: HashMap<String, String>,
}
/// Provider status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderStatus {
    Active,
    Inactive,
    Error,
    Unknown,
}
impl Default for CapabilityProvider {
    fn default() -> Self { Self {
            id: "default".to_string(),
            name: "Default Provider".to_string(),
            capabilities: vec!["storage".to_string()],
            endpoint: nestgate_core::constants::canonical_defaults::network::build_api_url(),
            status: ProviderStatus::Active,
            metadata: HashMap::new(),
         }
}
