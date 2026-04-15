// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Hybrid external + local authentication orchestration.
//!
//! Delegates cryptographic operations (JWT signing/verification, password
//! hashing) to the crypto capability provider via JSON-RPC IPC.
//! Falls back to cache-based validation when the crypto provider is unavailable.

use super::config::AuthenticationConfig;
use super::security_primal::call_security_primal;
use crate::crypto::jwt_claims::JwtClaims;
use crate::zero_cost_security_provider::types::{
    AuthMethod, ZeroCostAuthToken, ZeroCostCredentials,
};
use nestgate_discovery::primal_discovery::RuntimeDiscovery;
use nestgate_types::{NestGateError, Result};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::{debug, info, warn};

/// Hybrid authentication manager.
///
/// Routes authentication through the security primal (discovered at runtime)
/// and validates tokens via the crypto capability provider IPC.
/// Falls back to token-cache validation when external providers are unavailable.
#[derive(Debug)]
pub struct HybridAuthenticationManager {
    config: AuthenticationConfig,
    token_cache: tokio::sync::RwLock<HashMap<String, CachedToken>>,
    auth_attempts: tokio::sync::RwLock<HashMap<String, u32>>,
}

#[derive(Debug, Clone)]
struct CachedToken {
    token: ZeroCostAuthToken,
    created_at: SystemTime,
    _last_validated: SystemTime,
}

