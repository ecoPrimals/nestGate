// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Authentication configuration split by method, MFA, session, tokens, password policy, and external IdPs.

mod config;
mod external;
mod method;
mod mfa;
mod password_lockout;
mod session;
mod tokens;

pub use config::AuthenticationConfig;
pub use external::{ExternalAuthProvider, ExternalProviderType};
pub use method::AuthenticationMethod;
pub use mfa::{BackupCodesConfig, MfaConfig, MfaMethod, RememberDeviceConfig};
pub use password_lockout::{AccountLockoutConfig, PasswordPolicyConfig};
pub use session::{
    KeyRotationConfig, SameSitePolicy, SessionConfig, SessionEncryptionConfig,
    SessionRefreshConfig, SessionSecurityConfig, SessionStorageConfig, SessionStorageType,
};
pub use tokens::{
    AccessTokenConfig, ApiKeyConfig, JwtAlgorithm, JwtConfig, RateLimitConfig, RefreshTokenConfig,
    TokenConfig,
};
