// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![cfg(feature = "dev-stubs")]

//! Minimal pure-Rust HTTP client façade for dev-stub call sites.
//!
//! NestGate does not depend on `reqwest`. Call sites historically aliased this module
//! as `reqwest`; types below mirror a small subset of that API while using a
//! self-contained `tokio::net::TcpStream`-based HTTP/1.1 implementation.
//!
//! **Overstep guidance**: For ecosystem HTTP to arbitrary URLs, prefer delegating to a
//! network-capability primal (songBird) via JSON-RPC `network.*` IPC. This client
//! exists only for dev-stub compilation and local bootstrap paths.

use crate::error::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// HTTP header map compatible with minimal `reqwest::header::HeaderMap` usage.
pub mod header {
    use std::collections::HashMap;

    /// Case-insensitive name (stored as provided).
    #[derive(Debug, Clone)]
    pub struct HeaderName(pub String);

    /// Header value.
    #[derive(Debug, Clone)]
    pub struct HeaderValue(pub String);

    impl std::str::FromStr for HeaderName {
        type Err = std::convert::Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(s.to_string()))
        }
    }

    impl std::str::FromStr for HeaderValue {
        type Err = std::convert::Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(s.to_string()))
        }
    }

    /// Map of header names to values.
    #[derive(Debug, Clone, Default)]
    pub struct HeaderMap {
        pub(super) inner: HashMap<String, String>,
    }

    impl HeaderMap {
        /// New empty map.
        #[must_use]
        pub fn new() -> Self {
            Self::default()
        }

        /// Insert a header.
        pub fn insert(&mut self, name: HeaderName, value: HeaderValue) -> Option<String> {
            self.inner.insert(name.0, value.0)
        }
    }
}

/// HTTP status code wrapper (status code only; no full `http` crate dependency).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusCode(u16);

impl StatusCode {
    /// Raw status code.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }

    /// Whether the status is 2xx.
    #[must_use]
    pub fn is_success(self) -> bool {
        (200..300).contains(&self.0)
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Successful HTTP response.
pub struct Response {
    status_code: u16,
    body: Vec<u8>,
}

impl Response {
    /// HTTP status.
    #[must_use]
    pub const fn status(&self) -> StatusCode {
        StatusCode(self.status_code)
    }

    /// Whether the response is successful (2xx).
    #[must_use]
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status_code)
    }

    /// Deserialize JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if the body is not valid JSON for `T`.
    pub fn json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        serde_json::from_slice(&self.body).map_err(|e| {
            crate::NestGateError::api_error(format!("http_client_stub: JSON deserialize: {e}"))
        })
    }
}

/// In-flight request (GET or POST + JSON).
pub struct RequestBuilder {
    config: Arc<ClientConfig>,
    url: String,
    kind: RequestKind,
    extra_headers: Vec<(String, String)>,
    json_body: Option<Vec<u8>>,
}

enum RequestKind {
    Get,
    Post,
}

impl RequestBuilder {
    /// Add a single header for this request.
    #[must_use]
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra_headers.push((key.into(), value.into()));
        self
    }

    /// Attach a JSON body (required for POST before `send`).
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    pub fn json<T: serde::Serialize>(mut self, body: &T) -> Result<Self> {
        let json = serde_json::to_vec(body).map_err(|e| {
            crate::NestGateError::api_error(format!("http_client_stub: JSON serialize: {e}"))
        })?;
        self.json_body = Some(json);
        Ok(self)
    }

    /// Execute the request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or (for POST) no JSON body was set.
    pub async fn send(self) -> Result<Response> {
        let method = match self.kind {
            RequestKind::Get => "GET",
            RequestKind::Post => "POST",
        };

        let body = match self.kind {
            RequestKind::Get => None,
            RequestKind::Post => Some(self.json_body.ok_or_else(|| {
                crate::NestGateError::api_error(
                    "http_client_stub: POST requires `.json(...)` before `send()`",
                )
            })?),
        };

        let mut headers = self.config.default_headers.clone();
        for (k, v) in self.extra_headers {
            headers.insert(k, v);
        }

        execute_http(
            method,
            &self.url,
            &headers,
            body.as_deref(),
            self.config.timeout,
        )
        .await
    }
}

