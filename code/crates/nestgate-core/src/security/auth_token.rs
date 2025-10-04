/// Authentication Token Management
/// This module handles JWT tokens, API keys, and other authentication tokens
/// used in the NestGate system.
use super::auth_types::TokenType;
use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
/// Authentication token with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// The actual token string
    pub token: String,
    /// Token type
    pub token_type: TokenType,
    /// When the token expires
    pub expires_at: SystemTime,
    /// User ID associated with the token
    pub user_id: Option<String>,
    /// User roles
    pub roles: Vec<String>,
    /// User permissions
    pub permissions: Vec<String>,
    /// Token metadata
    pub metadata: HashMap<String, String>,
    /// When the token was issued
    pub issued_at: SystemTime,
}
impl AuthToken {
    /// Create a new authentication token
    #[must_use]
    pub fn new(token: String, token_type: TokenType) -> Self {
        let now = SystemTime::now();
        Self {
            token,
            token_type,
            expires_at: now + Duration::from_secs(3600), // 1 hour default
            user_id: None,
            roles: Vec::new(),
            permissions: Vec::new(),
            metadata: HashMap::new(),
            issued_at: now,
        }
    }

    /// Create a token with expiration
    #[must_use]
    pub fn with_expiration(token: String, token_type: TokenType, expires_in: Duration) -> Self {
        let now = SystemTime::now();
        Self {
            token,
            token_type,
            expires_at: now + expires_in,
            user_id: None,
            roles: Vec::new(),
            permissions: Vec::new(),
            metadata: HashMap::new(),
            issued_at: now,
        }
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }

    /// Check if the token is valid (not expired)
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }

    /// Get the remaining time until expiration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn time_until_expiration(&self) -> Result<Duration>  {
        self.expires_at
            .duration_since(SystemTime::now())
            .map_err(|_| NestGateError::internal_error(
    }

    /// Set user ID
    #[must_use]
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Add a role
    #[must_use]
    pub fn with_role(mut self, role: String) -> Self {
        self.roles.push(role);
        self
    }

    /// Add a permission
    #[must_use]
    pub fn with_permission(mut self, permission: String) -> Self {
        self.permissions.push(permission);
        self
    }

    /// Add metadata
    #[must_use]
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}
