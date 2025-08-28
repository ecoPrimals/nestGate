//
// This module contains canonical configuration types for the MCP system,
// using modern patterns and the unified configuration system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SECTION ====================

/// Canonical MCP client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpClientConfig {
    /// MCP server endpoint
    pub endpoint: String,
    /// Client identifier
    pub client_id: String,
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Authentication configuration
    pub auth: Option<McpAuthConfig>,
    /// TLS configuration
    pub tls: Option<McpTlsConfig>,
    /// Retry configuration
    pub retry: McpRetryConfig,
    /// Protocol version
    pub protocol_version: String,
    /// User agent string
    pub user_agent: String,
    /// Additional headers
    pub headers: HashMap<String, String>,
}

impl Default for McpClientConfig {
    fn default() -> Self {
        Self {
            endpoint: std::env::var("NESTGATE_MCP_ENDPOINT")
                .unwrap_or_else(|_| "http://127.0.0.1:8084".to_string()),
            client_id: std::env::var("NESTGATE_MCP_CLIENT_ID")
                .unwrap_or_else(|_| format!("nestgate-{}", uuid::Uuid::new_v4().simple())),
            connect_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            auth: None,
            tls: None,
            retry: McpRetryConfig::default(),
            protocol_version: "2024-11-05".to_string(),
            user_agent: format!("nestgate-mcp/{}", env!("CARGO_PKG_VERSION")),
            headers: HashMap::new(),
        }
    }
}

/// MCP authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpAuthConfig {
    /// Authentication method
    pub method: McpAuthMethod,
    /// Credentials
    pub credentials: McpCredentials,
}

/// MCP authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McpAuthMethod {
    /// No authentication
    None,
    /// API key authentication
    ApiKey,
    /// Bearer token authentication
    Bearer,
    /// Basic authentication
    Basic,
    /// OAuth2 authentication
    OAuth2,
    /// Custom authentication
    Custom(String),
}

/// MCP credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpCredentials {
    /// Primary credential (API key, token, username, etc.)
    pub primary: String,
    /// Secondary credential (password, secret, etc.)
    pub secondary: Option<String>,
    /// Additional parameters
    pub parameters: HashMap<String, String>,
}

/// MCP TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTlsConfig {
    /// Enable TLS verification
    pub verify_certificates: bool,
    /// CA certificate path
    pub ca_cert_path: Option<String>,
    /// Client certificate path
    pub client_cert_path: Option<String>,
    /// Client key path
    pub client_key_path: Option<String>,
    /// TLS version
    pub min_tls_version: Option<String>,
    /// Cipher suites
    pub cipher_suites: Option<Vec<String>>,
}

impl Default for McpTlsConfig {
    fn default() -> Self {
        Self {
            verify_certificates: true,
            ca_cert_path: None,
            client_cert_path: None,
            client_key_path: None,
            min_tls_version: Some("TLSv1.2".to_string()),
            cipher_suites: None,
        }
    }
}

/// MCP retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial retry delay
    pub initial_delay: Duration,
    /// Maximum retry delay
    pub max_delay: Duration,
    /// Retry multiplier for exponential backoff
    pub multiplier: f64,
    /// Jitter factor (0.0 to 1.0)
    pub jitter: f64,
    /// Retryable error types
    pub retryable_errors: Vec<String>,
}

impl Default for McpRetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(60),
            multiplier: 2.0,
            jitter: 0.1,
            retryable_errors: vec![
                "ConnectionError".to_string(),
                "TimeoutError".to_string(),
                "ServerError".to_string(),
                "Network".to_string(),
            ],
        }
    }
}

/// MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Server bind address
    pub bind_address: String,
    /// Server port
    pub port: u16,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// TLS configuration
    pub tls: Option<McpTlsConfig>,
    /// Authentication configuration
    pub auth: Option<McpAuthConfig>,
    /// Protocol version
    pub protocol_version: String,
    /// Server capabilities
    pub capabilities: McpServerCapabilities,
}

impl Default for McpServerConfig {
    fn default() -> Self {
        Self {
            bind_address: std::env::var("NESTGATE_MCP_BIND_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("NESTGATE_MCP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8084),
            max_connections: 1000,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            tls: None,
            auth: None,
            protocol_version: "2024-11-05".to_string(),
            capabilities: McpServerCapabilities::default(),
        }
    }
}

/// MCP server capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerCapabilities {
    /// Supported operations
    pub operations: Vec<String>,
    /// Maximum request size
    pub max_request_size: usize,
    /// Maximum response size
    pub max_response_size: usize,
    /// Streaming support
    pub streaming: bool,
    /// Batch operations support
    pub batch_operations: bool,
    /// Custom capabilities
    pub custom: HashMap<String, serde_json::Value>,
}

impl Default for McpServerCapabilities {
    fn default() -> Self {
        Self {
            operations: vec![
                "initialize".to_string(),
                "health_check".to_string(),
                "shutdown".to_string(),
            ],
            max_request_size: 1024 * 1024,  // 1MB
            max_response_size: 1024 * 1024, // 1MB
            streaming: false,
            batch_operations: false,
            custom: HashMap::new(),
        }
    }
}

// ==================== SECTION ====================

impl McpClientConfig {
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.endpoint.is_empty() {
            return Err("MCP endpoint cannot be empty".to_string());
        }

        if self.client_id.is_empty() {
            return Err("MCP client ID cannot be empty".to_string());
        }

        if self.connect_timeout.is_zero() {
            return Err("Connect timeout must be greater than zero".to_string());
        }

        if self.request_timeout.is_zero() {
            return Err("Request timeout must be greater than zero".to_string());
        }

        if self.protocol_version.is_empty() {
            return Err("Protocol version cannot be empty".to_string());
        }

        Ok(())
    }

    /// Create configuration from environment variables
    pub fn from_env() -> Result<Self, String> {
        let mut config = Self::default();

        if let Ok(endpoint) = std::env::var("NESTGATE_MCP_ENDPOINT") {
            config.endpoint = endpoint;
        }

        if let Ok(client_id) = std::env::var("NESTGATE_MCP_CLIENT_ID") {
            config.client_id = client_id;
        }

        if let Ok(timeout) = std::env::var("NESTGATE_MCP_CONNECT_TIMEOUT") {
            config.connect_timeout = Duration::from_secs(
                timeout
                    .parse()
                    .map_err(|e| format!("Invalid connect timeout: {e}"))?,
            );
        }

        if let Ok(timeout) = std::env::var("NESTGATE_MCP_REQUEST_TIMEOUT") {
            config.request_timeout = Duration::from_secs(
                timeout
                    .parse()
                    .map_err(|e| format!("Invalid request timeout: {e}"))?,
            );
        }

        if let Ok(version) = std::env::var("NESTGATE_MCP_PROTOCOL_VERSION") {
            config.protocol_version = version;
        }

        config.validate()?;
        Ok(config)
    }
}

impl McpServerConfig {
    /// Validate the server configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.bind_address.is_empty() {
            return Err("Bind address cannot be empty".to_string());
        }

        if self.port == 0 {
            return Err("Port must be greater than zero".to_string());
        }

        if self.max_connections == 0 {
            return Err("Max connections must be greater than zero".to_string());
        }

        if self.connection_timeout.is_zero() {
            return Err("Connection timeout must be greater than zero".to_string());
        }

        if self.request_timeout.is_zero() {
            return Err("Request timeout must be greater than zero".to_string());
        }

        Ok(())
    }
}
