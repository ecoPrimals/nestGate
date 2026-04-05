// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Primary and secondary authentication method identifiers.

use serde::{Deserialize, Serialize};

/// Authentication method variants (password, OAuth, LDAP, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    /// Username/password authentication
    UsernamePassword,
    /// Certificate-based authentication
    Certificate,
    /// Token-based authentication (JWT, API keys)
    Token,
    /// OAuth 2.0 / `OpenID` Connect
    OAuth2,
    /// SAML authentication
    Saml,
    /// LDAP authentication
    Ldap,
    /// Biometric authentication
    Biometric,
    /// Hardware token authentication
    HardwareToken,
    /// Custom authentication method
    Custom(String),
}
