//
// Network and security configuration structures for the canonical unified configuration system.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::canonical_modernization::canonical_constants::{
    network::{DEFAULT_API_PORT, REQUEST_TIMEOUT_SECS, DEFAULT_BIND_ADDRESS},
    storage::{MB},
};

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct NetworkConfig {
    /// HTTP server configuration
    pub http_server: HttpServerConfig,
    /// RPC configuration
    pub rpc: RpcConfig,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// Timeout configuration
    pub timeouts: TimeoutConfig,
    /// Connection pooling
    pub connection_pool: ConnectionPoolConfig,
    /// TLS/SSL configuration
    pub tls: TlsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpServerConfig {
    /// Server bind address
    pub bind_endpoint: String,
    /// Server port
    pub port: u16,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Keep-alive timeout
    pub keep_alive_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Maximum request body size
    pub max_request_body_size: usize,
    /// Enable compression
    pub enable_compression: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcConfig {
    /// RPC protocol (tarpc, json-rpc, websocket)
    pub protocol: String,
    /// RPC bind address
    pub bind_endpoint: String,
    /// RPC port
    pub port: u16,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SecurityConfig {
    /// Authentication configuration
    pub authentication: AuthConfig,
    /// Authorization configuration
    pub authorization: AuthzConfig,
    /// Encryption configuration
    pub encryption: EncryptionConfig,
    /// Certificate configuration
    pub certificates: CertificateConfig,
    /// Security scanning configuration
    pub security_scanning: SecurityScanConfig,
    /// Audit logging configuration
    pub audit_logging: AuditLoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication provider (local, oauth2, ldap)
    pub provider: String,
    /// Token expiration time
    pub token_expiration: Duration,
    /// JWT secret
    pub jwt_secret: Option<String>,
    /// OAuth2 configuration
    pub oauth2: Option<OAuth2Config>,
    /// LDAP configuration
    pub ldap: Option<LdapConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Config {
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Authorization URL
    pub auth_url: String,
    /// Token URL
    pub token_url: String,
    /// Redirect URL
    pub redirect_url: String,
    /// Scopes
    pub scopes: Vec<String>,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancingConfig {
    pub strategy: String,
    pub health_check_interval: Duration,
    pub max_retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    pub half_open_max_calls: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
    pub window_size: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimeoutConfig {
    pub connect: Duration,
    pub request: Duration,
    pub response: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionPoolConfig {
    pub max_size: u32,
    pub min_idle: u32,
    pub max_lifetime: Duration,
    pub idle_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TlsConfig {
    pub enabled: bool,
    pub cert_file: Option<PathBuf>,
    pub key_file: Option<PathBuf>,
    pub ca_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthzConfig {
    pub enabled: bool,
    pub policy_file: Option<PathBuf>,
    pub default_role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub key_size: u32,
    pub key_rotation_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CertificateConfig {
    pub auto_generate: bool,
    pub cert_dir: PathBuf,
    pub validity_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityScanConfig {
    pub enabled: bool,
    pub scan_interval: Duration,
    pub vulnerability_db_update_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLoggingConfig {
    pub enabled: bool,
    pub log_file: PathBuf,
    pub max_file_size: u64,
    pub max_files: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LdapConfig {
    pub server: String,
    pub port: u16,
    pub bind_dn: String,
    pub bind_password: String,
    pub user_base_dn: String,
    pub user_filter: String,
}

// ==================== SECTION ====================


impl Default for HttpServerConfig {
    fn default() -> Self {
        Self {
            bind_endpoint: DEFAULT_BIND_ADDRESS.to_string(),
            port: DEFAULT_API_PORT,
            max_connections: 1000,
            keep_alive_timeout: Duration::from_secs(60),
            request_timeout: Duration::from_secs(REQUEST_TIMEOUT_SECS),
            max_request_body_size: (16 * MB) as usize,
            enable_compression: true,
        }
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            protocol: "json-rpc".to_string(),
            bind_endpoint: "127.0.0.1".to_string(),
            port: 8081,
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
            max_concurrent_requests: 100,
        }
    }
}


impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            provider: "local".to_string(),
            token_expiration: Duration::from_secs(3600),
            jwt_secret: None,
            oauth2: None,
            ldap: None,
        }
    }
} 