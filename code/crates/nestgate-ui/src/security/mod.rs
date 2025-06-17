//! Security module for NestGate
//!
//! This module contains authentication, authorization, and other security-related
//! functionality for the NestGate system.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use crate::security::auth::{AuthError, AuthErrorKind};
use std::collections::HashSet;

/// Security role types for API access
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Admin role with full access
    Admin,
    /// Operator role with management access
    Operator,
    /// Read-only role with limited access
    ReadOnly,
    /// Custom role with specific permissions
    Custom(String),
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Operator => write!(f, "operator"),
            Role::ReadOnly => write!(f, "readonly"),
            Role::Custom(role) => write!(f, "{}", role),
        }
    }
}

impl Role {
    /// Convert a string to a Role
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => Role::Admin,
            "operator" => Role::Operator,
            "readonly" => Role::ReadOnly,
            other => Role::Custom(other.to_string()),
        }
    }
}

/// Permission types for access control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    /// Administrator access
    Admin,
    /// Read-only access
    ReadOnly,
    /// Read-write access
    ReadWrite,
    /// View storage health information
    ViewStorageHealth,
}

/// Authentication context with user information
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub username: String,
    pub role: String,
    pub permissions: HashSet<Permission>,
    pub user_id: String,
    pub api_key: String,
}

impl AuthContext {
    pub fn new(username: String, role: String, user_id: String, api_key: String) -> Self {
        // Initialize permissions based on role
        let permissions = match role.as_str() {
            "admin" => {
                let mut perms = HashSet::new();
                perms.insert(Permission::Admin);
                perms.insert(Permission::ReadWrite);
                perms.insert(Permission::ReadOnly);
                perms.insert(Permission::ViewStorageHealth);
                perms
            },
            "operator" => {
                let mut perms = HashSet::new();
                perms.insert(Permission::ReadWrite);
                perms.insert(Permission::ReadOnly);
                perms.insert(Permission::ViewStorageHealth);
                perms
            },
            "readonly" => {
                let mut perms = HashSet::new();
                perms.insert(Permission::ReadOnly);
                perms.insert(Permission::ViewStorageHealth);
                perms
            },
            _ => HashSet::new(), // No permissions for unknown roles
        };
        
        Self {
            username,
            role,
            permissions,
            user_id,
            api_key,
        }
    }

    /// Check if the context has a specific permission
    pub fn has_permission(&self, permission: Permission) -> bool {
        // Admin role always has all permissions
        if self.permissions.contains(&Permission::Admin) {
            return true;
        }
        self.permissions.contains(&permission)
    }
}

/// API token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToken {
    pub token: String,
    pub user_id: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_revoked: bool,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub role: Role,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}

/// Authentication and authorization manager
pub struct AuthManager {
    // In-memory token storage (would use a database in production)
    tokens: RwLock<HashMap<String, ApiToken>>,
    // In-memory user storage (would use a database in production)
    users: RwLock<HashMap<String, User>>,
}

impl AuthManager {
    /// Create a new auth manager with default admin, operator, and readonly users and tokens
    pub fn new() -> Self {
        let mut tokens = HashMap::new();
        let mut users = HashMap::new();
        
        // Create default users
        let admin_user = User {
            id: "admin-user".to_string(),
            username: "admin".to_string(),
            role: Role::Admin,
            is_active: true,
            created_at: chrono::Utc::now(),
            last_login: None,
        };
        
        let operator_user = User {
            id: "operator-user".to_string(),
            username: "operator".to_string(),
            role: Role::Operator,
            is_active: true,
            created_at: chrono::Utc::now(),
            last_login: None,
        };
        
        let readonly_user = User {
            id: "readonly-user".to_string(),
            username: "readonly".to_string(),
            role: Role::ReadOnly,
            is_active: true,
            created_at: chrono::Utc::now(),
            last_login: None,
        };
        
        // Create default tokens
        let admin_token = ApiToken {
            token: "admin-api-token".to_string(),
            user_id: admin_user.id.clone(),
            description: "Default admin token".to_string(),
            created_at: chrono::Utc::now(),
            expires_at: None,
            is_revoked: false,
        };
        
        let operator_token = ApiToken {
            token: "operator-api-token".to_string(),
            user_id: operator_user.id.clone(),
            description: "Default operator token".to_string(),
            created_at: chrono::Utc::now(),
            expires_at: None,
            is_revoked: false,
        };
        
        let readonly_token = ApiToken {
            token: "readonly-api-token".to_string(),
            user_id: readonly_user.id.clone(),
            description: "Default readonly token".to_string(),
            created_at: chrono::Utc::now(),
            expires_at: None,
            is_revoked: false,
        };
        
        // Add users and tokens to storage
        users.insert(admin_user.id.clone(), admin_user);
        users.insert(operator_user.id.clone(), operator_user);
        users.insert(readonly_user.id.clone(), readonly_user);
        
        tokens.insert(admin_token.token.clone(), admin_token);
        tokens.insert(operator_token.token.clone(), operator_token);
        tokens.insert(readonly_token.token.clone(), readonly_token);
        
        Self {
            tokens: RwLock::new(tokens),
            users: RwLock::new(users),
        }
    }
    
    /// Get authentication context from API key
    pub async fn get_context_from_api_key(&self, api_key: &str) -> Option<AuthContext> {
        let tokens = self.tokens.read().await;
        let token = tokens.get(api_key)?;
        
        // Check if token is expired or revoked
        if token.is_revoked {
            return None;
        }
        
        if let Some(expires_at) = token.expires_at {
            if expires_at < chrono::Utc::now() {
                return None;
            }
        }
        
        let users = self.users.read().await;
        let user = users.get(&token.user_id)?;
        
        // Check if user is active
        if !user.is_active {
            return None;
        }
        
        Some(AuthContext::new(
            user.username.clone(),
            user.role.to_string(),
            user.id.clone(),
            api_key.to_string(),
        ))
    }
    
