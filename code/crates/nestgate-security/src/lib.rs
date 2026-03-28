// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Security, cryptography, certificate management, and zero-cost security providers for NestGate.

/// TODO(nestgate-security): Wire to `nestgate_core::universal_adapter` when that module is available to this crate.
pub mod universal_adapter {
    /// Stub — was `nestgate_core::universal_adapter::PrimalAgnosticAdapter`.
    pub struct PrimalAgnosticAdapter {
        #[allow(dead_code)]
        endpoint: String,
    }

    impl PrimalAgnosticAdapter {
        /// Creates a stub adapter bound to the given endpoint URL.
        pub fn new(endpoint: String) -> Self {
            Self { endpoint }
        }
    }
}

pub mod cert;
pub mod crypto;
pub mod jwt_validation;
pub mod zero_cost;
pub mod zero_cost_security_provider;
