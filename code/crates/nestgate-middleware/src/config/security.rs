/// Middleware Security Configuration
/// All security-related middleware settings (auth, CORS, rate limiting, etc.)
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// ==================== SECURITY SETTINGS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiddlewareSecuritySettings {
    /// Authentication middleware config
    pub authentication: AuthenticationMiddlewareSettings,
    /// Authorization middleware config
    pub authorization: AuthorizationMiddlewareSettings,
    /// CORS configuration
    pub cors: CorsSettings,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitingSettings,
    /// Security headers
    pub security_headers: SecurityHeadersSettings,
    /// Input sanitization
    pub sanitization: SanitizationSettings,
}

// ==================== AUTHENTICATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationMiddlewareSettings {
    /// Enable authentication
    pub enabled: bool,
    /// Authentication providers
    pub providers: Vec<AuthProvider>,
    /// JWT configuration
    pub jwt: JwtSettings,
    /// Session configuration
    pub session: SessionSettings,
    /// Multi-factor authentication
    pub mfa: MfaSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthProvider {
    /// Local authentication
    Local(LocalAuthSettings),
    /// OAuth provider
    OAuth(OAuthSettings),
    /// LDAP provider
    Ldap(LdapSettings),
    /// SAML provider
    Saml(SamlSettings),
    /// Custom provider
    Custom(CustomAuthSettings),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalAuthSettings {
    /// Password policy
    pub password_policy: PasswordPolicy,
    /// User storage
    pub user_storage: UserStorageSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    /// Minimum length
    pub min_length: usize,
    /// Require uppercase
    pub require_uppercase: bool,
    /// Require lowercase
    pub require_lowercase: bool,
    /// Require numbers
    pub require_numbers: bool,
    /// Require special characters
    pub require_special: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStorageSettings {
    /// Storage backend
    pub backend: UserStorageBackend,
    /// Connection settings
    pub connection: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStorageBackend {
    Database,
    File,
    Ldap,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthSettings {
    /// Provider name
    pub provider: String,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Authorization URL
    pub auth_url: String,
    /// Token URL
    pub token_url: String,
    /// Scopes
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdapSettings {
    /// Server URL
    pub server_url: String,
    /// Bind DN
    pub bind_dn: String,
    /// Bind password
    pub bind_password: String,
    /// User search base
    pub user_search_base: String,
    /// User search filter
    pub user_search_filter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlSettings {
    /// Identity provider URL
    pub idp_url: String,
    /// Service provider URL
    pub sp_url: String,
    /// Certificate path
    pub certificate_path: PathBuf,
    /// Private key path
    pub private_key_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomAuthSettings {
    /// Custom provider configuration
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtSettings {
    /// Secret key
    pub secret: String,
    /// Algorithm
    pub algorithm: JwtAlgorithm,
    /// Expiration time
    pub expiration: Duration,
    /// Issuer
    pub issuer: String,
    /// Audience
    pub audience: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JwtAlgorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSettings {
    /// Session storage
    pub storage: SessionStorage,
    /// Session timeout
    pub timeout: Duration,
    /// Cookie settings
    pub cookie: CookieSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStorage {
    Memory,
    Redis(RedisSettings),
    Database,
    File,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisSettings {
    /// Redis URL
    pub url: String,
    /// Key prefix
    pub key_prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieSettings {
    /// Cookie name
    pub name: String,
    /// Domain
    pub domain: Option<String>,
    /// Path
    pub path: String,
    /// Secure flag
    pub secure: bool,
    /// HTTP only flag
    pub http_only: bool,
    /// Same site policy
    pub same_site: SameSitePolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SameSitePolicy {
    Strict,
    Lax,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaSettings {
    /// Enable MFA
    pub enabled: bool,
    /// MFA providers
    pub providers: Vec<MfaProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaProvider {
    Totp,
    Sms,
    Email,
    Hardware,
}

// ==================== AUTHORIZATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationMiddlewareSettings {
    /// Enable authorization
    pub enabled: bool,
    /// Authorization model
    pub model: AuthorizationModel,
    /// Policy definitions
    pub policies: Vec<AuthorizationPolicy>,
    /// Default policy
    pub default_policy: DefaultPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorizationModel {
    Rbac,
    Abac,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationPolicy {
    /// Policy name
    pub name: String,
    /// Policy rules
    pub rules: Vec<PolicyRule>,
    /// Policy effect
    pub effect: PolicyEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Resource pattern
    pub resource: String,
    /// Action pattern
    pub action: String,
    /// Condition
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyEffect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DefaultPolicy {
    Allow,
    Deny,
}

// ==================== CORS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsSettings {
    /// Enable CORS
    pub enabled: bool,
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
    /// Exposed headers
    pub exposed_headers: Vec<String>,
    /// Allow credentials
    pub allow_credentials: bool,
    /// Max age
    pub max_age: Option<Duration>,
}

// ==================== RATE LIMITING ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingSettings {
    /// Enable rate limiting
    pub enabled: bool,
    /// Rate limit rules
    pub rules: Vec<RateLimitRule>,
    /// Default rate limit
    pub default_limit: RateLimit,
    /// Storage backend
    pub storage: RateLimitStorage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitRule {
    /// Rule pattern
    pub pattern: String,
    /// Rate limit
    pub limit: RateLimit,
    /// Scope
    pub scope: RateLimitScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// Requests per window
    pub requests: u32,
    /// Time window
    pub window: Duration,
    /// Burst capacity
    pub burst: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateLimitScope {
    Global,
    PerIp,
    PerUser,
    PerEndpoint,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateLimitStorage {
    Memory,
    Redis(RedisSettings),
    Database,
}

// ==================== SECURITY HEADERS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeadersSettings {
    /// Enable security headers
    pub enabled: bool,
    /// Content Security Policy
    pub csp: Option<String>,
    /// Strict Transport Security
    pub hsts: Option<HstsSettings>,
    /// X-Frame-Options
    pub frame_options: Option<FrameOptions>,
    /// X-Content-Type-Options
    pub content_type_options: bool,
    /// Custom security headers
    pub custom_headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HstsSettings {
    /// Max age
    pub max_age: Duration,
    /// Include subdomains
    pub include_subdomains: bool,
    /// Preload
    pub preload: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrameOptions {
    Deny,
    SameOrigin,
    AllowFrom(String),
}

// ==================== SANITIZATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanitizationSettings {
    /// Enable input sanitization
    pub enabled: bool,
    /// HTML sanitization
    pub html: HtmlSanitizationSettings,
    /// SQL injection prevention
    pub sql_injection: SqlInjectionSettings,
    /// XSS prevention
    pub xss: XssSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlSanitizationSettings {
    /// Enable HTML sanitization
    pub enabled: bool,
    /// Allowed tags
    pub allowed_tags: Vec<String>,
    /// Allowed attributes
    pub allowed_attributes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlInjectionSettings {
    /// Enable SQL injection prevention
    pub enabled: bool,
    /// Parameterized queries only
    pub parameterized_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XssSettings {
    /// Enable XSS prevention
    pub enabled: bool,
    /// Input encoding
    pub input_encoding: bool,
    /// Output encoding
    pub output_encoding: bool,
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for MiddlewareSecuritySettings {
    fn default() -> Self {
        Self {
            authentication: AuthenticationMiddlewareSettings::default(),
            authorization: AuthorizationMiddlewareSettings::default(),
            cors: CorsSettings::default(),
            rate_limiting: RateLimitingSettings::default(),
            security_headers: SecurityHeadersSettings::default(),
            sanitization: SanitizationSettings::default(),
        }
    }
}

impl MiddlewareSecuritySettings {
    /// Development security settings
    pub fn development() -> Self {
        Self {
            cors: CorsSettings {
                enabled: true,
                allowed_origins: vec![
                    format!("http://{}:3000", "localhost".to_string()),
                    "http://localhost:8080".to_string(),
                ],
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                ],
                allow_credentials: true,
                ..Default::default()
            },
            rate_limiting: RateLimitingSettings {
                enabled: false, // Disabled for development
                ..Default::default()
            },
            authentication: AuthenticationMiddlewareSettings {
                enabled: false, // Simplified for development
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Production security settings
    pub fn production() -> Self {
        Self {
            authentication: AuthenticationMiddlewareSettings {
                enabled: true,
                ..Default::default()
            },
            authorization: AuthorizationMiddlewareSettings {
                enabled: true,
                ..Default::default()
            },
            cors: CorsSettings {
                enabled: true,
                allowed_origins: vec![], // Must be configured explicitly in production
                allow_credentials: false,
                ..Default::default()
            },
            rate_limiting: RateLimitingSettings {
                enabled: true,
                default_limit: RateLimit {
                    requests: 1000,
                    window: Duration::from_secs(3600),
                    burst: Some(100),
                },
                ..Default::default()
            },
            security_headers: SecurityHeadersSettings {
                enabled: true,
                content_type_options: true,
                ..Default::default()
            },
            sanitization: SanitizationSettings {
                enabled: true,
                ..Default::default()
            },
        }
    }
}

// Additional default implementations for nested structures
impl Default for AuthenticationMiddlewareSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            providers: Vec::new(),
            jwt: JwtSettings::default(),
            session: SessionSettings::default(),
            mfa: MfaSettings::default(),
        }
    }
}

impl Default for JwtSettings {
    fn default() -> Self {
        Self {
            secret: "change-me-in-production".to_string(),
            algorithm: JwtAlgorithm::HS256,
            expiration: Duration::from_secs(3600),
            issuer: "nestgate".to_string(),
            audience: vec!["nestgate".to_string()],
        }
    }
}

impl Default for SessionSettings {
    fn default() -> Self {
        Self {
            storage: SessionStorage::Memory,
            timeout: Duration::from_secs(3600),
            cookie: CookieSettings::default(),
        }
    }
}

impl Default for CookieSettings {
    fn default() -> Self {
        Self {
            name: "nestgate_session".to_string(),
            domain: None,
            path: "/".to_string(),
            secure: false,
            http_only: true,
            same_site: SameSitePolicy::Lax,
        }
    }
}

impl Default for MfaSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            providers: Vec::new(),
        }
    }
}

impl Default for AuthorizationMiddlewareSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            model: AuthorizationModel::Rbac,
            policies: Vec::new(),
            default_policy: DefaultPolicy::Deny,
        }
    }
}

impl Default for CorsSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_origins: Vec::new(),
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
            exposed_headers: Vec::new(),
            allow_credentials: false,
            max_age: Some(Duration::from_secs(86400)),
        }
    }
}

impl Default for RateLimitingSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            rules: Vec::new(),
            default_limit: RateLimit {
                requests: 100,
                window: Duration::from_secs(60),
                burst: Some(10),
            },
            storage: RateLimitStorage::Memory,
        }
    }
}

impl Default for SecurityHeadersSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            csp: None,
            hsts: None,
            frame_options: Some(FrameOptions::SameOrigin),
            content_type_options: false,
            custom_headers: HashMap::new(),
        }
    }
}

impl Default for SanitizationSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            html: HtmlSanitizationSettings::default(),
            sql_injection: SqlInjectionSettings::default(),
            xss: XssSettings::default(),
        }
    }
}

impl Default for HtmlSanitizationSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            allowed_tags: vec!["p".to_string(), "br".to_string(), "strong".to_string()],
            allowed_attributes: vec!["class".to_string()],
        }
    }
}

impl Default for SqlInjectionSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            parameterized_only: true,
        }
    }
}

impl Default for XssSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            input_encoding: true,
            output_encoding: true,
        }
    }
}
