
//! **PLUGGABLE AUTHENTICATION ARCHITECTURE**
//!
//! NestGate is a data service, not a security primal.
//! This module provides a capability-based authentication system that supports
//! multiple authentication providers (JWT, BearDog, future systems).
//!
//! ## Design Philosophy
//! - **Provider-agnostic**: Auth is a capability, not a core concern
//! - **Pluggable**: Easy to add new auth providers
//! - **Testable**: Both eco-internal and external auth can be tested
//! - **Fallback-ready**: Gracefully degrades if security primal unavailable

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use crate::Result;
use crate::error::NestGateError;

/// Authentication request from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    /// Optional JWT token
    pub token: Option<String>,
    /// Optional DID (Decentralized Identifier) for BearDog
    pub did: Option<String>,
    /// Optional cryptographic signature for BearDog
    pub signature: Option<String>,
    /// Request payload being signed (for BearDog)
    pub payload: Option<Vec<u8>>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Default for AuthRequest {
    fn default() -> Self {
        Self {
            token: None,
            did: None,
            signature: None,
            payload: None,
            metadata: HashMap::new(),
        }
    }
}

/// Authentication response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Whether authentication succeeded
    pub authenticated: bool,
    /// Authenticated user/service identifier
    pub principal: Option<String>,
    /// Granted permissions
    pub permissions: Vec<String>,
    /// Authentication method used
    pub auth_method: String,
    /// Additional metadata from provider
    pub metadata: HashMap<String, String>,
    /// Human-readable message
    pub message: String,
}

impl AuthResponse {
    /// Create a successful authentication response
    pub fn success(principal: String, permissions: Vec<String>, auth_method: String) -> Self {
        Self {
            authenticated: true,
            principal: Some(principal),
            permissions,
            auth_method,
            metadata: HashMap::new(),
            message: "Authentication successful".to_string(),
        }
    }

    /// Create a failed authentication response
    pub fn failure(reason: &str, auth_method: String) -> Self {
        Self {
            authenticated: false,
            principal: None,
            permissions: Vec::new(),
            auth_method,
            metadata: HashMap::new(),
            message: format!("Authentication failed: {}", reason),
        }
    }
}

/// Authentication provider trait
///
/// Implementors provide pluggable authentication mechanisms.
/// NestGate delegates auth to these providers based on configuration.
#[async_trait]
pub trait AuthProvider: Send + Sync + fmt::Debug {
    /// Provider name (e.g., "jwt", "beardog")
    fn name(&self) -> &str;

    /// Check if this provider can handle the given request
    fn can_handle(&self, request: &AuthRequest) -> bool;

    /// Authenticate the request
    async fn authenticate(&self, request: &AuthRequest) -> Result<AuthResponse>;

    /// Check if the provider is available/healthy
    async fn is_available(&self) -> bool;

    /// Get provider configuration/status
    fn status(&self) -> ProviderStatus;
}

/// Provider status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderStatus {
    /// Provider name
    pub name: String,
    /// Is the provider available?
    pub available: bool,
    /// Provider mode (e.g., "legacy", "primary", "fallback")
    pub mode: String,
    /// Additional status information
    pub info: HashMap<String, String>,
}

/// Authentication mode configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AuthMode {
    /// BearDog cryptographic authentication (default for primal network)
    BearDog,
    /// JWT with shared secret (legacy, for NAS and external clients)
    Jwt,
    /// Auto-detect based on request (try BearDog first, fallback to JWT)
    Auto,
    /// No authentication (development only)
    None,
}

impl Default for AuthMode {
    fn default() -> Self {
        Self::BearDog // Default to BearDog for sovereignty
    }
}

impl fmt::Display for AuthMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BearDog => write!(f, "beardog"),
            Self::Jwt => write!(f, "jwt"),
            Self::Auto => write!(f, "auto"),
            Self::None => write!(f, "none"),
        }
    }
}

/// Authentication router - selects the appropriate provider
#[derive(Debug)]
pub struct AuthRouter {
    /// Configured authentication mode
    mode: AuthMode,
    /// Available authentication providers
    providers: Vec<Box<dyn AuthProvider>>,
}

impl AuthRouter {
    /// Create a new authentication router
    pub fn new(mode: AuthMode) -> Self {
        Self {
            mode,
            providers: Vec::new(),
        }
    }

    /// Register an authentication provider
    pub fn register_provider(&mut self, provider: Box<dyn AuthProvider>) {
        tracing::info!("📝 Registered auth provider: {}", provider.name());
        self.providers.push(provider);
    }

