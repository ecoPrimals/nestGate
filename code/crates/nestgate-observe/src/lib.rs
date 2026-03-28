// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Observability, diagnostics, and event system for NestGate.

mod stubs;

pub use stubs::canonical_types;
pub use stubs::traits;

pub mod diagnostics;
pub mod events;
pub mod observability;
