// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Multi-factor authentication and backup / remember-device settings.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Multi-factor authentication configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    /// Enable multi-factor authentication
    pub enabled: bool,
    /// Required MFA methods
    pub required_methods: Vec<MfaMethod>,
    /// Optional MFA methods
    pub optional_methods: Vec<MfaMethod>,
    /// MFA timeout settings
    pub timeout: Duration,
    /// Backup codes configuration
    pub backup_codes: BackupCodesConfig,
    /// Remember device settings
    pub remember_device: RememberDeviceConfig,
}

/// MFA method variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    /// SMS-based OTP
    Sms,
    /// Email-based OTP
    Email,
    /// TOTP (Time-based One-Time Password)
    Totp,
    /// HOTP (HMAC-based One-Time Password)
    Hotp,
    /// Push notification
    Push,
    /// Hardware token (`YubiKey`, etc.)
    HardwareToken,
    /// Biometric verification
    Biometric,
    /// Backup codes
    BackupCodes,
    /// Custom MFA method
    Custom(String),
}

/// Backup codes configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCodesConfig {
    /// Enable backup codes
    pub enabled: bool,
    /// Number of backup codes to generate
    pub count: u32,
    /// Length of each backup code
    pub length: u32,
    /// Auto-regenerate after use
    pub auto_regenerate: bool,
}

/// Remember-device configuration for MFA.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RememberDeviceConfig {
    /// Enable remember device functionality
    pub enabled: bool,
    /// Duration to remember device
    pub duration: Duration,
    /// Maximum remembered devices per user
    pub max_devices: u32,
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            required_methods: vec![],
            optional_methods: vec![MfaMethod::Totp, MfaMethod::Email],
            timeout: Duration::from_secs(300),
            backup_codes: BackupCodesConfig::default(),
            remember_device: RememberDeviceConfig::default(),
        }
    }
}

impl Default for BackupCodesConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            count: 10,
            length: 8,
            auto_regenerate: false,
        }
    }
}

impl Default for RememberDeviceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            duration: Duration::from_secs(30 * 24 * 60 * 60), // 30 days
            max_devices: 5,
        }
    }
}
