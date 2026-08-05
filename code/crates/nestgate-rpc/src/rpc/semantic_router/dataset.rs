// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Dataset domain semantic methods (`dataset.*`)
//!
//! Delegates to the dataset handlers in `unix_socket_server::dataset_handlers`
//! through a shared `StorageState`.

use super::{MetadataBackend, SemanticRouter};
use crate::rpc::unix_socket_server::StorageState;
use nestgate_types::error::Result;
use serde_json::Value;
use std::sync::OnceLock;

use crate::rpc::unix_socket_server::dataset_handlers;

/// Cached [`StorageState`] for dataset handler delegation.
fn shared_state() -> &'static StorageState {
    static STATE: OnceLock<StorageState> = OnceLock::new();
    STATE.get_or_init(|| {
        #[expect(
            clippy::expect_used,
            reason = "StorageState::new only fails on unrecoverable I/O — crash is correct"
        )]
        StorageState::new()
            .expect("StorageState initialization must not fail for dataset routing")
    })
}

/// Route `dataset.convergence` → CAS provenance state scan.
pub(super) async fn dataset_convergence(
    _router: &SemanticRouter<impl MetadataBackend>,
    params: Value,
) -> Result<Value> {
    dataset_handlers::dataset_convergence(Some(&params), shared_state()).await
}
