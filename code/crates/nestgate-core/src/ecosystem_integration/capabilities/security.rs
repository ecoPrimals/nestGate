/// Security Capabilities (BearDog Primal Integration)
///
/// Defines capability interfaces for authentication, authorization, and encryption
/// through the BearDog security primal.
use super::{CapabilityRequest, CapabilityResponse, UniversalCapability};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Authentication request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationRequest {
    pub username: String,
    pub credential_type: String,
    pub credential_data: HashMap<String, serde_json::Value>,
    pub context: Option<HashMap<String, String>>,
}

/// Authentication response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    pub authenticated: bool,
    pub user_id: Option<String>,
    pub token: Option<String>,
    pub expires_at: Option<String>,
    pub permissions: Vec<String>,
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

/// Encryption request parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionRequest {
    pub data: String,
    pub algorithm: String,
    pub key_id: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Encryption response data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionResponse {
    pub encrypted_data: String,
    pub key_id: String,
    pub algorithm_used: String,
    pub checksum: String,
}

/// Security capability trait for BearDog integration
#[async_trait]
pub trait SecurityCapability: UniversalCapability {
    /// Authenticate user credentials
    async fn authenticate(
        &self,
        request: AuthenticationRequest,
    ) -> Result<AuthenticationResponse, Box<dyn std::error::Error + Send + Sync>>;

    /// Check authorization for resource access
    async fn authorize(
        &self,
        request: AuthorizationRequest,
    ) -> Result<AuthorizationResponse, Box<dyn std::error::Error + Send + Sync>>;

    /// Encrypt sensitive data
    async fn encrypt(
        &self,
        request: EncryptionRequest,
    ) -> Result<EncryptionResponse, Box<dyn std::error::Error + Send + Sync>>;
}

/// Mock implementation for testing
pub struct MockSecurityCapability {
    enabled: bool,
}

impl MockSecurityCapability {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl Default for MockSecurityCapability {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UniversalCapability for MockSecurityCapability {
    async fn execute(
        &self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse, Box<dyn std::error::Error + Send + Sync>> {
        if !self.enabled {
            return Ok(CapabilityResponse::error(
                "Mock security capability is disabled",
            ));
        }

        match request.capability_id.as_str() {
            "security.authentication" => {
                let response_data = serde_json::to_value(AuthenticationResponse {
                    authenticated: true,
                    user_id: Some("mock-user-123".to_string()),
                    token: Some("mock-token-456".to_string()),
                    expires_at: Some("2024-12-31T23:59:59Z".to_string()),
                    permissions: vec!["read".to_string(), "write".to_string()],
                })?;
                Ok(CapabilityResponse::success(response_data))
            }
            "security.authorization" => {
                let response_data = serde_json::to_value(AuthorizationResponse {
                    authorized: true,
                    reason: None,
                    required_permissions: vec![],
                })?;
                Ok(CapabilityResponse::success(response_data))
            }
            "security.encryption" => {
                let response_data = serde_json::to_value(EncryptionResponse {
                    encrypted_data: "encrypted-mock-data".to_string(),
                    key_id: "mock-key-789".to_string(),
                    algorithm_used: "AES-256-GCM".to_string(),
                    checksum: "mock-checksum".to_string(),
                })?;
                Ok(CapabilityResponse::success(response_data))
            }
            _ => Ok(CapabilityResponse::error(format!(
                "Unknown capability: {}",
                request.capability_id
            ))),
        }
    }

    fn get_metadata(&self) -> HashMap<String, serde_json::Value> {
        HashMap::from([
            (
                "name".to_string(),
                serde_json::Value::String("Mock Security Capability".to_string()),
            ),
            (
                "version".to_string(),
                serde_json::Value::String("1.0.0".to_string()),
            ),
            (
                "capabilities".to_string(),
                serde_json::json!([
                    "security.authentication",
                    "security.authorization",
                    "security.encryption"
                ]),
            ),
        ])
    }

    async fn health_check(&self) -> bool {
        self.enabled
    }
}

#[async_trait]
impl SecurityCapability for MockSecurityCapability {
    async fn authenticate(
        &self,
        _request: AuthenticationRequest,
    ) -> Result<AuthenticationResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(AuthenticationResponse {
            authenticated: true,
            user_id: Some("mock-user".to_string()),
            token: Some("mock-token".to_string()),
            expires_at: None,
            permissions: vec!["mock-permission".to_string()],
        })
    }

    async fn authorize(
        &self,
        _request: AuthorizationRequest,
    ) -> Result<AuthorizationResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(AuthorizationResponse {
            authorized: true,
            reason: None,
            required_permissions: vec![],
        })
    }

    async fn encrypt(
        &self,
        _request: EncryptionRequest,
    ) -> Result<EncryptionResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(EncryptionResponse {
            encrypted_data: "mock-encrypted".to_string(),
            key_id: "mock-key".to_string(),
            algorithm_used: "mock-algorithm".to_string(),
            checksum: "mock-checksum".to_string(),
        })
    }
}
