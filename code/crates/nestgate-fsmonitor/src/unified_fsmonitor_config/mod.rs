// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unified Fsmonitor Config module

use nestgate_core::config::canonical_primary::NestGateCanonicalConfig as StandardDomainConfig;
use serde::{Deserialize, Serialize};

// Re-export types from config.rs for backward compatibility
pub use crate::config::FsEventType;

// Import all module components
pub mod event_processing;
pub mod filters;
pub mod integrations;
pub mod notifications;
pub mod performance;
pub mod security;
pub mod storage;
pub mod watch_settings;

// Re-export all public types for seamless migration
pub use event_processing::*;
pub use filters::*;
pub use integrations::*;
pub use notifications::*;
pub use performance::*;
pub use security::*;
pub use storage::*;
pub use watch_settings::*;

//! **UNIFIED FILE SYSTEM MONITOR EXTENSIONS**
//! Main configuration structure that composes all specialized modules
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedFsMonitorExtensions {
    /// Watch configuration settings
    pub watch: WatchSettings,
    /// Event processing settings
    pub event_processing: EventProcessingSettings,
    /// Notification settings
    pub notifications: NotificationSettings,
    /// Performance and resource settings
    pub performance: FsMonitorPerformanceSettings,
    /// Filter and pattern settings
    pub filters: FilterSettings,
    /// Storage and persistence settings
    pub storage: FsMonitorStorageSettings,
    /// Integration settings
    pub integrations: IntegrationSettings,
    /// Security and access control settings
    pub security: FsMonitorSecuritySettings,
}
//! **UNIFIED FILE SYSTEM MONITOR CONFIGURATION**
//! The main configuration type following StandardDomainConfig pattern
//! CANONICAL MODERNIZATION: Simplified type alias without type parameters
pub type UnifiedFsMonitorConfig = StandardDomainConfig;
