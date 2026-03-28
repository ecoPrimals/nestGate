// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Crypto domain semantic methods
//!
//! TODO: wire to nestgate-core `crypto`.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

pub(super) async fn crypto_encrypt(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}

pub(super) async fn crypto_decrypt(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}

pub(super) async fn crypto_generate_key(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}

pub(super) async fn crypto_generate_nonce(
    _router: &SemanticRouter,
    _params: Value,
) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}

pub(super) async fn crypto_hash(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}

pub(super) async fn crypto_verify_hash(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core crypto",
    ))
}
