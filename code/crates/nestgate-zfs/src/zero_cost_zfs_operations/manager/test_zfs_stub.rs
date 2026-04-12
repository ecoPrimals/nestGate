// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Test-only synchronous stub for [`super::ZeroCostZfsManager::execute_zfs_command`].
//! Avoids spawning `zfs` in unit tests while exercising trait forwards and parsers.

use nestgate_core::Result;
use std::sync::{Mutex, OnceLock};

type StubFn = Box<dyn Fn(&[&str]) -> Result<String> + Send + Sync + 'static>;

fn stub_cell() -> &'static Mutex<Option<StubFn>> {
    static CELL: OnceLock<Mutex<Option<StubFn>>> = OnceLock::new();
    CELL.get_or_init(|| Mutex::new(None))
}

/// When set, [`super::ZeroCostZfsManager::execute_zfs_command`] returns the stub result instead of running `zfs`.
pub fn try_run_stub(args: &[&str]) -> Option<Result<String>> {
    let lock = stub_cell().lock().ok()?;
    let f = lock.as_ref()?;
    Some(f(args))
}

/// Installs a stub until dropped (restores `None` so other tests run the real command path).
pub struct ZfsCommandStubGuard;

impl ZfsCommandStubGuard {
    /// # Panics
    ///
    /// Panics if the stub mutex is poisoned.
    pub fn set(stub: StubFn) -> Self {
        *stub_cell().lock().expect("zfs stub mutex") = Some(stub);
        Self
    }
}

impl Drop for ZfsCommandStubGuard {
    fn drop(&mut self) {
        *stub_cell().lock().expect("zfs stub mutex") = None;
    }
}
