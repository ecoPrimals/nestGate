// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]
#![deprecated(
    since = "4.7.0",
    note = "Automation/orchestration concerns delegated to biomeOS. Zero production consumers remain in the NestGate workspace. This crate will be removed in a future version."
)]

//! **DEPRECATED** — Automation system for intelligent dataset management.
//!
//! This crate provided predictive analytics and automated optimization for storage
//! systems. As of v4.7.0, all production consumers have been removed:
//!
//! - Storage-tier prediction logic is inlined where needed
//! - Ecosystem/orchestration types belong with biomeOS
//! - Dataset lifecycle policies are handled by `nestgate-zfs` directly
//!
//! The crate remains in the workspace for compilation but has zero importers.

#![warn(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::float_cmp,
        clippy::uninlined_format_args,
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
    clippy::struct_excessive_bools,
    clippy::similar_names,
    clippy::unreadable_literal,
    clippy::or_fun_call,
    clippy::wildcard_in_or_patterns,
    clippy::no_effect_underscore_binding,
    clippy::needless_pass_by_value,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::must_use_candidate
)]

pub mod analysis;
pub mod error;
pub mod lifecycle;
pub mod manager;
pub mod types;

// Re-export commonly used types
pub use error::{AutomationError, Result};
pub use manager::IntelligentDatasetManager;
