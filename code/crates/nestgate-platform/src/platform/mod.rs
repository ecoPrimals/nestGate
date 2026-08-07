// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Platform-specific utilities — safe, cross-platform abstractions over system APIs.
//!
//! **G68 Platform Substrate.** Each sub-module owns one abstraction layer so
//! business-logic crates never import `std::os::unix::*`, `PermissionsExt`,
//! `rustix::process`, or `rustix::system` directly.
//!
//! | Module    | G68 Layer | Abstracts |
//! |-----------|-----------|-----------|
//! | [`links`] | L1        | symlink / junction / hard-link |
//! | [`fs`]    | L2        | permissions (`chmod` / ACL) |
//! | [`process`] | L3     | PID liveness, hostname, runtime base dir |
//! | [`uid`]   | L3        | UID / GID |

pub mod fs;
pub mod links;
pub mod process;
pub mod uid;

pub use uid::{get_current_gid, get_current_uid};
