//! Security module for MCP integration
//!
//! Handles authentication, authorization, and security policies

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, warn};

use crate::{Error, Result};

/// Authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// Token value
    pub token: String,
    /// User ID
    pub user_id: String,
    /// Username
    pub username: String,
    /// User role
    pub role: Role,
    /// Permissions
    pub permissions: Vec<Permission>,
    /// Token creation time
    pub _created_at: SystemTime,
    /// Token expiration time
    pub expires_at: Option<SystemTime>,
    /// Whether token is active
    pub is_active: bool,
}

impl AuthToken {
    /// Check if token is valid and not expired
    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }

        if let Some(expires_at) = self.expires_at {
            SystemTime::now() < expires_at
        } else {
            true
        }
    }

    /// Get remaining lifetime in seconds
    pub fn remaining_lifetime(&self) -> Option<u64> {
        if let Some(expires_at) = self.expires_at {
            expires_at
                .duration_since(SystemTime::now())
                .ok()
                .map(|d| d.as_secs())
        } else {
            None
        }
    }
}

/// User roles
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// System administrator with full privileges
    Admin,
    /// Service user for inter-service communication
    Service,
    /// Regular user with limited access
    User,
    /// Read-only access
    ReadOnly,
}

/// System permissions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    /// Read system information
    SystemRead,
    /// Modify system configuration
    SystemWrite,
    /// Manage services
    ServiceManage,
    /// Read service status
    ServiceRead,
    /// Access storage operations
    StorageAccess,
    /// Manage storage
    StorageManage,
    /// Access network configuration
    NetworkAccess,
    /// Manage network settings
    NetworkManage,
    /// Access user management
    UserManage,
    /// Read monitoring data
    MonitoringRead,
    /// Administrative operations
    AdminOperations,
}

impl Role {
    /// Get default permissions for a role
    pub fn default_permissions(&self) -> Vec<Permission> {
        match self {
            Role::Admin => vec![
                Permission::SystemRead,
                Permission::SystemWrite,
                Permission::ServiceManage,
                Permission::ServiceRead,
                Permission::StorageAccess,
                Permission::StorageManage,
                Permission::NetworkAccess,
                Permission::NetworkManage,
                Permission::UserManage,
                Permission::MonitoringRead,
                Permission::AdminOperations,
            ],
            Role::Service => vec![
                Permission::SystemRead,
                Permission::ServiceRead,
                Permission::StorageAccess,
                Permission::NetworkAccess,
                Permission::MonitoringRead,
            ],
            Role::User => vec![
                Permission::SystemRead,
                Permission::ServiceRead,
                Permission::StorageAccess,
                Permission::MonitoringRead,
            ],
            Role::ReadOnly => vec![
                Permission::SystemRead,
                Permission::ServiceRead,
                Permission::MonitoringRead,
            ],
        }
    }
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Token expiration time in seconds (None = no expiration)
    pub token_expiration: Option<u64>,
    /// Maximum number of active tokens per user
    pub max_tokens_per_user: usize,
    /// Whether to require secure password policies
    pub enforce_password_policy: bool,
    /// Minimum password length
    pub min_password_length: usize,
    /// Whether to enable audit logging
    pub enable_audit_logging: bool,
    /// Default role for new users
    pub default_role: Role,
    /// Whether authentication is required
    pub require_authentication: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            token_expiration: Some(3600), // 1 hour
            max_tokens_per_user: 5,
            enforce_password_policy: true,
            min_password_length: 8,
            enable_audit_logging: true,
            default_role: Role::User,
            require_authentication: true,
        }
    }
}

/// User credentials
#[derive(Debug, Clone)]
struct UserCredentials {
    user_id: String,
    username: String,
    password_hash: String,
    salt: String,
    role: Role,
    _created_at: SystemTime,
    _last_login: Option<SystemTime>,
    is_active: bool,
}

impl UserCredentials {
    /// Create new credentials with hashed password
    fn new(user_id: String, username: String, password: &str, role: Role) -> Result<Self> {
        let salt = generate_salt();
        let password_hash = hash_password(password, &salt)?;

        Ok(Self {
            user_id,
            username,
            password_hash,
            salt,
            role,
            _created_at: SystemTime::now(),
            _last_login: None,
            is_active: true,
        })
    }

    /// Verify password
    fn verify_password(&self, password: &str) -> Result<bool> {
        let computed_hash = hash_password(password, &self.salt)?;
        Ok(computed_hash == self.password_hash)
    }
}

