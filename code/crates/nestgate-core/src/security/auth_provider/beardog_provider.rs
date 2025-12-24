//! **BEARDOG CRYPTOGRAPHIC AUTHENTICATION PROVIDER**
//!
//! Primary authentication provider for primal-to-primal communication.
//! Uses decentralized identity (DID) and cryptographic signatures.
//!
//! Features:
//! - No shared secrets (sovereign)
//! - Decentralized identity
//! - Hardware Security Module (HSM) support via BearDog
//! - Algorithm-agnostic (genetic crypto)
//!
//! This is the recommended auth method for the ecoPrimals ecosystem.

use async_trait::async_trait;
use std::collections::HashMap;
use std::env;
use tracing::{debug, info, warn};

use super::super::auth_provider::{AuthProvider, AuthRequest, AuthResponse, ProviderStatus};
use crate::Result;

/// BearDog authentication provider
#[derive(Debug, Clone)]
pub struct BearDogAuthProvider {
    /// BearDog service URL
    beardog_url: Option<String>,
    /// Whether to fallback to mock validation if BearDog unavailable
    allow_fallback: bool,
}

impl BearDogAuthProvider {
    /// Create a new BearDog authentication provider
    pub fn new() -> Self {
        let beardog_url = env::var("BEARDOG_URL")
            .or_else(|_| env::var("BEARDOG_SERVICE_URL"))
            .ok();

        let allow_fallback = env::var("BEARDOG_ALLOW_FALLBACK")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false); // No fallback by default (strict)

        if beardog_url.is_none() {
            warn!(
                "⚠️  BearDog URL not configured. Set BEARDOG_URL environment variable. \
                 Falling back to JWT if available."
            );
        }

        Self {
            beardog_url,
            allow_fallback,
        }
    }

    /// Create a BearDog provider with explicit configuration
    pub fn with_config(beardog_url: Option<String>, allow_fallback: bool) -> Self {
        Self {
            beardog_url,
            allow_fallback,
        }
    }

    /// Verify a cryptographic signature via BearDog
    async fn verify_signature(
        &self,
        did: &str,
        signature: &[u8],
        payload: &[u8],
    ) -> Result<bool> {
        if let Some(url) = &self.beardog_url {
            debug!("🐻 Verifying signature via BearDog: {}", url);

            // TODO: Actual HTTP request to BearDog service
            // For now, this is a placeholder that demonstrates the pattern
            
            // In a real implementation:
            // 1. Send verification request to BearDog
            // 2. BearDog retrieves public key for DID
            // 3. BearDog verifies signature
            // 4. Return verification result

            info!("✅ Signature verification delegated to BearDog (mock)");
            
            // Mock success for development
            if self.allow_fallback {
                return Ok(true);
            }

            // In production, actually call BearDog here
            return Err(crate::error::NestGateError::Security(Box::new(
                crate::error::SecurityErrorData {
                    message: "BearDog integration not yet implemented - see HANDOFF_TO_BEARDOG.md"
                        .to_string(),
                    principal: Some(did.to_string()),
                },
            )));
        }

        if self.allow_fallback {
            warn!("🔄 BearDog unavailable, using fallback validation (INSECURE)");
            return Ok(true); // Allow in development
        }

        Err(crate::error::NestGateError::Security(Box::new(
            crate::error::SecurityErrorData {
                message: "BearDog service not configured and fallback disabled".to_string(),
                principal: Some(did.to_string()),
            },
        )))
    }
}

impl Default for BearDogAuthProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AuthProvider for BearDogAuthProvider {
    fn name(&self) -> &str {
        "beardog"
    }

    fn can_handle(&self, request: &AuthRequest) -> bool {
        // Can handle requests with DID and signature
        request.did.is_some() && request.signature.is_some()
    }

    async fn authenticate(&self, request: &AuthRequest) -> Result<AuthResponse> {
        let did = match &request.did {
            Some(d) => d,
            None => {
                return Ok(AuthResponse::failure(
                    "No DID provided",
                    "beardog".to_string(),
                ));
            }
        };

        let signature_hex = match &request.signature {
            Some(s) => s,
            None => {
                return Ok(AuthResponse::failure(
                    "No signature provided",
                    "beardog".to_string(),
                ));
            }
        };

        // Decode hex signature
        let signature = match hex::decode(signature_hex) {
            Ok(sig) => sig,
            Err(e) => {
                return Ok(AuthResponse::failure(
                    &format!("Invalid signature format: {}", e),
                    "beardog".to_string(),
                ));
            }
        };

        let payload = request.payload.as_deref().unwrap_or(&[]);

        match self.verify_signature(did, &signature, payload).await {
            Ok(true) => {
                info!("🐻 BearDog authentication successful for DID: {}", did);

                // Permissions for authenticated primal services
                let permissions = vec![
                    "read".to_string(),
                    "write".to_string(),
                    "list".to_string(),
                    "admin".to_string(), // Primals have elevated permissions
                ];

                let mut metadata = HashMap::new();
                metadata.insert("auth_type".to_string(), "decentralized".to_string());
                metadata.insert("crypto_system".to_string(), "beardog".to_string());
                metadata.insert("did".to_string(), did.clone());

                Ok(AuthResponse {
                    authenticated: true,
                    principal: Some(did.clone()),
                    permissions,
                    auth_method: "beardog".to_string(),
                    metadata,
                    message: "BearDog cryptographic authentication successful".to_string(),
                })
            }
            Ok(false) => {
                warn!("❌ BearDog signature verification failed for DID: {}", did);
                Ok(AuthResponse::failure(
                    "Signature verification failed",
                    "beardog".to_string(),
                ))
            }
            Err(e) => {
                warn!("❌ BearDog authentication error: {}", e);
                Ok(AuthResponse::failure(
                    &format!("BearDog service error: {}", e),
                    "beardog".to_string(),
                ))
            }
        }
    }

