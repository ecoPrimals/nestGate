// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Universal and temporal storage abstractions for `NestGate`.
//!
//! Extracted from nestgate-core to enable parallel compilation.

#![warn(missing_docs)]
#![allow(deprecated)] // `StorageResourceConfig` / `OptimalStorageConfig`: migration to canonical config is ongoing
#![allow(clippy::struct_excessive_bools)]
// Config/detection structs intentionally use many feature flags
// Stub pipelines and `Option`/`Result` carriers for `?` trigger pedantic style lints; kept for API stability.
#![allow(
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    dead_code
)]
#![cfg_attr(
    test,
    allow(
        clippy::expect_used,
        clippy::panic,
        clippy::uninlined_format_args,
        clippy::unwrap_used,
    )
)]

pub mod temporal_storage;
pub mod universal_storage;
