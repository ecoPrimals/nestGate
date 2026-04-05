// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS command types and execution results
//!
//! Domain: Command operations, results, status reporting

use std::collections::HashMap;

/// ZFS command execution result with output and status
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// Whether the command succeeded
    pub success: bool,
    /// Standard output from the command
    pub stdout: String,
    /// Standard error from the command
    pub stderr: String,
    /// Exit code if available
    pub exit_code: Option<i32>,
}

/// ZFS command operations for execution
#[derive(Debug, Clone)]
pub enum ZfsCommand {
    /// Create a new ZFS pool
    CreatePool {
        /// Pool name
        name: String,
        /// Block devices to use
        devices: Vec<String>,
    },

    /// Create a new dataset
    CreateDataset {
        /// Dataset name
        name: String,
        /// Initial properties
        properties: HashMap<String, String>,
    },

    /// Create a snapshot
    CreateSnapshot {
        /// Dataset name
        dataset: String,
        /// Snapshot name
        snapshot: String,
        /// Whether to snapshot recursively
        recursive: bool,
    },

    /// List pools
    ListPools,

    /// List datasets in a pool
    ListDatasets {
        /// Pool name (None = all pools)
        pool: Option<String>,
    },

    /// Get pool status
    GetPoolStatus {
        /// Pool name
        pool: String,
    },

    /// Set property on dataset
    SetProperty {
        /// Dataset name
        dataset: String,
        /// Property name
        property: String,
        /// Property value
        value: String,
    },

    /// Destroy dataset or snapshot
    Destroy {
        /// Target to destroy
        target: String,
        /// Force destruction
        force: bool,
    },
}

impl CommandResult {
    /// Create a successful command result
    #[must_use]
    pub const fn success(stdout: String) -> Self {
        Self {
            success: true,
            stdout,
            stderr: String::new(),
            exit_code: Some(0),
        }
    }

    /// Create a failed command result
    #[must_use]
    pub const fn failure(stderr: String, exit_code: Option<i32>) -> Self {
        Self {
            success: false,
            stdout: String::new(),
            stderr,
            exit_code,
        }
    }

    /// Check if command succeeded
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.success
    }

    /// Check if command failed
    #[must_use]
    pub const fn is_failure(&self) -> bool {
        !self.success
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn command_result_constructors() {
        let ok = CommandResult::success("out".to_string());
        assert!(ok.is_success());
        assert!(!ok.is_failure());
        let bad = CommandResult::failure("e".to_string(), Some(1));
        assert!(bad.is_failure());
        assert!(!bad.is_success());
    }

    #[test]
    fn zfs_command_variants_are_constructible() {
        let _ = ZfsCommand::CreatePool {
            name: "p".into(),
            devices: vec!["/dev/sdb".into()],
        };
        let _ = ZfsCommand::CreateDataset {
            name: "p/d".into(),
            properties: HashMap::new(),
        };
        let _ = ZfsCommand::CreateSnapshot {
            dataset: "p/d".into(),
            snapshot: "s".into(),
            recursive: true,
        };
        let _ = ZfsCommand::ListPools;
        let _ = ZfsCommand::ListDatasets {
            pool: Some("p".into()),
        };
        let _ = ZfsCommand::GetPoolStatus { pool: "p".into() };
        let _ = ZfsCommand::SetProperty {
            dataset: "p/d".into(),
            property: "atime".into(),
            value: "off".into(),
        };
        let _ = ZfsCommand::Destroy {
            target: "p/d@s".into(),
            force: false,
        };
    }
}
