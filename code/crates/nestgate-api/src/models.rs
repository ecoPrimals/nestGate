use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct LoginRequest {
    /// Username for authentication
    pub username: String,
    /// Password for authentication
    pub password: String,
}
/// Successful login response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    /// Authentication token for subsequent requests
    pub token: String,
    /// Authenticated user information
    pub user: User,
}
/// Authentication token with _metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
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
