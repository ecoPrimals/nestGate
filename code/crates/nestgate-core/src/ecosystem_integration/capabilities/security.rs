//! Security Capabilities - Real Implementation via Capability Discovery
//!
//! **REMOVED MOCKS**: This module previously contained MockSecurityCapability.
//! **MODERN SOLUTION**: Use `universal_adapter::CapabilityDiscovery` instead.
//!
//! # Migration Guide
//!
//! **Old (Mock)**:
//! ```rust,ignore
//! let mock = MockSecurityCapability::new();
//! mock.encrypt(data).await?;
//! ```
//!
//! **New (Real Discovery)**:
//! ```rust,ignore
//! use nestgate_core::universal_adapter::capability_discovery::*;
//!
//! let discovery = CapabilityDiscovery::new();
//! let security_providers = discovery
//!     .discover(CapabilityType::security())
//!     .await?;
//!
//! if let Some(provider) = security_providers.first() {
//!     // Use discovered security primal (e.g., BearDog)
//!     provider.call("encrypt", data).await?;
//! }
//! ```
//!
//! No hardcoded "BearDog" or other primal names - pure capability-based discovery.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Encryption request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionRequest {
    pub data: Vec<u8>,
    pub algorithm: String,
    pub key_id: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Encryption response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionResponse {
    pub encrypted_data: Vec<u8>,
    pub key_id: String,
    pub algorithm_used: String,
    pub checksum: String,
}

/// Authentication request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationRequest {
    pub credentials: serde_json::Value,
    pub auth_method: String,
    pub scopes: Vec<String>,
}

/// Authentication response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    pub authenticated: bool,
    pub token: Option<String>,
    pub expires_at: Option<String>,
    pub user_id: Option<String>,
}

/// Authorization request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub user_id: String,
    pub resource: String,
    pub action: String,
    pub context: HashMap<String, serde_json::Value>,
}

/// Authorization response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResponse {
    pub authorized: bool,
    pub reason: Option<String>,
    pub required_permissions: Vec<String>,
}

// Note: No mock implementations - use capability discovery system instead.
// See module documentation for migration guide.
