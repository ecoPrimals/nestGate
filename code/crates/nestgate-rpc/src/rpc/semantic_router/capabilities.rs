// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capabilities domain semantic methods (`capabilities.*`)

use super::SemanticRouter;
use nestgate_types::error::Result;
use serde_json::{Value, json};

/// Route `capabilities.list` → supported semantic RPC method identifiers.
#[expect(
    clippy::unnecessary_wraps,
    reason = "JSON-RPC semantic handlers use Result for uniform router dispatch"
)]
pub(super) fn capabilities_list(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Ok(json!({
        "primal": nestgate_config::constants::system::DEFAULT_SERVICE_NAME,
        "version": env!("CARGO_PKG_VERSION"),
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
            "health.liveness",
            "health.readiness",
            "health.metrics",
            "health.info",
            "metadata.store",
            "metadata.retrieve",
            "metadata.search",
            "crypto.encrypt",
            "crypto.decrypt",
            "crypto.generate_key",
            "crypto.generate_nonce",
            "crypto.hash",
            "crypto.verify_hash",
            "session.save",
            "session.load",
            "session.list",
            "session.delete",
            "capabilities.list",
            "identity.get"
        ]
    }))
}
