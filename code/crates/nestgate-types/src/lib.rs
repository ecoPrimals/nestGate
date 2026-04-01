// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Foundational error, result, and unified enum types shared across `NestGate` crates.

#![warn(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::float_cmp,
        clippy::single_char_pattern,
    )
)]

pub mod error;
pub mod result_types;
pub mod unified_enums;

pub use error::{
    CanonicalResult, ErrorContext, NestGateError, NestGateUnifiedError, Result, ResultExt,
    RetryInfo, TestResult,
};
pub use result_types::{ConnectionFactory, HealthCheckFn, ValidatorFn, VoidResult};
pub use unified_enums::UnifiedEnum;

#[cfg(test)]
mod round5_impl_coverage;

#[cfg(test)]
mod unified_enums_exhaustive_tests;

#[cfg(test)]
mod error_mod_coverage_tests {
    use crate::error::ResultExt;
    use crate::{NestGateError, NestGateUnifiedError};

    #[test]
    fn result_ext_to_canonical_ok_with_context_err() {
        let ok: std::result::Result<i32, NestGateError> = Ok(7);
        assert_eq!(ok.to_canonical().expect("ok"), 7);

        let err: std::result::Result<(), &str> = Err("e");
        let mapped = err.with_context(|| "ctx".to_string());
        assert!(mapped.is_err());
    }

    #[test]
    fn type_aliases_compile() {
        let _: crate::CanonicalResult<i32> = Ok(1);
        let _: crate::TestResult = Ok(());
        let _: crate::Result<i32> = Err(NestGateUnifiedError::internal("i"));
    }
}

#[cfg(test)]
mod result_types_macro_tests {
    use crate::NestGateError;
    use crate::validation_result;

    #[test]
    fn validation_result_ok_passthrough() {
        let r: Result<i32, NestGateError> = Ok(7);
        let out = validation_result!(r);
        assert_eq!(out.expect("ok"), 7);
    }

    #[test]
    fn validation_result_err_maps_via_from() {
        let r: Result<i32, NestGateError> = Err(NestGateError::simple("e"));
        let out = validation_result!(r);
        assert!(out.is_err());
    }
}
