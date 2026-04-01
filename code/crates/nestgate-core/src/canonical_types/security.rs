// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **SECURITY TYPES** — Authentication, authorization, and encryption

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Authmethod
pub enum AuthMethod {
    /// Token-based authentication
    Token,
    /// API key authentication
    ApiKey,
    /// Certificate-based authentication
    Certificate,
    /// OAuth2 authentication
    OAuth2,
    /// Basic authentication (username/password)
    Basic,
    /// No authentication required
    None,
}

/// Access levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Accesslevel
pub enum AccessLevel {
    /// Read-only access
    Read,
    /// Read and write access
    Write,
    /// Administrative access
    Admin,
    /// Owner-level access with full control
    Owner,
}

/// Security context
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Securitycontext
pub struct SecurityContext {
    /// Optional user identifier
    pub user_id: Option<String>,
    /// User roles for role-based access control
    pub roles: Vec<String>,
    /// Granted permissions
    pub permissions: Vec<String>,
    /// Access level for this context
    pub access_level: AccessLevel,
    /// Authentication method used
    pub auth_method: AuthMethod,
    /// Optional token expiration timestamp
    pub token_expires_at: Option<SystemTime>,
}