    /// Check if a user has a specific permission
    pub fn has_permission(&self, context: &AuthContext, permission: &Permission) -> bool {
        match context.role.as_str() {
            "admin" => true, // Admin has all permissions
            "operator" => {
                // Operator has all permissions except certain admin functions
                match permission {
                    Permission::Admin => false,
                    Permission::ReadWrite => true,
                    Permission::ReadOnly => true,
                    Permission::ViewStorageHealth => true,
                }
            },
            "readonly" => {
                // ReadOnly has only read permissions
                match permission {
                    Permission::ReadOnly => true,
                    Permission::ViewStorageHealth => true, // Read-only users can view storage health
                    _ => false,
                }
            }
            _ => false, // Custom roles are not allowed in this context
        }
    }
    
    /// Create a new API token for a user
    pub async fn create_token(
        &self,
        user_id: &str,
        description: &str,
        expires_in_days: Option<u32>,
    ) -> Option<ApiToken> {
        let users = self.users.read().await;
        let user = users.get(user_id)?;
        
        // Generate a random token (in production, use a secure method)
        let token = format!("{}-{}", user.username, uuid::Uuid::new_v4());
        
        let expires_at = expires_in_days.map(|days| {
            chrono::Utc::now() + chrono::Duration::days(days as i64)
        });
        
        let api_token = ApiToken {
            token: token.clone(),
            user_id: user_id.to_string(),
            description: description.to_string(),
            created_at: chrono::Utc::now(),
            expires_at,
            is_revoked: false,
        };
        
        let mut tokens = self.tokens.write().await;
        tokens.insert(token.clone(), api_token.clone());
        
        Some(api_token)
    }
    
    /// Revoke an API token
    pub async fn revoke_token(&self, token_value: &str) -> bool {
        let mut tokens = self.tokens.write().await;
        
        if let Some(mut token) = tokens.get_mut(token_value) {
            token.is_revoked = true;
            return true;
        }
        
        false
    }
    
    /// Get all tokens for a user
    pub async fn get_user_tokens(&self, user_id: &str) -> Vec<ApiToken> {
        let tokens = self.tokens.read().await;
        
        tokens.values()
            .filter(|token| token.user_id == user_id && !token.is_revoked)
            .cloned()
            .collect()
    }
    
    /// Create a new user
    pub async fn create_user(&self, username: &str, role: Role) -> User {
        let user_id = uuid::Uuid::new_v4().to_string();
        
        let user = User {
            id: user_id.clone(),
            username: username.to_string(),
            role,
            is_active: true,
            created_at: chrono::Utc::now(),
            last_login: None,
        };
        
        let mut users = self.users.write().await;
        users.insert(user_id, user.clone());
        
        user
    }
    
    /// Get a user by ID
    pub async fn get_user(&self, user_id: &str) -> Option<User> {
        let users = self.users.read().await;
        users.get(user_id).cloned()
    }
    
    /// Get all users
    pub async fn get_all_users(&self) -> Vec<User> {
        let users = self.users.read().await;
        users.values().cloned().collect()
    }
    
    /// Update a user's role
    pub async fn update_user_role(&self, user_id: &str, role: Role) -> Option<User> {
        let mut users = self.users.write().await;
        
        if let Some(user) = users.get_mut(user_id) {
            user.role = role;
            return Some(user.clone());
        }
        
        None
    }
    
    /// Deactivate a user
    pub async fn deactivate_user(&self, user_id: &str) -> Option<User> {
        let mut users = self.users.write().await;
        
        if let Some(user) = users.get_mut(user_id) {
            user.is_active = false;
            
            // Revoke all tokens for this user
            drop(users); // Release the lock before acquiring another one
            let mut tokens = self.tokens.write().await;
            for token in tokens.values_mut() {
                if token.user_id == user_id {
                    token.is_revoked = true;
                }
            }
            
            let users = self.users.read().await;
            return users.get(user_id).cloned();
        }
        
        None
    }
    
    /// Record a user login
    pub async fn record_login(&self, user_id: &str) -> Option<User> {
        let mut users = self.users.write().await;
        
        if let Some(user) = users.get_mut(user_id) {
            user.last_login = Some(chrono::Utc::now());
            return Some(user.clone());
        }
        
        None
    }
    
    /// Validate an API key and return an AuthContext
    pub fn validate_api_key(&self, api_key: &str) -> Result<AuthContext, AuthError> {
        // In real implementation, this would query a database
        // For test/demo, return predefined contexts based on API key
        match api_key {
            "admin-api-token" => {
                Ok(AuthContext::new(
                    "admin".to_string(),
                    "admin".to_string(),
                    "admin-user".to_string(),
                    api_key.to_string(),
                ))
            },
            "operator-api-token" => {
                Ok(AuthContext::new(
                    "operator".to_string(),
                    "operator".to_string(),
                    "operator-user".to_string(),
                    api_key.to_string(),
                ))
            },
            "readonly-api-token" => {
                Ok(AuthContext::new(
                    "readonly".to_string(),
                    "readonly".to_string(),
                    "readonly-user".to_string(),
                    api_key.to_string(),
                ))
            },
            _ => Err(AuthError {
                message: "Invalid API key".to_string(),
                kind: AuthErrorKind::Unauthorized,
            }),
        }
    }
}

/// API key management and validation
pub mod api_keys;

// Re-export common components
pub use api_keys::ApiKeyManager;
pub use api_keys::validate_api_key_from_headers;

pub mod auth;
pub mod crypto; 