    async fn is_available(&self) -> bool {
        // BearDog is available if URL is configured, or if fallback is allowed
        self.beardog_url.is_some() || self.allow_fallback
    }

    fn status(&self) -> ProviderStatus {
        let mut info = HashMap::new();
        info.insert("mode".to_string(), "primary".to_string());
        info.insert(
            "beardog_configured".to_string(),
            self.beardog_url.is_some().to_string(),
        );
        info.insert(
            "allow_fallback".to_string(),
            self.allow_fallback.to_string(),
        );
        info.insert("sovereignty".to_string(), "decentralized".to_string());

        if let Some(url) = &self.beardog_url {
            info.insert("service_url".to_string(), url.clone());
        }

        ProviderStatus {
            name: "beardog".to_string(),
            available: self.beardog_url.is_some() || self.allow_fallback,
            mode: "primary".to_string(),
            info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_beardog_provider_with_did_and_signature() {
        let provider = BearDogAuthProvider::with_config(
            Some("http://localhost:8080".to_string()),
            true, // Allow fallback for testing
        );

        let request = AuthRequest {
            token: None,
            did: Some("did:primal:beardog:123".to_string()),
            signature: Some(hex::encode("test-signature")),
            payload: Some(b"test-payload".to_vec()),
            metadata: HashMap::new(),
        };

        assert!(provider.can_handle(&request));

        let response = provider.authenticate(&request).await.unwrap();
        assert!(response.authenticated);
        assert_eq!(response.auth_method, "beardog");
        assert!(response.permissions.contains(&"admin".to_string()));
    }

    #[tokio::test]
    async fn test_beardog_provider_without_did() {
        let provider = BearDogAuthProvider::with_config(
            Some("http://localhost:8080".to_string()),
            false,
        );

        let request = AuthRequest {
            token: None,
            did: None,
            signature: Some(hex::encode("test-signature")),
            payload: None,
            metadata: HashMap::new(),
        };

        assert!(!provider.can_handle(&request));
    }

    #[tokio::test]
    async fn test_beardog_provider_without_signature() {
        let provider = BearDogAuthProvider::with_config(
            Some("http://localhost:8080".to_string()),
            false,
        );

        let request = AuthRequest {
            token: None,
            did: Some("did:primal:beardog:123".to_string()),
            signature: None,
            payload: None,
            metadata: HashMap::new(),
        };

        assert!(!provider.can_handle(&request));
    }

    #[tokio::test]
    async fn test_beardog_provider_invalid_signature_format() {
        let provider = BearDogAuthProvider::with_config(
            Some("http://localhost:8080".to_string()),
            false,
        );

        let request = AuthRequest {
            token: None,
            did: Some("did:primal:beardog:123".to_string()),
            signature: Some("not-valid-hex!!!".to_string()),
            payload: None,
            metadata: HashMap::new(),
        };

        let response = provider.authenticate(&request).await.unwrap();
        assert!(!response.authenticated);
        assert!(response.message.contains("Invalid signature format"));
    }

    #[tokio::test]
    async fn test_beardog_provider_status() {
        let provider = BearDogAuthProvider::with_config(
            Some("http://localhost:8080".to_string()),
            true,
        );

        let status = provider.status();
        assert_eq!(status.name, "beardog");
        assert_eq!(status.mode, "primary");
        assert_eq!(
            status.info.get("beardog_configured"),
            Some(&"true".to_string())
        );
        assert_eq!(status.info.get("sovereignty"), Some(&"decentralized".to_string()));
    }

    #[tokio::test]
    async fn test_beardog_provider_availability() {
        let with_url = BearDogAuthProvider::with_config(
            Some("http://localhost:8080".to_string()),
            false,
        );
        assert!(with_url.is_available().await);

        let without_url_strict = BearDogAuthProvider::with_config(None, false);
        assert!(!without_url_strict.is_available().await);

        let without_url_fallback = BearDogAuthProvider::with_config(None, true);
        assert!(without_url_fallback.is_available().await);
    }
}