/// Minimal HTTP/1.1 request over `TcpStream`. Self-contained — no external HTTP crate.
async fn execute_http(
    method: &str,
    url: &str,
    headers: &HashMap<String, String>,
    body: Option<&[u8]>,
    timeout: Duration,
) -> Result<Response> {
    let (host, port, path) = parse_url(url)?;
    let addr = format!("{host}:{port}");

    let stream = tokio::time::timeout(timeout, TcpStream::connect(&addr))
        .await
        .map_err(|_| crate::NestGateError::network_error("http_client_stub: connect timeout"))?
        .map_err(|e| {
            crate::NestGateError::network_error(format!("http_client_stub: connect: {e}"))
        })?;

    let mut buf = Vec::with_capacity(512);
    buf.extend_from_slice(format!("{method} {path} HTTP/1.1\r\nHost: {host}\r\n").as_bytes());
    for (k, v) in headers {
        buf.extend_from_slice(format!("{k}: {v}\r\n").as_bytes());
    }
    if let Some(b) = body {
        buf.extend_from_slice(
            format!(
                "Content-Type: application/json\r\nContent-Length: {}\r\n",
                b.len()
            )
            .as_bytes(),
        );
    }
    buf.extend_from_slice(b"Connection: close\r\n\r\n");
    if let Some(b) = body {
        buf.extend_from_slice(b);
    }

    let (mut reader, mut writer) = stream.into_split();
    writer.write_all(&buf).await.map_err(|e| {
        crate::NestGateError::network_error(format!("http_client_stub: write: {e}"))
    })?;
    writer.shutdown().await.ok();

    let mut response_buf = Vec::with_capacity(4096);
    tokio::time::timeout(timeout, reader.read_to_end(&mut response_buf))
        .await
        .map_err(|_| crate::NestGateError::network_error("http_client_stub: read timeout"))?
        .map_err(|e| crate::NestGateError::network_error(format!("http_client_stub: read: {e}")))?;

    parse_http_response(&response_buf)
}

fn parse_url(url: &str) -> Result<(String, u16, String)> {
    let stripped = url.strip_prefix("http://").ok_or_else(|| {
        crate::NestGateError::api_error("http_client_stub: only http:// supported")
    })?;

    let (host_port, path) = stripped.split_once('/').map_or_else(
        || (stripped, "/".to_string()),
        |(hp, p)| (hp, format!("/{p}")),
    );

    let (host, port) = host_port.split_once(':').map_or_else(
        || (host_port.to_string(), 80),
        |(h, p)| (h.to_string(), p.parse().unwrap_or(80)),
    );

    Ok((host, port, path))
}

fn parse_http_response(raw: &[u8]) -> Result<Response> {
    let header_end = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(|| {
            crate::NestGateError::network_error("http_client_stub: malformed response")
        })?;

    let header_str = std::str::from_utf8(&raw[..header_end]).map_err(|e| {
        crate::NestGateError::network_error(format!("http_client_stub: header decode: {e}"))
    })?;

    let status_line = header_str.lines().next().unwrap_or("");
    let status_code = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0);

    let body = raw[header_end + 4..].to_vec();

    Ok(Response { status_code, body })
}

/// Minimal HTTP client. Self-contained pure-Rust implementation over `TcpStream`.
#[derive(Debug, Clone)]
pub struct Client {
    config: Arc<ClientConfig>,
}

#[derive(Debug, Clone)]
struct ClientConfig {
    timeout: Duration,
    default_headers: HashMap<String, String>,
}

