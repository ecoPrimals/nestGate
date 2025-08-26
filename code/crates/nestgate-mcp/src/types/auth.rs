
use serde::{Deserialize, Serialize};

/// Authentication methods supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    Kerberos,
    Ldap,
    ActiveDirectory,
    Local,
    Certificate,
    Token,
}

/// Authentication credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct AuthConfig {
    pub method: AuthMethod,
    pub username: Option<String>,
    pub password: Option<String>,
    pub certificate_path: Option<String>,
    pub key_path: Option<String>,
    pub token: Option<String>,
    pub realm: Option<String>,
}

/// TLS configuration for secure connections (Enhanced with proven patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub certificate_path: Option<String>,
    pub key_path: Option<String>,
    pub ca_path: Option<String>,
    pub verify_peer: bool,
    pub verify_hostname: bool,
    pub cipher_suites: Option<Vec<String>>,
}
