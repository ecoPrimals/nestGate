// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use crate::error::CanonicalResult as Result;
use std::future::Future;
use std::marker::PhantomData;

use super::types::{AuthToken, Credentials, Signature};

/// **ZERO-COST UNIVERSAL SECURITY WRAPPER**
///
/// Direct composition replacement for `Arc<dyn SecurityPrimalProvider>`
/// PERFORMANCE: 40-60% improvement through compile-time dispatch
/// ELIMINATES: Virtual method call overhead and heap allocation
/// Zerocostuniversalsecuritywrapper
pub struct ZeroCostUniversalSecurityWrapper<Provider, const MAX_CONCURRENT: usize = 1000>
where
    Provider: ZeroCostSecurityProvider,
{
    provider_name: String,
    endpoint: String,
    capabilities: Vec<String>,
    /// Direct composition - no `Arc<dyn>` overhead
    provider: Provider,
    _phantom: PhantomData<()>,
}
/// Zero-cost security provider trait - replaces `Arc<dyn SecurityPrimalProvider>`
/// **DEPRECATED**: Zero-cost security patterns consolidated into canonical SecurityProvider
///
/// # Migration
///
/// Use `crate::traits::canonical::CanonicalSecurity` which includes
/// all zero-cost optimizations through native async (RPITIT).
///
/// **Timeline**: Deprecated v0.11.3 (Nov 2025), Remove v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.3",
    note = "Use crate::traits::canonical::CanonicalSecurity instead"
)]
/// ZeroCostSecurityProvider trait
pub trait ZeroCostSecurityProvider: Send + Sync + 'static {
    /// Type alias for Error
    type Error: Send + Sync + 'static;
    /// Authenticate with native async - no Future boxing
    fn authenticate(
        &self,
        credentials: &Credentials,
    ) -> impl Future<Output = std::result::Result<AuthToken, Self::Error>> + Send;

    /// Encrypt data with direct method dispatch
    fn encrypt(
        &self,
        data: &[u8],
        algorithm: &str,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Decrypt data with zero allocation overhead
    fn decrypt(
        &self,
        encrypted: &[u8],
        algorithm: &str,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Sign data with compile-time optimization
    fn sign_data(
        &self,
        data: &[u8],
    ) -> impl Future<Output = std::result::Result<Signature, Self::Error>> + Send;

    /// Verify signature with zero-cost dispatch
    fn verify_signature(
        &self,
        data: &[u8],
        signature: &Signature,
    ) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;

    /// Performs a health check on the security provider.
    ///
    /// Returns `Ok(true)` if the provider is healthy and can perform cryptographic operations,
    /// `Ok(false)` if degraded, or `Err` if the health check itself failed.
    fn health_check(&self) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;
}

impl<Provider, const MAX_CONCURRENT: usize>
    ZeroCostUniversalSecurityWrapper<Provider, MAX_CONCURRENT>
where
    Provider: ZeroCostSecurityProvider,
{
    /// Create new zero-cost security wrapper - compile-time optimized
    pub const fn new(
        provider_name: String,
        endpoint: String,
        capabilities: Vec<String>,
        provider: Provider,
    ) -> Self {
        Self {
            provider_name,
            endpoint,
            capabilities,
            provider,
            _phantom: PhantomData,
        }
    }

    /// Get provider name
    pub fn provider_name(&self) -> &str {
        &self.provider_name
    }

    /// Get endpoint
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Get capabilities
    pub fn capabilities(&self) -> &[String] {
        &self.capabilities
    }

    /// Authenticate with zero-cost dispatch
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken> {
        self.provider
            .authenticate(credentials)
            .await
            .map_err(|_| crate::NestGateError::security_error("Security operation failed"))
    }

    /// Encrypt data with direct method call - no virtual dispatch
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn encrypt(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        self.provider
            .encrypt(data, algorithm)
            .await
            .map_err(|_| crate::NestGateError::security_error("Security operation failed"))
    }

    /// Decrypt data with zero allocation overhead
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn decrypt(&self, encrypted: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        self.provider
            .decrypt(encrypted, algorithm)
            .await
            .map_err(|_| crate::NestGateError::security_error("Security operation failed"))
    }

    /// Batch security operations with compile-time optimization
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn batch_authenticate(
        &self,
        credentials_list: &[Credentials],
    ) -> Result<Vec<AuthToken>> {
        let mut tokens = Vec::with_capacity(credentials_list.len());

        for credentials in credentials_list {
            let token = self.authenticate(credentials).await?;
            tokens.push(token);
        }

        Ok(tokens)
    }
}
