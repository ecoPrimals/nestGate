// Security Adapter
//! Security Adapter functionality and utilities.
// This module provides the adapter-based implementation for security operations,
//! replacing hardcoded Security integrations with the universal adapter pattern.

use crate::universal_adapter::CapabilityRequest;
use crate::ecosystem_integration::{
    AuthenticationRequest, AuthenticationResponse, AuthorizationRequest, AuthorizationResponse,
    EncryptionRequest, EncryptionResponse, UniversalAdapter,
};
use crate::{NestGateError, Result};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

/// Credentials for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub additional_fields: HashMap<String, serde_json::Value>,
}
/// Authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
    pub expires_at: Option<String>,
    pub permissions: Vec<String>,
    pub metadata: HashMap<String, String>,
}
/// Digital signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub algorithm: String,
    pub signature_data: Vec<u8>,
    pub key_id: Option<String>,
}
/// Security adapter using universal adapter pattern
#[derive(Debug, Clone)]
pub struct SecurityAdapter {
    /// Universal adapter for external primal communication
    adapter: Arc<UniversalAdapter>,
    /// Service name for security operations
    service_name: String,
}
impl SecurityAdapter {
    /// Create new security adapter
    pub fn new(adapter: Arc<UniversalAdapter>, service_name: String) -> Self {
        info!("🔐 Creating Security Adapter via Universal Adapter");
        info!("🔐 Service: {}", service_name);

        Self {
            adapter,
            service_name,
        }
    }

    // CANONICAL MODERNIZATION: Mock methods removed from production code
    // All testing should use proper test doubles or feature-gated test implementations

