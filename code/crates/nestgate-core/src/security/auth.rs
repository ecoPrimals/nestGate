// Removed unused tracing import
use crate::error::{NetworkError};
/// Authentication and authorization module for NestGate
///
/// This module provides authentication and authorization functionality
/// for the NestGate system.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Removed unused tracing import
use std::time::{ Instant};
// Removed unused std import
use thiserror::Error;
use crate::error::{NestGateError, Result};
use crate::security::{Role, Permission, AccessLevel, AuthContext};
use std::time::Duration;
use std::fmt;

/// Authorization errors
#[derive(Debug, Clone)]
pub enum AuthErrorKind {
    /// User is not authorized
    Unauthorized,

    /// Access is forbidden
    Forbidden,

    /// Resource not found
    NotFound,

    /// Server error
    ServerError,
}

/// Authentication error
#[derive(Debug, Clone, Error)]
#[error("{message}")]
pub struct AuthError {
    /// Error message
    pub message: String,

    /// Error kind
    pub kind: AuthErrorKind,
}

impl AuthError {
    /// Permission denied error
    pub fn PermissionDenied() -> Self {
        Self {
            message: "Permission denied".to_string(),
            kind: AuthErrorKind::Forbidden,
        }
    }
}

// Using imports from above

/// Permission for read access
pub fn read_permission() -> Permission {
    Permission::new("system.read")
}

/// Permission for write access
pub fn write_permission() -> Permission {
    Permission::new("system.write")
}

/// Permission for admin access
pub fn admin_permission() -> Permission {
    Permission::new("system.admin")
}

// Using the AuthContext from the parent module

impl AuthContext {
    /// Create a new authentication context
    pub fn new(user_id: String, username: String, role: Role) -> Self {
        Self {
            user_id,
            username,
            role,
            permissions: Vec::new(),
            auth_method: AuthMethod::Password,
            auth_time: Instant::now(),
            expiration: None,
            metadata: HashMap::new(),
        }
    }

    /// Set authentication method
    pub fn with_auth_method(mut self, method: AuthMethod) -> Self {
        self.auth_method = method;
        self
    }

    /// Set expiration time
    pub fn with_expiration(mut self, duration: Duration) -> Self {
        self.expiration = Some(self.auth_time + duration);
        self
    }

    /// Add permission
    pub fn with_permission(mut self, permission: Permission) -> Self {
        self.permissions.push(permission);
        self
    }

    /// Add multiple permissions
    pub fn with_permissions(mut self, permissions: Vec<Permission>) -> Self {
        self.permissions.extend(permissions);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Check if the authentication context has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expiration) = self.expiration {
            Instant::now() > expiration
        } else {
            false
        }
    }

    /// Check if the user has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        // Admin role has all permissions
        if self.role == Role::Admin {
            return true;
        }

        // Check for specific permission
        self.permissions.iter().any(|p| p == permission)
    }

    /// Check if the user has admin role
    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }

    /// Check if the user has operator role
    pub fn is_operator(&self) -> bool {
        self.role == Role::Operator || self.role == Role::Admin
    }

    /// Check if the user has read-only role
    pub fn is_read_only(&self) -> bool {
        self.role == Role::ReadOnly || self.role == Role::Operator || self.role == Role::Admin
    }
}

/// Authentication method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMethod {
    /// Password authentication
    Password,

    /// API key authentication
    ApiKey,

    /// Session authentication
    Session,

    /// OAuth authentication
    OAuth,

    /// LDAP authentication
    Ldap,

    /// SAML authentication
    Saml,
}

/// Authentication manager
#[derive(Debug)]
pub struct AuthManager {
    /// Users
    users: Arc<RwLock<HashMap<String, AuthContext>>>,

    /// API keys
    api_keys: Arc<RwLock<HashMap<String, String>>>,
}

// Custom Clone implementation
impl Clone for AuthManager {
    fn clone(&self) -> Self {
        Self {
            users: self.users.clone(),
            api_keys: self.api_keys.clone(),
        }
    }
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            api_keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a user
    pub async fn add_user(&self, user_id: String, username: String, role: Role, permissions: Vec<Permission>) {
        let mut users = self.users.write().await;

        let context = AuthContext {
            user_id: user_id.clone(),
            role,
            permissions,
            username,
            auth_method: AuthMethod::Password,
            auth_time: Instant::now(),
            expiration: None,
            metadata: HashMap::new(),
        };

        users.insert(user_id, context);
    }

    /// Remove a user
    pub async fn remove_user(&self, user_id: &str) -> bool {
        let mut users = self.users.write().await;
        users.remove(user_id).is_some()
    }

    /// Add an API key
    pub async fn add_api_key(&self, key: String, user_id: String) {
        let mut api_keys = self.api_keys.write().await;
        api_keys.insert(key, user_id);
    }

    /// Remove an API key
    pub async fn remove_api_key(&self, key: &str) -> bool {
        let mut api_keys = self.api_keys.write().await;
        api_keys.remove(key).is_some()
    }

