// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use super::{
    AuthTokenManager, AuthenticationConfig, HybridAuthenticationManager, LocalTokenConfig,
};
use crate::zero_cost_security_provider::types::{AuthMethod, ZeroCostCredentials};
use nestgate_types::Result;
use std::time::Duration;

#[tokio::test]
async fn test_hybrid_authentication_manager() -> Result<()> {
    let config = AuthenticationConfig::default();
    let auth_manager = HybridAuthenticationManager::new(config);

    // Set up argon2 password hash for test credentials
    let hash = argon2::password_hash::PasswordHasher::hash_password(
        &argon2::Argon2::default(),
        b"admin",
        argon2::password_hash::SaltString::generate(
            &mut argon2::password_hash::rand_core::OsRng,
        )
        .as_salt(),
    )
    .expect("test hash")
    .to_string();
    std::env::set_var("NESTGATE_LOCAL_AUTH_HASH", &hash);

    let credentials =
        ZeroCostCredentials::new_password("admin".to_string(), "admin".to_string());
    let token = auth_manager.authenticate(&credentials).await?;

    std::env::remove_var("NESTGATE_LOCAL_AUTH_HASH");

    assert!(!token.token.is_empty());
    assert_eq!(token.user_id, "admin");

    let is_valid = auth_manager.validate_token(&token.token).await?;
    assert!(is_valid);

    Ok(())
}

#[tokio::test]
async fn test_token_refresh() -> Result<()> {
    let config = AuthenticationConfig::default();
    let auth_manager = HybridAuthenticationManager::new(config);

    let hash = argon2::password_hash::PasswordHasher::hash_password(
        &argon2::Argon2::default(),
        b"admin",
        argon2::password_hash::SaltString::generate(
            &mut argon2::password_hash::rand_core::OsRng,
        )
        .as_salt(),
    )
    .expect("test hash")
    .to_string();
    std::env::set_var("NESTGATE_LOCAL_AUTH_HASH", &hash);

    let credentials =
        ZeroCostCredentials::new_password("admin".to_string(), "admin".to_string());
    let token = auth_manager.authenticate(&credentials).await?;
    std::env::remove_var("NESTGATE_LOCAL_AUTH_HASH");

    let refreshed_token = auth_manager.refresh_token(&token.token).await?;
    assert_ne!(token.token, refreshed_token.token);

    Ok(())
}

#[test]
fn test_auth_token_manager() {
    let manager = AuthTokenManager::new("test-key".to_string());
    let token = manager.create_token(
        "user123",
        vec!["read".to_string()],
        Duration::from_secs(3600),
    );

    assert_eq!(token.user_id, "user123");
    assert!(manager.validate_token_signature(&token.token));
}

