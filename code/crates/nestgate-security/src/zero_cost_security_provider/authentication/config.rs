// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Authentication configuration types.

use nestgate_types::{EnvSource, ProcessEnv, env_var_or_default};
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
        Self::default_from_env_source(&ProcessEnv)
    }
}

impl AuthenticationConfig {
    /// Like [`Default::default`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn default_from_env_source(env: &dyn EnvSource) -> Self {
        // Capability-based auth discovery (no hardcoded provider):
        // 1. AUTH_CAPABILITY_ENDPOINT - capability-based (primary)
        // 2. NESTGATE_SECURITY_AUTH_ENDPOINT - generic security/auth endpoint
        // 3. AUTH_PROVIDER_ENDPOINT - legacy alias for external auth URL
        let auth_endpoint = env
            .get("AUTH_CAPABILITY_ENDPOINT")
            .or_else(|| env.get("NESTGATE_SECURITY_AUTH_ENDPOINT"))
            .or_else(|| env.get("AUTH_PROVIDER_ENDPOINT"));

        Self {
            use_external_auth: auth_endpoint.is_some(),
            external_auth_endpoint: auth_endpoint,
            local_token_settings: LocalTokenConfig::default_from_env_source(env),
            auth_timeout: Duration::from_secs(30),
            max_auth_attempts: 3,
        }
    }
}

/// Local token validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `LocalToken`
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
        Self::default_from_env_source(&ProcessEnv)
    }
}

impl LocalTokenConfig {
    /// Like [`Default::default`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn default_from_env_source(env: &dyn EnvSource) -> Self {
        Self {
            signing_key: env_var_or_default(env, "NESTGATE_TOKEN_KEY", "default-local-key"),
            token_expiry: Duration::from_secs(3600), // 1 hour
            enable_refresh: true,
        }
    }
}
