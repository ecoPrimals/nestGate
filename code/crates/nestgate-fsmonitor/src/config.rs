// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Handles file system monitoring configuration, event detection, and notification
/// **ECOSYSTEM UNIFICATION**: This module now uses the unified type system from nestgate-core
/// to eliminate file system monitor config fragmentation and ensure consistency.
use serde::{Deserialize, Serialize};
/// File system event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum FsEventType {
    /// File or directory created
    Created,
    /// Directory created specifically
    DirectoryCreated,
    /// File or directory modified
    #[default]
    Modified,
    /// File or directory deleted
    Deleted,
    /// Directory deleted specifically
    DirectoryDeleted,
    /// File or directory renamed
    Renamed,
    /// File or directory moved
    Moved,
    /// File accessed
    Accessed,
    /// Attributes changed
    AttributeChanged,
    /// Metadata changed
    MetadataChanged,
}
// **DEPRECATED IMPLEMENTATION REMOVED**
// FileSystemConfig struct has been removed - use UnifiedFsMonitorConfig from unified_fsmonitor_config.rs instead
