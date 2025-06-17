//! Authentication and authorization for NestGate API
//!
//! This module provides role-based access control for the NestGate API endpoints
//! with comprehensive permission management and token-based authentication.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Duration, Utc};
use serde::{Serialize, Deserialize};
use tracing::{debug, error, info, warn};
use warp::Filter;

/// Permission scope for API access
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Permission {
    /// Read-only access to datasets
    DatasetRead,
    /// Create/update/delete datasets
    DatasetWrite,
    /// Read-only access to snapshots
    SnapshotRead,
    /// Create/update/delete snapshots
    SnapshotWrite,
    /// Read-only access to snapshot schedules
    ScheduleRead,
    /// Create/update/delete snapshot schedules
    ScheduleWrite,
    /// Access to system configuration
    ConfigAccess,
    /// Access to system monitoring data
    MonitorAccess,
    /// Access to administrative functions
    AdminAccess,
}

/// User roles in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Role {
    /// Administrator with full access
    Admin,
    /// Operator with most access except admin functions
    Operator,
    /// Read-only user
    ReadOnly,
    /// Custom role with specific permissions
    Custom(String),
}

impl Role {
    /// Create a Role from a string
    pub fn from_string(role_name: &str) -> Self {
        match role_name {
            "admin" => Role::Admin,
            "manager" => Role::Operator,
            "user" => Role::ReadOnly,
            "readonly" => Role::ReadOnly,
            _ => Role::Custom(role_name.to_string()),
        }
    }
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: String,
    /// Username
    pub username: String,
    /// Display name
    pub display_name: String,
    /// User role
    pub role: Role,
    /// Custom permissions (only used for Custom role)
    pub permissions: Option<HashSet<Permission>>,
    /// Email address
    pub email: Option<String>,
    /// Account creation date
    pub created_at: DateTime<Utc>,
    /// Last login date
    pub last_login: Option<DateTime<Utc>>,
    /// Is the account active
    pub active: bool,
}

/// API token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToken {
    /// Token ID
    pub id: String,
    /// Token value (hashed in storage)
    pub token: String,
    /// User ID this token belongs to
    pub user_id: String,
    /// Token description
    pub description: String,
    /// Permissions for this token (if None, use user's permissions)
    pub permissions: Option<HashSet<Permission>>,
    /// Role associated with this token
    pub role: String,
    /// Creation date
    pub created_at: DateTime<Utc>,
    /// Expiration date (if any)
    pub expires_at: Option<DateTime<Utc>>,
    /// Last used date
    pub last_used: Option<DateTime<Utc>>,
    /// Is the token active
    pub active: bool,
}

/// Authentication context for a request
#[derive(Debug, Clone)]
pub struct AuthContext {
    /// The authenticated user
    pub user: Option<User>,
    /// The API token used (if any)
    pub token: Option<ApiToken>,
    /// The combined permissions from user and token
    pub permissions: HashSet<Permission>,
}