    /// Authenticate user credentials via security adapter
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken>  {
        info!(
            "🔐 Authenticating user via security adapter: {}",
            credentials.username
        );

        // Convert to security capability request
        let auth_request = AuthenticationRequest {
            username: credentials.username.clone(),
            credential_type: "password".to_string(),
            credential_data: {
                let mut data = HashMap::new();
                data.insert(
                    "password".to_string(),
                    serde_json::Value::String(credentials.password.clone()),
                );
                for (key, value) in &credentials.additional_fields {
                    data.insert(key.clone(), value.clone());
                }
                data
            },
            context: Some({
                let mut context = HashMap::new();
                context.insert("service".to_string(), self.name.clone());
                context.insert("auth_type".to_string(), "user_login".to_string());
                context
            }),
        };

        let payload = serde_json::to_vec(&auth_request).map_err(|e| NestGateError::internal_error(
            location: Some(format!("{})
            context: None)?;

        let request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "security.authentication".to_string(),
            payload,
            metadata: std::collections::HashMap::new(),
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(30)),
            priority: 8,               // High priority for authentication
            requires_encryption: true, // Authentication should be encrypted
        };

        // Execute via universal adapter
        match self.adapter.execute_capability(request).await {
            Ok(response) => {
                if response.success {
                    let auth_response: AuthenticationResponse =
                        serde_json::from_slice(&response.payload).map_err(|e| {
                            NestGateError::internal_error(
                                    "Failed to deserialize authentication response: {e)"
                                ),
                                location: Some(format!("{})
                                context: None}
                        )?;

                    if auth_response.authenticated {
                        let token = AuthToken {
                            token: auth_response
                                .token
                                .unwrap_or_else(|| "adapter_token".to_string()),
                            expires_at: auth_response.expires_at,
                            permissions: auth_response.permissions,
                            metadata: HashMap::new(),
                        };

                        info!(
                            "✅ User authenticated via security adapter: {}",
                            credentials.username
                        );
                        Ok(token)
                    } else {
                        error!(
                            "❌ Authentication failed via adapter for user: {}",
                            credentials.username
                        );
                        Err(NestGateError::Unauthorized {
                            message: "Authentication failed".to_string(),
                            location: Some(format!("{})
                        })
                    }
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{e:?}"))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    error!("❌ Authentication failed via adapter: {}", error_msg);
                    Err(NestGateError::Unauthorized {
                        message: format!("Authentication failed: {error_msg}"),
                        location: Some(format!("{e})
                    })
                }
            }
            Err(e) => {
                error!("❌ Security adapter communication failed: {e}");
                Err(NestGateError::internal_error(
                    location: Some(format!("{})
                    context: None})
            }
        }
    }

    /// Check authorization for resource access via security adapter
        info!(
            "🔐 Checking authorization via security adapter: {} -> {} ({})",
            user_id, resource, action
        );

        let auth_request = AuthorizationRequest {
            user_id: user_id.to_string(),
            action: action.to_string(),
            context: {
                let mut context = HashMap::new();
                context.insert(
                    "service".to_string(),
                    serde_json::Value::String(self.name.clone()),
                );
                context.insert(
                    "timestamp".to_string(),
                    serde_json::Value::String(chrono::Utc::now().to_rfc3339()),
                );
                context
            },
        };

        let payload = serde_json::to_vec(&auth_request).map_err(|e| NestGateError::internal_error(
            location: Some(format!("{})
            context: None)?;

        let request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "security.authorization".to_string(),
            payload,
            metadata: std::collections::HashMap::new(),
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(15)),
            priority: 7,                // High priority for authorization
            requires_encryption: false, // Authorization metadata doesn't need encryption
        };

        match self.adapter.execute_capability(request).await {
            Ok(response) => {
                if response.success {
                    let auth_response: AuthorizationResponse =
                        serde_json::from_slice(&response.payload).map_err(|e| {
                            NestGateError::internal_error(
                                    "Failed to deserialize authorization response: {e)"
                                ),
                                location: Some(format!("{})
                                context: None}
                        )?;

                    if auth_response.authorized {
                        info!(
                            "✅ Authorization granted via security adapter: {} -> {} ({})",
                            user_id, resource, action
                        );
                    } else {
                        warn!(
                            "⚠️ Authorization denied via security adapter: {} -> {} ({})",
                            user_id, resource, action
                        );
                        if let Some(reason) = &auth_response.reason {
                            debug!("Authorization denial reason: {}", reason);
                        }
                    }

                    Ok(auth_response.authorized)
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{e:?}"))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    error!("❌ Authorization check failed via adapter: {}", error_msg);
                    Err(NestGateError::internal_error(
                        location: Some(format!("{e})
                        context: None})
                }
            }
            Err(e) => {
                error!("❌ Security adapter communication failed: {e}");
                Err(NestGateError::internal_error(
                    location: Some(format!("{})
                    context: None})
            }
        }
    }

    /// Encrypt data via security adapter
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn encrypt(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>>  {
        info!(
            "🔐 Encrypting data via security adapter ({} bytes, {})",
            data.len(),
            algorithm
        );

        let encryption_request = EncryptionRequest {
            data: general_purpose::STANDARD.encode(data),
            algorithm: algorithm.to_string(),
            key_id: None, // Let the security primal choose the key
        };

        let payload =
            serde_json::to_vec(&encryption_request).map_err(|e| NestGateError::internal_error(
                location: Some(format!("{})
                context: None)?;

        let request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "security.encryption".to_string(),
            payload,
            metadata: std::collections::HashMap::new(),
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(60)),
            priority: 6,               // Medium-high priority for encryption
            requires_encryption: true, // Encryption requests should be secure
        };

        match self.adapter.execute_capability(request).await {
            Ok(response) => {
                if response.success {
                    let encryption_response: EncryptionResponse =
                        serde_json::from_slice(&response.payload).map_err(|e| {
                            NestGateError::internal_error(
                                location: Some(format!("{})
                                context: None}
                        )?;

                    let encrypted_data = general_purpose::STANDARD
                        .decode(&encryption_response.encrypted_data)
                        .map_err(|e| NestGateError::internal_error(
                            location: Some(format!("{})
                            context: None)?;

                    info!(
                        "✅ Data encrypted via security adapter ({} bytes -> {} bytes)",
                        data.len(),
                        encrypted_data.len()
                    );
                    Ok(encrypted_data)
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{e:?}"))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    error!("❌ Encryption failed via adapter: {}", error_msg);
                    Err(NestGateError::internal_error(
                        location: Some(format!("{e})
                        context: None})
                }
            }
            Err(e) => {
                error!("❌ Security adapter communication failed: {e}");
                Err(NestGateError::internal_error(
                    location: Some(format!("{})
                    context: None})
            }
        }
    }

    /// Sign data via security adapter
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn sign_data(&self, data: &[u8]) -> Result<Signature>  {
        info!(
            "🔐 Signing data via security adapter ({} bytes)",
            data.len()
        );

        let signing_request = serde_json::json!({
            "data": general_purpose::STANDARD.encode(data),
            "algorithm": "RSA-SHA256", // Default signing algorithm
            "service": self.name
        );

        let payload =
            serde_json::to_vec(&signing_request).map_err(|e| NestGateError::internal_error(
                location: Some(format!("{})
                context: None)?;

        let request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "security.digital_signing".to_string(),
            payload,
            metadata: std::collections::HashMap::new(),
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(30)),
            priority: 6,               // Medium-high priority for signing
            requires_encryption: true, // Signing should be secure
        };

        match self.adapter.execute_capability(request).await {
            Ok(response) => {
                if response.success {
                    let signing_response: serde_json::Value =
                        serde_json::from_slice(&response.payload).map_err(|e| {
                            NestGateError::internal_error(
                                location: Some(format!("{})
                                context: None}
                        )?;

                    let signature_data = signing_response
                        .get("signature")
                        .and_then(|s| s.as_str())
                        .ok_or_else(|| NestGateError::internal_error(
                            context: None)?;

                    let signature = Signature {
                        algorithm: signing_response
                            .get("algorithm")
                            .and_then(|a| a.as_str())
                            .unwrap_or("RSA-SHA256")
                            .to_string(),
                        signature_data: general_purpose::STANDARD.decode(signature_data).map_err(
                            |e| NestGateError::internal_error(
                                location: Some(format!("{})
                                context: None},
                        )?,
                        key_id: signing_response
                            .get("key_id")
                            .and_then(|k| k.as_str())
                            .map(|s| s.to_string()),
                    };

                    info!(
                        "✅ Data signed via security adapter ({} bytes signature)",
                        signature.signature_data.len()
                    );
                    Ok(signature)
                } else {
                    let error_msg = response
                        .error
                        .map(|e| format!("{e:?}"))
                        .unwrap_or_else(|| "Unknown error".to_string());
                    error!("❌ Data signing failed via adapter: {}", error_msg);
                    Err(NestGateError::internal_error(
                        location: Some(format!("{e})
                        context: None})
                }
            }
            Err(e) => {
                error!("❌ Security adapter communication failed: {e}");
                Err(NestGateError::internal_error(
                    location: Some(format!("{})
                    context: None})
            }
        }
    }

    /// Health check for security adapter
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn health_check(&self) -> Result<bool>  {
        info!("🔐 Performing security adapter health check");

        let health_request = serde_json::json!({
            "service": self.name,
            "check_type": "connectivity"
        );

        let payload = serde_json::to_vec(&health_request).map_err(|e| NestGateError::internal_error(
            location: Some(format!("{})
            context: None)?;

        let request = CapabilityRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            capability_id: "security.health_check".to_string(),
            payload,
            metadata: std::collections::HashMap::new(),
            performance_requirements: None,
            timeout: Some(std::time::Duration::from_secs(10)),
            priority: 4, // Medium priority for health checks
            requires_encryption: false,
        };

        match self.adapter.execute_capability(request).await {
            Ok(response) => {
                let healthy = response.success;
                if healthy {
                    info!("✅ Security adapter health check passed");
                } else {
                    warn!("⚠️ Security adapter health check failed");
                }
                Ok(healthy)
            }
            Err(e) => {
                warn!(
                    "⚠️ Security adapter health check communication failed: {}",
                    e
                );
                Ok(false) // Return false rather than error for health checks
            }
        }
    }
}
