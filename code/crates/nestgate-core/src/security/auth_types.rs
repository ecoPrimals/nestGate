/// Authentication Types and Core Structures
/// This module contains the core types, enums, and data structures used
/// throughout the NestGate authentication system.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::SystemTime;
/// User roles - local definition for NestGate-specific needs
/// Real auth decisions delegate to security primals via universal adapter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    /// System administrator with full privileges
    Admin,
    /// System operator with operational privileges
    Operator,
    /// Service user for inter-service communication
    Service,
    /// Regular user with limited access
    User,
    /// Read-only access
    ReadOnly,
}
impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Operator => write!(f, "operator"),
            Role::Service => write!(f, "service"),
            Role::User => write!(f, "user"),
            Role::ReadOnly => write!(f, "read-only"),
        }
    }
}

/// Permission structure - local definition for NestGate operations
/// Complex permissions delegate to security primals via universal adapter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Permission {
    /// Permission name/identifier
    pub name: String,
    /// Optional scope for the permission
    pub scope: Option<String>,
}
impl Permission {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            scope: None,
        }
    }

    pub fn with_scope(name: &str, scope: &str) -> Self {
        Self {
            name: name.to_string(),
            scope: Some(scope.to_string()),
        }
    }
}

/// Access levels for different operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessLevel {
    /// No access
    None,
    /// Read-only access
    Read,
    /// Read and write access
    Write,
    /// Administrative access
    Admin,
}
/// Authentication context for requests
#[derive(Debug, Clone)]
pub struct AuthContext {
    /// User ID making the request
    pub user_id: Option<String>,
    /// User roles
    pub roles: Vec<Role>,
    /// User permissions
    pub permissions: Vec<Permission>,
    /// Request timestamp
    pub timestamp: SystemTime,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}
impl AuthContext {
    /// Create a new authentication context
    #[must_use]
    pub fn new() -> Self {
        Self {
            user_id: None,
            roles: Vec::new(),
            permissions: Vec::new(),
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
        }
    }

    /// Set the user ID
    #[must_use]
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Add a role
    #[must_use]
    pub fn with_role(mut self, role: Role) -> Self {
        self.roles.push(role);
        self
    }

    /// Add a permission
    #[must_use]
    pub fn with_permission(mut self, permission: Permission) -> Self {
        self.permissions.push(permission);
        self
    }

    /// Check if the context has a specific role
    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(role)
    }

    /// Check if the context has a specific permission
    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }

    /// Check if the context has admin access
    pub fn is_admin(&self) -> bool {
        self.has_role(&Role::Admin)
    }

    /// Get access level for a resource
    pub fn get_access_level(&self, resource: &str) -> AccessLevel {
        if self.is_admin() {
            return AccessLevel::Admin;
        }

        // Check specific permissions
        for perm in &self.permissions {
            if let Some(scope) = &perm.scope {
                if scope == resource {
                    match perm.name.as_str() {
                        "admin" => return AccessLevel::Admin,
                        "write" => return AccessLevel::Write,
                        "read" => return AccessLevel::Read,
                        _ => {}
                    }
                }
            }
        }

        // Check general permissions
        for perm in &self.permissions {
            if perm.scope.is_none() {
                match perm.name.as_str() {
                    "admin" => return AccessLevel::Admin,
                    "write" => return AccessLevel::Write,
                    "read" => return AccessLevel::Read,
                    _ => {}
                }
            }
        }

        AccessLevel::None
    }
}

impl Default for AuthContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Common permission helpers
pub fn read_permission() -> Permission {
    Permission::new("read")
}
pub fn write_permission() -> Permission {
    Permission::new("write")
}

pub fn admin_permission() -> Permission {
    Permission::new("admin")
}

/// Authentication methods
#[derive(Debug, Clone, PartialEq)]
pub enum AuthMethod {
    /// No authentication
    None,
    /// API key authentication
    ApiKey(String),
    /// JWT token authentication
    JwtToken(String),
    /// Basic HTTP authentication
    Basic { username: String, password: String },
    /// OAuth2 token
    OAuth2(String),
    /// Custom authentication method
    Custom(HashMap<String, String>),
}

/// Resource-specific permission helpers
pub fn read_permission_for(resource: &str) -> Permission {
    Permission::with_scope("read", resource)
}

pub fn write_permission_for(resource: &str) -> Permission {
    Permission::with_scope("write", resource)
}

pub fn admin_permission_for(resource: &str) -> Permission {
    Permission::with_scope("admin", resource)
}