    /// Get authentication context from API key
    pub async fn get_context_from_api_key(&self, key: &str) -> Option<AuthContext> {
        let api_keys = self.api_keys.read().await;

        if let Some(user_id) = api_keys.get(key) {
            let users = self.users.read().await;
            users.get(user_id).cloned()
        } else {
            None
        }
    }

    /// Validate API key
    pub async fn validate_api_key(&self, key: &str) -> std::result::Result<AuthContext, String> {
        match self.get_context_from_api_key(key).await {
            Some(ctx) => Ok(ctx),
            None => Err("Invalid API key".to_string()),
        }
    }

    /// Check if a user has a permission
    pub async fn has_permission(&self, user_id: &str, permission: &Permission) -> bool {
        let users = self.users.read().await;

        if let Some(context) = users.get(user_id) {
            // Admins have all permissions
            if context.role == Role::Admin {
                return true;
            }

            // Check specific permission
            context.permissions.iter().any(|p| p.resource == permission.resource && p.action == permission.action)
        } else {
            false
        }
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

/// API key configuration
#[derive(Debug, Clone)]
pub struct ApiKeyConfig {
    /// Default expiration time in seconds
    pub default_expiration: u64,

    /// Maximum expiration time in seconds
    pub max_expiration: u64,

    /// Minimum key length
    pub min_key_length: usize,
}

impl Default for ApiKeyConfig {
    fn default() -> Self {
        Self {
            default_expiration: (crate::constants::time::DAY.as_secs() * 30) as u64, // 30 days
            max_expiration: (crate::constants::time::DAY.as_secs() * 365) as u64,    // 1 year
            min_key_length: 32,
        }
    }
}

/// Permission functions
pub fn resource_read_permission(resource: &str) -> Permission {
    Permission::new(&format!("{}.read", resource))
}

/// Write permission function
pub fn resource_write_permission(resource: &str) -> Permission {
    Permission::new(&format!("{}.write", resource))
}

/// Admin permission function
pub fn resource_admin_permission(resource: &str) -> Permission {
    Permission::new(&format!("{}.admin", resource))
}

/// Auth middleware to enforce permissions
pub struct AuthMiddleware {
    /// Required permissions for the endpoint
    required_permissions: Vec<String>,

    /// Required role for the endpoint
    required_role: Option<Role>,
}

impl AuthMiddleware {
    /// Create a new auth middleware that requires specific permissions
    pub fn with_permissions(permissions: Vec<String>) -> Self {
        Self {
            required_permissions: permissions,
            required_role: None,
        }
    }

    /// Create a new auth middleware that requires a specific role
    pub fn with_role(role: Role) -> Self {
        Self {
            required_permissions: vec![],
            required_role: Some(role),
        }
    }

    /// Check if the auth context satisfies the middleware requirements
    pub fn check(&self, context: &AuthContext) -> Result<()> {
        // Check role requirement if set
        if let Some(required_role) = self.required_role {
            match required_role {
                Role::Admin => {
                    if !context.is_admin() {
                        return Err(NestGateError::Authorization("Permission denied: admin role required".to_string()));
                    }
                }
                Role::Operator => {
                    if !context.is_operator() {
                        return Err(NestGateError::Authorization("Permission denied: operator role required".to_string()));
                    }
                }
                Role::ReadOnly => {
                    if !context.is_read_only() {
                        return Err(NestGateError::Authorization("Permission denied: read-only role required".to_string()));
                    }
                }
                _ => {
                    if context.role != required_role && !context.is_admin() {
                        return Err(NestGateError::Authorization("Permission denied: specific role required".to_string()));
                    }
                }
            }
        }

        // Check permissions if required
        for perm_name in &self.required_permissions {
            let perm = Permission::new(perm_name);
            if !context.has_permission(&perm) {
                return Err(NestGateError::Authorization(format!("Permission denied: {} required", perm_name)));
            }
        }

        Ok(())
    }
}

/// Helper function to create an auth middleware that requires admin role
pub fn require_admin() -> AuthMiddleware {
    AuthMiddleware::with_role(Role::Admin)
}

/// Helper function to create an auth middleware that requires operator role
pub fn require_operator() -> AuthMiddleware {
    AuthMiddleware::with_role(Role::Operator)
}

/// Helper function to create an auth middleware that requires read-only access
pub fn require_read_only() -> AuthMiddleware {
    AuthMiddleware::with_role(Role::ReadOnly)
}

/// Authentication token
#[derive(Debug, Clone)]
pub struct AuthToken {
    /// Token type
    pub token_type: TokenType,

    /// Token value
    pub token: String,

    /// Expiration timestamp
    pub expires_at: i64,
}

/// Token type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// API key
    ApiKey,

    /// JWT token
    Jwt,

    /// Session token
    Session,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::ApiKey => write!(f, "ApiKey"),
            TokenType::Jwt => write!(f, "JWT"),
            TokenType::Session => write!(f, "Session"),
        }
    }
}