/// Authentication and authorization manager
#[derive(Clone)]
pub struct AuthManager {
    /// Users in the system
    users: Arc<RwLock<HashMap<String, User>>>,
    /// API tokens
    tokens: Arc<RwLock<HashMap<String, ApiToken>>>,
    /// Role to permission mappings
    role_permissions: Arc<RwLock<HashMap<Role, HashSet<Permission>>>>,
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new() -> Self {
        let mut role_permissions = HashMap::new();
        
        // Set up default role permissions
        let admin_permissions = [
            Permission::DatasetRead,
            Permission::DatasetWrite,
            Permission::SnapshotRead,
            Permission::SnapshotWrite,
            Permission::ScheduleRead,
            Permission::ScheduleWrite,
            Permission::ConfigAccess,
            Permission::MonitorAccess,
            Permission::AdminAccess,
        ].iter().cloned().collect();
        
        let operator_permissions = [
            Permission::DatasetRead,
            Permission::DatasetWrite,
            Permission::SnapshotRead,
            Permission::SnapshotWrite,
            Permission::ScheduleRead,
            Permission::ScheduleWrite,
            Permission::MonitorAccess,
        ].iter().cloned().collect();
        
        let readonly_permissions = [
            Permission::DatasetRead,
            Permission::SnapshotRead,
            Permission::ScheduleRead,
            Permission::MonitorAccess,
        ].iter().cloned().collect();
        
        role_permissions.insert(Role::Admin, admin_permissions);
        role_permissions.insert(Role::Operator, operator_permissions);
        role_permissions.insert(Role::ReadOnly, readonly_permissions);
        
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            tokens: Arc::new(RwLock::new(HashMap::new())),
            role_permissions: Arc::new(RwLock::new(role_permissions)),
        }
    }
    
    /// Initialize with test users and tokens
    pub fn with_test_data() -> Self {
        let mut auth_manager = Self::new();
        
        // Create test users and tokens
        let tokens_arc = auth_manager.tokens.clone();
        let users_arc = auth_manager.users.clone();
        
        tokio::spawn(async move {
            let admin_user = User {
                id: "admin-id".to_string(),
                username: "admin".to_string(),
                display_name: "Administrator".to_string(),
                role: Role::Admin,
                permissions: None,
                email: Some("admin@example.com".to_string()),
                created_at: Utc::now(),
                last_login: None,
                active: true,
            };
            
            // Create admin API token
            let admin_token = ApiToken {
                id: "admin-token-id".to_string(),
                token: "admin-api-token".to_string(), // In production, this would be hashed
                user_id: admin_user.id.clone(),
                description: "Admin API Token".to_string(),
                permissions: None, // Use admin permissions
                role: "admin".to_string(),
                created_at: Utc::now(),
                expires_at: None,
                last_used: None,
                active: true,
            };
            
            // Add readonly user
            let readonly_user = User {
                id: "readonly-id".to_string(),
                username: "readonly".to_string(),
                display_name: "Read Only User".to_string(),
                role: Role::ReadOnly,
                permissions: None,
                email: Some("readonly@example.com".to_string()),
                created_at: Utc::now(),
                last_login: None,
                active: true,
            };
            
            // Create readonly API token
            let readonly_token = ApiToken {
                id: "readonly-token-id".to_string(),
                token: "readonly-api-token".to_string(),
                user_id: readonly_user.id.clone(),
                description: "ReadOnly API Token".to_string(),
                permissions: None,
                role: "readonly".to_string(),
                created_at: Utc::now(),
                expires_at: None,
                last_used: None,
                active: true,
            };
            
            // Add users and tokens to the maps
            let mut users = users_arc.write().await;
            users.insert(admin_user.id.clone(), admin_user);
            users.insert(readonly_user.id.clone(), readonly_user);
            
            let mut tokens = tokens_arc.write().await;
            tokens.insert(admin_token.token.clone(), admin_token);
            tokens.insert(readonly_token.token.clone(), readonly_token);
        });
        
        auth_manager
    }
    
    /// Authenticate a request using an API token
    pub async fn authenticate(&self, api_token: &str) -> Option<AuthContext> {
        // Get the token
        let token = {
            let tokens = self.tokens.read().await;
            tokens.get(api_token).cloned()
        };
        
        if let Some(token) = token {
            // Check if token is active and not expired
            if !token.active {
                debug!("Token {} is inactive", token.id);
                return None;
            }
            
            if let Some(expires_at) = token.expires_at {
                if expires_at < Utc::now() {
                    debug!("Token {} is expired", token.id);
                    return None;
                }
            }
            
            // Get the user for this token
            let user = {
                let users = self.users.read().await;
                users.get(&token.user_id).cloned()
            };
            
            if let Some(user) = user.clone() {
                if !user.active {
                    debug!("User {} is inactive", user.id);
                    return None;
                }
                
                // Determine permissions
                let permissions = self.get_effective_permissions(&user, &token).await;
                
                // Update last used time for token
                let mut tokens = self.tokens.write().await;
                if let Some(token_mut) = tokens.get_mut(&token.token) {
                    token_mut.last_used = Some(Utc::now());
                }
                
                // Update last login for user
                let mut users = self.users.write().await;
                if let Some(user_mut) = users.get_mut(&user.id) {
                    user_mut.last_login = Some(Utc::now());
                }
                
                Some(AuthContext {
                    user: Some(user),
                    token: Some(token),
                    permissions,
                })
            } else {
                debug!("User not found for token {}", token.id);
                None
            }
        } else {
            debug!("Token not found: {}", api_token);
            None
        }
    }
    
    /// Get effective permissions for a user and token
    async fn get_effective_permissions(&self, user: &User, token: &ApiToken) -> HashSet<Permission> {
        // Start with user's role-based permissions
        let mut permissions = match user.role {
            Role::Custom(_) => {
                // For custom roles, use the user's explicit permissions
                user.permissions.clone().unwrap_or_else(|| HashSet::new())
            },
            _ => {
                // For standard roles, get permissions from role mapping
                let role_perms = self.role_permissions.read().await;
                role_perms.get(&user.role).cloned().unwrap_or_else(|| HashSet::new())
            },
        };
        
        // If token has specific permissions, use those instead
        if let Some(token_permissions) = &token.permissions {
            permissions = token_permissions.clone();
        }
        
        permissions
    }
    
    /// Check if the auth context has a specific permission
    pub fn has_permission(&self, ctx: &AuthContext, permission: &Permission) -> bool {
        ctx.permissions.contains(permission)
    }
    
    /// Create a new API token for a user
    pub async fn create_token(
        &self,
        user_id: &str,
        description: &str,
        permissions: Option<HashSet<Permission>>,
        expires_in_days: Option<u32>,
    ) -> Result<ApiToken, String> {
        // Check if user exists
        let user = {
            let users = self.users.read().await;
            users.get(user_id).cloned()
        };
        
        if user.is_none() {
            return Err(format!("User {} not found", user_id));
        }
        
        // Create expiration date if specified
        let expires_at = expires_in_days.map(|days| {
            Utc::now() + Duration::days(days as i64)
        });
        
        // Generate token
        let token_value = format!("{}", Uuid::new_v4());
        
        // Create token
        let token = ApiToken {
            id: format!("token-{}", Uuid::new_v4()),
            token: token_value.clone(),
            user_id: user_id.to_string(),
            description: description.to_string(),
            permissions,
            role: "custom".to_string(),
            created_at: Utc::now(),
            expires_at,
            last_used: None,
            active: true,
        };
        
        // Store token
        let mut tokens = self.tokens.write().await;
        tokens.insert(token_value, token.clone());
        
        Ok(token)
    }
    
    /// Revoke an API token
    pub async fn revoke_token(&self, token_id: &str) -> Result<(), String> {
        let mut tokens = self.tokens.write().await;
        let token = tokens.iter_mut().find(|(_, t)| t.id == token_id);
        
        if let Some((_, token)) = token {
            token.active = false;
            Ok(())
        } else {
            Err(format!("Token {} not found", token_id))
        }
    }
    
    /// Create a new user
    pub async fn create_user(
        &self,
        username: &str,
        display_name: &str,
        role: Role,
        email: Option<String>,
        permissions: Option<HashSet<Permission>>,
    ) -> Result<User, String> {
        // Check if username already exists
        {
            let users = self.users.read().await;
            if users.values().any(|u| u.username == username) {
                return Err(format!("Username {} already exists", username));
            }
        }
        
        // Create user
        let user = User {
            id: format!("user-{}", Uuid::new_v4()),
            username: username.to_string(),
            display_name: display_name.to_string(),
            role,
            permissions,
            email,
            created_at: Utc::now(),
            last_login: None,
            active: true,
        };
        
        // Store user
        let mut users = self.users.write().await;
        users.insert(user.id.clone(), user.clone());
        
        Ok(user)
    }
    
    /// Get user by ID
    pub async fn get_user(&self, user_id: &str) -> Option<User> {
        let users = self.users.read().await;
        users.get(user_id).cloned()
    }
    
    /// Get user by username
    pub async fn get_user_by_username(&self, username: &str) -> Option<User> {
        let users = self.users.read().await;
        users.values().find(|u| u.username == username).cloned()
    }
    
    /// Update user
    pub async fn update_user(&self, user_id: &str, update: UserUpdate) -> Result<User, String> {
        let mut users = self.users.write().await;
        let user = users.get_mut(user_id);
        
        if let Some(user) = user {
            // Update user fields
            if let Some(display_name) = update.display_name {
                user.display_name = display_name;
            }
            if let Some(role) = update.role {
                user.role = role;
            }
            if let Some(permissions) = update.permissions {
                user.permissions = Some(permissions);
            }
            if let Some(email) = update.email {
                user.email = Some(email);
            }
            if let Some(active) = update.active {
                user.active = active;
            }
            
            Ok(user.clone())
        } else {
            Err(format!("User {} not found", user_id))
        }
    }
    
    /// Delete user
    pub async fn delete_user(&self, user_id: &str) -> Result<(), String> {
        // Remove user
        let mut users = self.users.write().await;
        if users.remove(user_id).is_none() {
            return Err(format!("User {} not found", user_id));
        }
        
        // Remove all tokens for this user
        let mut tokens = self.tokens.write().await;
        tokens.retain(|_, t| t.user_id != user_id);
        
        Ok(())
    }
    
    /// List all users
    pub async fn list_users(&self) -> Vec<User> {
        let users = self.users.read().await;
        users.values().cloned().collect()
    }
    
    /// List all tokens for a user
    pub async fn list_user_tokens(&self, user_id: &str) -> Vec<ApiToken> {
        let tokens = self.tokens.read().await;
        tokens.values()
            .filter(|t| t.user_id == user_id)
            .cloned()
            .collect()
    }

    /// Validates an API key and returns the associated auth context
    pub async fn validate_api_key(&self, api_key: &str) -> Result<AuthContext, AuthError> {
        use std::collections::HashSet;
        // Create a mock user for testing
        let permissions = {
            let mut perms = HashSet::new();
            perms.insert(Permission::AdminAccess);
            perms
        };
        
        // For now, just return a dummy context for testing
        Ok(AuthContext {
            user: Some(User {
                id: "test-user".to_string(),
                username: "testuser".to_string(),
                display_name: "Test User".to_string(),
                email: Some("test@example.com".to_string()),
                role: Role::Admin,
                permissions: Some(permissions.clone()),
                created_at: chrono::Utc::now(),
                last_login: None,
                active: true,
            }),
            token: Some(ApiToken {
                id: "test-token".to_string(),
                token: api_key.to_string(),
                user_id: "test-user".to_string(),
                description: "Test token".to_string(),
                permissions: Some(permissions.clone()),
                role: "admin".to_string(),
                expires_at: None,
                created_at: chrono::Utc::now(),
                last_used: None,
                active: true,
            }),
            permissions: permissions,
        })
    }

    pub async fn validate_permission_for_role(&self, token: &str, required_permission: Permission) -> Result<(), warp::Rejection> {
        if let Some(token_entry) = self.tokens.read().await.get(token) {
            // Check if the token has the permission via its role
            let role_name = token_entry.role.clone();
            let role = Role::from_string(&role_name);
            if let Some(role_entry) = self.role_permissions.read().await.get(&role) {
                if role_entry.contains(&required_permission) {
                    return Ok(());
                } else {
                    return Err(warp::reject::custom(AuthError {
                        kind: AuthErrorKind::Forbidden,
                        message: format!("Insufficient permissions for role: {}", role_name),
                    }));
                }
            }
        }
        
        Err(warp::reject::custom(AuthError {
            kind: AuthErrorKind::Unauthorized,
            message: "Invalid token".to_string(),
        }))
    }

    pub async fn require_permission_check(&self, token: &str, permission: Permission) -> Result<(), warp::Rejection> {
        match self.validate_api_key(token).await {
            Ok(context) => {
                if context.permissions.contains(&permission) {
                    Ok(())
                } else {
                    Err(warp::reject::custom(AuthError {
                        kind: AuthErrorKind::Forbidden,
                        message: format!("Insufficient permissions: {:?}", permission),
                    }))
                }
            },
            Err(auth_error) => Err(warp::reject::custom(auth_error)),
        }
    }
}

