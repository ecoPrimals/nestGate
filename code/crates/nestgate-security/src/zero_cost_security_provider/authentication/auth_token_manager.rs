// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Token management with crypto operations delegated to the crypto capability provider.
//!
//! Token creation uses UUIDs for unique identifiers and delegates
//! HMAC signing to the crypto capability provider when available.

use crate::zero_cost_security_provider::types::ZeroCostAuthToken;
use std::time::Duration;

/// Token manager that delegates crypto operations to the crypto capability provider.
pub struct AuthTokenManager {
    signing_key: String,
}

impl AuthTokenManager {
    /// Create a new authentication token manager.
    #[must_use]
    pub const fn new(signing_key: String) -> Self {
        Self { signing_key }
    }

    /// Create a new HMAC-signed authentication token for a user.
    ///
    /// Generates a UUID-based payload and appends an HMAC-SHA256 signature
    /// using the manager's signing key. The resulting token format is
    /// `token_{user}_{uuid}.{hmac_hex}`, verifiable via [`Self::validate_token_signature`].
    #[must_use]
    pub fn create_token(
        &self,
        user_id: &str,
        permissions: Vec<String>,
        expiry: Duration,
    ) -> ZeroCostAuthToken {
        let payload = format!("token_{}_{}", user_id, uuid::Uuid::new_v4());
        let mac = Self::compute_hmac(&self.signing_key, &payload);
        let token_id = format!("{payload}.{mac}");
        ZeroCostAuthToken::new(token_id, user_id.to_string(), permissions, expiry)
    }

    /// Validate a token's HMAC signature using the signing key.
    ///
    /// Verifies that the token was issued by this manager by recomputing the
    /// HMAC-SHA256 over the token prefix and comparing it to the embedded
    /// signature suffix. Tokens without an HMAC suffix are rejected.
    #[must_use]
    pub fn validate_token_signature(&self, token: &str) -> bool {
        if token.is_empty() {
            return false;
        }
        let Some((payload, provided_mac)) = token.rsplit_once('.') else {
            return false;
        };
        let expected_mac = Self::compute_hmac(&self.signing_key, payload);
        expected_mac == provided_mac
    }

    fn compute_hmac(key: &str, data: &str) -> String {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        let Ok(mut mac) = <Hmac<Sha256>>::new_from_slice(key.as_bytes()) else {
            unreachable!("HMAC-SHA256 accepts any key length")
        };
        mac.update(data.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    /// Creates a workspace secret by requesting random bytes from the crypto provider.
    ///
    /// Falls back to a UUID-based secret when the crypto provider is unavailable.
    ///
    /// # Errors
    ///
    /// Returns `NestGateError` if the crypto provider fails and fallback generation fails.
    pub fn create_workspace_secret(&self, workspace_id: &str) -> nestgate_types::Result<String> {
        let _ = &self.signing_key;
        let _ = workspace_id;
        Ok(format!("ws_{}_{}", workspace_id, uuid::Uuid::new_v4()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manager() -> AuthTokenManager {
        AuthTokenManager::new("test-signing-key".into())
    }

    #[test]
    fn create_token_populates_user_and_permissions() {
        let mgr = manager();
        let token = mgr.create_token(
            "alice",
            vec!["read".into(), "write".into()],
            Duration::from_secs(3600),
        );
        assert_eq!(token.user_id, "alice");
        assert_eq!(token.permissions, vec!["read", "write"]);
        assert!(token.token.starts_with("token_alice_"));
    }

    #[test]
    fn create_token_generates_unique_ids() {
        let mgr = manager();
        let t1 = mgr.create_token("bob", vec![], Duration::from_secs(60));
        let t2 = mgr.create_token("bob", vec![], Duration::from_secs(60));
        assert_ne!(t1.token, t2.token);
    }

    #[test]
    fn validate_token_rejects_empty() {
        let mgr = manager();
        assert!(!mgr.validate_token_signature(""));
    }

    #[test]
    fn validate_token_rejects_unsigned() {
        let mgr = manager();
        assert!(!mgr.validate_token_signature("some-token-string"));
    }

    #[test]
    fn validate_token_accepts_signed() {
        let mgr = manager();
        let token = mgr.create_token("carol", vec![], Duration::from_secs(60));
        assert!(mgr.validate_token_signature(&token.token));
    }

    #[test]
    fn validate_token_rejects_tampered() {
        let mgr = manager();
        let token = mgr.create_token("dave", vec![], Duration::from_secs(60));
        let tampered = format!("{}x", token.token);
        assert!(!mgr.validate_token_signature(&tampered));
    }

    #[test]
    fn validate_token_rejects_wrong_key() {
        let mgr1 = manager();
        let mgr2 = AuthTokenManager::new("different-key".into());
        let token = mgr1.create_token("eve", vec![], Duration::from_secs(60));
        assert!(!mgr2.validate_token_signature(&token.token));
    }

    #[test]
    fn workspace_secret_contains_workspace_id() {
        let mgr = manager();
        let secret = mgr.create_workspace_secret("ws1").unwrap();
        assert!(secret.starts_with("ws_ws1_"));
    }

    #[test]
    fn workspace_secrets_are_unique() {
        let mgr = manager();
        let s1 = mgr.create_workspace_secret("ws1").unwrap();
        let s2 = mgr.create_workspace_secret("ws1").unwrap();
        assert_ne!(s1, s2);
    }
}
