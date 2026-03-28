// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Authentication configuration types.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Authentication
pub struct AuthenticationConfig {
    /// Enable external Security authentication
    pub use_external_auth: bool,
    /// Security endpoint for authentication
    pub external_auth_endpoint: Option<String>,
    /// Local token validation settings
    pub local_token_settings: LocalTokenConfig,
    /// Authentication timeout
    pub auth_timeout: Duration,
    /// Maximum authentication attempts
    pub max_auth_attempts: u32,
}
impl Default for AuthenticationConfig {
    /// Returns the default instance
    fn default() -> Self {
        // Capability-based auth discovery (no hardcoded provider):
        // 1. AUTH_CAPABILITY_ENDPOINT - capability-based (primary)
        // 2. NESTGATE_SECURITY_AUTH_ENDPOINT - generic Security primal
        // 3. BEARDOG_AUTH_ENDPOINT - deprecated, for legacy compatibility
        let auth_endpoint = std::env::var("AUTH_CAPABILITY_ENDPOINT")
            .or_else(|_| std::env::var("NESTGATE_SECURITY_AUTH_ENDPOINT"))
            .or_else(|_| std::env::var("BEARDOG_AUTH_ENDPOINT"))
            .ok();

        Self {
            use_external_auth: auth_endpoint.is_some(),
            external_auth_endpoint: auth_endpoint,
            local_token_settings: LocalTokenConfig::default(),
            auth_timeout: Duration::from_secs(30),
            max_auth_attempts: 3,
        }
    }
}

/// Local token validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for LocalToken
pub struct LocalTokenConfig {
    /// Token signing key
    pub signing_key: String,
    /// Token expiration time
    pub token_expiry: Duration,
    /// Enable token refresh
    pub enable_refresh: bool,
}
impl Default for LocalTokenConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            signing_key: std::env::var("NESTGATE_TOKEN_KEY")
                .unwrap_or_else(|_| "default-local-key".to_string()),
            token_expiry: Duration::from_secs(3600), // 1 hour
            enable_refresh: true,
        }
    }
}