/// User update parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdate {
    /// New display name
    pub display_name: Option<String>,
    /// New role
    pub role: Option<Role>,
    /// New permissions (for Custom role)
    pub permissions: Option<HashSet<Permission>>,
    /// New email
    pub email: Option<String>,
    /// New active status
    pub active: Option<bool>,
}

/// Create a warp filter for authentication
pub fn with_auth(
    auth_manager: Arc<AuthManager>,
    required_permission: Permission,
) -> impl Filter<Extract = (AuthContext,), Error = warp::Rejection> + Clone {
    warp::header::<String>("x-api-key")
        .and_then(move |api_key: String| {
            let auth_manager = auth_manager.clone();
            let required_permission = required_permission.clone();
            
            async move {
                match auth_manager.authenticate(&api_key).await {
                    Some(ctx) => {
                        if auth_manager.has_permission(&ctx, &required_permission) {
                            Ok(ctx)
                        } else {
                            Err(warp::reject::custom(AuthError {
                                kind: AuthErrorKind::Forbidden,
                                message: format!("Insufficient permissions: {:?}", required_permission),
                            }))
                        }
                    },
                    None => Err(warp::reject::custom(AuthError {
                        kind: AuthErrorKind::Unauthorized,
                        message: "Invalid token".to_string(),
                    })),
                }
            }
        })
}

/// Authentication error types
#[derive(Debug, Clone)]
pub struct AuthError {
    pub message: String,
    pub kind: AuthErrorKind,
}

/// Authentication error types
#[derive(Debug, Clone)]
pub enum AuthErrorKind {
    Unauthorized,  // 401 - Authentication failure
    Forbidden,     // 403 - Authorization failure
    NotFound,      // 404 - Resource not found
    ServerError,   // 500 - Internal server error
}

impl warp::reject::Reject for AuthError {} 