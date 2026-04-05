// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! External identity providers (OAuth, SAML, `IdPs`).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// External authentication provider entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAuthProvider {
    /// Provider name
    pub name: String,
    /// Provider type
    pub provider_type: ExternalProviderType,
    /// Provider configuration
    pub config: HashMap<String, String>,
    /// Enabled status
    pub enabled: bool,
    /// Priority order
    pub priority: u32,
}

/// External provider integration kind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalProviderType {
    /// Oauth2
    OAuth2,
    /// Saml
    Saml,
    /// Ldap
    Ldap,
    /// Activedirectory
    ActiveDirectory,
    /// Google
    Google,
    /// Microsoft
    Microsoft,
    /// Github
    GitHub,
    /// Okta
    Okta,
    /// Auth0
    Auth0,
    /// Custom authentication provider
    Custom(String),
}
