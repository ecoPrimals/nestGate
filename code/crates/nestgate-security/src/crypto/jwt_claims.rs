// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JWT claim types for token-based authentication.
//!
//! This module provides the `JwtClaims` data structure used across
//! NestGate's authentication system. Actual JWT signing and verification
//! are delegated to the crypto capability provider via
//! `CryptoDelegate::sign_jwt` and `CryptoDelegate::verify_jwt`
//! in the `delegate` sibling module.

use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT claims payload.
///
/// Pure data structure — no cryptographic operations. Signing and
/// verification happen via the crypto capability provider IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject (user ID)
    pub sub: String,
    /// Issued at (Unix timestamp)
    pub iat: i64,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issuer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    /// Audience
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    /// Custom permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl JwtClaims {
    /// Create new claims with default values.
    pub fn new(subject: String, expiry_seconds: i64) -> Result<Self> {
        let now = i64::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| NestGateError::validation_error(format!("Time error: {e}")))?
                .as_secs(),
        )
        .unwrap_or(i64::MAX);

        Ok(Self {
            sub: subject,
            iat: now,
            exp: now + expiry_seconds,
            iss: Some(nestgate_config::constants::system::DEFAULT_SERVICE_NAME.to_string()),
            aud: None,
            permissions: None,
        })
    }

    /// Check if token is expired.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
            .unwrap_or(0);

        self.exp < now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claims_new_sets_fields() {
        let claims = JwtClaims::new("user123".to_string(), 3600).unwrap();
        assert_eq!(claims.sub, "user123");
        assert!(!claims.is_expired());
        assert!(claims.iss.is_some());
    }

    #[test]
    fn expired_claims_detected() {
        let mut claims = JwtClaims::new("user789".to_string(), -10).unwrap();
        claims.exp -= 100;
        assert!(claims.is_expired());
    }

    #[test]
    fn claims_serde_roundtrip() {
        let claims = JwtClaims::new("user456".to_string(), 3600).unwrap();
        let json = serde_json::to_string(&claims).unwrap();
        let back: JwtClaims = serde_json::from_str(&json).unwrap();
        assert_eq!(back.sub, "user456");
    }
}
