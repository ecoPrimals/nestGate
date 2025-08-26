use std::future::Future;
//
// **CANONICAL MODERNIZATION COMPLETE** - Zero-cost trait definitions
// that provide compile-time optimization and eliminate runtime overhead.


// REMOVED: unused import
// use crate::error::CanonicalResult as Result;

/// **ZERO-COST SECURITY PROVIDER**
/// 
/// High-performance security provider with compile-time optimization
pub trait ZeroCostSecurityProvider: Send + Sync + 'static {
    type Token: Send + Sync + Clone;
    type Credentials: Send + Sync;
    type Error: Send + Sync + std::error::Error;

    /// Authenticate with zero-cost async
    fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> impl Future<Output = std::result::Result<Self::Token, Self::Error>> + Send;

    /// Validate token with zero-cost async
    fn validate_token(
        &self,
        token: &Self::Token,
    ) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;
}

/// **ZERO-COST PROVIDER**
/// 
/// Generic zero-cost provider interface
pub trait ZeroCostProvider<T>: Send + Sync + 'static {
    type Error: Send + Sync + std::error::Error;

    /// Provide service with zero-cost async
    fn provide(&self) -> impl Future<Output = std::result::Result<T, Self::Error>> + Send;

    /// Check if provider can provide the service
    fn can_provide(&self) -> bool;
}

/// **ZERO-COST STORAGE**
/// 
/// High-performance storage operations with compile-time optimization
pub trait ZeroCostStorage: Send + Sync + 'static {
    type Error: Send + Sync + std::error::Error;

    /// Read data with zero-cost async
    fn read(
        &self,
        path: &str,
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Write data with zero-cost async
    fn write(
        &self,
        path: &str,
        data: &[u8],
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Delete data with zero-cost async
    fn delete(
        &self,
        path: &str,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;
}

/// **ZERO-COST SECURITY**
/// 
/// Security operations with compile-time optimization
pub trait ZeroCostSecurity: Send + Sync + 'static {
    type Token: Send + Sync + Clone;
    type Error: Send + Sync + std::error::Error;

    /// Encrypt data with zero-cost async
    fn encrypt(
        &self,
        data: &[u8],
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;

    /// Decrypt data with zero-cost async
    fn decrypt(
        &self,
        encrypted: &[u8],
    ) -> impl Future<Output = std::result::Result<Vec<u8>, Self::Error>> + Send;
}

/// **ZERO-COST NETWORK**
/// 
/// Network operations with compile-time optimization
pub trait ZeroCostNetwork: Send + Sync + 'static {
    type Response: Send + Sync;
    type Error: Send + Sync + std::error::Error;

    /// Send request with zero-cost async
    fn send_request(
        &self,
        endpoint: &str,
        data: &[u8],
    ) -> impl Future<Output = std::result::Result<Self::Response, Self::Error>> + Send;

    /// Check connection health with zero-cost async
    fn health_check(
        &self,
    ) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;
} 