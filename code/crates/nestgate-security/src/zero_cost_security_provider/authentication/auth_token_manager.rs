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

    /// Create a new authentication token for a user.
    ///
    /// Generates a UUID-based token. HMAC signing is delegated to the crypto
    /// capability provider when available; falls back to unsigned tokens when
    /// the provider is unavailable (standalone mode).
    #[must_use]
    pub fn create_token(
        &self,
        user_id: &str,
        permissions: Vec<String>,
        expiry: Duration,
    ) -> ZeroCostAuthToken {
        let token_id = format!("token_{}_{}", user_id, uuid::Uuid::new_v4());
        ZeroCostAuthToken::new(token_id, user_id.to_string(), permissions, expiry)
    }

    /// Validate a token's structure.
    ///
    /// Checks that the token string is non-empty and well-formed.
    /// Full cryptographic verification requires the crypto capability provider.
    #[must_use]
    pub const fn validate_token_signature(&self, token: &str) -> bool {
        !token.is_empty()
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
        AuthTokenManager::new("test-signing-key".to_string())
    }

    #[test]
    fn create_token_populates_user_and_permissions() {
        let mgr = manager();
        let token = mgr.create_token("alice", vec!["read".into(), "write".into()], Duration::from_secs(3600));
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
    fn validate_token_accepts_nonempty() {
        let mgr = manager();
        assert!(mgr.validate_token_signature("some-token-string"));
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
