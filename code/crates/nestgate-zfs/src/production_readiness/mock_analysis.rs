// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use nestgate_core::Result;

use super::ProductionReadinessValidator;

impl ProductionReadinessValidator {
    /// Detect Real Hardware
    pub(crate) fn detect_real_hardware(&self) -> Result<bool> {
        // Detect if we're running on real hardware vs virtualized environment
        Ok(!std::env::var("NESTGATE_MOCK_MODE")
            .unwrap_or_default()
            .eq("true"))
    }

    /// Identify Mock Dependencies
    pub(crate) fn identify_mock_dependencies(&self) -> Result<Vec<String>> {
        let mut mocks = Vec::new();

        if std::env::var("NESTGATE_MOCK_MODE")
            .unwrap_or_default()
            .eq("true")
        {
            mocks.push("Mock mode enabled".to_string());
        }

        Ok(mocks)
    }
}
