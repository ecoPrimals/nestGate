// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **ACTIONS CONFIGURATION**
///
/// Automated action execution settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Actions
pub struct ActionsConfig {
    /// Enable actions
    pub enabled: bool,

    /// Enable storage actions (move, copy, delete)
    pub storage_actions: bool,

    /// Enable notification actions
    pub notification_actions: bool,

    /// Enable script execution
    pub script_execution: bool,

    /// Maximum action retries
    pub max_retries: u32,

    /// Action timeout
    pub action_timeout: Duration,

    /// Allowed action types
    pub allowed_actions: Vec<String>,
}

impl Default for ActionsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl ActionsConfig {
    /// Creates a development-optimized configuration for automated actions
    ///
    /// Returns an `ActionsConfig` with actions disabled except notifications,
    /// suitable for safe development environments.
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false,
            storage_actions: false,
            notification_actions: true,
            script_execution: false,
            max_retries: 3,
            action_timeout: Duration::from_secs(60),
            allowed_actions: vec!["notify".to_string(), "log".to_string()],
        }
    }

    /// Creates a production-hardened configuration for automated actions
    ///
    /// Returns an `ActionsConfig` with all action types enabled, higher retry limits,
    /// and comprehensive action types for production workloads.
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            storage_actions: true,
            notification_actions: true,
            script_execution: true,
            max_retries: 5,
            action_timeout: Duration::from_secs(300),
            allowed_actions: vec![
                "notify".to_string(),
                "log".to_string(),
                "move".to_string(),
                "copy".to_string(),
                "tier".to_string(),
                "execute_script".to_string(),
            ],
        }
    }
}
