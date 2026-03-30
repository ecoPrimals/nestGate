// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![forbid(unsafe_code)]
// Clean, debt-free middleware system with unified configuration

// Core modules (canonical implementation)
//! Lib module

#![warn(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::float_cmp,
        clippy::uninlined_format_args,
        clippy::needless_pass_by_value,
        clippy::cast_precision_loss,
        clippy::items_after_statements,
    )
)]
#![allow(
    deprecated,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools
)]

pub mod config;

// Re-export core types (clean, no conflicts)
pub use config::*;

#[cfg(test)]
mod crate_smoke_tests {
    use super::*;

    #[test]
    fn reexports_create_default_config() {
        let c = create_default_config();
        assert!(!c.system.instance_name.is_empty());
    }
}