/// Token types for different authentication mechanisms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    /// Access token for API requests
    Access,
    /// Refresh token for obtaining new access tokens
    Refresh,
    /// API key for service-to-service communication
    ApiKey,
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Access => write!(f, "access"),
            TokenType::Refresh => write!(f, "refresh"),
            TokenType::ApiKey => write!(f, "api_key"),
        }
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Role Tests ====================

    #[test]
    fn test_role_display() {
        assert_eq!(Role::Admin.to_string(), "admin");
        assert_eq!(Role::Operator.to_string(), "operator");
        assert_eq!(Role::Service.to_string(), "service");
        assert_eq!(Role::User.to_string(), "user");
        assert_eq!(Role::ReadOnly.to_string(), "read-only");
    }

    #[test]
    fn test_role_equality() {
        assert_eq!(Role::Admin, Role::Admin);
        assert_ne!(Role::Admin, Role::User);
    }

    // ==================== Permission Tests ====================

    #[test]
    fn test_permission_creation_without_scope() {
        let perm = Permission::new("read");
        assert_eq!(perm.name, "read");
        assert_eq!(perm.scope, None);
    }

    #[test]
    fn test_permission_creation_with_scope() {
        let perm = Permission::with_scope("write", "users");
        assert_eq!(perm.name, "write");
        assert_eq!(perm.scope, Some("users".to_string()));
    }

    #[test]
    fn test_permission_helpers() {
        let read = read_permission();
        assert_eq!(read.name, "read");
        
        let write = write_permission();
        assert_eq!(write.name, "write");
        
        let admin = admin_permission();
        assert_eq!(admin.name, "admin");
    }

    // ==================== AccessLevel Tests ====================

    #[test]
    fn test_access_level_equality() {
        assert_eq!(AccessLevel::None, AccessLevel::None);
        assert_eq!(AccessLevel::Read, AccessLevel::Read);
        assert_eq!(AccessLevel::Write, AccessLevel::Write);
        assert_eq!(AccessLevel::Admin, AccessLevel::Admin);
        assert_ne!(AccessLevel::Read, AccessLevel::Write);
    }

    // ==================== AuthContext Tests ====================

    #[test]
    fn test_auth_context_creation() {
        let ctx = AuthContext::new();
        assert!(ctx.user_id.is_none());
        assert!(ctx.roles.is_empty());
        assert!(ctx.permissions.is_empty());
        assert!(ctx.metadata.is_empty());
    }

    #[test]
    fn test_auth_context_builder() {
        let ctx = AuthContext::new()
            .with_user_id("user123".to_string())
            .with_role(Role::Admin)
            .with_permission(Permission::new("read"));
        
        assert_eq!(ctx.user_id, Some("user123".to_string()));
        assert_eq!(ctx.roles.len(), 1);
        assert_eq!(ctx.permissions.len(), 1);
        assert!(ctx.has_role(&Role::Admin));
    }

    #[test]
    fn test_auth_context_role_checking() {
        let ctx = AuthContext::new()
            .with_role(Role::Admin)
            .with_role(Role::Operator);
        
        assert!(ctx.has_role(&Role::Admin));
        assert!(ctx.has_role(&Role::Operator));
        assert!(!ctx.has_role(&Role::User));
        assert!(ctx.is_admin());
    }

    #[test]
    fn test_auth_context_permission_checking() {
        let read_perm = Permission::new("read");
        let write_perm = Permission::new("write");
        
        let ctx = AuthContext::new()
            .with_permission(read_perm.clone());
        
        assert!(ctx.has_permission(&read_perm));
        assert!(!ctx.has_permission(&write_perm));
    }

    #[test]
    fn test_auth_context_access_level_admin() {
        let ctx = AuthContext::new().with_role(Role::Admin);
        let access = ctx.get_access_level("any_resource");
        assert_eq!(access, AccessLevel::Admin);
    }

    #[test]
    fn test_auth_context_access_level_scoped_permission() {
        let ctx = AuthContext::new()
            .with_permission(Permission::with_scope("write", "users"));
        
        let access = ctx.get_access_level("users");
        assert_eq!(access, AccessLevel::Write);
        
        let other_access = ctx.get_access_level("posts");
        assert_eq!(other_access, AccessLevel::None);
    }

    #[test]
    fn test_auth_context_access_level_general_permission() {
        let ctx = AuthContext::new()
            .with_permission(Permission::new("read"));
        
        let access = ctx.get_access_level("any_resource");
        assert_eq!(access, AccessLevel::Read);
    }

    #[test]
    fn test_auth_context_default() {
        let ctx = AuthContext::default();
        assert!(ctx.user_id.is_none());
        assert!(ctx.roles.is_empty());
        assert!(ctx.permissions.is_empty());
    }

    // ==================== AuthMethod Tests ====================

    #[test]
    fn test_auth_method_variants() {
        let none = AuthMethod::None;
        assert_eq!(none, AuthMethod::None);

        let api_key = AuthMethod::ApiKey("key123".to_string());
        assert_eq!(api_key, AuthMethod::ApiKey("key123".to_string()));

        let jwt = AuthMethod::JwtToken("token".to_string());
        assert_eq!(jwt, AuthMethod::JwtToken("token".to_string()));

        let basic = AuthMethod::Basic {
            username: "user".to_string(),
            password: "pass".to_string(),
        };
        assert!(matches!(basic, AuthMethod::Basic { .. }));
    }

    // ==================== TokenType Tests ====================

    #[test]
    fn test_token_type_display() {
        assert_eq!(TokenType::Access.to_string(), "access");
        assert_eq!(TokenType::Refresh.to_string(), "refresh");
        assert_eq!(TokenType::ApiKey.to_string(), "api_key");
    }

    #[test]
    fn test_token_type_equality() {
        assert_eq!(TokenType::Access, TokenType::Access);
        assert_ne!(TokenType::Access, TokenType::Refresh);
    }
}
