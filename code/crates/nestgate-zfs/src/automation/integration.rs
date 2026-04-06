// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS automation integration shims.
//!
//! The `nestgate-automation` crate was deprecated in v4.7.0 — automation /
//! orchestration concerns are delegated to biomeOS.  These thin types remain
//! for API compatibility until callers migrate to [`DatasetAutomation`](super::engine::DatasetAutomation).

use nestgate_core::Result;
use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;

/// Lightweight shim formerly backed by `nestgate-automation`.
#[deprecated(
    since = "4.7.0",
    note = "Use DatasetAutomation from the engine module instead"
)]
pub struct IntelligentDatasetManager;

/// Lightweight shim formerly backed by `nestgate-automation`.
#[deprecated(
    since = "4.7.0",
    note = "Use DatasetAutomationConfig from crate::config instead"
)]
pub struct AutomationConfig;

/// Initialize automation integration with canonical configuration.
///
/// Returns a no-op [`IntelligentDatasetManager`] shim.
#[deprecated(since = "4.7.0", note = "Use DatasetAutomation::new() instead")]
#[expect(deprecated)]
pub fn initialize_automation(
    _config: NestGateCanonicalConfig,
) -> Result<IntelligentDatasetManager> {
    Ok(IntelligentDatasetManager)
}

/// Initialize automation with custom config.
///
/// Returns a no-op [`IntelligentDatasetManager`] shim.
#[deprecated(since = "4.7.0", note = "Use DatasetAutomation::new() instead")]
#[expect(deprecated)]
pub fn initialize_automation_with_config(
    _config: NestGateCanonicalConfig,
    _automation_config: AutomationConfig,
) -> Result<IntelligentDatasetManager> {
    Ok(IntelligentDatasetManager)
}

/// Check if ecosystem services are available for ZFS automation.
///
/// Returns `false` — ecosystem availability is determined at runtime via
/// capability IPC, not compile-time feature flags.
#[must_use]
pub const fn check_zfs_ecosystem_availability() -> bool {
    false
}

#[cfg(test)]
mod tests {
    #[allow(deprecated)]
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_initialize_automation() {
        let config = NestGateCanonicalConfig::default();
        let result = initialize_automation(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_ecosystem_availability() {
        let available = check_zfs_ecosystem_availability();
        assert!(!available);
    }
}
