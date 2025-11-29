///
/// Provides token and session management for MCP protocol connections.
/// Part of the modular security architecture.
use crate::security::auth::AuthToken;
use nestgate_core::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
/// Session information for MCP connections
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Session
pub struct Session {
    /// Session ID
    pub session_id: String,
    /// Associated user ID
    pub user_id: String,
    /// Session creation time
    pub created_at: SystemTime,
    /// Last activity time
    pub last_activity: SystemTime,
    /// Session expiration time
    pub expires_at: SystemTime,
    /// Session metadata
    pub metadata: HashMap<String, String>,
    /// Active token for this session
    pub token: Option<AuthToken>,
}
impl Session {
    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }

    /// Check if session is idle (based on last activity)
    pub fn is_idle(&self, idle_timeout: Duration) -> bool {
        SystemTime::now() > self.last_activity + idle_timeout
    }

    /// Update last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = SystemTime::now();
    }
}

/// Token manager for MCP security
#[derive(Debug)]
/// Manager for Token operations
pub struct TokenManager {
    /// Active tokens
    tokens: HashMap<String, AuthToken>,
    /// Token expiration times
    token_expiry: HashMap<String, SystemTime>,
    /// Token refresh capabilities
    refresh_tokens: HashMap<String, String>,
}
impl TokenManager {
    /// Create new token manager
    #[must_use]
    pub fn new() -> Self { Self {
            tokens: HashMap::new(),
            token_expiry: HashMap::new(),
            refresh_tokens: HashMap::new(),
         }

    /// Generate a new token
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn generate_token(
        &mut self,
        user_id: &str,
        scopes: Vec<String>,
        lifetime: Duration,
    ) -> Result<AuthToken>  {
        let tokenvalue = format!(
            "mcp_{}_{}",
            user_id,
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|_e| {
                    tracing::error!("Unwrap failed: {:?}", e);
                    // Return a safe default duration (current timestamp approximation)
                    Duration::from_secs(1700000000) // Approximate timestamp around 2023
                })
                .as_secs()
        );
        let expires_at = SystemTime::now() + lifetime;

        let token = AuthToken {
            token: tokenvalue.clone(),
            expires_at,
            principal: user_id.to_string(),
            scopes,
        };

        self.tokens.insert(tokenvalue.clone(), token.clone());
        self.token_expiry.insert(tokenvalue, expires_at);

        Ok(token)
    }

    /// Validate a token
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate_token(&self, tokenvalue: &str) -> Result<&AuthToken>  {
        let token = self
            .tokens
            .get(tokenvalue)
            .ok_or_else(|| NestGateError::mcp_error("Token not found", "validate_token", None))?;

        if token.is_expired() {
            return Err(NestGateError::mcp_error(
                "Token expired",
                "validate_token",
                None,
            ));
        }

        Ok(token)
    }

    /// Refresh a token
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn refresh_token(&mut self, tokenvalue: &str, lifetime: Duration) -> Result<AuthToken>  {
        let old_token = self
            .tokens
            .get(tokenvalue)
            .ok_or_else(|| NestGateError::mcp_error("Token not found", "refresh_token", None))?
            .clone();

        // Generate new token with same scopes
        let new_token = self.generate_token(&old_token.principal, old_token.scopes, lifetime)?;

        // Remove old token
        self.revoke_token(tokenvalue)?;

        Ok(new_token)
    }

    /// Revoke a token
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn revoke_token(&mut self, tokenvalue: &str) -> Result<()>  {
        self.tokens.remove(tokenvalue);
        self.token_expiry.remove(tokenvalue);
        self.refresh_tokens.remove(tokenvalue);
        Ok(())
    }

    /// Clean up expired tokens
    pub fn cleanup_expired_tokens(&mut self) {
        let now = SystemTime::now();
        let expired_tokens: Vec<String> = self
            .token_expiry
            .iter()
            .filter(|(_, &expiry)| now > expiry)
            .map(|(token, _)| token.clone())
            .collect();

        for token in expired_tokens {
            self.revoke_token(&token).ok();
        }
    }

    /// List active tokens for a user
    pub fn list_user_tokens(&self, user_id: &str) -> Vec<&AuthToken> {
        self.tokens
            .values()
            .filter(|token| token.principal == user_id && !token.is_expired())
            .collect()
    }
}

