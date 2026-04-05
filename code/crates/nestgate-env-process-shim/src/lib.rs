// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![warn(missing_docs)]

//! Thin wrappers around [`std::env::set_var`] and [`std::env::remove_var`].
//!
//! This crate is **edition 2021**. In that edition those functions are still safe; in Rust 2024
//! they are `unsafe`, so higher edition crates can depend on this crate to perform the same
//! operations without an `unsafe` block (see Rust edition guide: newly unsafe functions).

#![forbid(unsafe_code)]

use std::ffi::OsStr;

/// Delegates to [`std::env::set_var`].
pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    std::env::set_var(key, value);
}

/// Delegates to [`std::env::remove_var`].
pub fn remove_var<K: AsRef<OsStr>>(key: K) {
    std::env::remove_var(key);
}

#[cfg(test)]
#[expect(clippy::expect_used)] // Tests assert env invariants; project prefers `expect` over `unwrap`.
mod tests {
    use super::{remove_var, set_var};
    use serial_test::serial;
    use std::ffi::OsString;

    /// Unique key so parallel test suites in the same process do not collide.
    const KEY: &str = "NESTGATE_ENV_PROCESS_SHIM_UNIT_TEST_KEY";

    #[test]
    #[serial]
    fn set_var_exposes_value_to_process_environment() {
        temp_env::with_var_unset(KEY, || {
            set_var(KEY, "alpha");
            let got = std::env::var_os(KEY).expect("set_var should define the variable");
            assert_eq!(got, OsString::from("alpha"));
        });
    }

    #[test]
    #[serial]
    fn remove_var_clears_variable() {
        temp_env::with_var(KEY, Some("seed"), || {
            set_var(KEY, "during");
            assert_eq!(
                std::env::var(KEY).expect("variable present after set_var"),
                "during"
            );
            remove_var(KEY);
            assert!(
                std::env::var_os(KEY).is_none(),
                "remove_var should delete the variable"
            );
        });
    }

    #[test]
    #[serial]
    fn set_then_remove_restores_absent_state() {
        temp_env::with_var_unset(KEY, || {
            assert!(std::env::var_os(KEY).is_none());
            set_var(KEY, "temporary");
            assert!(std::env::var_os(KEY).is_some());
            remove_var(KEY);
            assert!(
                std::env::var_os(KEY).is_none(),
                "after remove, key should be unset like before set_var"
            );
        });
    }

    #[test]
    #[serial]
    fn temp_env_restores_prior_value_after_shim_mutation() {
        temp_env::with_var(KEY, Some("original"), || {
            assert_eq!(std::env::var(KEY).expect("fixture value"), "original");
            set_var(KEY, "overridden");
            assert_eq!(
                std::env::var(KEY).expect("set_var updates value"),
                "overridden"
            );
            remove_var(KEY);
            assert!(std::env::var_os(KEY).is_none());
        });
    }
}
