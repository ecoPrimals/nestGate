//! **AUTHENTICATION MIDDLEWARE**
//!
//! Axum middleware for pluggable authentication.
//! Integrates with nestgate-core's auth provider system.

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use nestgate_core::security::{AuthRequest, AuthResponse, AuthRouter};
use std::sync::Arc;
use tracing::{debug, warn};

/// Authentication middleware state
#[derive(Clone)]
pub struct AuthMiddleware {
    router: Arc<AuthRouter>,
}

impl AuthMiddleware {
    /// Create new authentication middleware
    pub fn new(router: AuthRouter) -> Self {
        Self {
            router: Arc::new(router),
        }
    }

    /// Extract authentication request from HTTP headers
    fn extract_auth_request(headers: &HeaderMap) -> AuthRequest {
        let mut request = AuthRequest::default();

        // Extract JWT token from Authorization header
        if let Some(auth_header) = headers.get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    request.token = Some(token.to_string());
                }
            }
        }

        // Extract BearDog DID from X-Primal-DID header
        if let Some(did_header) = headers.get("x-primal-did") {
            if let Ok(did) = did_header.to_str() {
                request.did = Some(did.to_string());
            }
        }

        // Extract BearDog signature from X-Primal-Signature header
        if let Some(sig_header) = headers.get("x-primal-signature") {
            if let Ok(sig) = sig_header.to_str() {
                request.signature = Some(sig.to_string());
            }
        }

        request
    }
}

/// Axum middleware function
pub async fn auth_middleware(
    State(auth): State<AuthMiddleware>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let auth_request = AuthMiddleware::extract_auth_request(headers);

    debug!("🔐 Authenticating request");

    match auth.router.authenticate(&auth_request).await {
        Ok(auth_response) => {
            if auth_response.authenticated {
                debug!(
                    "✅ Authentication successful: {} via {}",
                    auth_response.principal.as_deref().unwrap_or("unknown"),
                    auth_response.auth_method
                );

                // TODO: Store auth_response in request extensions
                // so handlers can access authenticated principal and permissions
                
                Ok(next.run(request).await)
            } else {
                warn!("❌ Authentication failed: {}", auth_response.message);
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        Err(e) => {
            warn!("❌ Authentication error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_extract_jwt_token() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_static("Bearer test-token-123"),
        );

        let request = AuthMiddleware::extract_auth_request(&headers);
        assert_eq!(request.token, Some("test-token-123".to_string()));
    }

    #[test]
    fn test_extract_beardog_did() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-primal-did",
            HeaderValue::from_static("did:primal:beardog:123"),
        );

        let request = AuthMiddleware::extract_auth_request(&headers);
        assert_eq!(request.did, Some("did:primal:beardog:123".to_string()));
    }

    #[test]
    fn test_extract_beardog_signature() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-primal-signature",
            HeaderValue::from_static("abcd1234"),
        );

        let request = AuthMiddleware::extract_auth_request(&headers);
        assert_eq!(request.signature, Some("abcd1234".to_string()));
    }

    #[test]
    fn test_extract_combined_beardog_auth() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-primal-did",
            HeaderValue::from_static("did:primal:beardog:123"),
        );
        headers.insert(
            "x-primal-signature",
            HeaderValue::from_static("abcd1234"),
        );

        let request = AuthMiddleware::extract_auth_request(&headers);
        assert_eq!(request.did, Some("did:primal:beardog:123".to_string()));
        assert_eq!(request.signature, Some("abcd1234".to_string()));
    }
}

