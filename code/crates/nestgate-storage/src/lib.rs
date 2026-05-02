// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Universal and temporal storage abstractions for `NestGate`.
//!
//! Extracted from nestgate-core to enable parallel compilation.

#![warn(missing_docs)]
#![expect(clippy::struct_excessive_bools)]
// Config/detection structs intentionally use many feature flags
// Stub pipelines and `Option`/`Result` carriers for `?` trigger pedantic style lints; kept for API stability.
#![expect(clippy::unnecessary_wraps)]
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