impl HybridAuthenticationManager {
    /// Create new hybrid authentication manager.
    #[must_use]
    pub fn new(config: AuthenticationConfig) -> Self {
        info!("Initializing hybrid authentication manager");
        Self {
            config,
            token_cache: tokio::sync::RwLock::new(HashMap::new()),
            auth_attempts: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Authenticate user credentials.
    ///
    /// Tries external authentication (Security primal) first, then falls back
    /// to local credential verification delegating crypto to the capability provider.
    ///
    /// # Errors
    ///
    /// Returns error if authentication fails or rate limit is exceeded.
    pub async fn authenticate(
        &self,
        credentials: &ZeroCostCredentials,
    ) -> Result<ZeroCostAuthToken> {
        debug!("Authenticating user: {}", credentials.username);

        if !self.check_rate_limit(&credentials.username).await? {
            return Err(NestGateError::security_error("Security error"));
        }

        if self.config.use_external_auth {
            match self.authenticate_external(credentials).await {
                Ok(token) => {
                    self.reset_attempts(&credentials.username).await;
                    return Ok(token);
                }
                Err(e) => {
                    warn!(
                        "External authentication failed, falling back to local: {}",
                        e
                    );
                }
            }
        }

        self.authenticate_local(credentials).await
    }

    /// Validate authentication token.
    ///
    /// Checks local cache first, then delegates JWT verification to the
    /// crypto capability provider via IPC.
    ///
    /// # Errors
    ///
    /// Returns error if validation cannot be performed.
    pub async fn validate_token(&self, token_str: &str) -> Result<bool> {
        debug!("Validating token");

        {
            let created_at = {
                let cache = self.token_cache.read().await;
                cache.get(token_str).map(|cached| cached.created_at)
            };

            if let Some(created_at) = created_at {
                let elapsed = created_at
                    .elapsed()
                    .map_err(|e| NestGateError::internal(format!("System time error: {e}")))?;

                if elapsed < self.config.local_token_settings.token_expiry {
                    debug!("Token found in cache and still valid");
                    return Ok(true);
                }
                debug!("Token in cache but expired (age: {:?})", elapsed);
            }
        }

        // Validate via crypto capability provider if configured
        if self.config.use_external_auth {
            match self.validate_token_via_crypto(token_str).await {
                Ok(valid) => return Ok(valid),
                Err(e) => {
                    warn!(
                        "Crypto provider JWT validation failed, falling back to cache: {}",
                        e
                    );
                }
            }
        }

        self.validate_token_local(token_str).await
    }

    /// Refresh authentication token.
    ///
    /// # Errors
    ///
    /// Returns error if refresh is disabled or token is not found.
    pub async fn refresh_token(&self, token_str: &str) -> Result<ZeroCostAuthToken> {
        debug!("Refreshing token");

        if !self.config.local_token_settings.enable_refresh {
            return Err(NestGateError::security_error("Security error"));
        }

        if self.config.use_external_auth {
            match self.refresh_token_via_crypto(token_str).await {
                Ok(token) => return Ok(token),
                Err(e) => {
                    warn!(
                        "Crypto provider token refresh failed, falling back to cache: {}",
                        e
                    );
                }
            }
        }

        self.refresh_token_local(token_str).await
    }

    /// Revoke authentication token.
    ///
    /// # Errors
    ///
    /// Returns error if revocation fails.
    pub async fn revoke_token(&self, token_str: &str) -> Result<()> {
        debug!("Revoking token");

        {
            let mut cache = self.token_cache.write().await;
            cache.remove(token_str);
        }

        Ok(())
    }

    async fn check_rate_limit(&self, username: &str) -> Result<bool> {
        let (at_limit, current_attempts) = {
            let mut attempts = self.auth_attempts.write().await;
            let current_attempts = *attempts.get(username).unwrap_or(&0);
            if current_attempts >= self.config.max_auth_attempts {
                drop(attempts);
                (true, current_attempts)
            } else {
                attempts.insert(username.to_string(), current_attempts + 1);
                drop(attempts);
                (false, current_attempts)
            }
        };

        if at_limit {
            info!(
                "Rate limit exceeded for user: {} ({}/{} attempts)",
                username, current_attempts, self.config.max_auth_attempts
            );
            Ok(false)
        } else {
            Ok(true)
        }
    }

    async fn reset_attempts(&self, username: &str) {
        let mut attempts = self.auth_attempts.write().await;
        attempts.remove(username);
    }

    /// External authentication via Security primal discovered at runtime.
    async fn authenticate_external(
        &self,
        credentials: &ZeroCostCredentials,
    ) -> Result<ZeroCostAuthToken> {
        debug!("Attempting external authentication via capability discovery");

        let discovery_client = RuntimeDiscovery::new()?;

        match discovery_client.find_security_primal().await {
            Ok(connection) => {
                info!(
                    "Discovered Security primal at: {} (dynamic discovery)",
                    connection.endpoint
                );

                let token = call_security_primal(
                    &connection,
                    credentials,
                    self.config.local_token_settings.token_expiry,
                )
                .await?;

                {
                    let mut cache = self.token_cache.write().await;
                    cache.insert(
                        token.token.clone(),
                        CachedToken {
                            token: token.clone(),
                            created_at: SystemTime::now(),
                            _last_validated: SystemTime::now(),
                        },
                    );
                }

                Ok(token)
            }
            Err(e) => {
                warn!(
                    "Security primal discovery failed ({}), falling back to local auth",
                    e
                );
                self.authenticate_local(credentials).await
            }
        }
    }

    /// Local authentication — delegates password verification to crypto provider.
    async fn authenticate_local(
        &self,
        credentials: &ZeroCostCredentials,
    ) -> Result<ZeroCostAuthToken> {
        debug!("Performing local authentication");

        match credentials.auth_method {
            AuthMethod::Password => {
                let expected_hash = std::env::var("NESTGATE_LOCAL_AUTH_HASH").ok();
                if let Some(hash) = expected_hash {
                    // Delegate password verification to crypto capability provider
                    match crate::crypto::delegate::CryptoDelegate::new().await {
                        Ok(delegate) => {
                            let valid = delegate
                                .verify_password(&credentials.password, &hash)
                                .await
                                .map_err(|e| {
                                    NestGateError::security_error(format!(
                                        "Crypto provider password verification failed: {e}"
                                    ))
                                })?;

                            if !valid {
                                return Err(NestGateError::security_error("Invalid credentials"));
                            }
                        }
                        Err(e) => {
                            warn!("Crypto provider unavailable for password verification: {e}");
                            return Err(NestGateError::security_error(
                                "Password verification requires crypto capability provider",
                            ));
                        }
                    }

                    let token = ZeroCostAuthToken::new(
                        format!("local_{}", uuid::Uuid::new_v4()),
                        credentials.username.clone(),
                        vec!["authenticated".to_string()],
                        self.config.local_token_settings.token_expiry,
                    );

                    {
                        let mut cache = self.token_cache.write().await;
                        cache.insert(
                            token.token.clone(),
                            CachedToken {
                                token: token.clone(),
                                created_at: SystemTime::now(),
                                _last_validated: SystemTime::now(),
                            },
                        );
                    }

                    Ok(token)
                } else {
                    Err(NestGateError::security_error(
                        "Local password auth requires NESTGATE_LOCAL_AUTH_HASH — \
                         use security primal for production authentication",
                    ))
                }
            }
            AuthMethod::Token => {
                if credentials.password.is_empty() {
                    return Err(NestGateError::security_error("API token required"));
                }
                let token = ZeroCostAuthToken::new(
                    format!("api_{}", uuid::Uuid::new_v4()),
                    credentials.username.clone(),
                    vec!["api".to_string()],
                    self.config.local_token_settings.token_expiry,
                );

                {
                    let mut cache = self.token_cache.write().await;
                    cache.insert(
                        token.token.clone(),
                        CachedToken {
                            token: token.clone(),
                            created_at: SystemTime::now(),
                            _last_validated: SystemTime::now(),
                        },
                    );
                }

                Ok(token)
            }
            AuthMethod::Certificate => Err(NestGateError::security_error(
                "Certificate auth requires external security provider",
            )),
            AuthMethod::Biometric => Err(NestGateError::security_error(
                "Biometric auth requires external security provider",
            )),
            AuthMethod::MultiFactor { .. } => Err(NestGateError::security_error(
                "Multi-factor auth requires external security provider",
            )),
        }
    }

    /// Validate JWT via crypto capability provider IPC.
    async fn validate_token_via_crypto(&self, token_str: &str) -> Result<bool> {
        let delegate = crate::crypto::delegate::CryptoDelegate::new().await?;

        match delegate.verify_jwt(token_str, "HS256").await {
            Ok(claims_json) => {
                let claims: JwtClaims = serde_json::from_str(&claims_json).map_err(|e| {
                    NestGateError::validation_error(format!("JWT claims parse error: {e}"))
                })?;
                if claims.is_expired() {
                    return Ok(false);
                }
                debug!("JWT validated via crypto provider for user: {}", claims.sub);
                Ok(true)
            }
            Err(e) => {
                warn!("JWT verification via crypto provider failed: {}", e);
                Ok(false)
            }
        }
    }

    async fn validate_token_local(&self, token_str: &str) -> Result<bool> {
        let valid = {
            let cache = self.token_cache.read().await;
            cache.get(token_str).is_some_and(|cached| {
                let elapsed = cached.created_at.elapsed().unwrap_or(Duration::MAX);
                elapsed < self.config.local_token_settings.token_expiry
            })
        };
        Ok(valid)
    }

    /// Refresh JWT via crypto capability provider IPC.
    async fn refresh_token_via_crypto(&self, token_str: &str) -> Result<ZeroCostAuthToken> {
        let delegate = crate::crypto::delegate::CryptoDelegate::new().await?;

        let claims_json = delegate
            .verify_jwt(token_str, "HS256")
            .await
            .map_err(|_| NestGateError::security_error("Cannot refresh invalid token"))?;

        let old_claims: JwtClaims = serde_json::from_str(&claims_json)
            .map_err(|e| NestGateError::validation_error(format!("JWT claims parse error: {e}")))?;

        let new_expiry_seconds =
            i64::try_from(self.config.local_token_settings.token_expiry.as_secs())
                .unwrap_or(i64::MAX);
        let new_claims = JwtClaims {
            sub: old_claims.sub.clone(),
            iat: i64::try_from(
                SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            )
            .unwrap_or(i64::MAX),
            exp: i64::try_from(
                SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            )
            .unwrap_or(i64::MAX)
            .saturating_add(new_expiry_seconds),
            iss: old_claims.iss.clone(),
            aud: old_claims.aud.clone(),
            permissions: old_claims.permissions.clone(),
        };

        let new_claims_json = serde_json::to_string(&new_claims).map_err(|e| {
            NestGateError::validation_error(format!("Claims serialization error: {e}"))
        })?;

        let new_token_str = delegate.sign_jwt(&new_claims_json, "HS256").await?;

        debug!(
            "JWT refreshed via crypto provider for user: {}",
            old_claims.sub
        );

        Ok(ZeroCostAuthToken::new(
            new_token_str,
            old_claims.sub,
            old_claims.permissions.unwrap_or_default(),
            self.config.local_token_settings.token_expiry,
        ))
    }

    async fn refresh_token_local(&self, token_str: &str) -> Result<ZeroCostAuthToken> {
        let cached = {
            let cache = self.token_cache.read().await;
            cache
                .get(token_str)
                .map(|c| (c.token.user_id.clone(), c.token.permissions.clone()))
        };
        cached.map_or_else(
            || Err(NestGateError::security_error("Token not found for refresh")),
            |(user_id, permissions)| {
                Ok(ZeroCostAuthToken::new(
                    format!("refresh_{}", uuid::Uuid::new_v4()),
                    user_id,
                    permissions,
                    self.config.local_token_settings.token_expiry,
                ))
            },
        )
    }
}

#[cfg(test)]
mod hybrid_manager_direct_tests {
    use std::time::Duration;

    use super::HybridAuthenticationManager;
    use crate::zero_cost_security_provider::types::{AuthMethod, ZeroCostCredentials};

    #[tokio::test]
    async fn token_auth_with_empty_secret_fails() {
        let mut config = super::super::config::AuthenticationConfig::default();
        config.use_external_auth = false;
        let mgr = HybridAuthenticationManager::new(config);
        let creds = ZeroCostCredentials {
            username: "api".into(),
            password: String::new(),
            auth_method: AuthMethod::Token,
            metadata: std::collections::HashMap::new(),
        };
        let err = mgr.authenticate(&creds).await.expect_err("empty api token");
        assert!(
            err.to_string().contains("token") || err.to_string().contains("required"),
            "{err}"
        );
    }

    #[tokio::test]
    async fn validate_token_expired_cache_entry_falls_through() {
        let mut config = super::super::config::AuthenticationConfig::default();
        config.use_external_auth = false;
        config.local_token_settings.token_expiry = Duration::from_nanos(1);
        let mgr = HybridAuthenticationManager::new(config);
        let creds = ZeroCostCredentials::new_token("slow".into(), "k".into());
        let tok = mgr.authenticate(&creds).await.expect("auth");
        tokio::time::sleep(Duration::from_millis(30)).await;
        let ok = mgr.validate_token(&tok.token).await.expect("validate");
        assert!(!ok);
    }

    #[tokio::test]
    async fn external_auth_disabled_skips_discovery() {
        let mut config = super::super::config::AuthenticationConfig::default();
        config.use_external_auth = false;
        let mgr = HybridAuthenticationManager::new(config);
        let creds = ZeroCostCredentials::new_token("u".into(), "secret".into());
        let t = mgr.authenticate(&creds).await.expect("local token ok");
        assert!(t.token.starts_with("api_"));
    }
}
