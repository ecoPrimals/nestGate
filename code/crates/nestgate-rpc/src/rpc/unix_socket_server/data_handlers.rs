// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Data domain JSON-RPC handlers (live feeds, NOT storage).
//!
//! NestGate is a storage primal and does not implement data fetching.
//! Any `data.*` method returns a structured delegation error directing
//! callers to discover a primal that advertises the `"data"` capability
//! via `discovery.query` or `NESTGATE_CAPABILITY_DATA`.
//!
//! `data.*` is intentionally excluded from `capabilities.list` — NestGate
//! routes these method names for ecosystem convenience but does not provide
//! the underlying data service.

use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

/// Catch-all delegation for any `data.*` method on the legacy Unix wire path.
pub(super) fn data_delegation(method: &str, _params: Option<&Value>) -> Result<Value> {
    let operation = method.strip_prefix("data.").unwrap_or(method);
    tracing::debug!("data.{operation}: delegating to data capability provider");
    Err(NestGateError::not_implemented(format!(
        "data.{operation}: NestGate delegates data operations to data capability providers. \
         Discover them via `discovery.query` with capability=\"data\" or \
         `NESTGATE_CAPABILITY_DATA` environment variable."
    )))
}
