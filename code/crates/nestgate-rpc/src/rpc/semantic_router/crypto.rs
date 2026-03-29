// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Crypto domain semantic methods
//!
//! **Integration:** Cryptographic operations delegate to `nestgate-core` `crypto` when available.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

pub(super) fn crypto_encrypt(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: crypto.encrypt (nestgate-core crypto)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}

pub(super) fn crypto_decrypt(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: crypto.decrypt (nestgate-core crypto)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}

pub(super) fn crypto_generate_key(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: crypto.generate_key (nestgate-core crypto)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}

pub(super) fn crypto_generate_nonce(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: crypto.generate_nonce (nestgate-core crypto)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}

pub(super) fn crypto_hash(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: crypto.hash (nestgate-core crypto)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}

pub(super) fn crypto_verify_hash(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: crypto.verify_hash (nestgate-core crypto)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}