#[tokio::test]
async fn test_authenticate_invalid_credentials() -> Result<()> {
    let config = AuthenticationConfig::default();
    let auth_manager = HybridAuthenticationManager::new(config);
    let credentials =
        ZeroCostCredentials::new_password("wrong".to_string(), "wrong".to_string());
    let result = auth_manager.authenticate(&credentials).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_validate_token_unknown_token() -> Result<()> {
    let config = AuthenticationConfig::default();
    let auth_manager = HybridAuthenticationManager::new(config);
    let is_valid = auth_manager.validate_token("unknown-token").await?;
    assert!(!is_valid);
    Ok(())
}

#[tokio::test]
async fn test_revoke_token() -> Result<()> {
    let config = AuthenticationConfig::default();
    let auth_manager = HybridAuthenticationManager::new(config);

    let hash = argon2::password_hash::PasswordHasher::hash_password(
        &argon2::Argon2::default(),
        b"admin",
        argon2::password_hash::SaltString::generate(
            &mut argon2::password_hash::rand_core::OsRng,
        )
        .as_salt(),
    )
    .expect("test hash")
    .to_string();
    std::env::set_var("NESTGATE_LOCAL_AUTH_HASH", &hash);

    let credentials =
        ZeroCostCredentials::new_password("admin".to_string(), "admin".to_string());
    let token = auth_manager.authenticate(&credentials).await?;
    std::env::remove_var("NESTGATE_LOCAL_AUTH_HASH");
    auth_manager.revoke_token(&token.token).await?;
    let is_valid = auth_manager.validate_token(&token.token).await?;
    assert!(!is_valid);
    Ok(())
}

#[tokio::test]
async fn test_rate_limit_exceeded() -> Result<()> {
    let mut config = AuthenticationConfig::default();
    config.max_auth_attempts = 1;
    let auth_manager = HybridAuthenticationManager::new(config);
    let credentials =
        ZeroCostCredentials::new_password("wrong".to_string(), "wrong".to_string());
    let _ = auth_manager.authenticate(&credentials).await;
    let result = auth_manager.authenticate(&credentials).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_refresh_disabled() -> Result<()> {
    let mut config = AuthenticationConfig::default();
    config.local_token_settings.enable_refresh = false;
    let auth_manager = HybridAuthenticationManager::new(config);
    let result = auth_manager.refresh_token("any-token").await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_auth_token_method_certificate() -> Result<()> {
    let config = AuthenticationConfig::default();
    let auth_manager = HybridAuthenticationManager::new(config);
    let credentials =
        ZeroCostCredentials::new_certificate("user".to_string(), "cert-data".to_string());
    let result = auth_manager.authenticate(&credentials).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_auth_token_method_token() -> Result<()> {
    let config = AuthenticationConfig::default();
    let auth_manager = HybridAuthenticationManager::new(config);
    let credentials =
        ZeroCostCredentials::new_token("api-user".to_string(), "api-key".to_string());
    let token = auth_manager.authenticate(&credentials).await?;
    assert!(token.token.starts_with("api_"));
    Ok(())
}

#[test]
fn test_auth_token_manager_create_workspace_secret() {
    let manager = AuthTokenManager::new("key".to_string());
    let secret = manager
        .create_workspace_secret("ws-1")
        .expect("test: workspace secret");
    // HMAC-SHA256 output is 64 hex chars
    assert_eq!(secret.len(), 64);
    assert!(secret.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_authentication_config_default() {
    let config = AuthenticationConfig::default();
    assert_eq!(config.auth_timeout, Duration::from_secs(30));
    assert_eq!(config.max_auth_attempts, 3);
}

#[test]
fn test_local_token_config_default() {
    let config = LocalTokenConfig::default();
    assert_eq!(config.token_expiry, Duration::from_secs(3600));
    assert!(config.enable_refresh);
}

#[tokio::test]
async fn test_refresh_token_local_unknown_fails() -> Result<()> {
    let config = AuthenticationConfig::default();
    let auth_manager = HybridAuthenticationManager::new(config);
    let err = auth_manager
        .refresh_token("not-in-cache")
        .await
        .expect_err("test: unknown token refresh should fail");
    assert!(err.to_string().contains("not found") || err.to_string().contains("refresh"));
    Ok(())
}

#[tokio::test]
async fn test_biometric_requires_external() -> Result<()> {
    let config = AuthenticationConfig::default();
    let auth_manager = HybridAuthenticationManager::new(config);
    let credentials = ZeroCostCredentials {
        username: "u".to_string(),
        password: String::new(),
        auth_method: AuthMethod::Biometric,
        metadata: std::collections::HashMap::new(),
    };
    let err = auth_manager
        .authenticate(&credentials)
        .await
        .expect_err("test: biometric local should fail");
    assert!(err.to_string().contains("Biometric") || err.to_string().contains("external"));
    Ok(())
}

#[tokio::test]
async fn test_multifactor_requires_external() -> Result<()> {
    let config = AuthenticationConfig::default();
    let auth_manager = HybridAuthenticationManager::new(config);
    let credentials = ZeroCostCredentials {
        username: "u".to_string(),
        password: "p".to_string(),
        auth_method: AuthMethod::MultiFactor {
            methods: vec!["totp".to_string()],
        },
        metadata: std::collections::HashMap::new(),
    };
    let err = auth_manager
        .authenticate(&credentials)
        .await
        .expect_err("test: mfa local should fail");
    assert!(err.to_string().contains("Multi-factor") || err.to_string().contains("external"));
    Ok(())
}

#[tokio::test]
async fn test_rate_limit_blocks_after_max_attempts() -> Result<()> {
    let mut config = AuthenticationConfig::default();
    config.max_auth_attempts = 2;
    let mgr = HybridAuthenticationManager::new(config);
    let w1 = ZeroCostCredentials::new_password("alice".to_string(), "x".to_string());
    let _ = mgr.authenticate(&w1).await;
    let w2 = ZeroCostCredentials::new_password("alice".to_string(), "y".to_string());
    let _ = mgr.authenticate(&w2).await;
    let w3 = ZeroCostCredentials::new_password("alice".to_string(), "z".to_string());
    let blocked = mgr.authenticate(&w3).await;
    assert!(blocked.is_err(), "test: alice should be rate limited");
    Ok(())
}
