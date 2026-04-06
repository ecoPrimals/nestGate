// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Environment-driven flags for production readiness reporting.
//!
//! This module does **not** scan source code, imports, or binaries for mock types. It only reads
//! values from the injectable [`super::ProductionReadinessValidator`] environment ([`nestgate_types::EnvSource`])
//! so the readiness report can surface explicit operator choices (for example
//! `NESTGATE_MOCK_MODE=true`).

use nestgate_core::Result;

use super::ProductionReadinessValidator;

impl ProductionReadinessValidator {
    /// Returns whether the process is *not* in explicit mock mode (`NESTGATE_MOCK_MODE=true`).
    ///
    /// This is a coarse signal only; it does not probe hypervisors, CPU flags, or disk topology.
    pub(crate) fn detect_real_hardware(&self) -> Result<bool> {
        Ok(!self
            .env
            .get("NESTGATE_MOCK_MODE")
            .unwrap_or_default()
            .eq("true"))
    }

    /// Collects human-readable mock-related notices from the process environment.
    ///
    /// Currently this checks `NESTGATE_MOCK_MODE` only. Extend this when new env-gated mock or
    /// dry-run flags are added to the product.
    pub(crate) fn identify_mock_dependencies(&self) -> Result<Vec<String>> {
        let mut mocks = Vec::new();

        if self
            .env
            .get("NESTGATE_MOCK_MODE")
            .unwrap_or_default()
            .eq("true")
        {
            mocks.push("Mock mode enabled".to_string());
        }

        Ok(mocks)
    }
}
