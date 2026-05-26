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
    pub fn default_from_env_source(env: &(impl EnvSource + ?Sized)) -> Self {
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
    pub fn default_from_env_source(env: &(impl EnvSource + ?Sized)) -> Self {
        Self {
            signing_key: env_var_or_default(env, "NESTGATE_TOKEN_KEY", "default-local-key"),
            token_expiry: Duration::from_secs(3600), // 1 hour
            enable_refresh: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;

    fn empty_env() -> MapEnv {
        MapEnv::new()
    }

    fn env_with(pairs: &[(&str, &str)]) -> MapEnv {
        let map = pairs.iter().map(|(k, v)| ((*k).to_string(), (*v).to_string())).collect();
        MapEnv(map)
    }

    #[test]
    fn auth_config_defaults_to_local_when_no_env() {
        let cfg = AuthenticationConfig::default_from_env_source(&empty_env());
        assert!(!cfg.use_external_auth);
        assert!(cfg.external_auth_endpoint.is_none());
        assert_eq!(cfg.max_auth_attempts, 3);
        assert_eq!(cfg.auth_timeout, Duration::from_secs(30));
    }

    #[test]
    fn auth_config_uses_capability_endpoint_first() {
        let env = env_with(&[
            ("AUTH_CAPABILITY_ENDPOINT", "http://cap:7777"),
            ("NESTGATE_SECURITY_AUTH_ENDPOINT", "http://sec:8888"),
            ("AUTH_PROVIDER_ENDPOINT", "http://legacy:9999"),
        ]);
        let cfg = AuthenticationConfig::default_from_env_source(&env);
        assert!(cfg.use_external_auth);
        assert_eq!(cfg.external_auth_endpoint.as_deref(), Some("http://cap:7777"));
    }

    #[test]
    fn auth_config_falls_back_to_security_endpoint() {
        let env = env_with(&[("NESTGATE_SECURITY_AUTH_ENDPOINT", "http://sec:8888")]);
        let cfg = AuthenticationConfig::default_from_env_source(&env);
        assert!(cfg.use_external_auth);
        assert_eq!(cfg.external_auth_endpoint.as_deref(), Some("http://sec:8888"));
    }

    #[test]
    fn auth_config_falls_back_to_legacy_endpoint() {
        let env = env_with(&[("AUTH_PROVIDER_ENDPOINT", "http://legacy:9999")]);
        let cfg = AuthenticationConfig::default_from_env_source(&env);
        assert!(cfg.use_external_auth);
        assert_eq!(cfg.external_auth_endpoint.as_deref(), Some("http://legacy:9999"));
    }

    #[test]
    fn local_token_config_uses_default_key() {
        let cfg = LocalTokenConfig::default_from_env_source(&empty_env());
        assert_eq!(cfg.signing_key, "default-local-key");
        assert_eq!(cfg.token_expiry, Duration::from_secs(3600));
        assert!(cfg.enable_refresh);
    }

    #[test]
    fn local_token_config_reads_custom_key() {
        let env = env_with(&[("NESTGATE_TOKEN_KEY", "my-secret-key")]);
        let cfg = LocalTokenConfig::default_from_env_source(&env);
        assert_eq!(cfg.signing_key, "my-secret-key");
    }

    #[test]
    fn auth_config_serializes_roundtrip() {
        let cfg = AuthenticationConfig::default_from_env_source(&empty_env());
        let json = serde_json::to_string(&cfg).unwrap();
        let cfg2: AuthenticationConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(cfg.use_external_auth, cfg2.use_external_auth);
        assert_eq!(cfg.max_auth_attempts, cfg2.max_auth_attempts);
    }
}
