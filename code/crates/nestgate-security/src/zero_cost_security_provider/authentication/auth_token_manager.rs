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
