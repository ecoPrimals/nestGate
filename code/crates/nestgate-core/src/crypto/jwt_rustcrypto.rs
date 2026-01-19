//! Pure Rust JWT Implementation using RustCrypto
//!
//! This module provides JWT signing and validation using pure Rust cryptography (RustCrypto).
//! No external HTTP calls, no C dependencies, 100% pure Rust!
//!
//! **Philosophy**: TRUE PRIMAL architecture
//! - Local validation (no network calls)
//! - Pure Rust (no C dependencies)
//! - Audited crypto (RustCrypto/NCC Group)
//! - Fast and secure

use crate::error::{NestGateError, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// JWT header
#[derive(Debug, Serialize, Deserialize)]
struct JwtHeader {
    /// Algorithm (HS256 or EdDSA)
    alg: String,
    /// Type (always "JWT")
    typ: String,
}

/// JWT claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject (user ID)
    pub sub: String,
    /// Issued at (Unix timestamp)
    pub iat: i64,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issuer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    /// Audience
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    /// Custom permissions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl JwtClaims {
    /// Create new claims with default values
    pub fn new(subject: String, expiry_seconds: i64) -> Result<Self> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| NestGateError::validation_error(&format!("Time error: {}", e)))?
            .as_secs() as i64;

        Ok(Self {
            sub: subject,
            iat: now,
            exp: now + expiry_seconds,
            iss: Some("nestgate".to_string()),
            aud: None,
            permissions: None,
        })
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);

        self.exp < now
    }
}

/// JWT signer/verifier using HMAC-SHA256 (HS256)
pub struct JwtHmac {
    key: Vec<u8>,
}

impl JwtHmac {
    /// Create new HMAC JWT handler
    pub fn new(secret: &str) -> Self {
        Self {
            key: secret.as_bytes().to_vec(),
        }
    }

    /// Generate JWT token
    pub fn sign(&self, claims: &JwtClaims) -> Result<String> {
        // Create header
        let header = JwtHeader {
            alg: "HS256".to_string(),
            typ: "JWT".to_string(),
        };

        // Encode header and payload
        let header_json = serde_json::to_string(&header).map_err(|e| {
            NestGateError::validation_error(&format!("Header encoding failed: {}", e))
        })?;
        let payload_json = serde_json::to_string(claims).map_err(|e| {
            NestGateError::validation_error(&format!("Payload encoding failed: {}", e))
        })?;

        let header_b64 = URL_SAFE_NO_PAD.encode(header_json.as_bytes());
        let payload_b64 = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());

        // Create message to sign
        let message = format!("{}.{}", header_b64, payload_b64);

        // Sign with HMAC-SHA256
        let mut mac = HmacSha256::new_from_slice(&self.key)
            .map_err(|e| NestGateError::validation_error(&format!("HMAC key error: {}", e)))?;
        mac.update(message.as_bytes());
        let signature = mac.finalize().into_bytes();

        let signature_b64 = URL_SAFE_NO_PAD.encode(&signature);

        Ok(format!("{}.{}", message, signature_b64))
    }

    /// Verify and decode JWT token
    pub fn verify(&self, token: &str) -> Result<JwtClaims> {
        // Split token
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(NestGateError::validation_error("Invalid JWT format"));
        }

        let header_b64 = parts[0];
        let payload_b64 = parts[1];
        let signature_b64 = parts[2];

        // Verify signature
        let message = format!("{}.{}", header_b64, payload_b64);
        let mut mac = HmacSha256::new_from_slice(&self.key)
            .map_err(|e| NestGateError::validation_error(&format!("HMAC key error: {}", e)))?;
        mac.update(message.as_bytes());

        let expected_signature = URL_SAFE_NO_PAD.decode(signature_b64).map_err(|e| {
            NestGateError::validation_error(&format!("Signature decode error: {}", e))
        })?;

        mac.verify_slice(&expected_signature)
            .map_err(|_| NestGateError::validation_error("Invalid JWT signature"))?;

        // Decode payload
        let payload_bytes = URL_SAFE_NO_PAD.decode(payload_b64).map_err(|e| {
            NestGateError::validation_error(&format!("Payload decode error: {}", e))
        })?;

        let claims: JwtClaims = serde_json::from_slice(&payload_bytes).map_err(|e| {
            NestGateError::validation_error(&format!("Claims parsing error: {}", e))
        })?;

        // Check expiration
        if claims.is_expired() {
            return Err(NestGateError::validation_error("Token expired"));
        }

        Ok(claims)
    }
}

