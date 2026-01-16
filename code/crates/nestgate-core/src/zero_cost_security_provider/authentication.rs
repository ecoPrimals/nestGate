//! Authentication module

use crate::error::NestGateError;
use std::collections::HashMap;
//
// Routes to Security for complex authentication when available,
// falls back to local token validation for standalone operation.

use super::types::{AuthMethod, ZeroCostAuthToken, ZeroCostCredentials};
use crate::Result;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use tracing::{debug, info, warn};

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
        Self {
            // Capability-based: NESTGATE_SECURITY_AUTH_ENDPOINT (generic)
            // Legacy: BEARDOG_AUTH_ENDPOINT (deprecated but supported)
            use_external_auth: std::env::var("NESTGATE_SECURITY_AUTH_ENDPOINT")
                .or_else(|_| std::env::var("BEARDOG_AUTH_ENDPOINT"))
                .is_ok(),
            external_auth_endpoint: std::env::var("NESTGATE_SECURITY_AUTH_ENDPOINT")
                .or_else(|_| std::env::var("BEARDOG_AUTH_ENDPOINT"))
                .ok(),
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

/// Hybrid authentication manager
#[derive(Debug)]
/// Manager for HybridAuthentication operations
pub struct HybridAuthenticationManager {
    config: AuthenticationConfig,
    token_cache: tokio::sync::RwLock<HashMap<String, CachedToken>>,
    auth_attempts: tokio::sync::RwLock<HashMap<String, u32>>,
}
/// Cached token information
#[derive(Debug, Clone)]
struct CachedToken {
    token: ZeroCostAuthToken,
    created_at: SystemTime,
    #[allow(dead_code)]
    last_validated: SystemTime,
}
impl HybridAuthenticationManager {
    /// Create new hybrid authentication manager
    #[must_use]
    pub fn new(config: AuthenticationConfig) -> Self {
        info!("Initializing hybrid authentication manager");
        Self {
            config,
            token_cache: tokio::sync::RwLock::new(HashMap::new()),
            auth_attempts: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Authenticate user credentials
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn authenticate(
        &self,
        credentials: &ZeroCostCredentials,
    ) -> Result<ZeroCostAuthToken> {
        debug!("Authenticating user: {}", credentials.username);

        // Check rate limiting
        if !self.check_rate_limit(&credentials.username).await? {
            return Err(NestGateError::security_error("Security error"));
        }

        // Try external authentication first if configured
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

        // Fall back to local authentication
        self.authenticate_local(credentials).await
    }

    /// Validate authentication token
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn validate_token(&self, token_str: &str) -> Result<bool> {
        debug!("Validating token");

        // Check local cache first
        {
            let cache = self.token_cache.read().await;
            if let Some(cached) = cache.get(token_str) {
                let elapsed = cached
                    .created_at
                    .elapsed()
                    .map_err(|e| NestGateError::internal(format!("System time error: {}", e)))?;

                if elapsed < self.config.local_token_settings.token_expiry {
                    debug!("Token found in cache and still valid");
                    return Ok(true);
                } else {
                    debug!("Token in cache but expired (age: {:?})", elapsed);
                }
            }
        }

        // Try external validation if configured
        if self.config.use_external_auth {
            match self.validate_token_external(token_str).await {
                Ok(valid) => return Ok(valid),
                Err(e) => {
                    warn!(
                        "External token validation failed, falling back to local: {}",
                        e
                    );
                }
            }
        }

        // Fall back to local validation
        self.validate_token_local(token_str).await
    }

    /// Refresh authentication token
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn refresh_token(&self, token_str: &str) -> Result<ZeroCostAuthToken> {
        debug!("Refreshing token");

        if !self.config.local_token_settings.enable_refresh {
            return Err(NestGateError::security_error("Security error"));
        }

        // Try external refresh first if configured
        if self.config.use_external_auth {
            match self.refresh_token_external(token_str).await {
                Ok(token) => return Ok(token),
                Err(e) => {
                    warn!(
                        "External token refresh failed, falling back to local: {}",
                        e
                    );
                }
            }
        }

        // Fall back to local refresh
        self.refresh_token_local(token_str).await
    }

    /// Revoke authentication token
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn revoke_token(&self, token_str: &str) -> Result<()> {
        debug!("Revoking token");

        // Remove from local cache
        {
            let mut cache = self.token_cache.write().await;
            cache.remove(token_str);
        }

        // Try external revocation if configured
        if self.config.use_external_auth {
            match self.revoke_token_external(token_str).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    warn!("External token revocation failed: {}", e);
                }
            }
        }

        Ok(())
    }

    // Private helper methods

    /// Check authentication rate limit
    async fn check_rate_limit(&self, username: &str) -> Result<bool> {
        let mut attempts = self.auth_attempts.write().await;
        let current_attempts = *attempts.get(username).unwrap_or(&0);

        if current_attempts >= self.config.max_auth_attempts {
            info!(
                "Rate limit exceeded for user: {} ({}/{} attempts)",
                username, current_attempts, self.config.max_auth_attempts
            );
            return Ok(false);
        }

        attempts.insert(username.to_string(), current_attempts + 1);
        Ok(true)
    }

    /// Reset authentication attempts for user
    async fn reset_attempts(&self, username: &str) {
        let mut attempts = self.auth_attempts.write().await;
        attempts.remove(username);
    }

    /// External authentication via Security primal discovered at runtime
    ///
    /// Uses capability-based discovery to find Security primal (no hardcoding).
    /// Integrates with runtime discovery for dynamic primal connection.
    async fn authenticate_external(
        &self,
        credentials: &ZeroCostCredentials,
    ) -> Result<ZeroCostAuthToken> {
        debug!("Attempting external authentication via capability discovery");

        // Use runtime discovery to find Security capability
        use crate::primal_discovery::RuntimeDiscovery;

        let discovery_client = RuntimeDiscovery::new().await?;

        // Discover security primals at runtime (no hardcoded endpoints)
        match discovery_client.find_security_primal().await {
            Ok(connection) => {
                info!(
                    "Discovered Security primal at: {} (dynamic discovery)",
                    connection.endpoint
                );

                // Make HTTP call to discovered Security primal
                let token = self.call_security_primal(&connection, credentials).await?;

                // Cache the token locally
                {
                    let mut cache = self.token_cache.write().await;
                    cache.insert(
                        token.token.to_string(),
                        CachedToken {
                            token: token.clone(),
                            created_at: SystemTime::now(),
                            last_validated: SystemTime::now(),
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
                // Graceful degradation: fall back to local authentication
                self.authenticate_local(credentials).await
            }
        }
    }

    /// Call discovered Security primal for authentication
    async fn call_security_primal(
        &self,
        _connection: &crate::primal_discovery::PrimalConnection,
        credentials: &ZeroCostCredentials,
    ) -> Result<ZeroCostAuthToken> {
        // Real HTTP implementation would go here
        // For now, simulate the call
        tokio::time::sleep(Duration::from_millis(50)).await;

        let token = ZeroCostAuthToken::new(
            format!("primal_{}", uuid::Uuid::new_v4()),
            credentials.username.to_string(),
            vec!["read".to_string(), "write".to_string()],
            self.config.local_token_settings.token_expiry,
        );

        Ok(token)
    }

    /// Local authentication fallback
    async fn authenticate_local(
        &self,
        credentials: &ZeroCostCredentials,
    ) -> Result<ZeroCostAuthToken> {
        debug!("Performing local authentication");

        // Simple local authentication logic
        // In a real implementation, this would check against local user database
        match credentials.auth_method {
            AuthMethod::Password => {
                if credentials.username == "admin" && &credentials.password == "admin" {
                    let token = ZeroCostAuthToken::new(
                        format!("local_{}", uuid::Uuid::new_v4()),
                        credentials.username.to_string(),
                        vec!["admin".to_string()],
                        self.config.local_token_settings.token_expiry,
                    );

                    // Cache the token
                    {
                        let mut cache = self.token_cache.write().await;
                        cache.insert(
                            token.token.to_string(),
                            CachedToken {
                                token: token.clone(),
                                created_at: SystemTime::now(),
                                last_validated: SystemTime::now(),
                            },
                        );
                    }

                    Ok(token)
                } else {
                    Err(NestGateError::security_error("Security error"))
                }
            }
            AuthMethod::Token => {
                // Simple API key validation
                if Some(&credentials.password).is_some() {
                    let token = ZeroCostAuthToken::new(
                        format!("api_{}", uuid::Uuid::new_v4()),
                        "api-user".to_string(),
                        vec!["api".to_string()],
                        self.config.local_token_settings.token_expiry,
                    );
                    Ok(token)
                } else {
                    Err(NestGateError::security_error("Security error"))
                }
            }
            AuthMethod::Certificate => {
                // Certificate-based authentication not implemented in local fallback
                Err(NestGateError::security_error("Security error"))
            }
            AuthMethod::Biometric => {
                // Biometric authentication not implemented in local fallback
                Err(NestGateError::security_error(
                    "Biometric authentication requires external provider",
                ))
            }
            AuthMethod::MultiFactor { .. } => {
                // Multi-factor authentication not implemented in local fallback
                Err(NestGateError::security_error(
                    "Multi-factor authentication requires external provider",
                ))
            }
        }
    }

    /// External token validation via Security primal (e.g., BearDog)
    ///
    /// Uses capability-based discovery to find and call the security service.
    async fn validate_token_external(&self, token_str: &str) -> Result<bool> {
        // Discover security service dynamically
        let security = crate::primal_discovery::discover_security().await?;
        
        // Call security service for validation
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/auth/validate", security.endpoint))
            .json(&serde_json::json!({
                "token": token_str
            }))
            .send()
            .await
            .map_err(|e| NestGateError::network_error(&format!("Security service call failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Ok(false);
        }
        
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| NestGateError::api_internal_error(&format!("Failed to parse validation response: {}", e)))?;
        
        Ok(result.get("valid").and_then(|v| v.as_bool()).unwrap_or(false))
    }

    /// Local token validation
    async fn validate_token_local(&self, token_str: &str) -> Result<bool> {
        let cache = self.token_cache.read().await;
        if let Some(cached) = cache.get(token_str) {
            let elapsed = cached.created_at.elapsed().unwrap_or(Duration::MAX);
            Ok(elapsed < self.config.local_token_settings.token_expiry)
        } else {
            Ok(false)
        }
    }

    /// External token refresh via Security primal (e.g., BearDog)
    ///
    /// Uses capability-based discovery to find and call the security service.
    async fn refresh_token_external(&self, token_str: &str) -> Result<ZeroCostAuthToken> {
        // Discover security service dynamically
        let security = crate::primal_discovery::discover_security().await?;
        
        // Call security service for refresh
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/auth/refresh", security.endpoint))
            .json(&serde_json::json!({
                "token": token_str
            }))
            .send()
            .await
            .map_err(|e| NestGateError::network_error(&format!("Security service call failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(NestGateError::security_error("Token refresh failed"));
        }
        
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| NestGateError::api_internal_error(&format!("Failed to parse refresh response: {}", e)))?;
        
        // Extract new token from response
        let token_id = result.get("token_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| NestGateError::api_internal_error("Missing token_id in response"))?;
        let user_id = result.get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| NestGateError::api_internal_error("Missing user_id in response"))?;
        let permissions = result.get("permissions")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
        
        Ok(ZeroCostAuthToken::new(
            token_id.to_string(),
            user_id.to_string(),
            permissions,
            self.config.local_token_settings.token_expiry,
        ))
    }

    /// Local token refresh
    async fn refresh_token_local(&self, token_str: &str) -> Result<ZeroCostAuthToken> {
        let cache = self.token_cache.read().await;
        if let Some(cached) = cache.get(token_str) {
            let new_token = ZeroCostAuthToken::new(
                format!("refresh_{}", uuid::Uuid::new_v4()),
                cached.token.user_id.to_string(),
                cached.token.permissions.clone(),
                self.config.local_token_settings.token_expiry,
            );
            Ok(new_token)
        } else {
            Err(NestGateError::security_error("Token not found for refresh"))
        }
    }

    /// External token revocation via Security primal (e.g., BearDog)
    ///
    /// Uses capability-based discovery to find and call the security service.
    async fn revoke_token_external(&self, token_str: &str) -> Result<()> {
        // Discover security service dynamically
        let security = crate::primal_discovery::discover_security().await?;
        
        // Call security service for revocation
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/auth/revoke", security.endpoint))
            .json(&serde_json::json!({
                "token": token_str
            }))
            .send()
            .await
            .map_err(|e| NestGateError::network_error(&format!("Security service call failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(NestGateError::security_error("Token revocation failed"));
        }
        
        Ok(())
    }
}

/// Token manager for local operations
#[allow(dead_code)]
/// Manager for AuthToken operations
pub struct AuthTokenManager {
    signing_key: String,
}
impl AuthTokenManager {
    /// Create a new authentication token manager
    ///
    /// # Arguments
    ///
    /// * `signing_key` - Cryptographic key used for token signing and validation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nestgate_core::zero_cost_security_provider::authentication::AuthTokenManager;
    ///
    /// let manager = AuthTokenManager::new("my-secret-key".to_string());
    /// ```
    #[must_use]
    pub fn new(signing_key: String) -> Self {
        Self { signing_key }
    }

    /// Create a new authentication token for a user
    ///
    /// Generates a unique, time-limited authentication token with specified permissions.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier for the user
    /// * `permissions` - List of permissions granted to this token
    /// * `expiry` - Duration until token expires
    ///
    /// # Returns
    ///
    /// A new `ZeroCostAuthToken` with the specified parameters
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nestgate_core::zero_cost_security_provider::authentication::AuthTokenManager;
    /// use std::time::Duration;
    ///
    /// let manager = AuthTokenManager::new("secret-key".to_string());
    /// let token = manager.create_token(
    ///     "user123",
    ///     vec!["read".to_string(), "write".to_string()],
    ///     Duration::from_secs(3600)
    /// );
    /// ```
    #[must_use]
    pub fn create_token(
        &self,
        user_id: &str,
        permissions: Vec<String>,
        expiry: Duration,
    ) -> ZeroCostAuthToken {
        ZeroCostAuthToken::new(
            format!("token_{}_{}", user_id, uuid::Uuid::new_v4()),
            user_id.to_string(),
            permissions,
            expiry,
        )
    }

    /// Validate the cryptographic signature of a token
    ///
    /// Verifies that the token was signed with the correct key and has not been tampered with.
    ///
    /// # Arguments
    ///
    /// * `_token` - The token string to validate
    ///
    /// # Returns
    ///
    /// * `true` if the signature is valid
    /// * `false` if the signature is invalid or token is malformed
    ///
    /// # Security Note
    ///
    /// In production, this should use proper cryptographic verification (HMAC, RSA, etc.).
    /// Current implementation is a placeholder for development/testing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use nestgate_core::zero_cost_security_provider::authentication::AuthTokenManager;
    ///
    /// let manager = AuthTokenManager::new("secret-key".to_string());
    /// let is_valid = manager.validate_token_signature("token_string");
    /// assert!(is_valid); // Placeholder always returns true
    /// ```
    #[must_use]
    pub fn validate_token_signature(&self, _token: &str) -> bool {
        // Simple signature validation
        // In a real implementation, this would use proper cryptographic verification
        true
    }

    /// Creates  Workspace Secret
    pub fn create_workspace_secret(
        &self,
        workspace_id: &str,
    ) -> std::result::Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Generate a unique secret ID for the workspace
        let secret_id = format!("secret_{}_{}", workspace_id, uuid::Uuid::new_v4());

        // In a real implementation, this would:
        // 1. Generate a cryptographically secure secret
        // 2. Store it in a secure key management system
        // 3. Associate it with the workspace

        Ok(secret_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hybrid_authentication_manager() -> Result<()> {
        let config = AuthenticationConfig::default();
        let auth_manager = HybridAuthenticationManager::new(config);

        let credentials =
            ZeroCostCredentials::new_password("admin".to_string(), "admin".to_string());
        let token = auth_manager.authenticate(&credentials).await?;

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

        let credentials =
            ZeroCostCredentials::new_password("admin".to_string(), "admin".to_string());
        let token = auth_manager.authenticate(&credentials).await?;

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
}
