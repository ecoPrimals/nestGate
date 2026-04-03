// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]
// Transitional: this crate still references deprecated `service_discovery` and related modules
// internally; `#[deprecated]` on those modules warns downstream consumers, not this crate.
#![allow(deprecated)]

//! Capability and peer discovery helpers for the `NestGate` storage primal.
//!
//! # In scope (NUCLEUS-aligned)
//!
//! - **Self-knowledge** — who this primal is and what it can do (`primal_self_knowledge`, `self_knowledge`).
//! - **Capability-based peer lookup** — environment-driven and JSON-RPC IPC paths toward the orchestration provider and peers.
//! - **Thin registry client** — types and helpers that query or follow the ecosystem registry via IPC,
//!   without embedding a full registry implementation in `NestGate`.
//!
//! # Out of scope (orchestration / ecosystem platform)
//!
//! These concerns remain in this crate only as transitional, deprecated surfaces:
//!
//! - **Full service registry** — universal in-process registries belong with the orchestration provider.
//! - **Multi-backend discovery** (mDNS, Consul, Kubernetes, HTTP bootstrap) — platform discovery belongs with the orchestration provider.
//! - **Orchestration capability taxonomy** — ecosystem orchestration layer owns orchestration classification and routing.
//!
//! Peer and service discovery at production scale are delegated to the **orchestration provider** IPC and the **ecosystem platform**; `NestGate`
//! keeps the minimal glue needed for storage operations and capability-aware calls over existing transports.

#![cfg_attr(
    test,
    allow(
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
#[deprecated(
    since = "0.3.0",
    note = "Service registry and orchestration discovery are orchestration-provider concerns. NestGate retains only capability-based peer lookup via env and JSON-RPC IPC."
)]
/// Optional discovery mechanism implementations (mDNS, Consul, K8s, HTTP); generic platform discovery belongs with the orchestration provider.
pub mod discovery_mechanism;
pub mod infant_discovery;
pub mod primal_discovery;
pub mod primal_self_knowledge;
pub mod self_knowledge;
#[deprecated(
    since = "0.3.0",
    note = "Service registry and orchestration discovery are orchestration-provider concerns. NestGate retains only capability-based peer lookup via env and JSON-RPC IPC."
)]
/// Universal service registry patterns; full registry belongs with the orchestration provider.
pub mod service_discovery;
pub mod universal_primal_discovery;
