use crate::{NestGateError, Result};
use crate::security::AuthContext;
/// Authentication and authorization module for NestGate
///
/// This module provides authentication and authorization functionality
/// for the NestGate system.
use dashmap::DashMap;
use std::sync::Arc;
// Security types imported from auth_types module
use crate::security::{Permission, Role};
// AuthContext will be replaced with StorageAuthContext where needed
use std::fmt;
// Legacy AuthError and AuthErrorKind removed - use canonical SecurityError from crate::error

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

// Remove the duplicate AuthContext implementation since it conflicts with auth_types.rs
// The real AuthContext is defined in auth_types.rs and is already imported

/// Authentication method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Authmethod
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
/// 
/// **LOCK-FREE**: Uses DashMap for concurrent authentication
#[derive(Debug)]
/// Manager for Auth operations
pub struct AuthManager {
    /// Users
    users: Arc<DashMap<String, AuthContext>>,  // ✅ Lock-free
    /// API keys
    api_keys: Arc<DashMap<String, String>>,  // ✅ Lock-free
}

// Custom Clone implementation
impl Clone for AuthManager {
    /// Clone
    fn clone(&self) -> Self {
        Self {
            users: self.users.clone(),
            api_keys: self.api_keys.clone(),
        }
    }
}

impl AuthManager {
    /// Create a new authentication manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            api_keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a user
    pub async fn add_user(
        &self,
        user_id: String,
        _username: String,
        role: Role,
        permissions: Vec<Permission>,
    ) {
        let mut users = self.users.write().await;

        let context = AuthContext::new()
            .with_user_id(user_id.clone())
            .with_role(role);

        let context = permissions
            .into_iter()
            .fold(context, |ctx, perm| ctx.with_permission(perm));

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
            if context.roles.contains(&Role::Admin) {
                return true;
            }

            // Check specific permission
            context
                .permissions
                .iter()
                .any(|p| p.name == permission.name && p.scope == permission.scope)
        } else {
            false
        }
    }
}

impl Default for AuthManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// API key configuration
#[derive(Debug, Clone)]
/// Configuration for ApiKey
pub struct ApiKeyConfig {
    /// Default expiration time in seconds
    pub default_expiration: u64,
    /// Maximum expiration time in seconds
    pub max_expiration: u64,

    /// Minimum key length
    pub min_key_length: usize,
}

impl Default for ApiKeyConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            default_expiration: (24 * 60 * 60 * 30), // 30 days in seconds
            max_expiration: (24 * 60 * 60 * 365),    // 1 year in seconds
            min_key_length: 32,
        }
    }
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
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn check(&self, context: &AuthContext) -> Result<()>  {
        // Check role requirement if set
        if let Some(required_role) = &self.required_role {
            match *required_role {
                Role::Admin => {
                    if !context.is_admin() {
                        return Err(NestGateError::security(
                            "admin role required",
                            "access",
                            Some("admin-protected resource"),
                            None,
                        ));
                    }
                }
                Role::Operator => {
                    if !context.has_role(&Role::Operator) {
                        return Err(NestGateError::security(
                            "operator role required",
                            "access",
                            Some("operator-protected resource"),
                            None,
                        ));
                    }
                }
                Role::ReadOnly => {
                    if !context.has_role(&Role::ReadOnly) {
                        return Err(NestGateError::security(
                            "read-only role required",
                            "access",
                            Some("read-only resource"),
                            None,
                        ));
                    }
                }
                _ => {
                    if !context.has_role(required_role) && !context.is_admin() {
                        return Err(NestGateError::security(
                            "specific role required",
                            "access",
                            Some("role-protected resource"),
                            None,
                        ));
                    }
                }
            }
        }

        // Check permissions if required
        for perm_name in &self.required_permissions {
            let perm = Permission::new(perm_name);
            if !context.has_permission(&perm) {
                return Err(NestGateError::security(
                    &format!("{perm_name} permission required"),
                    "access",
                    Some("permission-protected resource"),
                    None,
                ));
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
/// Authtoken
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
/// Types of Token
pub enum TokenType {
    /// API key
    ApiKey,
    /// JWT token
    Jwt,

    /// Session token
    Session,
}

impl fmt::Display for TokenType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::ApiKey => write!(f, "ApiKey"),
            TokenType::Jwt => write!(f, "JWT"),
            TokenType::Session => write!(f, "Session"),
        }
    }
}
