// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Audit JSON-RPC Handlers
//!
//! Extracted from unix_socket_server for domain-based refactoring.
//! Handles: audit.store_execution

use nestgate_types::error::{NestGateError, Result};
use serde_json::{json, Value};
use tracing::debug;

use super::StorageState;

/// audit.store_execution - Store execution audit trail
pub(super) async fn audit_store_execution(
    params: &Option<Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .as_ref()
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    // Deserialize the entire audit structure from params
    let audit: crate::rpc::audit_storage::ExecutionAudit = serde_json::from_value(params.clone())
        .map_err(|e| {
        NestGateError::invalid_input_with_field(
            "audit_data",
            format!("Invalid audit data format: {}", e),
        )
    })?;

    let audit_id = state.audits.store_audit(audit).await?;

    debug!("Stored execution audit '{}'", audit_id);

    Ok(json!({
        "audit_id": audit_id,
        "stored_at": chrono::Utc::now().to_rfc3339(),
        "success": true
    }))
}
