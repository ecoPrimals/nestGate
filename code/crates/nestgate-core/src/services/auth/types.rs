// **AUTHENTICATION TYPES**
//! Type definitions and data structures.
// Core types and data structures for the authentication service.
// Extracted from the monolithic auth.rs file.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub salt: String,
    pub created_at: SystemTime,
    pub last_login: Option<SystemTime>,
    pub failed_attempts: u32,
    pub locked_until: Option<SystemTime>,
    pub roles: Vec<String>,
    pub mfa_enabled: bool,
    pub mfa_secret: Option<String>,
    pub backup_codes: Vec<String>,
    pub oauth_providers: HashMap<String, String>,
    pub profile: UserProfile,
}
/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub timezone: Option<String>,
    pub language: String,
    pub preferences: HashMap<String, String>,
}
impl Default for UserProfile {
    fn default() -> Self {
        Self {
            display_name: String::new(),
            avatar_url: None,
            timezone: None,
            language: "en".to_string(),
            preferences: HashMap::new(),
        }
    }
}

/// Authentication session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub last_activity: SystemTime,
    pub ip_address: String,
    pub user_agent: String,
    pub is_active: bool,
    pub device_info: DeviceInfo,
}
/// Device information for sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_type: String,
    pub os: String,
    pub browser: String,
    pub is_mobile: bool,
}
impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            device_type: "unknown".to_string(),
            os: "unknown".to_string(),
            browser: "unknown".to_string(),
            is_mobile: false,
        }
    }
}

/// OAuth provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
    pub scopes: Vec<String>,
    pub enabled: bool,
}
/// JWT token claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String, // Subject (user ID)
    pub exp: u64,    // Expiration time
    pub iat: u64,    // Issued at
    pub roles: Vec<String>,
    pub session_id: String,
}
/// Authentication request
#[derive(Debug, Clone, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
    pub mfa_code: Option<String>,
    pub device_info: Option<DeviceInfo>,
}
/// Authentication response
#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub token: Option<String>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub expires_at: Option<SystemTime>,
    pub mfa_required: bool,
    pub error: Option<String>,
}
/// User registration request
#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub profile: UserProfile,
}
/// Password change request
#[derive(Debug, Clone, Deserialize)]
pub struct PasswordChangeRequest {
    pub user_id: String,
    pub current_password: String,
    pub new_password: String,
}
/// MFA setup request
#[derive(Debug, Clone, Deserialize)]
pub struct MfaSetupRequest {
    pub user_id: String,
    pub method: MfaMethod,
}
/// MFA methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    TOTP,
    SMS,
    Email,
    BackupCode,
}
/// Authentication event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthEvent {
    Login { user_id: String, ip_address: String },
    Logout { user_id: String, session_id: String },
    FailedLogin { username: String, ip_address: String },
    AccountLocked { user_id: String },
    PasswordChanged { user_id: String },
    MfaEnabled { user_id: String },
    MfaDisabled { user_id: String },
}
/// Authentication statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStats {
    pub total_users: u64,
    pub active_sessions: u64,
    pub login_attempts: u64,
    pub failed_logins: u64,
    pub locked_accounts: u64,
    pub mfa_enabled_users: u64,
    pub oauth_users: HashMap<String, u64>,
    pub last_updated: SystemTime,
}
impl Default for AuthStats {
    fn default() -> Self {
        Self {
            total_users: 0,
            active_sessions: 0,
            login_attempts: 0,
            failed_logins: 0,
            locked_accounts: 0,
            mfa_enabled_users: 0,
            oauth_users: HashMap::new(),
            last_updated: SystemTime::now(),
        }
    }
}

/// Authentication error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthError {
    InvalidCredentials,
    AccountLocked,
    MfaRequired,
    InvalidMfaCode,
    TokenExpired,
    SessionExpired,
    InsufficientPermissions,
    UserNotFound,
    UserAlreadyExists,
    WeakPassword,
    InvalidEmail,
}
impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidCredentials => write!(f, "Invalid username or password"),
            AuthError::AccountLocked => write!(f, "Account is temporarily locked"),
            AuthError::MfaRequired => write!(f, "Multi-factor authentication required"),
            AuthError::InvalidMfaCode => write!(f, "Invalid MFA code"),
            AuthError::TokenExpired => write!(f, "Authentication token has expired"),
            AuthError::SessionExpired => write!(f, "Session has expired"),
            AuthError::InsufficientPermissions => write!(f, "Insufficient permissions"),
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::UserAlreadyExists => write!(f, "User already exists"),
            AuthError::WeakPassword => write!(f, "Password does not meet security requirements"),
            AuthError::InvalidEmail => write!(f, "Invalid email address"),
        }
    }
}

impl std::error::Error for AuthError {} 