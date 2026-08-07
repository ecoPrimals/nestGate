// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Platform-specific utilities: environment handling, Linux proc metrics, and OS abstractions.
//!
//! **G68 Platform Substrate** — the canonical home for all platform-divergent APIs.
//! Business-logic crates (`nestgate-rpc`, `nestgate-config`, `nestgate-storage`) call
//! through this crate instead of importing `std::os::unix::*`, `PermissionsExt`, or
//! raw `rustix` syscalls directly.

#![warn(missing_docs)]

pub mod env_process;
pub mod linux_proc;
pub mod platform;

pub use platform::{get_current_gid, get_current_uid};
