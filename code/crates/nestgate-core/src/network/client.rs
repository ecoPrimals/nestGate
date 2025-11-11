//! Modern HTTP Client Module
//!
//! Provides a high-performance, type-safe HTTP client with connection pooling,
//! retry mechanisms, and comprehensive error handling using modern Rust patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, Semaphore};

use crate::error::{NestGateError, Result};

// ==================== TYPE-SAFE PRIMITIVES ====================

/// Type-safe port number with validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Port(u16);

impl Port {
    /// Create a new port, validating the range
    pub fn new(port: u16) -> crate::Result<Self> {
        if port == 0 {
            return Err(NestGateError::validation_error("Port cannot be 0"));
        }
        Ok(Self(port))
    }

    /// Get the raw port value
    pub fn get(self) -> u16 {
        self.0
    }
}

/// Type-safe timeout duration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeoutMs(u64);

impl TimeoutMs {
    /// Create a new timeout
    pub fn new(ms: u64) -> Self {
        Self(ms)
    }

    /// Convert to Duration
    pub fn as_duration(self) -> Duration {
        Duration::from_millis(self.0)
    }
}

// ==================== HTTP TYPES ====================

/// HTTP method enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl Method {
    /// Check if this method is safe (no side effects)
    pub fn is_safe(self) -> bool {
        matches!(self, Self::Get | Self::Head | Self::Options)
    }

    /// Check if this method can have a request body
    pub fn can_have_body(self) -> bool {
        matches!(self, Self::Post | Self::Put | Self::Patch)
    }
}

/// HTTP status code with helper methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatusCode(u16);

impl StatusCode {
    pub const OK: Self = Self(200);
    pub const CREATED: Self = Self(201);
    pub const BAD_REQUEST: Self = Self(400);
    pub const NOT_FOUND: Self = Self(404);
    pub const INTERNAL_SERVER_ERROR: Self = Self(500);

    /// Create a new status code
    pub fn new(code: u16) -> Self {
        Self(code)
    }

    /// Get the raw status code
    pub fn as_u16(self) -> u16 {
        self.0
    }

    /// Check if this is a success status (2xx)
    pub fn is_success(self) -> bool {
        self.0 >= 200 && self.0 < 300
    }

    /// Check if this is an error status (4xx or 5xx)
    pub fn is_error(self) -> bool {
        self.0 >= 400
    }
}

/// Network endpoint
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Endpoint {
    pub host: String,
    pub port: Port,
    pub scheme: Scheme,
}

impl Endpoint {
    /// Create HTTP endpoint
    pub fn http(host: String, port: Port) -> Self {
        Self {
            host,
            port,
            scheme: Scheme::Http,
        }
    }

    /// Create HTTPS endpoint
    pub fn https(host: String, port: Port) -> Self {
        Self {
            host,
            port,
            scheme: Scheme::Https,
        }
    }

    /// Get the full URL
    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.scheme, self.host, self.port.get())
    }
}

/// URL scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scheme {
    Http,
    Https,
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http => write!(f, "http"),
            Self::Https => write!(f, "https"),
        }
    }
}

// ==================== REQUEST/RESPONSE TYPES ====================

/// HTTP request with zero-copy body support
#[derive(Debug)]
pub struct Request<'a> {
    pub method: Method,
    pub path: &'a str,
    pub headers: HeaderMap,
    pub body: RequestBody<'a>,
}

impl<'a> Request<'a> {
    /// Create a new GET request
    pub fn get(path: &'a str) -> Self {
        Self {
            method: Method::Get,
            path,
            headers: HeaderMap::new(),
            body: RequestBody::Empty,
        }
    }

    /// Create a new POST request with JSON body
    pub fn post_json(path: &'a str, body: &'a str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        Self {
            method: Method::Post,
            path,
            headers,
            body: RequestBody::String(body),
        }
    }

    /// Add a header to the request
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
}

