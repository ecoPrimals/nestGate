// SPDX-License-Identifier: AGPL-3.0-only
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
