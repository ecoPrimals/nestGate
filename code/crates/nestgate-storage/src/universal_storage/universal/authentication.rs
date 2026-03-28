// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Authentication Patterns
//!
//! Defines how authentication works, independent of vendor.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// How does authentication work?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthenticationPattern {
    /// No authentication required
    None,

    /// HTTP Basic Authentication (username:password in Base64)
    HttpBasic {
        /// Username
        username: String,
        /// Password (should be loaded from secure storage)
        password: SecretString,
    },

    /// Bearer token in Authorization header
    BearerToken {
        /// Token value
        token: SecretString,
        /// Token type (usually "Bearer")
        token_type: String,
    },

    /// API key in header or query parameter
    ApiKey {
        /// API key value
        key: SecretString,
        /// Where to place the API key
        location: ApiKeyLocation,
    },

    /// Signed request headers (like AWS Signature V4, but generic)
    ///
    /// This pattern is used by S3-like services, Azure, and others
    SignedHeaders {
        /// Signing algorithm to use
        signing_algorithm: SigningAlgorithm,
        /// Key identifier (access key, client ID, etc.)
        key_id: String,
        /// Secret key for signing
        secret_key: SecretString,
        /// Which headers to include in signature
        headers_to_sign: Vec<String>,
        /// Optional session token
        session_token: Option<SecretString>,
    },

    /// OAuth 2.0 / OIDC flow
    OAuth {
        /// Client ID
        client_id: String,
        /// Client secret
        client_secret: SecretString,
        /// Token endpoint URL
        token_endpoint: String,
        /// Requested scopes
        scopes: Vec<String>,
        /// Grant type
        grant_type: OAuthGrantType,
    },

    /// Mutual TLS (certificate-based authentication)
    MutualTls {
        /// Client certificate (PEM format)
        client_cert_pem: String,
        /// Client private key (PEM format)
        client_key_pem: SecretString,
    },

    /// Custom authentication scheme
    Custom {
        /// Scheme name
        scheme_name: String,
        /// Credential parameters
        credentials: HashMap<String, String>,
    },
}

impl AuthenticationPattern {
    /// Get a description of this authentication pattern
    pub fn description(&self) -> &str {
        match self {
            Self::None => "No authentication",
            Self::HttpBasic { .. } => "HTTP Basic",
            Self::BearerToken { .. } => "Bearer token",
            Self::ApiKey { .. } => "API key",
            Self::SignedHeaders { .. } => "Signed headers",
            Self::OAuth { .. } => "OAuth 2.0",
            Self::MutualTls { .. } => "Mutual TLS",
            Self::Custom { scheme_name, .. } => scheme_name,
        }
    }

    /// Is this authentication pattern secure?
    pub fn is_secure(&self) -> bool {
        match self {
            Self::None => false,
            Self::HttpBasic { .. } => false, // Only secure over TLS
            _ => true,
        }
    }
}

/// Secure string wrapper (doesn't impl Debug to avoid accidental logging)
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretString(String);

impl SecretString {
    /// Create from string
    pub fn new(s: String) -> Self {
        Self(s)
    }

    /// Create from environment variable
    pub fn from_env(var_name: &str) -> Option<Self> {
        std::env::var(var_name).ok().map(Self::new)
    }

    /// Get the inner value (use sparingly)
    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SecretString([REDACTED])")
    }
}

impl From<String> for SecretString {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// Where to place the API key
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiKeyLocation {
    /// In HTTP header
    Header {
        /// Header name (e.g., "X-API-Key")
        name: String,
    },

    /// In query parameter
    QueryParameter {
        /// Parameter name (e.g., "api_key")
        name: String,
    },

    /// In request body
    Body {
        /// Field name
        field: String,
    },
}

/// Signing algorithm for signed headers
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SigningAlgorithm {
    /// HMAC with SHA-256 (most common)
    HmacSha256,

    /// HMAC with SHA-512
    HmacSha512,

    /// RSA with SHA-256
    RsaSha256,

    /// Ed25519 signature
    Ed25519,

    /// Custom signing algorithm
    Custom {
        /// Algorithm name
        name: String,
    },
}

/// OAuth grant type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OAuthGrantType {
    /// Client credentials flow
    ClientCredentials,

    /// Authorization code flow
    AuthorizationCode,

    /// Refresh token flow
    RefreshToken,

    /// Password grant (not recommended)
    Password,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_string_no_debug_leak() {
        let secret = SecretString::new("super-secret-password".to_string());
        let debug_output = format!("{:?}", secret);

        // Should not contain the actual secret
        assert!(!debug_output.contains("super-secret-password"));
        assert!(debug_output.contains("REDACTED"));
    }

    #[test]
    fn test_authentication_descriptions() {
        let none = AuthenticationPattern::None;
        assert_eq!(none.description(), "No authentication");

        let basic = AuthenticationPattern::HttpBasic {
            username: "user".to_string(),
            password: SecretString::new("pass".to_string()),
        };
        assert_eq!(basic.description(), "HTTP Basic");
    }

    #[test]
    fn test_authentication_security() {
        let none = AuthenticationPattern::None;
        assert!(!none.is_secure());

        let bearer = AuthenticationPattern::BearerToken {
            token: SecretString::new("token".to_string()),
            token_type: "Bearer".to_string(),
        };
        assert!(bearer.is_secure());
    }
}
