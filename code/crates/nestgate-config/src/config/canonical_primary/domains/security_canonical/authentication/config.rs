// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Root [`AuthenticationConfig`] and environment-specific constructors.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::external::ExternalAuthProvider;
use super::method::AuthenticationMethod;
use super::mfa::{BackupCodesConfig, MfaConfig, MfaMethod, RememberDeviceConfig};
use super::password_lockout::{AccountLockoutConfig, PasswordPolicyConfig};
use super::session::SessionConfig;
use super::tokens::TokenConfig;

/// Comprehensive authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    /// Primary authentication method
    pub primary_method: AuthenticationMethod,
    /// Secondary authentication methods
    pub secondary_methods: Vec<AuthenticationMethod>,
    /// Multi-factor authentication
    pub mfa: MfaConfig,
    /// Session management
    pub session: SessionConfig,
    /// Token configuration
    pub tokens: TokenConfig,
    /// Password policies
    pub password_policy: PasswordPolicyConfig,
    /// Account lockout policies
    pub lockout: AccountLockoutConfig,
    /// External authentication providers
    pub external_providers: Vec<ExternalAuthProvider>,
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            primary_method: AuthenticationMethod::UsernamePassword,
            secondary_methods: vec![],
            mfa: MfaConfig::default(),
            session: SessionConfig::default(),
            tokens: TokenConfig::default(),
            password_policy: PasswordPolicyConfig::default(),
            lockout: AccountLockoutConfig::default(),
            external_providers: vec![],
        }
    }
}

impl AuthenticationConfig {
    /// Create a configuration optimized for production environments
    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            primary_method: AuthenticationMethod::OAuth2,
            secondary_methods: vec![AuthenticationMethod::Certificate],
            mfa: MfaConfig {
                enabled: true,
                required_methods: vec![MfaMethod::Totp, MfaMethod::Push],
                optional_methods: vec![MfaMethod::BackupCodes],
                timeout: Duration::from_secs(180), // 3 minutes
                backup_codes: BackupCodesConfig {
                    enabled: true,
                    count: 10,
                    length: 12,
                    auto_regenerate: true,
                },
                remember_device: RememberDeviceConfig {
                    enabled: true,
                    duration: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
                    max_devices: 3,
                },
            },
            password_policy: PasswordPolicyConfig {
                min_length: 12,
                max_length: 256,
                require_uppercase: true,
                require_lowercase: true,
                require_numbers: true,
                require_special_chars: true,
                allowed_special_chars: "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string(),
                history_count: 10,
                expiration: Some(Duration::from_secs(60 * 24 * 60 * 60)), // 60 days
                denylist: vec![], // Would be populated from external source
                dictionary_check: true,
            },
            lockout: AccountLockoutConfig {
                enabled: true,
                max_attempts: 3,
                lockout_duration: Duration::from_secs(30 * 60), // 30 minutes
                reset_duration: Duration::from_secs(2 * 60 * 60), // 2 hours
                progressive_lockout: true,
                escalation_multiplier: 3.0,
            },
            ..Default::default()
        }
    }

    /// Create a configuration optimized for development environments
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            primary_method: AuthenticationMethod::UsernamePassword,
            secondary_methods: vec![],
            mfa: MfaConfig {
                enabled: false,
                ..Default::default()
            },
            password_policy: PasswordPolicyConfig {
                min_length: 6,
                max_length: 128,
                require_uppercase: false,
                require_lowercase: false,
                require_numbers: false,
                require_special_chars: false,
                allowed_special_chars: "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string(),
                history_count: 1,
                expiration: None,
                denylist: vec![],
                dictionary_check: false,
            },
            lockout: AccountLockoutConfig {
                enabled: false,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create a configuration for compliance-focused environments
    #[must_use]
    pub fn compliance_focused() -> Self {
        Self::production_hardened() // Start with production hardened
        // Add compliance-specific overrides here
    }

    /// Merge with another configuration
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        // Simple merge - in a real implementation, you'd want more sophisticated merging
        self
    }

    /// Validate the authentication configuration
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        // Validate password policy
        if self.password_policy.min_length > self.password_policy.max_length {
            return Err(nestgate_types::error::NestGateError::validation_error(
                "Password policy: min_length cannot be greater than max_length",
            ));
        }

        // Validate MFA configuration
        if self.mfa.enabled && self.mfa.required_methods.is_empty() {
            return Err(nestgate_types::error::NestGateError::validation_error(
                "MFA is enabled but no required methods are configured",
            ));
        }

        // Validate session configuration
        if self.session.timeout < Duration::from_secs(60) {
            return Err(nestgate_types::error::NestGateError::validation_error(
                "Session timeout cannot be less than 1 minute",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{AuthenticationMethod, JwtAlgorithm, MfaConfig};
    use super::AuthenticationConfig;

    #[test]
    fn test_authentication_config_default() {
        let config = AuthenticationConfig::default();
        assert!(matches!(
            config.primary_method,
            AuthenticationMethod::UsernamePassword
        ));
    }

    #[test]
    fn test_authentication_method_variants() {
        let _up = AuthenticationMethod::UsernamePassword;
        let _oauth = AuthenticationMethod::OAuth2;
        let custom = AuthenticationMethod::Custom("biometric_v2".to_string());
        assert!(matches!(custom, AuthenticationMethod::Custom(_)));
    }

    #[test]
    fn test_mfa_config_default() {
        let mfa = MfaConfig::default();
        assert!(!mfa.enabled);
    }

    #[test]
    fn test_authentication_config_validate() {
        let config = AuthenticationConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_jwt_algorithm_serialization() {
        let alg = JwtAlgorithm::HS256;
        let json = serde_json::to_string(&alg).unwrap();
        let parsed: JwtAlgorithm = serde_json::from_str(&json).unwrap();
        assert_eq!(format!("{alg:?}"), format!("{parsed:?}"));
    }
}
