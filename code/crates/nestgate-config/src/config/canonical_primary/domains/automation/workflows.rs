// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **WORKFLOWS CONFIGURATION**
///
/// Automated workflow engine settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Workflows
pub struct WorkflowsConfig {
    /// Enable workflows
    pub enabled: bool,

    /// Maximum concurrent workflows
    pub max_concurrent_workflows: usize,

    /// Workflow timeout
    pub workflow_timeout: Duration,

    /// Enable workflow scheduling
    pub scheduling_enabled: bool,

    /// Workflow definitions directory
    pub definitions_dir: String,

    /// Enable workflow versioning
    pub versioning_enabled: bool,
}

impl Default for WorkflowsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl WorkflowsConfig {
    /// Creates a development-optimized configuration for workflow engine
    ///
    /// Returns a `WorkflowsConfig` with workflows disabled and reduced concurrency
    /// suitable for development environments.
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: false,
            max_concurrent_workflows: 5,
            workflow_timeout: Duration::from_secs(600),
            scheduling_enabled: false,
            definitions_dir: "./workflows".to_string(),
            versioning_enabled: false,
        }
    }

    /// Creates a production-hardened configuration for workflow engine
    ///
    /// Returns a `WorkflowsConfig` with workflows enabled, high concurrency, longer timeouts,
    /// scheduling, and versioning for production workloads.
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            max_concurrent_workflows: 20,
            workflow_timeout: Duration::from_secs(1800),
            scheduling_enabled: true,
            definitions_dir: "/etc/nestgate/workflows".to_string(),
            versioning_enabled: true,
        }
    }
}
