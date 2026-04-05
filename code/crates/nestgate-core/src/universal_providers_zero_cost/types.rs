// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

// **CANONICAL SECURITY TYPES** - Replacing universal_traits imports
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authtoken
pub struct AuthToken {
    /// Token
    pub token: String,
    /// Expires At
    pub expires_at: SystemTime,
    /// Permissions
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Credentials
pub struct Credentials {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
    /// Additional Data
    pub additional_data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Signature
pub struct Signature {
    /// Algorithm
    pub algorithm: String,
    /// Signature
    pub signature: Vec<u8>,
    /// Public Key
    pub public_key: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Security decision with context and remediation information
///
/// Represents the outcome of a security check with detailed reasoning
/// and actionable information for the user.
pub enum SecurityDecision {
    /// Access is allowed
    ///
    /// The requested operation has been approved based on security policies.
    Allow {
        /// Explanation of why access was granted
        reason: String,
        /// Whether the decision was enhanced by a security provider
        ///
        /// `true` if a dedicated security primal participated in the decision,
        /// `false` if using built-in security logic only
        enhanced_by_security_provider: bool,
    },
    /// Access is denied
    ///
    /// The requested operation has been rejected by security policies.
    Deny {
        /// Explanation of why access was denied
        ///
        /// Should be user-friendly and respectful, explaining the security
        /// concern without exposing sensitive system details.
        reason: String,
        /// Optional guidance on how to gain access
        ///
        /// Provides actionable steps the user can take to resolve the issue,
        /// such as requesting permissions or providing additional credentials.
        remediation: Option<String>,
    },
    /// License agreement required
    ///
    /// The requested operation requires acceptance of specific terms.
    RequireLicense {
        /// License terms that must be accepted
        terms: String,
        /// Contact information for license inquiries
        contact: String,
    },
}