/// Security manager for MCP operations
#[derive(Debug, Clone)]
pub struct SecurityManager {
    config: SecurityConfig,
    /// Active authentication tokens
    tokens: std::sync::Arc<std::sync::RwLock<HashMap<String, AuthToken>>>,
    /// User credentials storage
    users: std::sync::Arc<std::sync::RwLock<HashMap<String, UserCredentials>>>,
}

impl SecurityManager {
    /// Create a new security manager
    pub fn new(config: SecurityConfig) -> Self {
        let security_manager = Self {
            config: config.clone(),
            tokens: std::sync::Arc::new(std::sync::RwLock::new(HashMap::new())),
            users: std::sync::Arc::new(std::sync::RwLock::new(HashMap::new())),
        };

        // Create default admin user if authentication is enabled
        if config.require_authentication {
            let _ = security_manager.create_default_admin();
        }

        security_manager
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(SecurityConfig::default())
    }

    /// Initialize the security manager
    pub async fn initialize(&self) -> Result<()> {
        debug!("Initializing security manager");
        debug!(
            "Authentication required: {}",
            self.config.require_authentication
        );
        debug!(
            "Token expiration: {:?} seconds",
            self.config.token_expiration
        );
        debug!(
            "Password policy enforced: {}",
            self.config.enforce_password_policy
        );

        // Clean up expired tokens
        self.cleanup_expired_tokens().await?;

        debug!("Security manager initialized successfully");
        Ok(())
    }

    /// Create default admin user
    fn create_default_admin(&self) -> Result<()> {
        let mut users = self
            .users
            .write()
            .map_err(|_| Error::internal("Failed to acquire users lock".to_string()))?;

        // Only create if no admin exists
        if !users.values().any(|u| u.role == Role::Admin) {
            // Generate secure random password
            let secure_password = generate_secure_password();
            let admin_credentials = UserCredentials::new(
                "admin".to_string(),
                "Administrator".to_string(),
                &secure_password,
                Role::Admin,
            )?;

            users.insert("admin".to_string(), admin_credentials);
            warn!("🔐 Created default admin user with generated password");
            warn!("🔑 Admin password: {}", secure_password);
            warn!("⚠️  SECURITY: Change this password immediately after first login!");
            warn!("💡 Use: nestgate user change-password admin");
        }

        Ok(())
    }

    /// Register a new user
    pub async fn register_user(
        &self,
        user_id: String,
        username: String,
        password: &str,
        role: Option<Role>,
    ) -> Result<()> {
        if self.config.enforce_password_policy && password.len() < self.config.min_password_length {
            return Err(Error::authentication(format!(
                "Password must be at least {} characters",
                self.config.min_password_length
            )));
        }

        let role = role.unwrap_or_else(|| self.config.default_role.clone());
        let credentials = UserCredentials::new(user_id.clone(), username, password, role)?;

        let mut users = self
            .users
            .write()
            .map_err(|_| Error::internal("Failed to acquire users lock".to_string()))?;

        if users.contains_key(&user_id) {
            return Err(Error::authentication("User already exists".to_string()));
        }

        users.insert(user_id.clone(), credentials);

        if self.config.enable_audit_logging {
            debug!("User registered: {}", user_id);
        }

        Ok(())
    }

    /// Authenticate user and create token
    pub async fn authenticate(&self, user_id: &str, password: &str) -> Result<AuthToken> {
        if !self.config.require_authentication {
            // Return a basic token for development mode
            return Ok(AuthToken {
                token: generate_token(),
                user_id: user_id.to_string(),
                username: user_id.to_string(),
                role: Role::Admin,
                permissions: Role::Admin.default_permissions(),
                _created_at: SystemTime::now(),
                expires_at: None,
                is_active: true,
            });
        }

        let users = self
            .users
            .read()
            .map_err(|_| Error::internal("Failed to acquire users lock".to_string()))?;

        let credentials = users
            .get(user_id)
            .ok_or_else(|| Error::authentication("Invalid credentials".to_string()))?;

        if !credentials.is_active {
            return Err(Error::authentication("Account is disabled".to_string()));
        }

        if !credentials.verify_password(password)? {
            if self.config.enable_audit_logging {
                warn!("Authentication failed for user: {}", user_id);
            }
            return Err(Error::authentication("Invalid credentials".to_string()));
        }

        // Check token limit
        let tokens = self
            .tokens
            .read()
            .map_err(|_| Error::internal("Failed to acquire tokens lock".to_string()))?;
        let user_token_count = tokens
            .values()
            .filter(|t| t.user_id == user_id && t.is_valid())
            .count();

        if user_token_count >= self.config.max_tokens_per_user {
            return Err(Error::authentication(
                "Maximum number of active tokens reached".to_string(),
            ));
        }
        drop(tokens);

        // Create new token
        let token = AuthToken {
            token: generate_token(),
            user_id: credentials.user_id.clone(),
            username: credentials.username.clone(),
            role: credentials.role.clone(),
            permissions: credentials.role.default_permissions(),
            _created_at: SystemTime::now(),
            expires_at: self
                .config
                .token_expiration
                .map(|exp| SystemTime::now() + Duration::from_secs(exp)),
            is_active: true,
        };

        // Store token
        let mut tokens = self
            .tokens
            .write()
            .map_err(|_| Error::internal("Failed to acquire tokens lock".to_string()))?;
        tokens.insert(token.token.clone(), token.clone());

        if self.config.enable_audit_logging {
            debug!("User authenticated: {}", user_id);
        }

        Ok(token)
    }

