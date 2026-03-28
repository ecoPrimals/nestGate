// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Process environment mutations.
//!
//! In Rust 2024, `std::env::set_var` and `std::env::remove_var` are `unsafe`
//! because mutating the environment is not thread-safe on all platforms.
//! This module centralizes those calls so the rest of the codebase stays free
//! of `unsafe` blocks while the workspace lint `unsafe_code` remains denied.

#![allow(unsafe_code)]

use std::ffi::OsStr;

/// Set an environment variable (wraps the `unsafe` `std::env::set_var`).
pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    unsafe {
        std::env::set_var(key, value);
    }
}

/// Remove an environment variable (wraps the `unsafe` `std::env::remove_var`).
pub fn remove_var<K: AsRef<OsStr>>(key: K) {
    unsafe {
        std::env::remove_var(key);
    }
}
