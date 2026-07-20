// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Production readiness signals: explicit mock flags plus Linux procfs health checks.

use nestgate_core::Result;

use super::ProductionReadinessValidator;

impl ProductionReadinessValidator {
    /// Returns true when the host exposes normal CPU/memory procfs and mock mode is off.
    pub(crate) fn detect_real_hardware(&self) -> Result<bool> {
        if self
            .env
            .get("NESTGATE_MOCK_MODE")
            .unwrap_or_default()
            .eq("true")
        {
            return Ok(false);
        }
        Ok(host_has_procfs_cpu_and_mem())
    }

    /// Lists operator mock flags and missing Linux procfs inputs that block trustworthy reporting.
    pub(crate) fn identify_mock_dependencies(&self) -> Result<Vec<String>> {
        let mut mocks = Vec::new();

        if self
            .env
            .get("NESTGATE_MOCK_MODE")
            .unwrap_or_default()
            .eq("true")
        {
            mocks.push("NESTGATE_MOCK_MODE=true (explicit mock mode)".into());
        }

        #[cfg(target_os = "linux")]
        {
            if !proc_cpuinfo_readable() {
                mocks.push(String::from(
                    "/proc/cpuinfo unreadable (no CPU topology signal)",
                ));
            }
            if !proc_meminfo_readable() {
                mocks.push("/proc/meminfo unreadable (no memory signal)".into());
            }
        }

        Ok(mocks)
    }
}

#[cfg(target_os = "linux")]
fn host_has_procfs_cpu_and_mem() -> bool {
    proc_cpuinfo_readable() && proc_meminfo_readable()
}

#[cfg(not(target_os = "linux"))]
const fn host_has_procfs_cpu_and_mem() -> bool {
    false
}

#[cfg(target_os = "linux")]
fn proc_cpuinfo_readable() -> bool {
    nestgate_platform::linux_proc::physical_cpu_count() > 0
}

#[cfg(target_os = "linux")]
fn proc_meminfo_readable() -> bool {
    nestgate_platform::linux_proc::total_memory_bytes().is_some()
}
