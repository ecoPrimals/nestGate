// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::future::Future;

use super::canonical_service::CanonicalService;

/// **THE** canonical security trait
///
/// Replaces ALL security provider traits:
/// - ZeroCostSecurityProvider (3 versions)
/// - SecurityProvider (multiple)
/// - AuthenticationProvider
/// - EncryptionProvider
/// - SigningProvider
/// - 3+ other security trait variants
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::canonical_hierarchy::{CanonicalService, CanonicalSecurity};
///
/// pub struct JwtSecurity {
///     config: JwtConfig,
/// }
///
/// impl CanonicalService for JwtSecurity {
///     // ... implement CanonicalService
/// }
///
/// impl CanonicalSecurity for JwtSecurity {
///     type Token = String;
///     type Credentials = UserCredentials;
///     type Principal = User;
///
///     async fn authenticate(
///         &self,
///         credentials: Self::Credentials,
///     ) -> Result<Self::Token, Self::Error> {
///         // JWT authentication
///         todo!()
///     }
///
///     async fn validate_token(
///         &self,
///         token: &Self::Token,
///     ) -> Result<Self::Principal, Self::Error> {
///         // JWT validation
///         todo!()
///     }
///
///     async fn revoke_token(&self, token: &Self::Token) -> Result<(), Self::Error> {
///         // JWT revocation
///         todo!()
///     }
///
///     async fn authorize(
///         &self,
///         principal: &Self::Principal,
///         resource: &str,
///         action: &str,
///     ) -> Result<bool, Self::Error> {
///         // Authorization check
///         todo!()
///     }
///
///     async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
///         // Encryption
///         todo!()
///     }
///
///     async fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
///         // Decryption
///         todo!()
///     }
///
///     async fn sign(&self, data: &[u8]) -> Result<Vec<u8>, Self::Error> {
///         // Signing
///         todo!()
///     }
///
///     async fn verify(&self, data: &[u8], signature: &[u8]) -> Result<bool, Self::Error> {
///         // Signature verification
///         todo!()
///     }
/// }
/// ```
/// **DEPRECATED**: Use canonical::CanonicalSecurity instead
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::canonical::CanonicalSecurity instead - unified in canonical_unified_traits module"
)]
/// CanonicalSecurity trait
pub trait CanonicalSecurity: CanonicalService {
    /// Token type (JWT, session, etc.)
    type Token: Clone + Send + Sync + 'static;

    /// Credentials type
    type Credentials: Clone + Send + Sync + 'static;

    /// Principal type (user, service, etc.)
    type Principal: Clone + Send + Sync + 'static;

    // ==================== AUTHENTICATION ====================

    /// Authenticate credentials and return a token
    fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> impl Future<Output = Result<Self::Token, Self::Error>> + Send;

    /// Validate a token
    fn validate_token(
        &self,
        token: &Self::Token,
    ) -> impl Future<Output = Result<Self::Principal, Self::Error>> + Send;

    /// Revoke a token
    fn revoke_token(
        &self,
        token: &Self::Token,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    // ==================== AUTHORIZATION ====================

    /// Check if principal has permission
    fn authorize(
        &self,
        principal: &Self::Principal,
        resource: &str,
        action: &str,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    // ==================== CRYPTOGRAPHY ====================

    /// Encrypt data
    fn encrypt(&self, data: &[u8]) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// Decrypt data
    fn decrypt(&self, data: &[u8]) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// Sign data
    fn sign(&self, data: &[u8]) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send;

    /// Verify signature
    fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;

    // ==================== AUDIT ====================

    /// Log security event
    ///
    /// Default implementation is a no-op.
    /// Override to implement audit logging.
    fn audit_log(
        &self,
        _event: &str,
        _principal: Option<&Self::Principal>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            // Default: no-op
            Ok(())
        }
    }

    // ==================== ADVANCED CRYPTOGRAPHY (OPTIONAL) ====================
    // These methods provide advanced cryptographic operations beyond the basic
    // encrypt/decrypt/sign/verify. They have default implementations that either
    // delegate to the simpler methods or return "not supported" errors.

    /// Sign data and return structured signature (optional, delegates to sign by default)
    ///
    /// Default implementation delegates to `sign()` and wraps in a generic signature structure.
    /// Override for more sophisticated signature formats (e.g., with key IDs, algorithms).
    fn sign_data(&self, data: &[u8]) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send {
        self.sign(data)
    }

    /// Verify structured signature (optional, delegates to verify by default)
    ///
    /// Default implementation delegates to `verify()`.
    /// Override for more sophisticated signature verification.
    fn verify_signature(
        &self,
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        self.verify(data, signature)
    }

    /// Get the key ID used for signing (optional)
    ///
    /// Default implementation returns None to indicate no key ID tracking.
    /// Override if your implementation tracks key IDs.
    fn get_key_id(&self) -> impl Future<Output = Result<Option<String>, Self::Error>> + Send {
        async { Ok(None) }
    }

    /// Hash data with specific algorithm (optional)
    ///
    /// Default implementation returns "not supported" error.
    /// Override to provide hashing capabilities.
    fn hash_data(
        &self,
        _data: &[u8],
        _algorithm: &str,
    ) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send
    where
        Self: Sized,
        crate::NestGateError: Into<Self::Error>,
    {
        async {
            // Default: not supported - implementations must override
            Err(crate::NestGateError::internal_error(
                "hash_data not implemented - override this method to provide hashing",
                "canonical_hierarchy",
            )
            .into())
        }
    }

    /// Generate random bytes (optional)
    ///
    /// Default implementation returns "not supported" error.
    /// Override to provide random generation capabilities.
    fn generate_random(
        &self,
        _length: usize,
    ) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send
    where
        Self: Sized,
        crate::NestGateError: Into<Self::Error>,
    {
        async {
            // Default: not supported - implementations must override
            Err(crate::NestGateError::internal_error(
                "generate_random not implemented - override this method to provide random generation",
                "canonical_hierarchy",
            )
            .into())
        }
    }

    /// Derive key from primary key (optional)
    ///
    /// Default implementation returns "not supported" error.
    /// Override to provide key derivation capabilities.
    fn derive_key(
        &self,
        _primary_key: &[u8],
        _salt: &[u8],
        _info: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, Self::Error>> + Send
    where
        Self: Sized,
        crate::NestGateError: Into<Self::Error>,
    {
        async {
            // Default: not supported - implementations must override
            Err(crate::NestGateError::internal_error(
                "derive_key not implemented - override this method to provide key derivation",
                "canonical_hierarchy",
            )
            .into())
        }
    }

    /// Evaluate boundary access control (optional)
    ///
    /// Default implementation delegates to `authorize()`.
    /// Override for more sophisticated boundary access control.
    fn evaluate_boundary_access(
        &self,
        principal: &Self::Principal,
        boundary: &str,
        action: &str,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        self.authorize(principal, boundary, action)
    }

    /// Create session for principal (optional)
    ///
    /// Default implementation creates a token via `authenticate`.
    /// Override for session-based authentication systems.
    fn create_session(
        &self,
        credentials: Self::Credentials,
    ) -> impl Future<Output = Result<Self::Token, Self::Error>> + Send {
        self.authenticate(credentials)
    }

    /// Validate session token (optional)
    ///
    /// Default implementation delegates to `validate_token`.
    /// Override for session-specific validation logic.
    fn validate_session(
        &self,
        token: &Self::Token,
    ) -> impl Future<Output = Result<Self::Principal, Self::Error>> + Send {
        self.validate_token(token)
    }
}
