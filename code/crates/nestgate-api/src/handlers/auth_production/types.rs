// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Request and response types for production authentication handlers.

use serde::{Deserialize, Serialize};

/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authcredentials
pub struct AuthCredentials {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
}

/// Authentication response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Auth operation
pub struct AuthResponse {
    /// Success
    pub success: bool,
    /// Token
    pub token: Option<String>,
    /// User identifier
    pub user_id: Option<String>,
    /// Role
    pub role: String,
    /// Permissions
    pub permissions: Vec<String>,
}

/// API key creation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyRequest {
    /// User identifier
    pub user_id: String,
    /// Name
    pub name: String,
}

/// API key response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    /// Api Key
    pub api_key: String,
    /// User identifier
    pub user_id: String,
    /// Name
    pub name: String,
}

/// User creation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    /// User identifier
    pub user_id: String,
    /// Username
    pub username: String,
    /// Role
    pub role: String,
    /// Permissions
    pub permissions: Vec<String>,
}
