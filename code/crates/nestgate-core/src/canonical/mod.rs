// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module provides canonical type aliases for complex types used throughout
// the NestGate ecosystem. This enables zero-copy operations and consistent typing.

//! Canonical module

pub mod types;

// Re-export all canonical types
pub use types::*;
