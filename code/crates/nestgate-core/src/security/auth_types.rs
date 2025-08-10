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
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Add a role
    pub fn with_role(mut self, role: Role) -> Self {
        self.roles.push(role);
        self
    }

    /// Add a permission
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
    pub fn access_level_for(&self, resource: &str) -> AccessLevel {
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
pub fn resource_read_permission(resource: &str) -> Permission {
    Permission::with_scope("read", resource)
}

pub fn resource_write_permission(resource: &str) -> Permission {
    Permission::with_scope("write", resource)
}

pub fn resource_admin_permission(resource: &str) -> Permission {
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
