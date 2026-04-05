// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Password policy and account lockout rules.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Password complexity and history rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicyConfig {
    /// Minimum password length
    pub min_length: u32,
    /// Maximum password length
    pub max_length: u32,
    /// Require uppercase letters
    pub require_uppercase: bool,
    /// Require lowercase letters
    pub require_lowercase: bool,
    /// Require numbers
    pub require_numbers: bool,
    /// Require special characters
    pub require_special_chars: bool,
    /// Allowed special characters
    pub allowed_special_chars: String,
    /// Password history count
    pub history_count: u32,
    /// Password expiration
    pub expiration: Option<Duration>,
    /// Common password denylist (prohibited passwords)
    pub denylist: Vec<String>,
    /// Dictionary check
    pub dictionary_check: bool,
}

/// Brute-force lockout and escalation settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLockoutConfig {
    /// Enable account lockout
    pub enabled: bool,
    /// Maximum failed attempts before lockout
    pub max_attempts: u32,
    /// Lockout duration
    pub lockout_duration: Duration,
    /// Reset attempt counter after duration
    pub reset_duration: Duration,
    /// Progressive lockout (increasing duration)
    pub progressive_lockout: bool,
    /// Lockout escalation multiplier
    pub escalation_multiplier: f64,
}

impl Default for PasswordPolicyConfig {
    fn default() -> Self {
        Self {
            min_length: 8,
            max_length: 128,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special_chars: true,
            allowed_special_chars: "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string(),
            history_count: 5,
            expiration: Some(Duration::from_secs(90 * 24 * 60 * 60)), // 90 days
            denylist: vec![],
            dictionary_check: true,
        }
    }
}

impl Default for AccountLockoutConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: 5,
            lockout_duration: Duration::from_secs(15 * 60), // 15 minutes
            reset_duration: Duration::from_secs(60 * 60),   // 1 hour
            progressive_lockout: true,
            escalation_multiplier: 2.0,
        }
    }
}
