// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! In-process auth manager backing [`super::handler::ProductionAuthHandler`].
//!
//! This is a lightweight registry used by production auth HTTP handlers until a single
//! `nestgate-security` integration path is wired for this module.

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Role assignment for RBAC-style checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Administrator
    Admin,
    /// Operator — used when identity provider is wired
    #[expect(dead_code, reason = "forward-looking RBAC variant for IdP integration")]
    Operator,
    /// Service account — used when identity provider is wired
    #[expect(dead_code, reason = "forward-looking RBAC variant for IdP integration")]
    Service,
    /// Read-only — used when identity provider is wired
    #[expect(dead_code, reason = "forward-looking RBAC variant for IdP integration")]
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
    #[expect(dead_code, reason = "reserved for authorization logging")]
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
    pub const fn role(&self) -> &Role {
        &self.role
    }
}

#[derive(Debug)]
#[expect(dead_code, reason = "fields used by AuthManager internally")]
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
            String::from("admin"),
            UserRecord {
                user_id: String::from("admin"),
                username: String::from("admin"),
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
    #[expect(dead_code, reason = "reserved for user management API")]
    pub fn user_exists(&self, username: &str) -> Result<(), String> {
        if self.users.values().any(|u| u.username == username) {
            Ok(())
        } else {
            Err(format!("user not found: {username}"))
        }
    }

    /// Registers or replaces a user entry.
    #[expect(
        dead_code,
        reason = "forward-looking API for IdP-backed user provisioning"
    )]
    pub fn add_user(
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
    pub fn add_api_key(&mut self, api_key: String, user_id: String) {
        self.api_keys.insert(api_key, user_id);
    }

    /// Validates an API key and returns [`AuthContext`] when known.
    pub fn validate_api_key(&self, api_key: &str) -> Result<AuthContext, String> {
        let user_id = self.api_keys.get(api_key).ok_or("invalid api key")?.clone();
        let role = self.users.get(&user_id).map_or(Role::User, |u| u.role);
        Ok(AuthContext { uid: user_id, role })
    }
}

impl Default for AuthManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_user_and_validate_api_key() {
        let mut mgr = AuthManager::new();
        mgr.add_user(
            String::from("u1"),
            String::from("alice"),
            Role::Operator,
            vec![Permission::new(&String::from("read"))],
        );
        mgr.add_api_key(String::from("key-1"), String::from("u1"));

        let ctx = mgr.validate_api_key("key-1").expect("valid key");
        assert_eq!(ctx.role, Role::Operator);
    }

    #[test]
    fn all_role_variants_display() {
        for role in [
            Role::Admin,
            Role::Operator,
            Role::Service,
            Role::ReadOnly,
            Role::User,
        ] {
            assert!(!role.to_string().is_empty());
        }
    }

    #[test]
    fn invalid_api_key_returns_error() {
        let mgr = AuthManager::new();
        assert!(mgr.validate_api_key("nonexistent").is_err());
    }
}
