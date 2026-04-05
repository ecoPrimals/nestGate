// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Platform-specific utilities - Safe abstractions over system APIs
//!
//! This module provides safe, cross-platform abstractions for system-level
//! operations, eliminating the need for unsafe code in application logic.

pub mod uid;

pub use uid::{get_current_gid, get_current_uid};
