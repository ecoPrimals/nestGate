// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Crypto domain semantic methods
//!
//! `NestGate` does NOT perform cryptographic operations locally.
//! Per `ecoBin` v3.0, crypto is delegated to whichever primal advertises
//! the `"security"` capability (discovered at runtime via capability-based
//! discovery, not by name).
//!
//! These handlers return structured JSON-RPC errors directing callers to
//! discover and contact the security capability provider via IPC.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

fn crypto_delegation_error(operation: &str) -> NestGateError {
    NestGateError::not_implemented(format!(
        "crypto.{operation}: NestGate delegates cryptographic operations to the security \
         capability provider. Discover it via `discovery.query` with capability=\"security\" \
         or `NESTGATE_CAPABILITY_SECURITY` environment variable."
    ))
}

/// Route `crypto.encrypt` → security capability provider (IPC delegation)
pub(super) fn crypto_encrypt(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("crypto.encrypt: delegating to security capability provider");
    Err(crypto_delegation_error("encrypt"))
}

/// Route `crypto.decrypt` → security capability provider (IPC delegation)
pub(super) fn crypto_decrypt(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("crypto.decrypt: delegating to security capability provider");
    Err(crypto_delegation_error("decrypt"))
}

/// Route `crypto.generate_key` → security capability provider (IPC delegation)
pub(super) fn crypto_generate_key(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("crypto.generate_key: delegating to security capability provider");
    Err(crypto_delegation_error("generate_key"))
}

/// Route `crypto.generate_nonce` → security capability provider (IPC delegation)
pub(super) fn crypto_generate_nonce(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("crypto.generate_nonce: delegating to security capability provider");
    Err(crypto_delegation_error("generate_nonce"))
}

/// Route `crypto.hash` → security capability provider (IPC delegation)
pub(super) fn crypto_hash(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("crypto.hash: delegating to security capability provider");
    Err(crypto_delegation_error("hash"))
}

/// Route `crypto.verify_hash` → security capability provider (IPC delegation)
pub(super) fn crypto_verify_hash(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("crypto.verify_hash: delegating to security capability provider");
    Err(crypto_delegation_error("verify_hash"))
}