    /// Authenticate a request using the appropriate provider
    pub async fn authenticate(&self, request: &AuthRequest) -> Result<AuthResponse> {
        match self.mode {
            AuthMode::None => {
                tracing::warn!("⚠️  Authentication disabled (development mode)");
                return Ok(AuthResponse::success(
                    "dev-user".to_string(),
                    vec!["read".to_string(), "write".to_string()],
                    "none".to_string(),
                ));
            }
            AuthMode::BearDog => {
                // Try BearDog provider only
                if let Some(provider) = self.find_provider("beardog") {
                    if provider.can_handle(request) {
                        return provider.authenticate(request).await;
                    }
                }
                return Err(NestGateError::Security(Box::new(
                    crate::error::SecurityErrorData {
                        message: "BearDog authentication required but provider not available or request invalid".to_string(),
                        principal: None,
                    },
                )));
            }
            AuthMode::Jwt => {
                // Try JWT provider only
                if let Some(provider) = self.find_provider("jwt") {
                    if provider.can_handle(request) {
                        return provider.authenticate(request).await;
                    }
                }
                return Err(NestGateError::Security(Box::new(
                    crate::error::SecurityErrorData {
                        message: "JWT authentication required but provider not available or request invalid".to_string(),
                        principal: None,
                    },
                )));
            }
            AuthMode::Auto => {
                // Try BearDog first (preferred), then JWT (legacy)
                if let Some(beardog) = self.find_provider("beardog") {
                    if beardog.can_handle(request) {
                        tracing::debug!("🐻 Using BearDog auth (primary)");
                        return beardog.authenticate(request).await;
                    }
                }

                if let Some(jwt) = self.find_provider("jwt") {
                    if jwt.can_handle(request) {
                        tracing::debug!("🔑 Using JWT auth (legacy fallback)");
                        return jwt.authenticate(request).await;
                    }
                }

                return Err(NestGateError::Security(Box::new(
                    crate::error::SecurityErrorData {
                        message: "No authentication provider could handle the request".to_string(),
                        principal: None,
                    },
                )));
            }
        }
    }

    /// Find a provider by name
    fn find_provider(&self, name: &str) -> Option<&Box<dyn AuthProvider>> {
        self.providers.iter().find(|p| p.name() == name)
    }

    /// Get status of all providers
    pub async fn get_status(&self) -> Vec<ProviderStatus> {
        let mut statuses = Vec::new();
        for provider in &self.providers {
            let mut status = provider.status();
            status.available = provider.is_available().await;
            statuses.push(status);
        }
        statuses
    }

    /// Get current authentication mode
    pub fn mode(&self) -> &AuthMode {
        &self.mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock provider for testing
    #[derive(Debug)]
    struct MockProvider {
        name: String,
        can_handle: bool,
    }

    #[async_trait]
    impl AuthProvider for MockProvider {
        fn name(&self) -> &str {
            &self.name
        }

        fn can_handle(&self, _request: &AuthRequest) -> bool {
            self.can_handle
        }

        async fn authenticate(&self, request: &AuthRequest) -> Result<AuthResponse> {
            Ok(AuthResponse::success(
                "test-user".to_string(),
                vec!["read".to_string()],
                self.name.clone(),
            ))
        }

        async fn is_available(&self) -> bool {
            true
        }

        fn status(&self) -> ProviderStatus {
            ProviderStatus {
                name: self.name.clone(),
                available: true,
                mode: "test".to_string(),
                info: HashMap::new(),
            }
        }
    }

    #[tokio::test]
    async fn test_auth_mode_none() {
        let router = AuthRouter::new(AuthMode::None);
        let request = AuthRequest {
            token: None,
            did: None,
            signature: None,
            payload: None,
            metadata: HashMap::new(),
        };

        let response = router.authenticate(&request).await.unwrap();
        assert!(response.authenticated);
        assert_eq!(response.auth_method, "none");
    }

    #[tokio::test]
    async fn test_provider_registration() {
        let mut router = AuthRouter::new(AuthMode::Auto);
        
        let provider = Box::new(MockProvider {
            name: "test-provider".to_string(),
            can_handle: true,
        });
        
        router.register_provider(provider);
        
        let statuses = router.get_status().await;
        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].name, "test-provider");
    }

    #[test]
    fn test_auth_response_success() {
        let response = AuthResponse::success(
            "user123".to_string(),
            vec!["read".to_string(), "write".to_string()],
            "jwt".to_string(),
        );

        assert!(response.authenticated);
        assert_eq!(response.principal, Some("user123".to_string()));
        assert_eq!(response.permissions.len(), 2);
    }

    #[test]
    fn test_auth_response_failure() {
        let response = AuthResponse::failure("Invalid token", "jwt".to_string());

        assert!(!response.authenticated);
        assert_eq!(response.principal, None);
        assert_eq!(response.permissions.len(), 0);
        assert!(response.message.contains("Invalid token"));
    }
}

