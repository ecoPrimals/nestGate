// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Data domain semantic methods (live feeds, NOT storage)
//!
//! NestGate is a **storage** primal and does not implement data fetching.
//! Any `data.*` method is delegated to whichever primal advertises the
//! `"data"` capability at runtime (resolved via `discovery.query`, not by
//! hardcoding external-domain method names like NCBI or NOAA).
//!
//! Per wateringHole capability-based discovery, callers must discover a primal
//! that advertises the `"data"` capability and delegate there via IPC.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

/// Catch-all delegation for any `data.*` method.
///
/// Returns a structured `NotImplemented` error directing callers to discover
/// the data capability provider at runtime rather than calling NestGate.
pub(super) fn data_delegation(
    _router: &SemanticRouter,
    method: &str,
    _params: Value,
) -> Result<Value> {
    let operation = method.strip_prefix("data.").unwrap_or(method);
    tracing::debug!("data.{operation}: delegating to data capability provider");
    Err(NestGateError::not_implemented(format!(
        "data.{operation}: NestGate delegates data operations to data capability providers. \
         Discover them via `discovery.query` with capability=\"data\" or \
         `NESTGATE_CAPABILITY_DATA` environment variable."
    )))
}
