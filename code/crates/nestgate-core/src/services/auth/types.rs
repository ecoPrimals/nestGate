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
/// User
pub struct User {
    /// Unique identifier
    pub id: String,
    /// Username
    pub username: String,
    /// Email
    pub email: String,
    /// Password Hash
    pub password_hash: String,
    /// Salt
    pub salt: String,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Last Login
    pub last_login: Option<SystemTime>,
    /// Failed Attempts
    pub failed_attempts: u32,
    /// Locked Until
    pub locked_until: Option<SystemTime>,
    /// Roles
    pub roles: Vec<String>,
    /// Mfa Enabled
    pub mfa_enabled: bool,
    /// Mfa Secret
    pub mfa_secret: Option<String>,
    /// Backup Codes
    pub backup_codes: Vec<String>,
    /// Oauth Providers
    pub oauth_providers: HashMap<String, String>,
    /// Profile
    pub profile: UserProfile,
}
/// User profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Userprofile
pub struct UserProfile {
    /// Display name
    pub display_name: String,
    /// Avatar Url
    pub avatar_url: Option<String>,
    /// Timezone
    pub timezone: Option<String>,
    /// Language
    pub language: String,
    /// Preferences
    pub preferences: HashMap<String, String>,
}
impl Default for UserProfile {
    /// Returns the default instance
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
/// Session
pub struct Session {
    /// Session identifier
    pub session_id: String,
    /// User identifier
    pub user_id: String,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Expires At
    pub expires_at: SystemTime,
    /// Last Activity
    pub last_activity: SystemTime,
    /// Ip Address
    pub ip_address: String,
    /// User Agent
    pub user_agent: String,
    /// Whether active
    pub is_active: bool,
    /// Device Info
    pub device_info: DeviceInfo,
}
/// Device information for sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Deviceinfo
pub struct DeviceInfo {
    /// Device Type
    pub device_type: String,
    /// Os
    pub os: String,
    /// Browser
    pub browser: String,
    /// Whether mobile
    pub is_mobile: bool,
}
impl Default for DeviceInfo {
    /// Returns the default instance
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
/// Oauthprovider
pub struct OAuthProvider {
    /// Name
    pub name: String,
    /// Client identifier
    pub client_id: String,
    /// Client Secret
    pub client_secret: String,
    /// Auth Url
    pub auth_url: String,
    /// Token Url
    pub token_url: String,
    /// User Info Url
    pub user_info_url: String,
    /// Scopes
    pub scopes: Vec<String>,
    /// Whether this feature is enabled
    pub enabled: bool,
}
/// JWT token claims
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tokenclaims
pub struct TokenClaims {
    /// Sub
    pub sub: String, // Subject (user ID)
    /// Exp
    pub exp: u64,    // Expiration time
    /// Iat
    pub iat: u64,    // Issued at
    /// Roles
    pub roles: Vec<String>,
    /// Session identifier
    pub session_id: String,
}
/// Authentication request
#[derive(Debug, Clone, Deserialize)]
/// Request parameters for Auth operation
pub struct AuthRequest {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// Mfa Code
    pub mfa_code: Option<String>,
    /// Device Info
    pub device_info: Option<DeviceInfo>,
}
/// Authentication response
#[derive(Debug, Clone, Serialize)]
/// Response data for Auth operation
pub struct AuthResponse {
    /// Success
    pub success: bool,
    /// Token
    pub token: Option<String>,
    /// User identifier
    pub user_id: Option<String>,
    /// Session identifier
    pub session_id: Option<String>,
    /// Expires At
    pub expires_at: Option<SystemTime>,
    /// Mfa Required
    pub mfa_required: bool,
    /// Error
    pub error: Option<String>,
}
/// User registration request
#[derive(Debug, Clone, Deserialize)]
/// Request parameters for Register operation
pub struct RegisterRequest {
    /// Username
    pub username: String,
    /// Email
    pub email: String,
    /// Password
    pub password: String,
    /// Profile
    pub profile: UserProfile,
}
/// Password change request
#[derive(Debug, Clone, Deserialize)]
/// Request parameters for PasswordChange operation
pub struct PasswordChangeRequest {
    /// User identifier
    pub user_id: String,
    /// Current Password
    pub current_password: String,
    /// New Password
    pub new_password: String,
}
/// MFA setup request
#[derive(Debug, Clone, Deserialize)]
/// Request parameters for MfaSetup operation
pub struct MfaSetupRequest {
    /// User identifier
    pub user_id: String,
    /// Method
    pub method: MfaMethod,
}
/// MFA methods
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Mfamethod
pub enum MfaMethod {
    /// Totp
    TOTP,
    /// Sms
    SMS,
    /// Email
    Email,
    /// Backupcode
    BackupCode,
}
/// Authentication event types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authevent
pub enum AuthEvent {
    /// Login
    Login { user_id: String, ip_address: String },
    /// Logout
    Logout { user_id: String, session_id: String },
    /// Failedlogin
    FailedLogin { username: String, ip_address: String },
    /// Accountlocked
    AccountLocked { user_id: String },
    /// Passwordchanged
    PasswordChanged { user_id: String },
    /// Mfaenabled
    MfaEnabled { user_id: String },
    /// Mfadisabled
    MfaDisabled { user_id: String },
}
/// Authentication statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authstats
pub struct AuthStats {
    /// Total Users
    pub total_users: u64,
    /// Active Sessions
    pub active_sessions: u64,
    /// Login Attempts
    pub login_attempts: u64,
    /// Failed Logins
    pub failed_logins: u64,
    /// Locked Accounts
    pub locked_accounts: u64,
    /// Mfa Enabled Users
    pub mfa_enabled_users: u64,
    /// Oauth Users
    pub oauth_users: HashMap<String, u64>,
    /// Last Updated
    pub last_updated: SystemTime,
}
impl Default for AuthStats {
    /// Returns the default instance
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
/// Errors that can occur during Auth operations
pub enum AuthError {
    /// Invalidcredentials
    InvalidCredentials,
    /// Accountlocked
    AccountLocked,
    /// Mfarequired
    MfaRequired,
    /// Invalidmfacode
    InvalidMfaCode,
    /// Tokenexpired
    TokenExpired,
    /// Sessionexpired
    SessionExpired,
    /// Insufficientpermissions
    InsufficientPermissions,
    /// Usernotfound
    UserNotFound,
    /// Useralreadyexists
    UserAlreadyExists,
    /// Weakpassword
    WeakPassword,
    InvalidEmail,
}
impl std::fmt::Display for AuthError {
    /// Fmt
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