// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Minimal pure-Rust HTTP client for discovery bootstrap.
//!
//! Discovery mechanisms face a bootstrap problem: they need HTTP to find
//! other primals, but HTTP delegation requires knowing where primals are.
//! This minimal client resolves that chicken-and-egg by providing just
//! enough HTTP/1.1 over `TcpStream` for Consul and Kubernetes REST APIs.
//!
//! **Not a general-purpose HTTP client.** Only supports:
//! - GET with optional headers
//! - PUT with JSON body
//! - HTTP/1.1 (no TLS — use k8s API proxy or service mesh for HTTPS)
//!
//! Zero external dependencies beyond `tokio` and `serde_json`.

use nestgate_types::error::NestGateError;
use nestgate_types::error::Result;
use std::collections::HashMap;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Parsed URL components (scheme, host, port, path)
struct ParsedUrl {
    host: String,
    port: u16,
    path: String,
    is_https: bool,
}

fn parse_url(url: &str) -> Result<ParsedUrl> {
    let (scheme, rest) = if let Some(rest) = url.strip_prefix("https://") {
        (true, rest)
    } else if let Some(rest) = url.strip_prefix("http://") {
        (false, rest)
    } else {
        return Err(NestGateError::config(format!(
            "Discovery HTTP: unsupported scheme in URL: {url}"
        )));
    };

    let (authority, path) = rest.split_once('/').unwrap_or((rest, ""));
    let path = format!("/{path}");

    let default_port: u16 = if scheme { 443 } else { 80 };
    let (host, port) = if let Some((h, p)) = authority.rsplit_once(':') {
        let port = p.parse::<u16>().unwrap_or(default_port);
        (h.to_string(), port)
    } else {
        (authority.to_string(), default_port)
    };

    Ok(ParsedUrl {
        host,
        port,
        path,
        is_https: scheme,
    })
}

/// Minimal HTTP response
pub struct HttpResponse {
    /// HTTP status code
    pub status: u16,
    body: Vec<u8>,
}

impl HttpResponse {
    /// Whether the status code indicates success (2xx)
    #[must_use]
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Deserialize the response body as JSON
    ///
    /// # Errors
    ///
    /// Returns an error if the body is not valid JSON for type `T`.
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        serde_json::from_slice(&self.body).map_err(|e| {
            NestGateError::api_error(&format!("Discovery HTTP: JSON parse error: {e}"))
        })
    }
}

/// Pure-Rust HTTP client for discovery bootstrap
#[derive(Clone, Debug)]
pub struct DiscoveryHttpClient {
    timeout: Duration,
    headers: HashMap<String, String>,
}

impl DiscoveryHttpClient {
    /// Create a new discovery HTTP client with the given timeout.
    #[must_use]
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            headers: HashMap::new(),
        }
    }

    /// Add a default header to all requests.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Send an HTTP GET request.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid, connection fails, or the
    /// response cannot be read.
    pub async fn get(&self, url: &str) -> Result<HttpResponse> {
        self.request("GET", url, None).await
    }

    /// Send an HTTP PUT request with a JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization, connection, or response reading fails.
    pub async fn put_json<T: serde::Serialize + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<HttpResponse> {
        let json = serde_json::to_vec(body).map_err(|e| {
            NestGateError::api_error(&format!("Discovery HTTP: JSON serialize error: {e}"))
        })?;
        self.request("PUT", url, Some(&json)).await
    }

    /// Send an HTTP POST request with a JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error if serialization, connection, or response reading fails.
    pub async fn post_json<T: serde::Serialize + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<HttpResponse> {
        let json = serde_json::to_vec(body).map_err(|e| {
            NestGateError::api_error(&format!("Discovery HTTP: JSON serialize error: {e}"))
        })?;
        self.request("POST", url, Some(&json)).await
    }

    /// POST with an already-serialized JSON body.
    pub async fn post_json_bytes(&self, url: &str, json: &[u8]) -> Result<HttpResponse> {
        self.request("POST", url, Some(json)).await
    }

    async fn request(&self, method: &str, url: &str, body: Option<&[u8]>) -> Result<HttpResponse> {
        let parsed = parse_url(url)?;

        if parsed.is_https {
            return Err(NestGateError::config(
                "Discovery HTTP: HTTPS not supported in bootstrap client. \
                 Use a k8s API proxy, service mesh sidecar, or HTTP endpoint.",
            ));
        }

        let addr = format!("{}:{}", parsed.host, parsed.port);
        let stream = tokio::time::timeout(self.timeout, TcpStream::connect(&addr))
            .await
            .map_err(|_| {
                NestGateError::api_error(&format!("Discovery HTTP: connection timeout to {addr}"))
            })?
            .map_err(|e| {
                NestGateError::api_error(&format!(
                    "Discovery HTTP: connection failed to {addr}: {e}"
                ))
            })?;

        self.send_and_receive(stream, method, &parsed, body).await
    }

    async fn send_and_receive(
        &self,
        mut stream: TcpStream,
        method: &str,
        parsed: &ParsedUrl,
        body: Option<&[u8]>,
    ) -> Result<HttpResponse> {
        let mut request = format!(
            "{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n",
            path = parsed.path,
            host = parsed.host,
        );

        for (key, value) in &self.headers {
            request.push_str(&format!("{key}: {value}\r\n"));
        }

        if let Some(b) = body {
            request.push_str("Content-Type: application/json\r\n");
            request.push_str(&format!("Content-Length: {}\r\n", b.len()));
        }

        request.push_str("\r\n");

        tokio::time::timeout(self.timeout, async {
            stream.write_all(request.as_bytes()).await.map_err(|e| {
                NestGateError::api_error(&format!("Discovery HTTP: write failed: {e}"))
            })?;

            if let Some(b) = body {
                stream.write_all(b).await.map_err(|e| {
                    NestGateError::api_error(&format!("Discovery HTTP: body write failed: {e}"))
                })?;
            }

            stream.flush().await.map_err(|e| {
                NestGateError::api_error(&format!("Discovery HTTP: flush failed: {e}"))
            })?;

            let mut buf = Vec::with_capacity(8192);
            stream.read_to_end(&mut buf).await.map_err(|e| {
                NestGateError::api_error(&format!("Discovery HTTP: read failed: {e}"))
            })?;

            parse_http_response(&buf)
        })
        .await
        .map_err(|_| NestGateError::api_error("Discovery HTTP: request timeout"))?
    }
}