/// Request body that can be zero-copy
#[derive(Debug)]
pub enum RequestBody<'a> {
    Empty,
    Bytes(&'a [u8]),
    String(&'a str),
}

/// HTTP response
#[derive(Debug)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

impl Response {
    /// Check if the response is successful
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// Get response body as string
    pub async fn text(&self) -> crate::Result<String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| NestGateError::validation_error(&format!("Invalid response body: {}", e)))
    }

    /// Parse response body as JSON
    pub async fn json<T: for<'de> Deserialize<'de>>(&self) -> crate::Result<T> {
        let text = self.text().await?;
        serde_json::from_str(&text)
            .map_err(|e| NestGateError::validation_error(&format!("Failed to parse JSON: {}", e)))
    }
}

/// Header map type alias
pub type HeaderMap = HashMap<String, String>;

// ==================== CLIENT CONFIGURATION ====================

/// HTTP client configuration with const generics for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::ClientConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::ClientConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct ClientConfig<const DEFAULT_TIMEOUT_MS: u64 = 30000> {
    pub timeout: TimeoutMs,
    pub max_connections: usize,
    pub max_connections_per_host: usize,
    pub enable_compression: bool,
    pub follow_redirects: bool,
    pub max_redirects: usize,
    pub user_agent: String,
}

impl<const DEFAULT_TIMEOUT_MS: u64> Default for ClientConfig<DEFAULT_TIMEOUT_MS> {
    fn default() -> Self {
        Self {
            timeout: TimeoutMs::new(DEFAULT_TIMEOUT_MS),
            max_connections: 100,
            max_connections_per_host: 10,
            enable_compression: true,
            follow_redirects: true,
            max_redirects: 5,
            user_agent: "NestGate-Client/2.0".to_string(),
        }
    }
}

// ==================== CONNECTION POOL ====================

/// Connection pool for HTTP connections
#[derive(Debug)]
pub struct ConnectionPool {
    connections: Arc<RwLock<HashMap<String, Vec<Connection>>>>,
    semaphore: Arc<Semaphore>,
    config: ClientConfig,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub fn new(config: ClientConfig) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            semaphore: Arc::new(Semaphore::new(config.max_connections)),
            config,
        }
    }

    /// Get a connection for the given endpoint
    pub async fn get_connection(&self, endpoint: &Endpoint) -> Result<Connection> {
        let _permit = self.semaphore.acquire().await.map_err(|e| {
            NestGateError::network_error(&format!("Failed to acquire connection: {}", e))
        })?;

        let key = endpoint.url();
        let mut connections = self.connections.write().await;

        if let Some(pool) = connections.get_mut(&key) {
            if let Some(conn) = pool.pop() {
                if conn.is_alive() {
                    return Ok(conn);
                }
            }
        }

        // Create new connection
        Connection::new(endpoint.clone()).await
    }

    /// Return a connection to the pool
    pub async fn return_connection(&self, endpoint: &Endpoint, connection: Connection) {
        if !connection.is_alive() {
            return;
        }

        let key = endpoint.url();
        let mut connections = self.connections.write().await;

        let pool = connections.entry(key).or_insert_with(Vec::new);
        if pool.len() < self.config.max_connections_per_host {
            pool.push(connection);
        }
    }
}

// ==================== CONNECTION ====================

/// HTTP connection handle
#[derive(Debug)]
pub struct Connection {
    endpoint: Endpoint,
    created_at: std::time::Instant,
    last_used: std::time::Instant,
    request_count: u64,
}

impl Connection {
    /// Create a new connection
    pub async fn new(endpoint: Endpoint) -> Result<Self> {
        // In a real implementation, this would establish the actual connection
        let now = std::time::Instant::now();
        Ok(Self {
            endpoint,
            created_at: now,
            last_used: now,
            request_count: 0,
        })
    }

    /// Check if the connection is still alive
    pub fn is_alive(&self) -> bool {
        // Simple heuristic: connection is alive if used recently
        self.last_used.elapsed() < Duration::from_secs(300) // 5 minutes
    }

