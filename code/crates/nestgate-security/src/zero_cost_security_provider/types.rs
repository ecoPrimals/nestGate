// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

///
/// This module contains all the core security data structures and types
/// used throughout the zero-cost security provider system.
///
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;
// ==================== SECTION ====================

/// **Credentials for authentication**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zerocostcredentials
pub struct ZeroCostCredentials {
    /// Username for authentication
    pub username: String,
    /// Password or authentication secret
    pub password: String,
    /// Authentication method to use
    pub auth_method: AuthMethod,
    /// Additional metadata for authentication
    pub metadata: HashMap<String, String>,
}
impl ZeroCostCredentials {
    /// Create new credentials with password authentication
    #[must_use]
    pub fn new_password(username: String, password: String) -> Self {
        Self {
            username,
            password,
            auth_method: AuthMethod::Password,
            metadata: HashMap::new(),
        }
    }

    /// Create new credentials with token authentication
    #[must_use]
    pub fn new_token(username: String, token: String) -> Self {
        Self {
            username,
            password: token,
            auth_method: AuthMethod::Token,
            metadata: HashMap::new(),
        }
    }

    /// Create new credentials with certificate authentication
    #[must_use]
    pub fn new_certificate(username: String, cert_data: String) -> Self {
        Self {
            username,
            password: cert_data,
            auth_method: AuthMethod::Certificate,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to credentials
    #[must_use]
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if credentials are valid (non-empty)
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        !self.username.is_empty() && !self.password.is_empty()
    }
}

/// **Authentication methods**
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
/// Authmethod
pub enum AuthMethod {
    /// Password-based authentication
    #[default]
    /// Password
    Password,
    /// Token-based authentication
    Token,
    /// Certificate-based authentication
    Certificate,
    /// Biometric authentication
    Biometric,
    /// Multi-factor authentication
    MultiFactor {
        /// List of authentication methods to use
        methods: Vec<String>,
    },
}
impl AuthMethod {
    /// Check if this is a multi-factor authentication method
    #[must_use]
    pub const fn is_multi_factor(&self) -> bool {
        matches!(self, Self::MultiFactor { .. })
    }

    /// Get the primary authentication method name
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Password => "password",
            Self::Token => "token",
            Self::Certificate => "certificate",
            Self::Biometric => "biometric",
            Self::MultiFactor { .. } => "multi_factor",
        }
    }

    /// Check if this method requires secure transport
    #[must_use]
    pub const fn requires_secure_transport(&self) -> bool {
        matches!(self, Self::Password | Self::MultiFactor { .. })
    }
}

/// **Authentication token**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zerocostauthtoken
pub struct ZeroCostAuthToken {
    /// Token string
    pub token: String,
    /// Token expiration time
    pub expires_at: SystemTime,
    /// Permissions granted by this token
    pub permissions: Vec<String>,
    /// User ID associated with this token
    pub user_id: String,
    /// Session ID for this token
    pub session_id: String,
    /// Token issued timestamp
    pub issued_at: SystemTime,
    /// Token issuer
    pub issuer: Option<String>,
    /// Token audience
    pub audience: Option<String>,
    /// Token metadata
    pub metadata: HashMap<String, String>,
}
impl ZeroCostAuthToken {
    /// Create a new authentication token
    #[must_use]
    pub fn new(
        token: String,
        user_id: String,
        permissions: Vec<String>,
        expires_in: std::time::Duration,
    ) -> Self {
        let now = SystemTime::now();
        Self {
            token,
            user_id,
            permissions,
            expires_at: now + expires_in,
            session_id: Uuid::new_v4().to_string(),
            issued_at: now,
            issuer: None,
            audience: None,
            metadata: HashMap::new(),
        }
    }

    /// Check if the token is expired
    #[must_use]
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }

    /// Check if the token has a specific permission
    #[must_use]
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }

    /// Get remaining validity duration
    #[must_use]
    pub fn remaining_validity(&self) -> Option<std::time::Duration> {
        self.expires_at.duration_since(SystemTime::now()).ok()
    }

    /// Add metadata to the token
    #[must_use]
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Set token issuer
    #[must_use]
    pub fn with_issuer(mut self, issuer: String) -> Self {
        self.issuer = Some(issuer);
        self
    }

    /// Set token audience
    #[must_use]
    pub fn with_audience(mut self, audience: String) -> Self {
        self.audience = Some(audience);
        self
    }
}

