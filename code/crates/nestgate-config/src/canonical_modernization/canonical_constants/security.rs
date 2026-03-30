// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

    /// Token expiration time in seconds (1 hour)
    pub const TOKEN_EXPIRATION_S: u64 = 3600;

    /// Encryption algorithms
    ///
    /// AES-256-GCM encryption algorithm
    pub const AES_256_GCM: &str = "aes-256-gcm";
    /// ChaCha20-Poly1305 encryption algorithm
    pub const CHACHA20_POLY1305: &str = "chacha20-poly1305";

    /// User roles
    ///
    /// Administrator role with full permissions
    pub const ROLE_ADMIN: &str = "admin";
    /// Standard user role with normal permissions
    pub const ROLE_USER: &str = "user";
    /// Guest role with limited permissions
    pub const ROLE_GUEST: &str = "guest";

    /// Password requirements
    ///
    /// Minimum password length (8 characters)
    pub const MIN_PASSWORD_LENGTH: usize = 8;
    /// Maximum login attempts before lockout
    pub const MAX_LOGIN_ATTEMPTS: u32 = 3;
    /// Account lockout duration in seconds (5 minutes)
    pub const LOCKOUT_DURATION_SECS: u64 = 300;