    /// Send a request over this connection
    pub async fn send_request(&mut self, request: &Request<'_>) -> Result<Response> {
        self.last_used = std::time::Instant::now();
        self.request_count += 1;

        // Build the full URL
        let url = format!("{}{}", self.endpoint.url(), request.path);

        // Create reqwest client
        let client = reqwest::Client::new();

        // Convert method
        let method = match request.method {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
            Method::Patch => reqwest::Method::PATCH,
            Method::Head => reqwest::Method::HEAD,
            Method::Options => reqwest::Method::OPTIONS,
        };

        // Build request
        let mut req_builder = client.request(method, &url);

        // Add headers
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }

        // Add body if present
        req_builder = match &request.body {
            RequestBody::Empty => req_builder,
            RequestBody::Bytes(bytes) => req_builder.body(bytes.to_vec()),
            RequestBody::String(s) => req_builder.body(s.to_string()),
        };

        // Send request
        let resp = req_builder
            .send()
            .await
            .map_err(|e| NestGateError::network_error(&format!("HTTP request failed: {}", e)))?;

        // Convert response
        let status = StatusCode::new(resp.status().as_u16());

        // Extract headers
        let mut headers = HeaderMap::new();
        for (key, value) in resp.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }

        // Get body
        let body = resp.bytes().await.map_err(|e| {
            NestGateError::network_error(&format!("Failed to read response body: {}", e))
        })?;

        Ok(Response {
            status,
            headers,
            body: body.to_vec(),
        })
    }

    /// Get connection statistics
    pub fn stats(&self) -> ConnectionStats {
        ConnectionStats {
            endpoint: self.endpoint.clone(),
            age: self.created_at.elapsed(),
            idle_time: self.last_used.elapsed(),
            request_count: self.request_count,
        }
    }
}

/// Connection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStats {
    pub endpoint: Endpoint,
    pub age: Duration,
    pub idle_time: Duration,
    pub request_count: u64,
}

// ==================== HTTP CLIENT ====================

/// Modern HTTP client with connection pooling and retry logic
#[derive(Debug)]
pub struct HttpClient {
    pool: ConnectionPool,
    config: ClientConfig,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new(config: ClientConfig) -> Self {
        let pool = ConnectionPool::new(config.clone());
        Self { pool, config }
    }

    /// Send a GET request
    pub async fn get(&self, endpoint: &Endpoint, path: &str) -> Result<Response> {
        let request = Request::get(path);
        self.send_request(endpoint, &request).await
    }

    /// Send a POST request with JSON body
    pub async fn post_json(&self, endpoint: &Endpoint, path: &str, body: &str) -> Result<Response> {
        let request = Request::post_json(path, body);
        self.send_request(endpoint, &request).await
    }

