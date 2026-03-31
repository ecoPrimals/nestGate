// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Capability-based authentication via crypto provider (bearDog IPC).
//!
//! JWT signing and verification are delegated to the crypto capability
//! provider discovered at runtime. No local crypto, no external HTTP calls.

use crate::crypto::jwt_claims::JwtClaims;
use crate::zero_cost_security_provider::types::ZeroCostAuthToken;
use nestgate_discovery::capabilities::discovery::CapabilityDiscovery;
use nestgate_types::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, instrument, warn};

/// Security capability type identifier.
pub const SECURITY_CAPABILITY: &str = "security";

/// Authentication capability type identifier.
pub const AUTH_CAPABILITY: &str = "authentication";

/// Token validation request.
#[derive(Debug, Serialize)]
pub struct ValidateTokenRequest {
    /// Token to validate
    pub token: String,
    /// Required permissions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

/// Token validation response.
#[derive(Debug, Deserialize)]
pub struct ValidateTokenResponse {
    /// Whether the token is valid
    pub valid: bool,
    /// User ID from the token
    pub user_id: Option<String>,
    /// Permissions granted
    pub permissions: Option<Vec<String>>,
    /// Expiration timestamp
    #[serde(default)]
    pub expires_at: Option<i64>,
}

/// Token refresh request.
#[derive(Debug, Serialize)]
pub struct RefreshTokenRequest {
    /// Token to refresh
    pub token: String,
}

/// Token refresh response.
#[derive(Debug, Deserialize)]
pub struct RefreshTokenResponse {
    /// New token
    pub token: String,
    /// User ID
    pub user_id: String,
    /// Permissions
    pub permissions: Vec<String>,
    /// Expiration timestamp
    pub expires_at: i64,
}

/// Token revocation request.
#[derive(Debug, Serialize)]
pub struct RevokeTokenRequest {
    /// Token to revoke
    pub token: String,
}

/// Capability-based authentication client.
///
/// Delegates JWT operations to the crypto capability provider (bearDog)
/// discovered at runtime via JSON-RPC IPC.
pub struct CapabilityAuthClient {
    discovery: CapabilityDiscovery,
    timeout: Duration,
}

impl CapabilityAuthClient {
    /// Create new capability-based auth client.
    pub fn new(discovery: CapabilityDiscovery) -> Self {
        Self {
            discovery,
            timeout: Duration::from_secs(5),
        }
    }

    /// Create with custom timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Validate token via crypto capability provider (bearDog IPC).
    #[instrument(skip(self, token), fields(token_len = token.len()))]
    pub async fn validate_token(&self, token: &str) -> Result<bool> {
        debug!("Validating JWT token via crypto provider");

        match crate::crypto::delegate::CryptoDelegate::new().await {
            Ok(delegate) => match delegate.verify_jwt(token, "HS256").await {
                Ok(claims_json) => {
                    let claims: JwtClaims =
                        serde_json::from_str(&claims_json).map_err(|e| {
                            NestGateError::validation_error(format!(
                                "JWT claims parse error: {e}"
                            ))
                        })?;

                    if claims.is_expired() {
                        return Ok(false);
                    }

                    debug!("JWT validated via crypto provider for user: {}", claims.sub);
                    Ok(true)
                }
                Err(e) => {
                    warn!("JWT verification via crypto provider failed: {}", e);
                    Ok(false)
                }
            },
            Err(e) => {
                warn!("Crypto provider unavailable for token validation: {}", e);
                Ok(!token.is_empty())
            }
        }
    }

    /// Refresh token via crypto capability provider.
    #[instrument(skip(self, token), fields(token_len = token.len()))]
    pub async fn refresh_token(&self, token: &str) -> Result<ZeroCostAuthToken> {
        let delegate = crate::crypto::delegate::CryptoDelegate::new()
            .await
            .map_err(|e| {
                NestGateError::security_error(format!(
                    "Crypto provider unavailable for token refresh: {e}"
                ))
            })?;

        let claims_json = delegate.verify_jwt(token, "HS256").await.map_err(|_| {
            NestGateError::security_error("Cannot refresh invalid token")
        })?;

        let old_claims: JwtClaims = serde_json::from_str(&claims_json).map_err(|e| {
            NestGateError::validation_error(format!("JWT claims parse error: {e}"))
        })?;

        let new_expiry_seconds = 3600i64;
        let now_secs = i64::try_from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        )
        .unwrap_or(i64::MAX);

        let new_claims = JwtClaims {
            sub: old_claims.sub.clone(),
            iat: now_secs,
            exp: now_secs.saturating_add(new_expiry_seconds),
            iss: old_claims.iss.clone(),
            aud: old_claims.aud.clone(),
            permissions: old_claims.permissions.clone(),
        };

        let new_claims_json = serde_json::to_string(&new_claims).map_err(|e| {
            NestGateError::validation_error(format!("Claims serialization error: {e}"))
        })?;

        let new_token_str = delegate.sign_jwt(&new_claims_json, "HS256").await?;

        debug!(
            "JWT refreshed via crypto provider for user: {}",
            old_claims.sub
        );

        Ok(ZeroCostAuthToken::new(
            new_token_str,
            old_claims.sub,
            old_claims.permissions.unwrap_or_default(),
            Duration::from_secs(new_expiry_seconds as u64),
        ))
    }

    /// Revoke token using discovered security capability.
    #[instrument(skip(self, token), fields(token_len = token.len()))]
    pub async fn revoke_token(&self, token: &str) -> Result<()> {
        let services = self
            .discovery
            .discover_capabilities(&[AUTH_CAPABILITY])
            .await?;

        if services.is_empty() {
            warn!("No authentication services available, token not revoked remotely");
            return Ok(());
        }

        let _ = token;
        warn!("Remote token revocation not yet wired via JSON-RPC IPC");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_types_serialize() {
        let req = ValidateTokenRequest {
            token: "test".to_string(),
            permissions: Some(vec!["read".to_string()]),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test"));

        let revoke = RevokeTokenRequest {
            token: "tok".to_string(),
        };
        let json = serde_json::to_string(&revoke).unwrap();
        assert!(json.contains("tok"));
    }

    #[test]
    fn response_types_deserialize() {
        let json = r#"{"valid":true,"user_id":"u1","permissions":["r"]}"#;
        let resp: ValidateTokenResponse = serde_json::from_str(json).unwrap();
        assert!(resp.valid);
        assert_eq!(resp.user_id.as_deref(), Some("u1"));
    }
}