/// JWT signer/verifier using Ed25519 (EdDSA)
pub struct JwtEd25519 {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl JwtEd25519 {
    /// Create new Ed25519 JWT handler from secret seed
    pub fn new(seed_bytes: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(seed_bytes);
        let verifying_key = signing_key.verifying_key();

        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Create from existing signing key
    pub fn from_signing_key(signing_key: SigningKey) -> Self {
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Generate JWT token
    pub fn sign(&self, claims: &JwtClaims) -> Result<String> {
        // Create header
        let header = JwtHeader {
            alg: "EdDSA".to_string(),
            typ: "JWT".to_string(),
        };

        // Encode header and payload
        let header_json = serde_json::to_string(&header).map_err(|e| {
            NestGateError::validation_error(&format!("Header encoding failed: {}", e))
        })?;
        let payload_json = serde_json::to_string(claims).map_err(|e| {
            NestGateError::validation_error(&format!("Payload encoding failed: {}", e))
        })?;

        let header_b64 = URL_SAFE_NO_PAD.encode(header_json.as_bytes());
        let payload_b64 = URL_SAFE_NO_PAD.encode(payload_json.as_bytes());

        // Create message to sign
        let message = format!("{}.{}", header_b64, payload_b64);

        // Sign with Ed25519
        let signature: Signature = self.signing_key.sign(message.as_bytes());
        let signature_b64 = URL_SAFE_NO_PAD.encode(signature.to_bytes());

        Ok(format!("{}.{}", message, signature_b64))
    }

    /// Verify and decode JWT token
    pub fn verify(&self, token: &str) -> Result<JwtClaims> {
        // Split token
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(NestGateError::validation_error("Invalid JWT format"));
        }

        let header_b64 = parts[0];
        let payload_b64 = parts[1];
        let signature_b64 = parts[2];

        // Verify signature
        let message = format!("{}.{}", header_b64, payload_b64);
        let signature_bytes = URL_SAFE_NO_PAD.decode(signature_b64).map_err(|e| {
            NestGateError::validation_error(&format!("Signature decode error: {}", e))
        })?;

        let signature = Signature::from_bytes(
            signature_bytes
                .as_slice()
                .try_into()
                .map_err(|_| NestGateError::validation_error("Invalid signature length"))?,
        );

        self.verifying_key
            .verify(message.as_bytes(), &signature)
            .map_err(|_| NestGateError::validation_error("Invalid JWT signature"))?;

        // Decode payload
        let payload_bytes = URL_SAFE_NO_PAD.decode(payload_b64).map_err(|e| {
            NestGateError::validation_error(&format!("Payload decode error: {}", e))
        })?;

        let claims: JwtClaims = serde_json::from_slice(&payload_bytes).map_err(|e| {
            NestGateError::validation_error(&format!("Claims parsing error: {}", e))
        })?;

        // Check expiration
        if claims.is_expired() {
            return Err(NestGateError::validation_error("Token expired"));
        }

        Ok(claims)
    }

    /// Get verifying key (for sharing public key)
    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_jwt_sign_verify() {
        let jwt = JwtHmac::new("test-secret-key");
        let claims = JwtClaims::new("user123".to_string(), 3600).unwrap();

        let token = jwt.sign(&claims).unwrap();
        let verified_claims = jwt.verify(&token).unwrap();

        assert_eq!(verified_claims.sub, "user123");
        assert!(!verified_claims.is_expired());
    }

    #[test]
    fn test_hmac_jwt_invalid_signature() {
        let jwt1 = JwtHmac::new("secret1");
        let jwt2 = JwtHmac::new("secret2");
        let claims = JwtClaims::new("user123".to_string(), 3600).unwrap();

        let token = jwt1.sign(&claims).unwrap();
        let result = jwt2.verify(&token);

        assert!(result.is_err());
    }

    #[test]
    fn test_ed25519_jwt_sign_verify() {
        let seed = [0u8; 32]; // In production, use secure random seed
        let jwt = JwtEd25519::new(&seed);
        let claims = JwtClaims::new("user456".to_string(), 3600).unwrap();

        let token = jwt.sign(&claims).unwrap();
        let verified_claims = jwt.verify(&token).unwrap();

        assert_eq!(verified_claims.sub, "user456");
        assert!(!verified_claims.is_expired());
    }

    #[test]
    fn test_expired_token() {
        let jwt = JwtHmac::new("test-secret");
        let mut claims = JwtClaims::new("user789".to_string(), -10).unwrap(); // Already expired
        claims.exp -= 100; // Make it more expired

        let token = jwt.sign(&claims).unwrap();
        let result = jwt.verify(&token);

        assert!(result.is_err());
    }
}
