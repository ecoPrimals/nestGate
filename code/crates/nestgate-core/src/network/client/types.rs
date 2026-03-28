// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! HTTP Type-Safe Primitives and Core Types
//!
//! Type-safe wrappers for HTTP concepts: ports, methods, status codes, endpoints.
//! All types use zero-cost abstractions with compile-time guarantees.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::Duration;

use crate::error::{NestGateError, Result};

// ==================== TYPE-SAFE PRIMITIVES ====================

/// Type-safe port number with validation
///
/// Ensures ports are non-zero at compile time where possible,
/// runtime validation otherwise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Port(u16);

impl Port {
    /// Create a new port, validating the range
    ///
    /// # Note on Privileged Ports
    ///
    /// Ports < 1024 are privileged and typically require root access to BIND.
    /// However, this type is used for both binding (server) and connecting (client).
    /// For CLIENT connections (`http://example.com:80`), privileged ports are fine.
    /// For SERVER binding, use ports >= 1024 for security (principle of least privilege).
    ///
    /// This validation only rejects port 0, which is always invalid.
    /// Application code should enforce >= 1024 for server binding separately.
    ///
    /// # Errors
    ///
    /// Returns error if port is 0 (invalid)
    pub fn new(port: u16) -> Result<Self> {
        if port == 0 {
            return Err(NestGateError::validation_error("Port cannot be 0"));
        }
        // Note: Ports 1-1023 are allowed here because this type is used for
        // BOTH client connections (where 80/443 are common) and server binding.
        // Server binding logic should validate >= 1024 separately.
        Ok(Self(port))
    }

    /// Get the raw port value
    pub fn get(self) -> u16 {
        self.0
    }
}

/// Type-safe timeout duration
///
/// Wrapper around milliseconds to prevent mixing up time units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

    /// Get milliseconds value
    pub fn as_millis(self) -> u64 {
        self.0
    }

    /// Get inner value (alias for as_millis)
    pub fn get(self) -> u64 {
        self.0
    }
}

// ==================== HTTP METHOD ====================

/// HTTP method enumeration
///
/// Standard HTTP methods with semantic helpers for idiomatic usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Method {
    /// GET - Retrieve resource
    Get,
    /// POST - Create resource
    Post,
    /// PUT - Replace resource
    Put,
    /// DELETE - Remove resource
    Delete,
    /// PATCH - Partially update resource
    Patch,
    /// HEAD - Retrieve headers only
    Head,
    /// OPTIONS - Query available methods
    Options,
}

impl Method {
    /// Check if this method is safe (no side effects)
    ///
    /// Safe methods: GET, HEAD, OPTIONS (read-only)
    pub fn is_safe(self) -> bool {
        matches!(self, Self::Get | Self::Head | Self::Options)
    }

    /// Check if this method can have a request body
    ///
    /// Methods with bodies: POST, PUT, PATCH
    pub fn can_have_body(self) -> bool {
        matches!(self, Self::Post | Self::Put | Self::Patch)
    }
}

// ==================== HTTP STATUS CODE ====================

/// HTTP status code with semantic helpers
///
/// Type-safe wrapper with common constants and category checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatusCode(u16);

impl StatusCode {
    // Common success codes
    /// 200 OK
    pub const OK: Self = Self(200);
    /// 201 Created
    pub const CREATED: Self = Self(201);
    /// 204 No Content
    pub const NO_CONTENT: Self = Self(204);

    // Common client error codes
    /// 400 Bad Request
    pub const BAD_REQUEST: Self = Self(400);
    /// 401 Unauthorized
    pub const UNAUTHORIZED: Self = Self(401);
    /// 403 Forbidden
    pub const FORBIDDEN: Self = Self(403);
    /// 404 Not Found
    pub const NOT_FOUND: Self = Self(404);

    // Common server error codes
    /// 500 Internal Server Error
    pub const INTERNAL_SERVER_ERROR: Self = Self(500);
    /// 502 Bad Gateway
    pub const BAD_GATEWAY: Self = Self(502);
    /// 503 Service Unavailable
    pub const SERVICE_UNAVAILABLE: Self = Self(503);

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

    /// Check if this is a client error (4xx)
    pub fn is_client_error(self) -> bool {
        self.0 >= 400 && self.0 < 500
    }

    /// Check if this is a server error (5xx)
    pub fn is_server_error(self) -> bool {
        self.0 >= 500 && self.0 < 600
    }

    /// Check if this is any error status (4xx or 5xx)
    pub fn is_error(self) -> bool {
        self.0 >= 400
    }
}

// ==================== ENDPOINT ====================