impl Default for TokenManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Session manager for MCP connections
#[derive(Debug)]
/// Manager for Session operations
pub struct SessionManager {
    /// Active sessions
    sessions: HashMap<String, Session>,
    /// User session mapping
    user_sessions: HashMap<String, Vec<String>>,
    /// Session configuration
    max_sessions_per_user: u32,
    default_session_lifetime: Duration,
    idle_timeout: Duration,
}
impl SessionManager {
    /// Create new session manager
    #[must_use]
    pub fn new() -> Self { Self {
            sessions: HashMap::new(),
            user_sessions: HashMap::new(),
            max_sessions_per_user: 10,
            default_session_lifetime: Duration::from_secs(3600), // 1 hour
            idle_timeout: Duration::from_secs(1800),             // 30 minutes
         }

    /// Create a new session
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn create_session(&mut self, user_id: &str, token: Option<AuthToken>) -> Result<Session>  {
        // Check session limits
        let user_session_count = self
            .user_sessions
            .get(user_id)
            .map(|sessions| sessions.len())
            .unwrap_or(0);

        if user_session_count >= self.max_sessions_per_user as usize {
            return Err(NestGateError::mcp_error(
                "Maximum sessions per user exceeded",
                "create_session",
                None,
            ));
        }

        let session_id = format!(
            "sess_{}_{}",
            user_id,
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_else(|_e| {
                    tracing::error!("Unwrap failed: {:?}", e);
                    // Return a safe default duration (current timestamp approximation)
                    Duration::from_secs(1700000000) // Approximate timestamp around 2023
                })
                .as_secs()
        );
        let now = SystemTime::now();

        let session = Session {
            session_id: session_id.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            last_activity: now,
            expires_at: now + self.default_session_lifetime,
            metadata: HashMap::new(),
            token,
        };

        self.sessions.insert(session_id.clone(), session.clone());
        self.user_sessions
            .entry(user_id.to_string())
            .or_default()
            .push(session_id);

        Ok(session)
    }

    /// Get a session by ID
    pub fn get_session(&self, session_id: &str) -> Option<&Session> {
        self.sessions.get(session_id)
    }

    /// Get a mutable session by ID
    #[must_use]
    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(session_id)
    }

    /// Update session activity
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn update_session_activity(&mut self, session_id: &str) -> Result<()>  {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.update_activity();
            Ok(())
        } else {
            Err(NestGateError::mcp_error(
                "Session not found",
                "update_session_activity",
                None,
            ))
        }
    }

    /// Terminate a session
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn terminate_session(&mut self, session_id: &str) -> Result<()>  {
        if let Some(session) = self.sessions.remove(session_id) {
            // Remove from user sessions
            if let Some(user_sessions) = self.user_sessions.get_mut(&session.user_id) {
                user_sessions.retain(|id| id != session_id);
                if user_sessions.is_empty() {
                    self.user_sessions.remove(&session.user_id);
                }
            }
            Ok(())
        } else {
            Err(NestGateError::mcp_error(
                "Session not found",
                "terminate_session",
                None,
            ))
        }
    }

    /// Clean up expired and idle sessions
    pub fn cleanup_sessions(&mut self) {
        let expired_sessions: Vec<String> = self
            .sessions
            .iter()
            .filter(|(_, session)| session.is_expired() || session.is_idle(self.idle_timeout))
            .map(|(id, _)| id.clone())
            .collect();

        for session_id in expired_sessions {
            self.terminate_session(&session_id).ok();
        }
    }

    /// Get all sessions for a user
    pub fn get_user_sessions(&self, user_id: &str) -> Vec<&Session> {
        self.user_sessions
            .get(user_id)
            .map(|session_ids| {
                session_ids
                    .iter()
                    .filter_map(|id| self.sessions.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Terminate all sessions for a user
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn terminate_user_sessions(&mut self, user_id: &str) -> Result<u32>  {
        let session_ids = self.user_sessions.get(user_id).cloned().unwrap_or_default();

        let mut terminated_count = 0;
        for session_id in session_ids {
            if self.terminate_session(&session_id).is_ok() {
                terminated_count += 1;
            }
        }

        Ok(terminated_count)
    }
}

impl Default for SessionManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Token validator for validating tokens without managing them
pub struct TokenValidator {
    /// Validation rules
    validation_rules: TokenValidationRules,
}
/// Token validation rules
#[derive(Debug, Clone)]
/// Tokenvalidationrules
pub struct TokenValidationRules {
    /// Minimum token length
    pub min_token_length: usize,
    /// Maximum token age
    pub max_token_age: Duration,
    /// Required token prefix
    pub required_prefix: Option<String>,
    /// Allowed token formats
    pub allowed_formats: Vec<String>,
}
impl Default for TokenValidationRules {
    /// Returns the default instance
    fn default() -> Self { Self {
            min_token_length: 32,
            max_token_age: Duration::from_secs(3600),
            required_prefix: Some("mcp_".to_string()),
            allowed_formats: vec!["bearer".to_string()],
         }
}

impl TokenValidator {
    /// Create new token validator
    pub fn new(rules: TokenValidationRules) -> Self { Self {
            validation_rules: rules,
         }

    /// Validate token format and basic properties
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate_format(&self, token: &str) -> Result<()>  {
        if token.len() < self.validation_rules.min_token_length {
            return Err(NestGateError::mcp_error(
                "Token too short",
                "validate_format",
                None,
            ));
        }

        if let Some(prefix) = &self.validation_rules.required_prefix {
            if !token.starts_with(prefix) {
                return Err(NestGateError::mcp_error(
                    "Invalid token prefix",
                    "validate_format",
                    None,
                ));
            }
        }

        Ok(())
    }
}

impl Default for TokenValidator {
    /// Returns the default instance
    fn default() -> Self {
        Self::new(TokenValidationRules::default())
    }
}
