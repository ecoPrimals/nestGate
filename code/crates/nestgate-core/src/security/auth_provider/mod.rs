//! **AUTHENTICATION PROVIDER SUBSYSTEM**
//!
//! Pluggable authentication architecture for NestGate.
//! 
//! ## Design
//! - NestGate is a data service, not a security primal
//! - Authentication is a capability, delegated to providers
//! - Multiple providers can coexist (JWT, BearDog, future systems)
//! - Configuration-driven routing between providers
//!
//! ## Providers
//! - **BearDog**: Primary for primal-to-primal (decentralized, cryptographic)
//! - **JWT**: Legacy for NAS and external clients (shared secret)
//!
//! ## Usage
//! ```no_run
//! use nestgate_core::security::auth_provider::*;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create router with auto-detect mode
//! let mut router = AuthRouter::new(AuthMode::Auto);
//!
//! // Register providers
//! router.register_provider(Box::new(BearDogAuthProvider::new()));
//! router.register_provider(Box::new(JwtAuthProvider::new()));
//!
//! // Authenticate request
//! let request = AuthRequest {
//!     did: Some("did:primal:beardog:123".to_string()),
//!     signature: Some("abcd1234".to_string()),
//!     ..Default::default()
//! };
//!
//! let response = router.authenticate(&request).await?;
//! assert!(response.authenticated);
//! # Ok(())
//! # }
//! ```

mod jwt_provider;
mod beardog_provider;

pub use jwt_provider::JwtAuthProvider;
pub use beardog_provider::BearDogAuthProvider;

// Re-export core types from parent module
pub use super::auth_provider::{
    AuthMode, AuthProvider, AuthRequest, AuthResponse, AuthRouter, ProviderStatus,
};

/// Create a default authentication router with both providers
///
/// This is a convenience function that creates a router in Auto mode
/// with both JWT and BearDog providers registered.
pub fn create_default_router() -> AuthRouter {
    let mut router = AuthRouter::new(AuthMode::Auto);
    
    // Register BearDog (primary)
    router.register_provider(Box::new(BearDogAuthProvider::new()));
    
    // Register JWT (legacy fallback)
    router.register_provider(Box::new(JwtAuthProvider::new()));
    
    router
}

/// Create an authentication router from environment configuration
///
/// Reads NESTGATE_AUTH_MODE environment variable:
/// - "beardog": BearDog only (strict)
/// - "jwt": JWT only (legacy)
/// - "auto": Auto-detect (default)
/// - "none": No auth (development only)
pub fn create_router_from_env() -> AuthRouter {
    use std::env;
    
    let mode_str = env::var("NESTGATE_AUTH_MODE")
        .unwrap_or_else(|_| "auto".to_string())
        .to_lowercase();

    let mode = match mode_str.as_str() {
        "beardog" => AuthMode::BearDog,
        "jwt" => AuthMode::Jwt,
        "auto" => AuthMode::Auto,
        "none" => {
            tracing::warn!("⚠️  Authentication disabled (NESTGATE_AUTH_MODE=none). Only use in development!");
            AuthMode::None
        }
        _ => {
            tracing::warn!(
                "⚠️  Unknown auth mode '{}', defaulting to 'auto'",
                mode_str
            );
            AuthMode::Auto
        }
    };

    tracing::info!("🔐 Creating auth router with mode: {}", mode);

    let mut router = AuthRouter::new(mode);
    
    // Always register both providers (router will choose based on mode)
    router.register_provider(Box::new(BearDogAuthProvider::new()));
    router.register_provider(Box::new(JwtAuthProvider::new()));
    
    router
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_router() {
        let router = create_default_router();
        assert_eq!(*router.mode(), AuthMode::Auto);
    }

    #[tokio::test]
    async fn test_default_router_providers() {
        let router = create_default_router();
        let statuses = router.get_status().await;
        
        // Should have both providers registered
        assert_eq!(statuses.len(), 2);
        
        let provider_names: Vec<&str> = statuses.iter().map(|s| s.name.as_str()).collect();
        assert!(provider_names.contains(&"beardog"));
        assert!(provider_names.contains(&"jwt"));
    }

    #[test]
    fn test_create_router_from_env_default() {
        std::env::remove_var("NESTGATE_AUTH_MODE");
        let router = create_router_from_env();
        assert_eq!(*router.mode(), AuthMode::Auto);
    }

    #[test]
    fn test_create_router_from_env_beardog() {
        std::env::set_var("NESTGATE_AUTH_MODE", "beardog");
        let router = create_router_from_env();
        assert_eq!(*router.mode(), AuthMode::BearDog);
        std::env::remove_var("NESTGATE_AUTH_MODE");
    }

    #[test]
    fn test_create_router_from_env_jwt() {
        std::env::set_var("NESTGATE_AUTH_MODE", "jwt");
        let router = create_router_from_env();
        assert_eq!(*router.mode(), AuthMode::Jwt);
        std::env::remove_var("NESTGATE_AUTH_MODE");
    }

    #[test]
    fn test_create_router_from_env_none() {
        std::env::set_var("NESTGATE_AUTH_MODE", "none");
        let router = create_router_from_env();
        assert_eq!(*router.mode(), AuthMode::None);
        std::env::remove_var("NESTGATE_AUTH_MODE");
    }
}

