//! **JWT AUTHENTICATION PROVIDER**
//!
//! Legacy authentication provider using JSON Web Tokens with shared secrets.
//! Intended for:
//! - Standalone NAS deployments
//! - External client integrations
//! - Legacy systems
//!
//! Not recommended for primal-to-primal communication (use BearDog instead).

use async_trait::async_trait;
use std::collections::HashMap;
use std::env;
use tracing::{debug, warn};

use super::super::auth_provider::{AuthProvider, AuthRequest, AuthResponse, ProviderStatus};
use crate::Result;

/// JWT authentication provider
#[derive(Debug, Clone)]
pub struct JwtAuthProvider {
    /// JWT secret from environment
    secret: Option<String>,
    /// Whether to enforce secret validation
    enforce_secret: bool,
}

impl JwtAuthProvider {
    /// Create a new JWT authentication provider
    pub fn new() -> Self {
        let secret = env::var("NESTGATE_JWT_SECRET")
            .or_else(|_| env::var("JWT_SECRET"))
            .ok();

        let enforce_secret = env::var("NESTGATE_ENFORCE_JWT")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(true); // Enforce by default

        if secret.is_none() && enforce_secret {
            warn!(
                "⚠️  JWT authentication enabled but NESTGATE_JWT_SECRET not set. \
                 Set environment variable or use BearDog auth mode for production."
            );
        }

        Self {
            secret,
            enforce_secret,
        }
    }

    /// Create a JWT provider with explicit configuration
    pub fn with_config(secret: Option<String>, enforce_secret: bool) -> Self {
        Self {
            secret,
            enforce_secret,
        }
    }

    /// Validate a JWT token
    ///
    /// This is a simplified JWT validation.
    /// For production, you'd use a proper JWT library like `jsonwebtoken`.
    async fn validate_token(&self, token: &str) -> Result<String> {
        // Check if we have a secret
        let secret = match &self.secret {
            Some(s) => s,
            None => {
                if self.enforce_secret {
                    return Err(crate::error::NestGateError::Security(Box::new(
                        crate::error::SecurityErrorData {
                            message: "JWT secret not configured".to_string(),
                            principal: None,
                        },
                    )));
                } else {
                    // Development mode - accept any token
                    debug!("🔓 JWT validation bypassed (development mode)");
                    return Ok("dev-user".to_string());
                }
            }
        };

        // Basic JWT structure validation
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(crate::error::NestGateError::Security(Box::new(
                crate::error::SecurityErrorData {
                    message: "Invalid JWT format (expected 3 parts)".to_string(),
                    principal: None,
                },
            )));
        }

        // TODO: Proper JWT validation using jsonwebtoken crate
        // For now, this is a placeholder that demonstrates the pattern
        
        // In a real implementation:
        // 1. Decode and verify signature using secret
        // 2. Check expiration
        // 3. Validate claims
        // 4. Extract principal from claims

        debug!("✅ JWT token validated (simplified validation)");
        
        // Extract user from token (in real implementation, decode from JWT claims)
        // For now, return a placeholder
        Ok("jwt-user".to_string())
    }
}

impl Default for JwtAuthProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AuthProvider for JwtAuthProvider {
    fn name(&self) -> &str {
        "jwt"
    }

    fn can_handle(&self, request: &AuthRequest) -> bool {
        // Can handle requests with JWT token
        request.token.is_some()
    }

    async fn authenticate(&self, request: &AuthRequest) -> Result<AuthResponse> {
        let token = match &request.token {
            Some(t) => t,
            None => {
                return Ok(AuthResponse::failure(
                    "No JWT token provided",
                    "jwt".to_string(),
                ));
            }
        };

        match self.validate_token(token).await {
            Ok(principal) => {
                debug!("🔐 JWT authentication successful for: {}", principal);
                
                // Default permissions for JWT users
                // In production, these would come from JWT claims
                let permissions = vec![
                    "read".to_string(),
                    "write".to_string(),
                    "list".to_string(),
                ];

                let mut metadata = HashMap::new();
                metadata.insert("auth_type".to_string(), "legacy".to_string());
                metadata.insert("token_type".to_string(), "jwt".to_string());

                Ok(AuthResponse {
                    authenticated: true,
                    principal: Some(principal),
                    permissions,
                    auth_method: "jwt".to_string(),
                    metadata,
                    message: "JWT authentication successful".to_string(),
                })
            }
            Err(e) => {
                warn!("❌ JWT authentication failed: {}", e);
                Ok(AuthResponse::failure(
                    &format!("JWT validation failed: {}", e),
                    "jwt".to_string(),
                ))
            }
        }
    }

    async fn is_available(&self) -> bool {
        // JWT is available if secret is configured, or if enforcement is disabled
        self.secret.is_some() || !self.enforce_secret
    }

    fn status(&self) -> ProviderStatus {
        let mut info = HashMap::new();
        info.insert("mode".to_string(), "legacy".to_string());
        info.insert(
            "secret_configured".to_string(),
            self.secret.is_some().to_string(),
        );
        info.insert(
            "enforce_secret".to_string(),
            self.enforce_secret.to_string(),
        );

        ProviderStatus {
            name: "jwt".to_string(),
            available: self.secret.is_some() || !self.enforce_secret,
            mode: "legacy".to_string(),
            info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_jwt_provider_with_token() {
        let provider = JwtAuthProvider::with_config(
            Some("test-secret-at-least-32-characters-long".to_string()),
            false,
        );

        let request = AuthRequest {
            token: Some("header.payload.signature".to_string()),
            did: None,
            signature: None,
            payload: None,
            metadata: HashMap::new(),
        };

        assert!(provider.can_handle(&request));

        let response = provider.authenticate(&request).await.unwrap();
        assert!(response.authenticated);
        assert_eq!(response.auth_method, "jwt");
    }

    #[tokio::test]
    async fn test_jwt_provider_without_token() {
        let provider = JwtAuthProvider::with_config(
            Some("test-secret-at-least-32-characters-long".to_string()),
            false,
        );

        let request = AuthRequest {
            token: None,
            did: None,
            signature: None,
            payload: None,
            metadata: HashMap::new(),
        };

        assert!(!provider.can_handle(&request));
    }

    #[tokio::test]
    async fn test_jwt_provider_invalid_format() {
        let provider = JwtAuthProvider::with_config(
            Some("test-secret-at-least-32-characters-long".to_string()),
            true,
        );

        let request = AuthRequest {
            token: Some("invalid-token".to_string()),
            did: None,
            signature: None,
            payload: None,
            metadata: HashMap::new(),
        };

        let response = provider.authenticate(&request).await.unwrap();
        assert!(!response.authenticated);
        assert!(response.message.contains("Invalid JWT format"));
    }

    #[tokio::test]
    async fn test_jwt_provider_status() {
        let provider = JwtAuthProvider::with_config(
            Some("test-secret".to_string()),
            true,
        );

        let status = provider.status();
        assert_eq!(status.name, "jwt");
        assert_eq!(status.mode, "legacy");
        assert_eq!(status.info.get("secret_configured"), Some(&"true".to_string()));
    }

    #[tokio::test]
    async fn test_jwt_provider_availability() {
        let with_secret = JwtAuthProvider::with_config(
            Some("test-secret".to_string()),
            true,
        );
        assert!(with_secret.is_available().await);

        let without_secret_enforced = JwtAuthProvider::with_config(None, true);
        assert!(!without_secret_enforced.is_available().await);

        let without_secret_permissive = JwtAuthProvider::with_config(None, false);
        assert!(without_secret_permissive.is_available().await);
    }
}

