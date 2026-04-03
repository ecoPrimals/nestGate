// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Security delegation, certificate parsing, and zero-cost security providers for `NestGate`.
//!
//! # Delegation Model
//!
//! Core cryptographic operations (encrypt, decrypt, sign, verify, hash) are **delegated**
//! to the security capability provider (bearDog) via `crypto.*` JSON-RPC IPC. This crate
//! provides:
//!
//! - **`CryptoDelegate`** — IPC client that forwards crypto requests to the security provider
//! - **Certificate parsing** — local X.509 PEM/DER parsing and validity checking (`x509-parser`)
//! - **Certificate fingerprinting** — local SHA-256 digest of certificate material for identification
//! - **JWT claims** — claim structure and validation types (signing delegated)
//! - **Zero-cost security providers** — trait-based auth/security abstractions
//!
//! `NestGate` does **not** perform encryption, signing, or key generation locally.
//! Those operations route through the ecosystem's security capability provider.

#![warn(missing_docs)]
#![allow(deprecated, clippy::missing_errors_doc)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::float_cmp,
    )
)]

/// Crate-local primal-agnostic adapter handle used by certificate utilities.
///
/// **Integration:** `nestgate_core::universal_adapter::UniversalAdapter` (exported as
/// `PrimalAgnosticAdapter` in core) is the full implementation. `nestgate-security` cannot depend on
/// `nestgate-core` because `nestgate-core` already depends on this crate; inject or replace this
/// type only after a shared trait or `nestgate-types` bridge breaks that cycle.
pub mod universal_adapter {
    /// Lightweight endpoint holder for security flows that only need a resolved adapter URL.
    pub struct PrimalAgnosticAdapter {
        _endpoint: String,
    }

    impl PrimalAgnosticAdapter {
        /// Creates an adapter bound to the given endpoint URL.
        pub fn new(endpoint: String) -> Self {
            tracing::debug!(
                "feature pending: full universal adapter integration; using local endpoint holder"
            );
            Self {
                _endpoint: endpoint,
            }
        }
    }
}

pub mod cert;
pub mod crypto;
pub mod jwt_validation;
pub mod zero_cost;
pub mod zero_cost_security_provider;

#[cfg(test)]
mod lib_smoke_tests {
    use super::universal_adapter::PrimalAgnosticAdapter;

    #[test]
    fn primal_agnostic_adapter_new_is_constructible() {
        let _ = PrimalAgnosticAdapter::new("http://localhost:0/adapter".to_string());
    }
}
