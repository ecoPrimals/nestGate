use serde::{Deserialize, Serialize};

/// Authentication methods supported
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authmethod
pub enum AuthMethod {
    /// Kerberos
    Kerberos,
    /// Ldap
    Ldap,
    /// Activedirectory
    ActiveDirectory,
    /// Local
    Local,
    /// Certificate
    Certificate,
    /// Token
    Token,
}
/// Authentication credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authcredentials
pub struct AuthCredentials {
    /// Optional API key.
    pub api_key: Option<String>,
    /// Optional OAuth2 client ID.
    pub client_id: Option<String>,
    /// Optional OAuth2 client secret.
    pub client_secret: Option<String>,
}
/// Authentication configuration for a provider (Enhanced with proven patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Auth
pub struct AuthConfig {
    /// Method
    pub method: AuthMethod,
    /// Username
    pub username: Option<String>,
    /// Password
    pub password: Option<String>,
    /// Certificate Path
    pub certificate_path: Option<String>,
    /// Key Path
    pub key_path: Option<String>,
    /// Token
    pub token: Option<String>,
    /// Realm
    pub realm: Option<String>,
}
/// TLS configuration for secure connections (Enhanced with proven patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Tls
pub struct TlsConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Certificate Path
    pub certificate_path: Option<String>,
    /// Key Path
    pub key_path: Option<String>,
    /// Ca Path
    pub ca_path: Option<String>,
    /// Verify Peer
    pub verify_peer: bool,
    /// Verify Hostname
    pub verify_hostname: bool,
    /// Cipher Suites
    pub cipher_suites: Option<Vec<String>>,
}
