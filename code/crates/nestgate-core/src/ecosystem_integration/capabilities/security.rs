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
/// Request parameters for Encryption operation
pub struct EncryptionRequest {
    /// Data
    pub data: Vec<u8>,
    /// Algorithm
    pub algorithm: String,
    /// Key identifier
    pub key_id: Option<String>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

/// Encryption response data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Encryption operation
pub struct EncryptionResponse {
    /// Encrypted Data
    pub encrypted_data: Vec<u8>,
    /// Key identifier
    pub key_id: String,
    /// Algorithm Used
    pub algorithm_used: String,
    /// Checksum
    pub checksum: String,
}

/// Authentication request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Authentication operation
pub struct AuthenticationRequest {
    /// Credentials
    pub credentials: serde_json::Value,
    /// Auth Method
    pub auth_method: String,
    /// Scopes
    pub scopes: Vec<String>,
}

/// Authentication response data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Authentication operation
pub struct AuthenticationResponse {
    /// Authenticated
    pub authenticated: bool,
    /// Token
    pub token: Option<String>,
    /// Expires At
    pub expires_at: Option<String>,
    /// User identifier
    pub user_id: Option<String>,
}

/// Authorization request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Authorization operation
pub struct AuthorizationRequest {
    /// User identifier
    pub user_id: String,
    /// Resource
    pub resource: String,
    /// Action
    pub action: String,
    /// Context
    pub context: HashMap<String, serde_json::Value>,
}

/// Authorization response data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Authorization operation
pub struct AuthorizationResponse {
    /// Authorized
    pub authorized: bool,
    /// Reason
    pub reason: Option<String>,
    /// Required Permissions
    pub required_permissions: Vec<String>,
}

// Note: No mock implementations - use capability discovery system instead.
// See module documentation for migration guide.
