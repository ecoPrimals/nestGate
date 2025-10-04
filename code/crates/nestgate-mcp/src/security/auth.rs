/// **MCP Authentication Module**
///
/// Provides authentication management for MCP protocol connections.
/// Part of the modular security architecture.
use crate::types::auth::{AuthConfig, AuthCredentials, AuthMethod};
use nestgate_core::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
/// Authentication token for MCP sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// Token value
    pub token: String,
    /// Token expiration time
    pub expires_at: SystemTime,
    /// Associated user/client ID
    pub principal: String,
    /// Token scopes/permissions
    pub scopes: Vec<String>,
}
impl AuthToken {
    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }

    /// Check if token has required scope
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.contains(&scope.to_string())
    }
}

/// Authentication manager for MCP connections
#[derive(Debug)]
pub struct AuthManager {
    /// Active tokens
    tokens: HashMap<String, AuthToken>,
    /// Authentication configuration
    config: AuthConfig,
}
impl AuthManager {
    /// Create new authentication manager
    #[must_use]
    pub fn new(config: AuthConfig) -> Self { Self {
            tokens: HashMap::new(),
            config,
         }

    /// Authenticate a user/client
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn authenticate(&mut self, credentials: AuthCredentials) -> Result<AuthToken>  {
        match self.config.method {
            AuthMethod::Token => {
                if let Some(token) = credentials.api_key {
                    let auth_token = AuthToken {
                        token: token.clone(),
                        expires_at: SystemTime::now() + Duration::from_secs(3600),
                        principal: "mcp-client".to_string(),
                        scopes: vec!["read".to_string(), "write".to_string()],
                    };
                    self.tokens.insert(token, auth_token.clone());
                    Ok(auth_token)
                } else {
                    Err(NestGateError::mcp_error(
                        "API key required for token authentication",
                        "authenticate",
                        None,
                    ))
                }
            }
            _ => Err(NestGateError::mcp_error(
                &format!(
                    "Authentication method {:?} not implemented",
                    self.config.method
                ),
                "authenticate",
                None,
            )),
        }
    }

    /// Validate an existing token
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate_token(&self, token: &str) -> Result<&AuthToken>  {
        self.tokens
            .get(token)
            .filter(|t| !t.is_expired())
            .ok_or_else(|| {
                NestGateError::mcp_error("Invalid or expired token", "validate_token", None)
            })
    }

    /// Revoke a token
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        #[must_use]
        pub fn revoke_token(&mut self, token: &str) -> Result<()>  {
        self.tokens.remove(token);
        Ok(())
    }

    /// Clean up expired tokens
    pub fn cleanup_expired_tokens(&mut self) {
        self.tokens.retain(|_, token| !token.is_expired());
    }
}

/// Generic authenticator trait for different auth methods
pub trait Authenticator {
    /// Authenticate with the given credentials
    fn authenticate(
        &self,
        credentials: &AuthCredentials,
    ) -> impl std::future::Future<Output = Result<AuthToken>> + Send;
    /// Validate an existing token
    fn validate(&self, token: &str) -> impl std::future::Future<Output = Result<bool>> + Send;
}

/// Default token-based authenticator
pub struct TokenAuthenticator {
    /// Valid tokens
    valid_tokens: HashMap<String, AuthToken>,
}
impl TokenAuthenticator {
    #[must_use]
    pub fn new() -> Self { Self {
            valid_tokens: HashMap::new(),
         }

    pub fn add_token(&mut self, token: String, auth_token: AuthToken) {
        self.valid_tokens.insert(token, auth_token);
    }
}

impl Authenticator for TokenAuthenticator {
    fn authenticate(&self, credentials: &AuthCredentials) -> Result<AuthToken> {
        if let Some(api_key) = &credentials.api_key {
            if let Some(token) = self.valid_tokens.get(api_key) {
                if !token.is_expired() {
                    return Ok(token.clone());
                }
            }
        }

        Err(NestGateError::mcp_error(
            "Invalid credentials",
            "authenticate",
            None,
        ))
    }

    fn validate(&self, token: &str) -> Result<bool> {
        Ok(self
            .valid_tokens
            .get(token)
            .map(|t| !t.is_expired())
            .unwrap_or(false))
    }
}

impl Default for TokenAuthenticator {
    fn default() -> Self {
        Self::new()
    }
}
