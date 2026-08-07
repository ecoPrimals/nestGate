// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Security capability self-knowledge registry.
//!
//! Provides the **static capability manifest** for nestGate's own security domain.
//! These are not discovered at runtime from external primals — they are nestGate's
//! self-knowledge of what it can serve. External primals discover these via
//! `capabilities.list` / `primal.announce` over IPC.

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security capability types that can be advertised.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SecurityCapabilityType {
    /// User authentication services.
    Authentication,
    /// Authorization and permission management.
    Authorization,
    /// Encryption and cryptographic services.
    Encryption,
    /// TLS/SSL certificate management.
    CertificateManagement,
    /// Audit logging and compliance.
    AuditLogging,
    /// Threat detection and security monitoring.
    ThreatDetection,
    /// Access control and policy enforcement.
    AccessControl,
    /// Secret and credential management.
    SecretManagement,
}

/// Static metadata for an advertised security capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilityInfo {
    /// Type of security capability provided.
    pub capability_type: SecurityCapabilityType,
    /// Internal capability domain URI (not a network address).
    pub endpoint: String,
    /// API version string.
    pub version: String,
    /// List of supported operations for this capability.
    pub supported_operations: Vec<String>,
    /// Additional metadata key-value pairs.
    pub metadata: HashMap<String, String>,
}

/// Security capability self-knowledge registry.
///
/// Returns nestGate's own capability manifest. These are internal domain URIs
/// for routing, not external endpoints discovered at runtime.
#[derive(Debug)]
pub struct SecurityCapabilityDiscovery {
    discovered_capabilities:
        tokio::sync::RwLock<HashMap<SecurityCapabilityType, SecurityCapabilityInfo>>,
}

impl SecurityCapabilityDiscovery {
    /// Create new security capability registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            discovered_capabilities: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Return nestGate's static security capability manifest.
    ///
    /// # Errors
    ///
    /// Returns an error if the internal cache lock is poisoned.
    pub async fn discover_capabilities(&self) -> Result<Vec<SecurityCapabilityInfo>> {
        let capabilities = vec![
            Self::authentication_capability(),
            Self::authorization_capability(),
            Self::encryption_capability(),
        ];

        let mut cache = self.discovered_capabilities.write().await;
        for capability in &capabilities {
            cache.insert(capability.capability_type.clone(), capability.clone());
        }

        Ok(capabilities)
    }

    /// Get specific security capability by type.
    ///
    /// # Errors
    ///
    /// Returns an error if the internal cache lock is poisoned.
    pub async fn get_capability(
        &self,
        capability_type: &SecurityCapabilityType,
    ) -> Result<Option<SecurityCapabilityInfo>> {
        let cache = self.discovered_capabilities.read().await;
        Ok(cache.get(capability_type).cloned())
    }

    fn authentication_capability() -> SecurityCapabilityInfo {
        SecurityCapabilityInfo {
            capability_type: SecurityCapabilityType::Authentication,
            endpoint: "local://identity.authenticate".into(),
            version: "1.0.0".into(),
            supported_operations: vec![
                "authenticate".into(),
                "validate_token".into(),
                "refresh_token".into(),
                "logout".into(),
            ],
            metadata: HashMap::from([("source".into(), "self-knowledge".into())]),
        }
    }

    fn authorization_capability() -> SecurityCapabilityInfo {
        SecurityCapabilityInfo {
            capability_type: SecurityCapabilityType::Authorization,
            endpoint: "local://identity.authorize".into(),
            version: "1.0.0".into(),
            supported_operations: vec![
                "check_permission".into(),
                "grant_permission".into(),
                "revoke_permission".into(),
                "list_permissions".into(),
            ],
            metadata: HashMap::from([("source".into(), "self-knowledge".into())]),
        }
    }

    fn encryption_capability() -> SecurityCapabilityInfo {
        SecurityCapabilityInfo {
            capability_type: SecurityCapabilityType::Encryption,
            endpoint: "local://security.encrypt".into(),
            version: "1.0.0".into(),
            supported_operations: vec![
                "encrypt_data".into(),
                "decrypt_data".into(),
                "generate_key".into(),
                "rotate_keys".into(),
            ],
            metadata: HashMap::from([("source".into(), "self-knowledge".into())]),
        }
    }
}

impl Default for SecurityCapabilityDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

/// Get authentication endpoint from self-knowledge manifest.
pub async fn get_auth_endpoint(
    #[expect(unused_variables, reason = "adapter reserved for future capability-provider injection")]
    _adapter: &(),
) -> Result<String> {
    let discovery = SecurityCapabilityDiscovery::new();
    let capabilities = discovery.discover_capabilities().await?;
    for capability in capabilities {
        if matches!(
            capability.capability_type,
            SecurityCapabilityType::Authentication
        ) {
            return Ok(capability.endpoint);
        }
    }
    Ok("local://identity.authenticate".into())
}