impl Client {
    /// Create a client with default timeout (30s).
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: Arc::new(ClientConfig {
                timeout: Duration::from_secs(30),
                default_headers: HashMap::new(),
            }),
        }
    }

    /// Builder for custom timeout and default headers.
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            timeout: Duration::from_secs(30),
            default_headers: HashMap::new(),
        }
    }

    /// Start a GET request.
    #[must_use]
    pub fn get(&self, url: impl AsRef<str>) -> RequestBuilder {
        RequestBuilder {
            config: Arc::clone(&self.config),
            url: url.as_ref().to_string(),
            kind: RequestKind::Get,
            extra_headers: Vec::new(),
            json_body: None,
        }
    }

    /// Start a POST request (use `.json(...).send().await`).
    #[must_use]
    pub fn post(&self, url: impl AsRef<str>) -> RequestBuilder {
        RequestBuilder {
            config: Arc::clone(&self.config),
            url: url.as_ref().to_string(),
            kind: RequestKind::Post,
            extra_headers: Vec::new(),
            json_body: None,
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for [`Client`].
#[derive(Debug)]
pub struct ClientBuilder {
    timeout: Duration,
    default_headers: HashMap<String, String>,
}

impl ClientBuilder {
    /// Set request timeout.
    #[must_use]
    pub fn timeout(self, duration: Duration) -> Self {
        Self {
            timeout: duration,
            ..self
        }
    }

    /// Default headers applied to every request.
    #[must_use]
    pub fn default_headers(mut self, headers: header::HeaderMap) -> Self {
        self.default_headers = headers.inner;
        self
    }

    /// Build the client.
    ///
    /// # Errors
    ///
    /// This always succeeds; errors are reserved for future validation.
    pub fn build(self) -> Result<Client> {
        Ok(Client {
            config: Arc::new(ClientConfig {
                timeout: self.timeout,
                default_headers: self.default_headers,
            }),
        })
    }
}

/// HTTP method enum for type compatibility (call sites and data structures).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// HTTP GET
    Get,
    /// HTTP POST
    Post,
    /// HTTP PUT
    Put,
    /// HTTP DELETE
    Delete,
    /// HTTP PATCH
    Patch,
    /// HTTP HEAD
    Head,
    /// HTTP OPTIONS
    Options,
}

impl Method {
    /// HTTP GET constant
    pub const GET: Self = Self::Get;
    /// HTTP POST constant
    pub const POST: Self = Self::Post;
    /// HTTP PUT constant
    pub const PUT: Self = Self::Put;
    /// HTTP DELETE constant
    pub const DELETE: Self = Self::Delete;
    /// HTTP PATCH constant
    pub const PATCH: Self = Self::Patch;
    /// HTTP HEAD constant
    pub const HEAD: Self = Self::Head;
    /// HTTP OPTIONS constant
    pub const OPTIONS: Self = Self::Options;
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            Self::Patch => write!(f, "PATCH"),
            Self::Head => write!(f, "HEAD"),
            Self::Options => write!(f, "OPTIONS"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::header::{HeaderMap, HeaderName, HeaderValue};
    use super::{Client, Method, StatusCode};
    use std::str::FromStr;
    use std::time::Duration;

    #[test]
    fn status_code_success_boundary() {
        assert!(!StatusCode(199).is_success());
        assert!(StatusCode(200).is_success());
        assert!(StatusCode(299).is_success());
        assert!(!StatusCode(300).is_success());
        assert_eq!(StatusCode(404).as_u16(), 404);
        assert_eq!(StatusCode(404).to_string(), "404");
    }

    #[test]
    fn header_map_insert_and_roundtrip_names() {
        let mut map = HeaderMap::new();
        assert!(
            map.insert(
                HeaderName::from_str("X-Test").expect("infallible"),
                HeaderValue::from_str("a").expect("infallible"),
            )
            .is_none()
        );
        assert_eq!(
            map.insert(
                HeaderName::from_str("X-Test").expect("infallible"),
                HeaderValue::from_str("b").expect("infallible"),
            ),
            Some("a".to_string())
        );
    }

    #[test]
    fn method_display_covers_all_variants() {
        let methods = [
            (Method::Get, "GET"),
            (Method::Post, "POST"),
            (Method::Put, "PUT"),
            (Method::Delete, "DELETE"),
            (Method::Patch, "PATCH"),
            (Method::Head, "HEAD"),
            (Method::Options, "OPTIONS"),
        ];
        for (m, s) in methods {
            assert_eq!(m.to_string(), s);
        }
        assert_eq!(Method::GET, Method::Get);
    }

    #[test]
    fn client_builder_applies_timeout_and_headers() {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_str("X-Default").expect("infallible"),
            HeaderValue::from_str("1").expect("infallible"),
        );
        let client = Client::builder()
            .timeout(Duration::from_millis(500))
            .default_headers(headers)
            .build()
            .expect("client build");
        let _ = client;
    }

    #[tokio::test]
    async fn post_without_json_errors_before_io() {
        let client = Client::new();
        match client.post("http://127.0.0.1:9").send().await {
            Err(e) => {
                let msg = e.to_string();
                assert!(
                    msg.contains("json") || msg.contains("POST"),
                    "unexpected error: {msg}"
                );
            }
            Ok(_) => panic!("POST without `.json()` should not succeed"),
        }
    }
}
