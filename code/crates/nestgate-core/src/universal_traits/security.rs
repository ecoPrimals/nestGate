// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **SECURITY TRAITS - CANONICAL MODERNIZED**
//! Security trait definitions for universal providers
// Security-related traits and types for universal primal integration.
// Native async traits without async_trait overhead for optimal performance.

use serde::{Deserialize, Serialize};

/// Security decision enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Securitydecision
pub enum SecurityDecision {
    /// Allow
    Allow,
    /// Deny
    Deny,
    /// Requireadditionalauth
    RequireAdditionalAuth,
    /// Requiremfa
    RequireMFA,
    /// Rate limit exceeded - includes time in seconds to wait before retrying
    RateLimit {
        /// Number of seconds to wait before retrying
        retry_after: u64,
    },
}
/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Credentials
pub struct Credentials {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// Mfa Token
    pub mfa_token: Option<String>,
    /// Client Info
    pub client_info: Option<String>,
}
/// Authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authtoken
pub struct AuthToken {
    /// Token
    pub token: String,
    /// Expires At
    pub expires_at: std::time::SystemTime,
    /// Permissions
    pub permissions: Vec<String>,
}
/// Cryptographic signature
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Signature
pub struct Signature {
    /// Algorithm
    pub algorithm: String,
    /// Signature
    pub signature: Vec<u8>,
    /// Key identifier
    pub key_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_security_decision_serialization() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let decision = SecurityDecision::Allow;
        let serialized = serde_json::to_string(&decision)?;
        let deserialized: SecurityDecision = serde_json::from_str(&serialized)?;
        assert_eq!(decision, deserialized);

        let rate_limit = SecurityDecision::RateLimit { retry_after: 300 };
        let serialized = serde_json::to_string(&rate_limit)?;
        let deserialized: SecurityDecision = serde_json::from_str(&serialized)?;
        assert_eq!(rate_limit, deserialized);
        Ok(())
    }

    #[test]
    fn test_credentials_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let credentials = Credentials {
            username: "test".to_string(),
            password: "pass".to_string(),
            mfa_token: Some("123456".to_string()),
            client_info: Some("mobile_app".to_string()),
        };

        let serialized = serde_json::to_string(&credentials)?;
        let deserialized: Credentials = serde_json::from_str(&serialized)?;
        assert_eq!(credentials.username, deserialized.username);
        assert_eq!(credentials.password, deserialized.password);
        assert_eq!(credentials.mfa_token, deserialized.mfa_token);
        assert_eq!(credentials.client_info, deserialized.client_info);
        Ok(())
    }

    #[test]
    fn test_auth_token_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let token = AuthToken {
            token: "test_token".to_string(),
            expires_at: SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1000),
            permissions: vec!["read".to_string()],
        };

        let serialized = serde_json::to_string(&token)?;
        let deserialized: AuthToken = serde_json::from_str(&serialized)?;
        assert_eq!(token.token, deserialized.token);
        assert_eq!(token.permissions, deserialized.permissions);
        Ok(())
    }

    #[test]
    fn test_signature_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let signature = Signature {
            algorithm: "RS256".to_string(),
            signature: vec![1, 2, 3, 4],
            key_id: Some("test-key".to_string()),
        };

        let serialized = serde_json::to_string(&signature)?;
        let deserialized: Signature = serde_json::from_str(&serialized)?;
        assert_eq!(signature.algorithm, deserialized.algorithm);
        assert_eq!(signature.signature, deserialized.signature);
        Ok(())
    }
}
