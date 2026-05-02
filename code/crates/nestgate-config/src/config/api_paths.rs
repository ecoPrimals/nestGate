// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! API path and endpoint layout is defined on the canonical network configuration type.
//!
//! The former `ApiPathsConfig` struct was removed; use `ApiPathsConfigCanonical`.

/// Canonical network configuration (successor to the removed `ApiPathsConfig` DTO).
pub type ApiPathsConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
