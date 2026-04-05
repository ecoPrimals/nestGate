// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! **AWS Signature Version 4 Implementation**
//!
//! Pure Rust implementation of AWS SigV4 authentication.
//! NO AWS SDK dependencies - speaks pure HTTP.
//!
//! ## Features
//!
//! - ✅ Zero dependencies on AWS SDK
//! - ✅ Standard HMAC-SHA256 implementation
//! - ✅ Canonical request generation
//! - ✅ Signature calculation
//! - ✅ Authorization header creation
//!
//! ## References
//!
//! - [AWS Signature V4 Docs](https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html)
//! - [Signing Process](https://docs.aws.amazon.com/general/latest/gr/sigv4_signing.html)

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt::Write as _;

type HmacSha256 = Hmac<Sha256>;

/// AWS credentials for Signature V4
#[derive(Debug, Clone)]
pub struct AwsCredentials {
    /// Access key ID
    pub access_key: String,
    /// Secret access key
    pub secret_key: String,
    /// Optional session token (for temporary credentials)
    pub session_token: Option<String>,
}

/// AWS Signature V4 signer
pub struct AwsSigV4 {
    credentials: AwsCredentials,
    region: String,
    service: String,
}

impl AwsSigV4 {
    /// Create a new AWS Signature V4 signer
    ///
    /// # Arguments
    ///
    /// * `credentials` - AWS credentials
    /// * `region` - AWS region (e.g., "us-east-1")
    /// * `service` - AWS service name (e.g., "s3")
    #[must_use]
    pub const fn new(credentials: AwsCredentials, region: String, service: String) -> Self {
        Self {
            credentials,
            region,
            service,
        }
    }

    /// Sign a request and return the Authorization header value
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method (GET, PUT, POST, DELETE, etc.)
    /// * `url` - Full URL including query string
    /// * `headers` - Request headers (will be included in signature)
    /// * `payload` - Request body (empty for GET/DELETE)
    ///
    /// # Returns
    ///
    /// Returns a tuple of (`authorization_header`, `x_amz_date`)
    pub fn sign(
        &self,
        method: &str,
        url: &str,
        headers: &BTreeMap<String, String>,
        payload: &[u8],
    ) -> Result<(String, String), SigningError> {
        // Parse URL
        let parsed_url =
            url::Url::parse(url).map_err(|e| SigningError::InvalidUrl(e.to_string()))?;

        // Get current timestamp
        let timestamp = chrono::Utc::now();
        let amz_date = timestamp.format("%Y%m%dT%H%M%SZ").to_string();
        let date_stamp = timestamp.format("%Y%m%d").to_string();

        // Build canonical headers (including x-amz-date and host)
        let mut canonical_headers = headers.clone();
        canonical_headers.insert("x-amz-date".to_string(), amz_date.clone());
        canonical_headers.insert(
            "host".to_string(),
            parsed_url.host_str().unwrap_or("").to_string(),
        );

        // Add session token if present
        if let Some(ref token) = self.credentials.session_token {
            canonical_headers.insert("x-amz-security-token".to_string(), token.clone());
        }

        // Step 1: Create canonical request
        let canonical_request =
            self.create_canonical_request(method, &parsed_url, &canonical_headers, payload)?;

        // Step 2: Create string to sign
        let string_to_sign =
            self.create_string_to_sign(&amz_date, &date_stamp, &canonical_request)?;

        // Step 3: Calculate signature
        let signature = self.calculate_signature(&date_stamp, &string_to_sign)?;

        // Step 4: Create authorization header
        let signed_headers = self.get_signed_headers(&canonical_headers);
        let authorization = format!(
            "AWS4-HMAC-SHA256 Credential={}/{}/{}/{}/aws4_request, SignedHeaders={}, Signature={}",
            self.credentials.access_key,
            date_stamp,
            self.region,
            self.service,
            signed_headers,
            signature
        );

        Ok((authorization, amz_date))
    }

    /// Create canonical request (Step 1 of signing process)
    fn create_canonical_request(
        &self,
        method: &str,
        url: &url::Url,
        headers: &BTreeMap<String, String>,
        payload: &[u8],
    ) -> Result<String, SigningError> {
        // Canonical URI (path)
        let canonical_uri = url.path();

        // Canonical query string
        let canonical_query = self.create_canonical_query_string(url);

        // Canonical headers (sorted, lowercase keys)
        let canonical_headers_str = headers
            .iter()
            .map(|(k, v)| format!("{}:{}", k.to_lowercase(), v.trim()))
            .collect::<Vec<_>>()
            .join("\n");

        // Signed headers (sorted, lowercase, semicolon-separated)
        let signed_headers = self.get_signed_headers(headers);

        // Payload hash
        let payload_hash = hex_encode(&sha256_hash(payload));

        // Combine into canonical request
        let canonical_request = format!(
            "{method}\n{canonical_uri}\n{canonical_query}\n{canonical_headers_str}\n\n{signed_headers}\n{payload_hash}"
        );

        Ok(canonical_request)
    }

