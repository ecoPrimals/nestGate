// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// User
pub struct User {
    /// Unique user identifier
    pub id: Uuid,
    /// User's chosen username
    pub username: String,
    /// User's email address
    pub email: String,
    /// Account creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last account update timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
/// User login request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Login operation
pub struct LoginRequest {
    /// Username for authentication
    pub username: String,
    /// Password for authentication
    pub password: String,
}
/// Successful login response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Login operation
pub struct LoginResponse {
    /// Authentication token for subsequent requests
    pub token: String,
    /// Authenticated user information
    pub user: User,
}
/// Authentication token with _metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authtoken
pub struct AuthToken {
    /// The authentication token string
    pub token: String,
    /// ID of the user this token belongs to
    pub user_id: Uuid,
    /// Token expiration timestamp
    pub expires_at: chrono::DateTime<chrono::Utc>,
}
/// Re-export universal response types from nestgate-core to eliminate duplication
pub use nestgate_core::response::{ApiResponse as Response, UnifiedErrorResponse as ErrorResponse};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
    }

    #[test]
    fn test_login_request_structure() {
        let request = LoginRequest {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        assert_eq!(request.username, "testuser");
        assert_eq!(request.password, "password123");
    }

    #[test]
    fn test_login_request_serialization() {
        let request = LoginRequest {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok(), "LoginRequest should serialize");

        let json = serialized.expect("Operation failed");
        assert!(json.contains("\"username\":\"testuser\""));
        assert!(json.contains("\"password\":\"password123\""));
    }

    #[test]
    fn test_login_response_structure() {
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let response = LoginResponse {
            token: "test_token_123".to_string(),
            user,
        };

        assert_eq!(response.token, "test_token_123");
        assert_eq!(response.user.username, "testuser");
    }

    #[test]
    fn test_auth_token_expiration() {
        let user_id = Uuid::new_v4();
        let future_time = chrono::Utc::now() + chrono::Duration::hours(1);

        let token = AuthToken {
            token: "test_token".to_string(),
            user_id,
            expires_at: future_time,
        };

        assert_eq!(token.token, "test_token");
        assert_eq!(token.user_id, user_id);
        assert!(
            token.expires_at > chrono::Utc::now(),
            "Token should not be expired"
        );
    }
}
