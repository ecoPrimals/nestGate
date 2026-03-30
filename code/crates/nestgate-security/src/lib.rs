// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Security, cryptography, certificate management, and zero-cost security providers for `NestGate`.

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
        #[allow(dead_code)]
        endpoint: String,
    }

    impl PrimalAgnosticAdapter {
        /// Creates an adapter bound to the given endpoint URL.
        pub fn new(endpoint: String) -> Self {
            tracing::debug!(
                "feature pending: full universal adapter integration; using local endpoint holder"
            );
            Self { endpoint }
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