    /// Create canonical query string (sorted parameters)
    fn create_canonical_query_string(&self, url: &url::Url) -> String {
        let mut params: Vec<(String, String)> = url
            .query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        params.sort_by(|a, b| a.0.cmp(&b.0));

        params
            .iter()
            .map(|(k, v)| format!("{}={}", uri_encode(k, true), uri_encode(v, true)))
            .collect::<Vec<_>>()
            .join("&")
    }

    /// Get signed headers list (sorted, lowercase)
    fn get_signed_headers(&self, headers: &BTreeMap<String, String>) -> String {
        let mut keys: Vec<String> = headers.keys().map(|k| k.to_lowercase()).collect();
        keys.sort();
        keys.join(";")
    }

    /// Create string to sign (Step 2 of signing process)
    fn create_string_to_sign(
        &self,
        amz_date: &str,
        date_stamp: &str,
        canonical_request: &str,
    ) -> Result<String, SigningError> {
        let credential_scope = format!(
            "{}/{}/{}/aws4_request",
            date_stamp, self.region, self.service
        );
        let canonical_request_hash = hex_encode(&sha256_hash(canonical_request.as_bytes()));

        Ok(format!(
            "AWS4-HMAC-SHA256\n{amz_date}\n{credential_scope}\n{canonical_request_hash}"
        ))
    }

    /// Calculate signature (Step 3 of signing process)
    fn calculate_signature(
        &self,
        date_stamp: &str,
        string_to_sign: &str,
    ) -> Result<String, SigningError> {
        // Derive signing key
        let k_secret = format!("AWS4{}", self.credentials.secret_key);
        let k_date = hmac_sha256(k_secret.as_bytes(), date_stamp.as_bytes())?;
        let k_region = hmac_sha256(&k_date, self.region.as_bytes())?;
        let k_service = hmac_sha256(&k_region, self.service.as_bytes())?;
        let k_signing = hmac_sha256(&k_service, b"aws4_request")?;

        // Calculate signature
        let signature = hmac_sha256(&k_signing, string_to_sign.as_bytes())?;

        Ok(hex_encode(&signature))
    }
}

/// Signing errors
/// Errors that can occur during AWS Signature V4 signing
#[derive(Debug, thiserror::Error)]
pub enum SigningError {
    /// URL parsing or construction failed
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// HMAC computation failed
    #[error("HMAC error: {0}")]
    HmacError(String),

    /// Invalid signing key provided
    #[error("Invalid key")]
    InvalidKey,
}

/// Compute HMAC-SHA256
fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<Vec<u8>, SigningError> {
    let mut mac = HmacSha256::new_from_slice(key).map_err(|_| SigningError::InvalidKey)?;
    mac.update(data);
    Ok(mac.finalize().into_bytes().to_vec())
}

/// Compute SHA256 hash
fn sha256_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Hex encode bytes
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02x}");
        output
    })
}

/// URI encode a string (RFC 3986)
fn uri_encode(s: &str, encode_slash: bool) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            b'/' if !encode_slash => "/".to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let data = b"hello world";
        let hash = sha256_hash(data);
        assert_eq!(hash.len(), 32); // SHA256 is 32 bytes
    }

    #[test]
    fn test_hex_encode() {
        let bytes = vec![0, 15, 255];
        let hex = hex_encode(&bytes);
        assert_eq!(hex, "000fff");
    }

    #[test]
    fn test_uri_encode() {
        assert_eq!(uri_encode("hello", true), "hello");
        assert_eq!(uri_encode("hello world", true), "hello%20world");
        assert_eq!(uri_encode("path/to/file", false), "path/to/file");
        assert_eq!(uri_encode("path/to/file", true), "path%2Fto%2Ffile");
    }

    #[test]
    fn test_hmac_sha256() {
        let key = b"key";
        let data = b"message";
        let result = hmac_sha256(key, data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32); // HMAC-SHA256 is 32 bytes
    }

    #[test]
    fn test_aws_sigv4_creation() {
        let credentials = AwsCredentials {
            access_key: "AKIAIOSFODNN7EXAMPLE".to_string(),
            secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string(),
            session_token: None,
        };

        let signer = AwsSigV4::new(credentials, "us-east-1".to_string(), "s3".to_string());

        // Just test that it creates successfully
        assert_eq!(signer.region, "us-east-1");
        assert_eq!(signer.service, "s3");
    }

    #[test]
    fn test_canonical_query_string() {
        let credentials = AwsCredentials {
            access_key: "test".to_string(),
            secret_key: "test".to_string(),
            session_token: None,
        };

        let signer = AwsSigV4::new(credentials, "us-east-1".to_string(), "s3".to_string());

        let url = url::Url::parse("https://example.com/path?z=last&a=first&b=second").unwrap();
        let query = signer.create_canonical_query_string(&url);

        // Should be sorted alphabetically
        assert!(query.starts_with("a=first&b=second"));
    }
}
