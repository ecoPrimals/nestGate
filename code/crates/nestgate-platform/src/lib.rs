// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Platform-specific utilities: environment handling, Linux proc metrics, and OS abstractions.

pub mod env_process;
pub mod linux_proc;
pub mod platform;

pub use platform::{get_current_gid, get_current_uid};
