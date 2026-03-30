// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! JWT, API key, refresh token, and access token configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Aggregate token-related settings (JWT, API keys, refresh/access tokens).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenConfig {
    /// JWT configuration
    pub jwt: JwtConfig,
    /// API key configuration
    pub api_keys: ApiKeyConfig,
    /// Refresh token configuration
    pub refresh_tokens: RefreshTokenConfig,
    /// Access token configuration
    pub access_tokens: AccessTokenConfig,
}

/// JWT signing and claims configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// JWT signing algorithm
    pub algorithm: JwtAlgorithm,
    /// JWT secret or key
    pub secret: String,
    /// JWT expiration time
    pub expiration: Duration,
    /// JWT issuer
    pub issuer: String,
    /// JWT audience
    pub audience: Vec<String>,
    /// Custom claims
    pub custom_claims: HashMap<String, serde_json::Value>,
}

/// Supported JWT signing algorithms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JwtAlgorithm {
    /// Hs256
    HS256,
    /// Hs384
    HS384,
    /// Hs512
    HS512,
    /// Rs256
    RS256,
    /// Rs384
    RS384,
    /// Rs512
    RS512,
    /// Es256
    ES256,
    /// Es384
    ES384,
    /// Es512
    ES512,
    /// Ps256
    PS256,
    /// Ps384
    PS384,
    /// Ps512
    PS512,
}

/// API key issuance and rate limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    /// Enable API key authentication
    pub enabled: bool,
    /// API key length
    pub key_length: u32,
    /// API key prefix
    pub prefix: String,
    /// API key expiration
    pub expiration: Option<Duration>,
    /// Rate limiting per API key
    pub rate_limit: Option<RateLimitConfig>,
}

/// Simple request rate limit window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per window
    pub requests: u32,
    /// Time window
    pub window: Duration,
    /// Burst allowance
    pub burst: u32,
}

/// Refresh token lifetime and rotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenConfig {
    /// Enable refresh tokens
    pub enabled: bool,
    /// Refresh token lifetime
    pub lifetime: Duration,
    /// Refresh token rotation
    pub rotation: bool,
    /// Maximum refresh token age
    pub max_age: Duration,
}

/// Access token shape and scopes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenConfig {
    /// Access token lifetime
    pub lifetime: Duration,
    /// Token type
    pub token_type: String,
    /// Scope configuration
    pub scopes: Vec<String>,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            algorithm: JwtAlgorithm::HS256,
            secret: "change-me-in-production".to_string(),
            expiration: Duration::from_secs(15 * 60), // 15 minutes
            issuer: "nestgate".to_string(),
            audience: vec!["nestgate-api".to_string()],
            custom_claims: HashMap::new(),
        }
    }
}

impl Default for ApiKeyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            key_length: 32,
            prefix: "nk_".to_string(),
            expiration: None,
            rate_limit: Some(RateLimitConfig::default()),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests: 1000,
            window: Duration::from_secs(60 * 60), // 1 hour
            burst: 100,
        }
    }
}

impl Default for RefreshTokenConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            lifetime: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
            rotation: true,
            max_age: Duration::from_secs(30 * 24 * 60 * 60), // 30 days
        }
    }
}

impl Default for AccessTokenConfig {
    fn default() -> Self {
        Self {
            lifetime: Duration::from_secs(15 * 60), // 15 minutes
            token_type: "Bearer".to_string(),
            scopes: vec!["read".to_string(), "write".to_string()],
        }
    }
}
