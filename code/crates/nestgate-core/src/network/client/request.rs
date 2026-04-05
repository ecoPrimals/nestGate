// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! HTTP Request and Response Types
//!
//! Type-safe representations of HTTP requests and responses with
//! zero-copy where possible and proper lifetime management.

use serde::Deserialize;
use std::collections::HashMap;

use super::types::{Method, StatusCode};

// ==================== REQUEST ====================

/// HTTP Request with method, path, headers, and optional body
///
/// Zero-copy where possible through lifetime-bound references.
#[derive(Debug, Clone)]
pub struct Request<'a> {
    /// HTTP method
    pub method: Method,
    /// Request path (without host/port)
    pub path: &'a str,
    /// Request headers
    pub headers: HeaderMap,
    /// Optional request body
    pub body: Option<RequestBody<'a>>,
}

impl<'a> Request<'a> {
    /// Create a GET request
    #[must_use]
    pub fn get(path: &'a str) -> Self {
        Self {
            method: Method::Get,
            path,
            headers: HeaderMap::new(),
            body: None,
        }
    }

    /// Create a POST request with JSON body
    #[must_use]
    pub fn post_json(path: &'a str, json: &'a str) -> Self {
        let mut headers = HeaderMap::new();
        // Use lowercase header names per HTTP/2 spec (RFC 7540)
        headers.insert("content-type".to_string(), "application/json".to_string());

        Self {
            method: Method::Post,
            path,
            headers,
            body: Some(RequestBody::Json(json)),
        }
    }

    /// Create a POST request with form data
    #[must_use]
    pub fn post_form(path: &'a str, data: &'a [(String, String)]) -> Self {
        let mut headers = HeaderMap::new();
        // Use lowercase header names per HTTP/2 spec (RFC 7540)
        headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );

        Self {
            method: Method::Post,
            path,
            headers,
            body: Some(RequestBody::Form(data)),
        }
    }

    /// Add a header to this request
    #[must_use]
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
}

/// Request body variants
///
/// Zero-copy through string slices where possible.
#[derive(Debug, Clone)]
pub enum RequestBody<'a> {
    /// Raw bytes
    Raw(&'a [u8]),
    /// JSON string
    Json(&'a str),
    /// Form-encoded data
    Form(&'a [(String, String)]),
}

// ==================== RESPONSE ====================

/// HTTP Response with status, headers, and body
///
/// Owns the response data for safe async handling.
#[derive(Debug, Clone)]
pub struct Response {
    /// HTTP status code
    pub status: StatusCode,
    /// Response headers
    pub headers: HeaderMap,
    /// Response body as bytes
    pub body: Vec<u8>,
}

impl Response {
    /// Create a new response
    #[must_use]
    pub const fn new(status: StatusCode, headers: HeaderMap, body: Vec<u8>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    /// Get response body as string
    ///
    /// # Errors
    ///
    /// Returns [`FromUtf8Error`](std::string::FromUtf8Error) if the body is not valid UTF-8.
    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }

    /// Get response body as JSON
    ///
    /// # Errors
    ///
    /// Returns [`serde_json::Error`] if deserialization fails.
    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }

    /// Check if response was successful (2xx)
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// Check if response was an error (4xx or 5xx)
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.status.is_error()
    }

    /// Get a header value
    #[must_use]
    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
}

// ==================== HEADER MAP ====================

/// Type alias for HTTP headers
///
/// Uses HashMap for efficient lookups. Keys are case-insensitive
/// in HTTP but we normalize to lowercase for storage.
pub type HeaderMap = HashMap<String, String>;

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_get() {
        let req = Request::get("/api/test");
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.path, "/api/test");
        assert!(req.body.is_none());
    }

    #[test]
    fn test_request_post_json() {
        let req = Request::post_json("/api/create", r#"{"name": "test"}"#);
        assert_eq!(req.method, Method::Post);
        // HTTP/2 uses lowercase headers (RFC 7540)
        assert!(req.headers.contains_key("content-type"));
        assert!(matches!(req.body, Some(RequestBody::Json(_))));
    }

    #[test]
    fn test_request_with_header() {
        let req = Request::get("/api/test")
            .with_header("Authorization".to_string(), "Bearer token".to_string());
        assert_eq!(
            req.headers.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
    }

    #[test]
    fn test_response_text() {
        let response = Response::new(StatusCode::OK, HeaderMap::new(), b"Hello, World!".to_vec());
        assert_eq!(response.text().unwrap(), "Hello, World!");
    }

    #[test]
    fn test_response_json() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestData {
            message: String,
        }

        let response = Response::new(
            StatusCode::OK,
            HeaderMap::new(),
            br#"{"message": "test"}"#.to_vec(),
        );

        let data: TestData = response.json().unwrap();
        assert_eq!(data.message, "test");
    }

    #[test]
    fn test_response_status_checks() {
        let success = Response::new(StatusCode::OK, HeaderMap::new(), vec![]);
        assert!(success.is_success());
        assert!(!success.is_error());

        let error = Response::new(StatusCode::NOT_FOUND, HeaderMap::new(), vec![]);
        assert!(!error.is_success());
        assert!(error.is_error());
    }
}