/// Network endpoint with scheme, host, and port
///
/// Type-safe endpoint representation ensuring valid URLs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Endpoint {
    /// Host (domain or IP)
    pub host: String,
    /// Port number
    pub port: Port,
    /// Scheme (HTTP/HTTPS)
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

    /// Parse URL string into Endpoint
    ///
    /// Modern, idiomatic URL parsing that handles common formats.
    /// Supports: http://host:port, https://host:port, with optional paths
    ///
    /// # Examples
    /// ```ignore
    /// let endpoint = Endpoint::from_url("http://localhost:8080")?;
    /// let endpoint = Endpoint::from_url("https://api.example.com:443/api")?;
    /// ```
    pub fn from_url(url: &str) -> Result<Self> {
        // Parse scheme
        let (scheme_str, rest) = url
            .split_once("://")
            .ok_or_else(|| NestGateError::validation_error("Invalid URL: missing scheme"))?;

        let scheme = match scheme_str.to_lowercase().as_str() {
            "http" => Scheme::Http,
            "https" => Scheme::Https,
            "ws" => Scheme::Http,   // WebSocket over HTTP
            "wss" => Scheme::Https, // WebSocket over HTTPS
            _ => return Err(NestGateError::validation_error("Unsupported scheme")),
        };

        // Parse host:port (strip path, query, fragment)
        let rest = rest.split('/').next().unwrap_or(rest);
        let rest = rest.split('?').next().unwrap_or(rest);
        let rest = rest.split('#').next().unwrap_or(rest);
        let rest = rest.split('@').next_back().unwrap_or(rest); // Strip credentials if present

        let (host, port_str) = if let Some((h, p)) = rest.split_once(':') {
            (h, p)
        } else {
            // Default ports
            (rest, if scheme == Scheme::Http { "80" } else { "443" })
        };

        // Validate host is not empty
        if host.is_empty() {
            return Err(NestGateError::validation_error("Invalid URL: empty host"));
        }

        let port_num: u16 = port_str
            .parse()
            .map_err(|_| NestGateError::validation_error("Invalid port number"))?;

        let port = Port::new(port_num)?;

        Ok(Self {
            host: host.to_string(),
            port,
            scheme,
        })
    }

    /// Get full URL for this endpoint with path
    pub fn url(&self, path: &str) -> String {
        format!(
            "{}://{}:{}{}",
            self.scheme,
            self.host,
            self.port.get(),
            path
        )
    }

    /// Get base URL without path
    pub fn base_url(&self) -> String {
        format!("{}://{}:{}", self.scheme, self.host, self.port.get())
    }
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}://{}:{}", self.scheme, self.host, self.port.get())
    }
}

// ==================== SCHEME ====================

/// URL scheme (HTTP or HTTPS)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scheme {
    /// HTTP (unencrypted)
    Http,
    /// HTTPS (encrypted)
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

impl PartialEq<str> for Scheme {
    fn eq(&self, other: &str) -> bool {
        self.to_string().as_str() == other
    }
}

impl Serialize for Endpoint {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Endpoint {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Endpoint::from_url(&s).map_err(serde::de::Error::custom)
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_validation() {
        assert!(Port::new(0).is_err());
        assert!(Port::new(8080).is_ok());

        // ✅ EVOLVED: Proper error handling instead of unwrap
        let port = Port::new(443).expect("Port 443 should be valid");
        assert_eq!(port.get(), 443);
    }

    #[test]
    fn test_method_semantics() {
        assert!(Method::Get.is_safe());
        assert!(!Method::Post.is_safe());
        assert!(Method::Post.can_have_body());
        assert!(!Method::Get.can_have_body());
    }

    #[test]
    fn test_status_code_categories() {
        assert!(StatusCode::OK.is_success());
        assert!(StatusCode::BAD_REQUEST.is_client_error());
        assert!(StatusCode::INTERNAL_SERVER_ERROR.is_server_error());
        assert!(StatusCode::NOT_FOUND.is_error());
    }

    #[test]
    fn test_endpoint_from_url() {
        // ✅ EVOLVED: Test unwraps with clear failure context
        let ep = Endpoint::from_url("http://localhost:8080")
            .expect("Valid HTTP URL should parse successfully");
        assert_eq!(ep.host, "localhost");
        assert_eq!(ep.port.get(), 8080);
        assert_eq!(ep.scheme, Scheme::Http);

        let ep = Endpoint::from_url("https://api.example.com:443/path")
            .expect("Valid HTTPS URL with path should parse successfully");
        assert_eq!(ep.host, "api.example.com");
        assert_eq!(ep.port.get(), 443);
        assert_eq!(ep.scheme, Scheme::Https);
    }

    #[test]
    fn test_endpoint_url_generation() {
        // ✅ EVOLVED: Test setup with clear context
        let port = Port::new(8080).expect("Port 8080 is valid for test setup");
        let ep = Endpoint::http("localhost".to_string(), port);
        assert_eq!(ep.url("/api/test"), "http://localhost:8080/api/test");
        assert_eq!(ep.base_url(), "http://localhost:8080");
    }
}