    /// Validate authentication token
    pub async fn validate_token(&self, token_value: &str) -> Result<AuthToken> {
        if !self.config.require_authentication {
            // Return a basic valid token for development mode
            return Ok(AuthToken {
                token: token_value.to_string(),
                user_id: "system".to_string(),
                username: "System".to_string(),
                role: Role::Admin,
                permissions: Role::Admin.default_permissions(),
                _created_at: SystemTime::now(),
                expires_at: None,
                is_active: true,
            });
        }

        let tokens = self
            .tokens
            .read()
            .map_err(|_| Error::internal("Failed to acquire tokens lock".to_string()))?;

        let token = tokens
            .get(token_value)
            .ok_or_else(|| Error::authentication("Invalid token".to_string()))?;

        if !token.is_valid() {
            return Err(Error::authentication(
                "Token expired or inactive".to_string(),
            ));
        }

        Ok(token.clone())
    }

    /// Validate authentication credentials (legacy method)
    pub async fn validate_credentials(&self, token: &str) -> Result<bool> {
        match self.validate_token(token).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Check authorization for an operation
    pub async fn check_authorization(&self, user_id: &str, operation: &str) -> Result<bool> {
        if !self.config.require_authentication {
            return Ok(true);
        }

        // Parse operation into permission
        let required_permission = match operation.to_lowercase().as_str() {
            "system:read" => Permission::SystemRead,
            "system:write" => Permission::SystemWrite,
            "service:manage" => Permission::ServiceManage,
            "service:read" => Permission::ServiceRead,
            "storage:access" => Permission::StorageAccess,
            "storage:manage" => Permission::StorageManage,
            "network:access" => Permission::NetworkAccess,
            "network:manage" => Permission::NetworkManage,
            "user:manage" => Permission::UserManage,
            "monitoring:read" => Permission::MonitoringRead,
            "admin:operations" => Permission::AdminOperations,
            _ => {
                warn!("Unknown operation requested: {}", operation);
                return Ok(false);
            }
        };

        // Find active token for user
        let tokens = self
            .tokens
            .read()
            .map_err(|_| Error::internal("Failed to acquire tokens lock".to_string()))?;

        let user_token = tokens
            .values()
            .find(|t| t.user_id == user_id && t.is_valid());

        match user_token {
            Some(token) => Ok(token.permissions.contains(&required_permission)),
            None => Ok(false),
        }
    }

    /// Revoke a specific token
    pub async fn revoke_token(&self, token_value: &str) -> Result<()> {
        let mut tokens = self
            .tokens
            .write()
            .map_err(|_| Error::internal("Failed to acquire tokens lock".to_string()))?;

        if let Some(token) = tokens.get_mut(token_value) {
            token.is_active = false;
            if self.config.enable_audit_logging {
                debug!("Token revoked: {}", token.user_id);
            }
        }

        Ok(())
    }

    /// Revoke all tokens for a user
    pub async fn revoke_user_tokens(&self, user_id: &str) -> Result<()> {
        let mut tokens = self
            .tokens
            .write()
            .map_err(|_| Error::internal("Failed to acquire tokens lock".to_string()))?;

        let mut revoked_count = 0;
        for token in tokens.values_mut() {
            if token.user_id == user_id && token.is_active {
                token.is_active = false;
                revoked_count += 1;
            }
        }

        if self.config.enable_audit_logging {
            debug!("Revoked {} tokens for user: {}", revoked_count, user_id);
        }

        Ok(())
    }

    /// Clean up expired tokens
    pub async fn cleanup_expired_tokens(&self) -> Result<()> {
        let mut tokens = self
            .tokens
            .write()
            .map_err(|_| Error::internal("Failed to acquire tokens lock".to_string()))?;

        let initial_count = tokens.len();
        tokens.retain(|_, token| token.is_valid());
        let cleaned_count = initial_count - tokens.len();

        if cleaned_count > 0 && self.config.enable_audit_logging {
            debug!("Cleaned up {} expired tokens", cleaned_count);
        }

        Ok(())
    }

    /// Get active token count for a user
    pub async fn get_user_token_count(&self, user_id: &str) -> Result<usize> {
        let tokens = self
            .tokens
            .read()
            .map_err(|_| Error::internal("Failed to acquire tokens lock".to_string()))?;

        let count = tokens
            .values()
            .filter(|t| t.user_id == user_id && t.is_valid())
            .count();

        Ok(count)
    }

    /// Get security statistics
    pub async fn get_security_stats(&self) -> Result<SecurityStats> {
        let tokens = self
            .tokens
            .read()
            .map_err(|_| Error::internal("Failed to acquire tokens lock".to_string()))?;
        let users = self
            .users
            .read()
            .map_err(|_| Error::internal("Failed to acquire users lock".to_string()))?;

        let active_tokens = tokens.values().filter(|t| t.is_valid()).count();
        let expired_tokens = tokens.len() - active_tokens;
        let total_users = users.len();
        let active_users = users.values().filter(|u| u.is_active).count();

        Ok(SecurityStats {
            total_users,
            active_users,
            active_tokens,
            expired_tokens,
            authentication_required: self.config.require_authentication,
        })
    }
}

/// Security statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStats {
    pub total_users: usize,
    pub active_users: usize,
    pub active_tokens: usize,
    pub expired_tokens: usize,
    pub authentication_required: bool,
}

