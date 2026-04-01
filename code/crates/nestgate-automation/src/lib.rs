// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Automation system for intelligent dataset management
//!
//! Provides predictive analytics and automated optimization for storage systems.

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
