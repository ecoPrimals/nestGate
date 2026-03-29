// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Process environment mutations.
//!
//! Mutating the process environment is not synchronized with other threads on all platforms.
//! This module centralizes environment updates via [`nestgate_env_process_shim`] so the rest of
//! the codebase keeps a single audited surface while the workspace lint `unsafe_code` remains
//! denied at call sites.
//!
//! **Tests:** For unit tests, prefer [`test_support`] helpers built on [`temp_env`] so changes are
//! scoped and less likely to race with parallel tests.
//!
//! [`test_support`]: crate::env_process::test_support

use std::ffi::OsStr;

/// Set an environment variable (delegates to [`nestgate_env_process_shim::set_var`]).
///
/// Logs a warning on every call because mutating the environment is hazardous in concurrent programs:
/// other threads may read stale or torn values, and platform documentation requires avoiding
/// concurrent env access except where externally synchronized.
///
/// Callers should avoid unsynchronized concurrent access to the process environment (including
/// through C libraries, FFI, or `std::env::var`) while the environment is mutated.
///
/// Do not pass secret values into logging; this function logs only the variable key, not `value`.
pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    let key_ref = key.as_ref();
    tracing::warn!(
        target: "nestgate_platform::env_process",
        key = ?key_ref,
        "mutating process environment (set_var); unsafe on concurrent access — see env_process docs"
    );
    nestgate_env_process_shim::set_var(key_ref, value);
}

/// Remove an environment variable (delegates to [`nestgate_env_process_shim::remove_var`]).
///
/// Logs a warning on every call; see [`set_var`] for rationale.
///
/// Same concurrency considerations as [`set_var`].
pub fn remove_var<K: AsRef<OsStr>>(key: K) {
    let key_ref = key.as_ref();
    tracing::warn!(
        target: "nestgate_platform::env_process",
        key = ?key_ref,
        "mutating process environment (remove_var); unsafe on concurrent access — see env_process docs"
    );
    nestgate_env_process_shim::remove_var(key_ref);
}

/// Scoped environment helpers for unit tests (backed by [`temp_env`]).
///
/// Prefer [`with_var`](temp_env::with_var), [`with_vars`](temp_env::with_vars), and related
/// `temp_env` APIs over [`set_var`] / [`remove_var`] in tests so each test restores the previous
/// environment and parallel test runs are less likely to observe races.
#[cfg(test)]
pub mod test_support {
    pub use temp_env::{async_with_vars, with_var, with_var_unset, with_vars, with_vars_unset};
}

#[cfg(test)]
mod tests {
    use super::test_support::with_var;

    #[test]
    fn test_support_scopes_env_var() {
        with_var("NESTGATE_ENV_PROCESS_TEST_VAR", Some("scoped"), || {
            assert_eq!(
                std::env::var("NESTGATE_ENV_PROCESS_TEST_VAR"),
                Ok("scoped".to_owned())
            );
        });
        assert!(std::env::var("NESTGATE_ENV_PROCESS_TEST_VAR").is_err());
    }

    #[test]
    fn set_var_logs_and_sets() {
        let key = "NESTGATE_ENV_PROCESS_SET_TEST";
        super::set_var(key, "x");
        assert_eq!(std::env::var(key), Ok("x".to_owned()));
        super::remove_var(key);
        assert!(std::env::var(key).is_err());
    }
}