/// Generate a random salt for password hashing
fn generate_salt() -> String {
    let mut rng = thread_rng();
    (0..32)
        .map(|_| format!("{:02x}", rng.gen::<u8>()))
        .collect()
}

/// Hash password with salt
fn hash_password(password: &str, salt: &str) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());
    Ok(format!("{:x}", hasher.finalize()))
}

/// Generate a secure random token
fn generate_token() -> String {
    let mut rng = thread_rng();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    format!("nestgate-{:x}-{:016x}", timestamp, rng.gen::<u64>())
}

/// Generate a secure random password
fn generate_secure_password() -> String {
    let mut rng = thread_rng();
    (0..32)
        .map(|_| format!("{:02x}", rng.gen::<u8>()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_manager_creation() {
        let manager = SecurityManager::with_defaults();
        assert!(manager.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_user_registration_and_authentication() {
        let manager = SecurityManager::with_defaults();
        manager.initialize().await.unwrap();

        // Register user
        manager
            .register_user(
                "test_user".to_string(),
                "Test User".to_string(),
                "test_password_123",
                Some(Role::User),
            )
            .await
            .unwrap();

        // Authenticate
        let token = manager
            .authenticate("test_user", "test_password_123")
            .await
            .unwrap();
        assert_eq!(token.user_id, "test_user");
        assert_eq!(token.role, Role::User);

        // Validate token
        let validated = manager.validate_token(&token.token).await.unwrap();
        assert_eq!(validated.user_id, "test_user");
    }

    #[tokio::test]
    async fn test_authorization() {
        let manager = SecurityManager::with_defaults();
        manager.initialize().await.unwrap();

        // Register user with limited permissions
        manager
            .register_user(
                "limited_user".to_string(),
                "Limited User".to_string(),
                "password123",
                Some(Role::ReadOnly),
            )
            .await
            .unwrap();

        // Authenticate to get a token (needed for authorization checks)
        let _token = manager
            .authenticate("limited_user", "password123")
            .await
            .unwrap();

        // Should not have write permissions
        let has_write = manager
            .check_authorization("limited_user", "system:write")
            .await
            .unwrap();
        assert!(!has_write);

        // Should have read permissions
        let has_read = manager
            .check_authorization("limited_user", "system:read")
            .await
            .unwrap();
        assert!(has_read);
    }

    #[tokio::test]
    async fn test_token_expiration() {
        let config = SecurityConfig {
            token_expiration: Some(1), // 1 second expiration
            ..Default::default()
        };

        let manager = SecurityManager::new(config);
        manager.initialize().await.unwrap();

        manager
            .register_user(
                "expire_user".to_string(),
                "Expire User".to_string(),
                "password123",
                Some(Role::User),
            )
            .await
            .unwrap();

        let token = manager
            .authenticate("expire_user", "password123")
            .await
            .unwrap();

        // Token should be valid initially
        assert!(manager.validate_token(&token.token).await.is_ok());

        // Wait for expiration
        tokio::time::sleep(nestgate_core::constants::timeouts::retry_interval()).await;

        // Token should be expired now
        assert!(manager.validate_token(&token.token).await.is_err());
    }
}