fn parse_http_response(raw: &[u8]) -> Result<HttpResponse> {
    let header_end = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or_else(|| {
            NestGateError::api_error("Discovery HTTP: malformed response (no header end)")
        })?;

    let header_str = std::str::from_utf8(&raw[..header_end]).map_err(|e| {
        NestGateError::api_error(&format!("Discovery HTTP: invalid header encoding: {e}"))
    })?;

    let status_line = header_str
        .lines()
        .next()
        .ok_or_else(|| NestGateError::api_error("Discovery HTTP: empty response"))?;

    let status = status_line
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .ok_or_else(|| {
            NestGateError::api_error(&format!(
                "Discovery HTTP: invalid status line: {status_line}"
            ))
        })?;

    let body = raw[header_end + 4..].to_vec();

    Ok(HttpResponse { status, body })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_http_url() {
        let parsed = parse_url("http://localhost:8500/v1/agent/services").unwrap();
        assert_eq!(parsed.host, "localhost");
        assert_eq!(parsed.port, 8500);
        assert_eq!(parsed.path, "/v1/agent/services");
        assert!(!parsed.is_https);
    }

    #[test]
    fn parse_http_url_default_port() {
        let parsed = parse_url("http://consul.example.com/v1/health").unwrap();
        assert_eq!(parsed.host, "consul.example.com");
        assert_eq!(parsed.port, 80);
        assert_eq!(parsed.path, "/v1/health");
    }

    #[test]
    fn parse_https_url() {
        let parsed = parse_url("https://k8s.local:6443/api/v1/services").unwrap();
        assert_eq!(parsed.host, "k8s.local");
        assert_eq!(parsed.port, 6443);
        assert!(parsed.is_https);
    }

    #[test]
    fn parse_url_no_path() {
        let parsed = parse_url("http://localhost:8500").unwrap();
        assert_eq!(parsed.path, "/");
    }

    #[test]
    fn parse_url_bad_scheme() {
        assert!(parse_url("ftp://example.com").is_err());
    }

    #[test]
    fn parse_response_basic() {
        let raw = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"ok\":true}";
        let resp = parse_http_response(raw).unwrap();
        assert_eq!(resp.status, 200);
        assert!(resp.is_success());
        let val: serde_json::Value = resp.json().unwrap();
        assert_eq!(val["ok"], true);
    }

    #[test]
    fn parse_response_404() {
        let raw = b"HTTP/1.1 404 Not Found\r\n\r\n";
        let resp = parse_http_response(raw).unwrap();
        assert_eq!(resp.status, 404);
        assert!(!resp.is_success());
    }

    #[test]
    fn parse_response_malformed() {
        assert!(parse_http_response(b"garbage").is_err());
    }

    #[tokio::test]
    async fn https_rejected_in_bootstrap_client() {
        let client = DiscoveryHttpClient::new(Duration::from_secs(5));
        let result = client.get("https://example.com/api").await;
        assert!(result.is_err());
        let err_msg = result.err().expect("expected HTTPS rejection").to_string();
        assert!(err_msg.contains("HTTPS not supported"));
    }

    #[tokio::test]
    async fn connection_refused_returns_error() {
        let client = DiscoveryHttpClient::new(Duration::from_secs(1));
        let result = client.get("http://127.0.0.1:1/nonexistent").await;
        assert!(result.is_err());
    }
}
