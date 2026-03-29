// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Standalone token signing and workspace secret helpers (HMAC-SHA256).

use crate::zero_cost_security_provider::types::ZeroCostAuthToken;
use std::time::Duration;

/// Token manager for local operations
#[allow(dead_code)]
/// Manager for `AuthToken` operations
pub struct AuthTokenManager {
    signing_key: String,
}
impl AuthTokenManager {
    /// Create a new authentication token manager
    ///
    /// # Arguments
    ///
    /// * `signing_key` - Cryptographic key used for token signing and validation
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use nestgate_core::zero_cost_security_provider::authentication::AuthTokenManager;
    ///
    /// let manager = AuthTokenManager::new("my-secret-key".to_string());
    /// ```
    #[must_use]
    pub const fn new(signing_key: String) -> Self {
        Self { signing_key }
    }

    /// Create a new authentication token for a user
    ///
    /// Generates a unique, time-limited authentication token with specified permissions.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier for the user
    /// * `permissions` - List of permissions granted to this token
    /// * `expiry` - Duration until token expires
    ///
    /// # Returns
    ///
    /// A new `ZeroCostAuthToken` with the specified parameters
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use nestgate_core::zero_cost_security_provider::authentication::AuthTokenManager;
    /// use std::time::Duration;
    ///
    /// let manager = AuthTokenManager::new("secret-key".to_string());
    /// let token = manager.create_token(
    ///     "user123",
    ///     vec!["read".to_string(), "write".to_string()],
    ///     Duration::from_secs(3600)
    /// );
    /// ```
    #[must_use]
    pub fn create_token(
        &self,
        user_id: &str,
        permissions: Vec<String>,
        expiry: Duration,
    ) -> ZeroCostAuthToken {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        let payload = format!("token_{}_{}", user_id, uuid::Uuid::new_v4());
        let mut mac = Hmac::<Sha256>::new_from_slice(self.signing_key.as_bytes())
            .expect("HMAC accepts any key length");
        mac.update(payload.as_bytes());
        let sig = hex::encode(mac.finalize().into_bytes());
        let signed_token = format!("{payload}.{sig}");

        ZeroCostAuthToken::new(signed_token, user_id.to_string(), permissions, expiry)
    }

    /// Validate the cryptographic signature of a token
    ///
    /// Verifies that the token was signed with the correct key and has not been tampered with.
    ///
    /// # Arguments
    ///
    /// * `_token` - The token string to validate
    ///
    /// # Returns
    ///
    /// * `true` if the signature is valid
    /// * `false` if the signature is invalid or token is malformed
    ///
    /// # Security Note
    ///
    /// In production, this should use proper cryptographic verification (HMAC, RSA, etc.).
    /// Current implementation is a placeholder for development/testing.
    ///
    /// Validates a token's HMAC-SHA256 signature against the manager's secret key.
    ///
    /// Tokens are expected in the format `payload.signature` where signature is
    /// a hex-encoded HMAC-SHA256 of the payload.
    #[must_use]
    pub fn validate_token_signature(&self, token: &str) -> bool {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        let Some((payload, sig_hex)) = token.rsplit_once('.') else {
            return false;
        };
        let Ok(expected_sig) = hex::decode(sig_hex) else {
            return false;
        };
        let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(self.signing_key.as_bytes()) else {
            return false;
        };
        mac.update(payload.as_bytes());
        mac.verify_slice(&expected_sig).is_ok()
    }

    /// Creates a workspace secret using OS entropy and HMAC key derivation.
    /// # Errors
    ///
    /// Returns `NestGateError` if OS entropy or HMAC key derivation fails.
    pub fn create_workspace_secret(&self, workspace_id: &str) -> nestgate_types::Result<String> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        let mut entropy = [0u8; 32];
        getrandom::getrandom(&mut entropy).map_err(|e| {
            nestgate_types::error::NestGateError::internal_error(
                format!("OS entropy failed: {e}"),
                "security",
            )
        })?;

        let mac = Hmac::<Sha256>::new_from_slice(self.signing_key.as_bytes()).map_err(|e| {
            nestgate_types::error::NestGateError::internal_error(
                format!("HMAC key derivation failed: {e}"),
                "security",
            )
        })?;
        let mut mac = mac;
        mac.update(workspace_id.as_bytes());
        mac.update(&entropy);
        let result = mac.finalize().into_bytes();

        Ok(hex::encode(result))
    }
}
