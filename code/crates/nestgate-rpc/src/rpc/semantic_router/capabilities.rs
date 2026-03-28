// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Capabilities domain semantic methods (`capabilities.*`)

use super::SemanticRouter;
use nestgate_types::error::Result;
use serde_json::{json, Value};

/// Route `capabilities.list` → supported semantic RPC method identifiers.
pub(super) async fn capabilities_list(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Ok(json!({
        "methods": [
            "storage.put",
            "storage.get",
            "storage.delete",
            "storage.list",
            "storage.exists",
            "storage.metadata",
            "storage.dataset.create",
            "storage.dataset.get",
            "storage.dataset.list",
            "storage.dataset.delete",
            "discovery.announce",
            "discovery.query",
            "discovery.list",
            "discovery.capabilities",
            "health.check",
            "health.metrics",
            "health.info",
            "health.liveness",
            "health.readiness",
            "metadata.store",
            "metadata.retrieve",
            "metadata.search",
            "crypto.encrypt",
            "crypto.decrypt",
            "crypto.generate_key",
            "crypto.generate_nonce",
            "crypto.hash",
            "crypto.verify_hash",
            "capabilities.list"
        ]
    }))
}
