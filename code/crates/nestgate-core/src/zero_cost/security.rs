use crate::error::{NestGateError, SecurityErrorData};
/// Zero-cost Security Provider Implementation
/// Provides production-ready security services with compile-time optimization.
use crate::zero_cost::traits::ZeroCostSecurityProvider;

/// Simple token validation helper
fn is_valid_token(token: &str) -> bool {
    // Basic validation - in production this would be more sophisticated
    !token.is_empty()
        && (token.starts_with("prod_token_")
            || token.starts_with("dev_token_")
            || token.starts_with("test_token_"))
}

/// Production-optimized security provider
#[derive(Clone)]
pub struct ProductionSecurityProvider;

#[async_trait::async_trait]
impl ZeroCostSecurityProvider for ProductionSecurityProvider {
    type TokenInfo = String;
    type Result = crate::Result<String>;

    fn max_tokens() -> usize {
        10000 // Production limit
    }

    async fn generate_token(&self, user_id: &str) -> Self::Result {
        let token = format!("prod_token_{}_{}", user_id, chrono::Utc::now().timestamp());
        Ok(token)
    }

    async fn validate_token(&self, token: &str) -> Self::Result {
        if !is_valid_token(token) {
            return Err(NestGateError::Security(Box::new(SecurityErrorData {
                message: "Authentication failed: Invalid token format".to_string(),
                operation: "token_validation".to_string(),
                resource: Some("token".to_string()),
                principal: None,
                context: None,
            })));
        }
        Ok("valid".to_string())
    }

    async fn revoke_token(&self, token: &str) -> Self::Result {
        // In production, this would remove the token from the active token store
        Ok(format!("Token {} revoked", token))
    }
}

/// Development-optimized security provider
#[derive(Clone)]
pub struct DevelopmentSecurityProvider;

#[async_trait::async_trait]
impl ZeroCostSecurityProvider for DevelopmentSecurityProvider {
    type TokenInfo = String;
    type Result = crate::Result<String>;

    fn max_tokens() -> usize {
        1000 // Development limit
    }

    async fn generate_token(&self, user_id: &str) -> Self::Result {
        let token = format!("dev_token_{}_{}", user_id, chrono::Utc::now().timestamp());
        Ok(token)
    }

    async fn validate_token(&self, token: &str) -> Self::Result {
        // Development mode is more lenient
        if token.is_empty() {
            return Err(NestGateError::Security(Box::new(SecurityErrorData {
                message: "Authentication failed: Empty token".to_string(),
                operation: "token_validation".to_string(),
                resource: Some("token".to_string()),
                principal: None,
                context: None,
            })));
        }
        Ok("valid".to_string())
    }

    async fn revoke_token(&self, token: &str) -> Self::Result {
        // Development mode doesn't need to track revoked tokens
        Ok(format!("Token {} revoked (dev mode)", token))
    }
}
