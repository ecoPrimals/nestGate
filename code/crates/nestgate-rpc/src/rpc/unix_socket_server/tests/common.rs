// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use nestgate_config::config::storage_paths::get_storage_base_path;

use crate::rpc::unix_socket_server::StorageState;

/// Cleanup a test family's dataset directory after a test.
pub async fn cleanup_family(family_id: &str) {
    let _ =
        tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(family_id)).await;
}

/// Build a minimal `StorageState` for handler tests.
pub async fn mock_state(family_id: Option<&str>) -> StorageState {
    StorageState {
        templates: crate::rpc::template_storage::TemplateStorage::new(),
        audits: crate::rpc::audit_storage::AuditStorage::new(),
        family_id: family_id.map(String::from),
        storage_initialized: true,
        encryption: None,
        method_gate: crate::rpc::method_gate::MethodGate::new(
            crate::rpc::method_gate::EnforcementMode::Permissive,
        ),
        caller_context: crate::rpc::method_gate::CallerContext::unix(),
    }
}

/// Build an encrypt-at-rest `StorageState` for handler tests.
pub fn encrypted_state(family_id: &str) -> StorageState {
    let mut key = [0u8; 32];
    for (i, b) in key.iter_mut().enumerate() {
        *b = i as u8;
    }
    StorageState {
        templates: crate::rpc::template_storage::TemplateStorage::new(),
        audits: crate::rpc::audit_storage::AuditStorage::new(),
        family_id: Some(family_id.to_string()),
        storage_initialized: true,
        encryption: Some(std::sync::Arc::new(
            crate::rpc::storage_encryption::StorageEncryption::new(key),
        )),
        method_gate: crate::rpc::method_gate::MethodGate::new(
            crate::rpc::method_gate::EnforcementMode::Permissive,
        ),
        caller_context: crate::rpc::method_gate::CallerContext::unix(),
    }
}
