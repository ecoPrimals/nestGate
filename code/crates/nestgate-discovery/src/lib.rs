// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Thin capability discovery client for `NestGate`.
//!
//! Discovery of peers is delegated to songBird IPC; this crate retains only
//! the types and runtime helpers needed for capability-based peer lookup via
//! environment variables and JSON-RPC IPC.

#![cfg_attr(
    test,
    allow(
        deprecated,
        clippy::borrow_as_ptr,
        clippy::cast_lossless,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::default_constructed_unit_structs,
        clippy::default_trait_access,
        clippy::doc_markdown,
        clippy::drop_non_drop,
        clippy::expect_used,
        clippy::field_reassign_with_default,
        clippy::float_cmp,
        clippy::ip_constant,
        clippy::items_after_statements,
        clippy::iter_on_single_items,
        clippy::manual_string_new,
        clippy::needless_collect,
        clippy::needless_pass_by_value,
        clippy::panic,
        clippy::ref_as_ptr,
        clippy::redundant_clone,
        clippy::redundant_closure,
        clippy::redundant_closure_for_method_calls,
        clippy::similar_names,
        clippy::single_match,
        clippy::single_match_else,
        clippy::unchecked_time_subtraction,
        clippy::unnecessary_unwrap,
        clippy::unnecessary_wraps,
        clippy::uninlined_format_args,
        clippy::unused_async,
        clippy::unwrap_used,
    )
)]
// Many discovery surfaces return `Result` for forward-compatible evolution; `# Errors` is added
// incrementally on hot paths; pedantic `missing_errors_doc` is relaxed at crate level.
#![allow(clippy::missing_errors_doc)]
#![warn(missing_docs)]

pub mod capabilities;
pub mod capability_discovery;
/// Optional discovery mechanism implementations (see crate-level note: production discovery is delegated).
pub mod discovery_mechanism;
pub mod infant_discovery;
pub mod primal_discovery;
pub mod primal_self_knowledge;
pub mod self_knowledge;
pub mod service_discovery;
pub mod universal_primal_discovery;