    /// Send a request with retry logic
    pub async fn send_request(
        &self,
        endpoint: &Endpoint,
        request: &Request<'_>,
    ) -> Result<Response> {
        let mut attempts = 0;
        let max_attempts = 3;

        loop {
            attempts += 1;

            match self.send_request_once(endpoint, request).await {
                Ok(response) => return Ok(response),
                Err(e) if attempts >= max_attempts => return Err(e),
                Err(_) => {
                    // Exponential backoff
                    let delay = Duration::from_millis(100 * (1 << (attempts - 1)));
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    /// Send a single request attempt
    async fn send_request_once(
        &self,
        endpoint: &Endpoint,
        request: &Request<'_>,
    ) -> Result<Response> {
        let mut connection = self.pool.get_connection(endpoint).await?;

        let response = tokio::time::timeout(
            self.config.timeout.as_duration(),
            connection.send_request(request),
        )
        .await
        .map_err(|_| NestGateError::timeout_error("Request", self.config.timeout.as_duration()))?;

        // Return connection to pool
        self.pool.return_connection(endpoint, connection).await;

        response
    }

    /// Get client statistics
    pub async fn stats(&self) -> ClientStats {
        ClientStats {
            total_connections: 0, // Would be calculated from pool
            active_requests: 0,   // Would be tracked
            total_requests: 0,    // Would be tracked
            failed_requests: 0,   // Would be tracked
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new(ClientConfig::default())
    }
}

/// Client statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientStats {
    pub total_connections: usize,
    pub active_requests: usize,
    pub total_requests: u64,
    pub failed_requests: u64,
}

// ==================== ERROR TYPES ====================

/// HTTP client specific errors
#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    #[error("Connection failed: {message}")]
    ConnectionFailed { message: String },

    #[error("Request timeout after {timeout:?}")]
    Timeout { timeout: Duration },

    #[error("Invalid response: {message}")]
    InvalidResponse { message: String },

    #[error("Too many redirects: {count}")]
    TooManyRedirects { count: usize },
}

impl From<HttpClientError> for NestGateError {
    fn from(err: HttpClientError) -> Self {
        match err {
            HttpClientError::ConnectionFailed { message } => NestGateError::network_error(&message),
            HttpClientError::Timeout { timeout } => {
                NestGateError::timeout_error("HTTP request", timeout)
            }
            HttpClientError::InvalidResponse { message } => {
                NestGateError::validation_error(&format!("Invalid HTTP response: {}", message))
            }
            HttpClientError::TooManyRedirects { count } => {
                NestGateError::network_error(&format!("Too many redirects: {}", count))
            }
        }
    }
}

// ==================== UTILITY FUNCTIONS ====================

/// Create a default HTTP client
pub fn create_client() -> HttpClient {
    HttpClient::default()
}

/// Create an HTTPS endpoint
pub async fn https_endpoint(host: &str, port: u16) -> crate::Result<Endpoint> {
    let port = Port::new(port)?;
    Ok(Endpoint::https(host.to_string(), port))
}

/// Create an HTTP endpoint
pub async fn http_endpoint(host: &str, port: u16) -> crate::Result<Endpoint> {
    let port = Port::new(port)?;
    Ok(Endpoint::http(host.to_string(), port))
}

// ==================== TESTS ====================


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type ClientConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ClientConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_validation() {
        assert!(Port::new(0).is_err());
        assert!(Port::new(8080).is_ok());
        assert_eq!(
            Port::new(8080).expect("Network operation failed").get(),
            8080
        );
    }

    #[test]
    fn test_status_code() {
        assert!(StatusCode::OK.is_success());
        assert!(!StatusCode::NOT_FOUND.is_success());
        assert!(StatusCode::NOT_FOUND.is_error());
    }

    #[test]
    fn test_method_properties() {
        assert!(Method::Get.is_safe());
        assert!(!Method::Get.can_have_body());
        assert!(!Method::Post.is_safe());
        assert!(Method::Post.can_have_body());
    }

    #[test]
    fn test_endpoint_creation() {
        use crate::constants::hardcoding::{addresses, ports};
        let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
        let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);
        assert_eq!(
            endpoint.url(),
            format!(
                "http://{}:{}",
                addresses::LOCALHOST_NAME,
                ports::HTTP_DEFAULT
            )
        );
    }

    #[tokio::test]
    async fn test_client_creation() {
        let client = HttpClient::default();
        let stats = client.stats().await;
        assert_eq!(stats.total_connections, 0);
    }

    #[tokio::test]
    async fn test_connection_pool() {
        let config = ClientConfig::default();
        let pool = ConnectionPool::new(config);

        let endpoint = Endpoint::http(
            "example.com".to_string(),
            Port::new(80).expect("Network operation failed"),
        );
        let connection = pool
            .get_connection(&endpoint)
            .await
            .expect("Network operation failed");

        assert!(connection.is_alive());
    }

    #[test]
    fn test_request_builder() {
        let request = Request::get("/api/test")
            .with_header("authorization".to_string(), "Bearer token".to_string());

        assert_eq!(request.method, Method::Get);
        assert_eq!(request.path, "/api/test");
        assert!(request.headers.contains_key("authorization"));
    }
}
