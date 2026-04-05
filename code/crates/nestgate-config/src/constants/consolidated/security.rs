// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! JWT, encryption, TLS, and rate-limit defaults.

use std::sync::{Arc, OnceLock};

use super::defaults::{env_or, env_or_parse};

/// Security and authentication constants
#[derive(Debug, Clone)]
/// Securityconstants
pub struct SecurityConstants {
    // JWT
    jwt_secret: String,
    jwt_expiration_secs: u64,
    jwt_refresh_expiration_secs: u64,

    // Encryption
    encryption_algorithm: String,
    key_size_bits: u32,

    // TLS
    tls_enabled: bool,
    tls_cert_path: String,
    tls_key_path: String,
    tls_ca_path: String,

    // Rate limiting
    rate_limit_requests_per_minute: u32,
    rate_limit_burst_size: u32,
}

impl Default for SecurityConstants {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            // JWT
            jwt_secret: env_or("NESTGATE_JWT_SECRET", "CHANGE_ME_IN_PRODUCTION"),
            jwt_expiration_secs: env_or_parse("NESTGATE_JWT_EXP", 3600),
            jwt_refresh_expiration_secs: env_or_parse("NESTGATE_JWT_REFRESH_EXP", 86400),

            // Encryption
            encryption_algorithm: env_or("NESTGATE_ENC_ALGO", "AES-256-GCM"),
            key_size_bits: env_or_parse("NESTGATE_KEY_SIZE", 256),

            // TLS
            tls_enabled: env_or_parse("NESTGATE_TLS_ENABLED", false),
            tls_cert_path: env_or("NESTGATE_TLS_CERT", "./certs/server.crt"),
            tls_key_path: env_or("NESTGATE_TLS_KEY", "./certs/server.key"),
            tls_ca_path: env_or("NESTGATE_TLS_CA", "./certs/ca.crt"),

            // Rate limiting
            rate_limit_requests_per_minute: env_or_parse("NESTGATE_RATE_LIMIT_RPM", 60),
            rate_limit_burst_size: env_or_parse("NESTGATE_RATE_LIMIT_BURST", 10),
        }
    }
}

impl SecurityConstants {
    /// Get or initialize the global security constants
    pub fn get() -> Arc<Self> {
        static INSTANCE: OnceLock<Arc<SecurityConstants>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::default())).clone()
    }

    // JWT getters

    /// Returns the JWT secret key for token signing
    #[must_use]
    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
    /// Jwt Expiration
    #[must_use]
    pub const fn jwt_expiration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.jwt_expiration_secs)
    }
    /// Jwt Refresh Expiration
    #[must_use]
    pub const fn jwt_refresh_expiration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.jwt_refresh_expiration_secs)
    }

    // Encryption getters

    /// Returns the encryption algorithm name (e.g., "AES-256-GCM")
    #[must_use]
    pub fn encryption_algorithm(&self) -> &str {
        &self.encryption_algorithm
    }
    /// Key Size Bits
    #[must_use]
    pub const fn key_size_bits(&self) -> u32 {
        self.key_size_bits
    }

    // TLS getters

    /// Returns whether TLS is enabled for secure connections
    #[must_use]
    pub const fn tls_enabled(&self) -> bool {
        self.tls_enabled
    }
    /// Tls Cert Path
    #[must_use]
    pub fn tls_cert_path(&self) -> &str {
        &self.tls_cert_path
    }
    /// Tls Key Path
    #[must_use]
    pub fn tls_key_path(&self) -> &str {
        &self.tls_key_path
    }
    /// Tls Ca Path
    #[must_use]
    pub fn tls_ca_path(&self) -> &str {
        &self.tls_ca_path
    }

    // Rate limiting getters

    /// Returns the maximum number of requests allowed per minute for rate limiting
    #[must_use]
    pub const fn rate_limit_requests_per_minute(&self) -> u32 {
        self.rate_limit_requests_per_minute
    }
    /// Rate Limit Burst Size
    #[must_use]
    pub const fn rate_limit_burst_size(&self) -> u32 {
        self.rate_limit_burst_size
    }
}