/// **Digital signature**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zerocostsignature
pub struct ZeroCostSignature {
    /// Signature algorithm used
    pub algorithm: String,
    /// Base64-encoded signature
    pub signature: String,
    /// Key ID used for signing
    pub key_id: String,
    /// Signature timestamp
    pub timestamp: SystemTime,
    /// Additional signature metadata
    pub metadata: HashMap<String, String>,
}
impl ZeroCostSignature {
    /// Create a new signature
    #[must_use]
    pub fn new(algorithm: String, signature: String, key_id: String) -> Self {
        Self {
            algorithm,
            signature,
            key_id,
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to the signature
    #[must_use]
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if the signature is valid (non-empty fields)
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        !self.algorithm.is_empty() && !self.signature.is_empty() && !self.key_id.is_empty()
    }

    /// Get signature age
    #[must_use]
    pub fn age(&self) -> std::time::Duration {
        SystemTime::now()
            .duration_since(self.timestamp)
            .unwrap_or_default()
    }
}

/// **Security operation result**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Securityoperationresult
pub struct SecurityOperationResult<T> {
    /// Operation success status
    pub success: bool,
    /// Operation result data
    pub data: Option<T>,
    /// Error message if operation failed
    pub error: Option<String>,
    /// Operation duration
    pub duration: std::time::Duration,
    /// Operation timestamp
    pub timestamp: SystemTime,
}
impl<T> SecurityOperationResult<T> {
    /// Create a successful operation result
    pub fn success(data: T, duration: std::time::Duration) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            duration,
            timestamp: SystemTime::now(),
        }
    }

    /// Create a failed operation result
    #[must_use]
    pub fn failure(error: String, duration: std::time::Duration) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            duration,
            timestamp: SystemTime::now(),
        }
    }

    /// Check if the operation was successful
    pub const fn is_success(&self) -> bool {
        self.success
    }

    /// Get the operation data if successful
    pub const fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// Get the error message if failed
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

/// **Security context for operations**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Securitycontext
pub struct SecurityContext {
    /// User ID performing the operation
    pub user_id: String,
    /// Session ID
    pub session_id: String,
    /// Request ID for tracing
    pub request_id: String,
    /// Client IP address
    pub client_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Additional context metadata
    pub metadata: HashMap<String, String>,
}
impl SecurityContext {
    /// Create a new security context
    #[must_use]
    pub fn new(user_id: String, session_id: String) -> Self {
        Self {
            user_id,
            session_id,
            request_id: Uuid::new_v4().to_string(),
            client_ip: None,
            user_agent: None,
            metadata: HashMap::new(),
        }
    }

    /// Add client information
    #[must_use]
    pub fn with_client_info(mut self, ip: Option<String>, user_agent: Option<String>) -> Self {
        self.client_ip = ip;
        self.user_agent = user_agent;
        self
    }

    /// Add metadata
    #[must_use]
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_creation() {
        let creds =
            ZeroCostCredentials::new_password("testuser".to_string(), "testpass".to_string());

        assert_eq!(creds.username, "testuser");
        assert_eq!(creds.password, "testpass");
        assert_eq!(creds.auth_method, AuthMethod::Password);
        assert!(creds.is_valid());
    }

    #[test]
    fn test_auth_token_expiration() {
        let token = ZeroCostAuthToken::new(
            "test-token".to_string(),
            "user123".to_string(),
            vec!["read".to_string()],
            std::time::Duration::from_secs(3600),
        );

        assert!(!token.is_expired());
        assert!(token.has_permission("read"));
        assert!(!token.has_permission("write"));
    }

    #[test]
    fn test_signature_validation() {
        let sig = ZeroCostSignature::new(
            "ECDSA-P256".to_string(),
            "base64signature".to_string(),
            "key123".to_string(),
        );

        assert!(sig.is_valid());
        assert_eq!(sig.algorithm, "ECDSA-P256");
    }
}
