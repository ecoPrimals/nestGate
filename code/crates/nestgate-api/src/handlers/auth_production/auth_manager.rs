// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! In-process auth manager backing [`super::handler::ProductionAuthHandler`].
//!
//! This is a lightweight registry used by production auth HTTP handlers until a single
//! `nestgate-security` integration path is wired for this module.

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

/// API token classification (mirrors legacy handler expectations).
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    /// API key style token
    ApiKey,
}

/// Opaque auth token handle returned to clients.
#[derive(Debug, Clone)]
pub struct AuthToken {
    /// Raw token string
    pub token: String,
}

impl AuthToken {
    /// Creates a token with the given classification (classification is reserved for future use).
    #[must_use]
    pub fn new(token: String, _ty: TokenType) -> Self {
        Self { token }
    }
}

/// Role assignment for RBAC-style checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Administrator
    Admin,
    /// Operator
    Operator,
    /// Service account
    Service,
    /// Read-only
    ReadOnly,
    /// Standard user
    User,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Admin => "admin",
            Self::Operator => "operator",
            Self::Service => "service",
            Self::ReadOnly => "read_only",
            Self::User => "user",
        };
        f.write_str(s)
    }
}

/// Named permission string.
#[derive(Debug, Clone)]
pub struct Permission {
    name: String,
}

impl Permission {
    /// Wraps a permission label.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Permission label.
    #[must_use]
    #[allow(dead_code)] // Reserved for future authorization logging / policy surfaces
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Hash for Permission {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Permission {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Permission {}

/// Context returned after validating an API key.
#[derive(Debug, Clone)]
pub struct AuthContext {
    uid: String,
    role: Role,
}

impl AuthContext {
    /// User identifier bound to the key.
    #[must_use]
    pub fn user_id(&self) -> &str {
        &self.uid
    }

    /// Role for authorization hints.
    #[must_use]
    pub fn role(&self) -> &Role {
        &self.role
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct UserRecord {
    user_id: String,
    username: String,
    role: Role,
    permissions: Vec<Permission>,
}

/// Registry for users and API keys (async API matches prior handler code).
#[derive(Debug)]
pub struct AuthManager {
    users: HashMap<String, UserRecord>,
    api_keys: HashMap<String, String>,
}

impl AuthManager {
    /// Creates a manager with a default `admin` user (matches existing tests).
    #[must_use]
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(
            "admin".to_string(),
            UserRecord {
                user_id: "admin".to_string(),
                username: "admin".to_string(),
                role: Role::Admin,
                permissions: vec![Permission::new("all")],
            },
        );
        Self {
            users,
            api_keys: HashMap::new(),
        }
    }

    /// Whether a username is registered.
    #[allow(dead_code)]
    pub async fn user_exists(&self, username: &str) -> Result<(), String> {
        if self.users.values().any(|u| u.username == username) {
            Ok(())
        } else {
            Err(format!("user not found: {username}"))
        }
    }

    /// Registers or replaces a user entry.
    pub async fn add_user(
        &mut self,
        user_id: String,
        username: String,
        role: Role,
        permissions: Vec<Permission>,
    ) {
        self.users.insert(
            user_id.clone(),
            UserRecord {
                user_id,
                username,
                role,
                permissions,
            },
        );
    }

    /// Associates an API key with a user id.
    pub async fn add_api_key(&mut self, api_key: String, user_id: String) {
        self.api_keys.insert(api_key, user_id);
    }

    /// Validates an API key and returns [`AuthContext`] when known.
    pub async fn validate_api_key(&self, api_key: &str) -> Result<AuthContext, String> {
        let user_id = self
            .api_keys
            .get(api_key)
            .cloned()
            .ok_or_else(|| "invalid api key".to_string())?;
        let role = self
            .users
            .get(&user_id)
            .map(|u| u.role)
            .unwrap_or(Role::User);
        Ok(AuthContext { uid: user_id, role })
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}
