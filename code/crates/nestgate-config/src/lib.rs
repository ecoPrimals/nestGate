// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! Configuration, constants, and defaults for the `NestGate` ecosystem.
//!
//! Extracted from nestgate-core to enable parallel compilation.

#![warn(missing_docs)]
// Unit/integration tests use `unwrap`/`expect` and other patterns that are intentionally strict.
#![cfg_attr(
    test,
    allow(
        clippy::assertions_on_constants,
        clippy::bool_assert_comparison,
        clippy::expect_used,
        clippy::field_reassign_with_default,
        clippy::float_cmp,
        clippy::for_kv_map,
        clippy::implicit_clone,
        clippy::ip_constant,
        clippy::items_after_statements,
        clippy::manual_string_new,
        clippy::map_unwrap_or,
        clippy::no_effect_underscore_binding,
        clippy::panic,
        clippy::redundant_clone,
        clippy::redundant_closure_for_method_calls,
        clippy::single_char_pattern,
        clippy::single_match,
        clippy::stable_sort_primitive,
        clippy::unnecessary_map_on_constructor,
        clippy::unnecessary_wraps,
        clippy::uninlined_format_args,
        clippy::unwrap_used
    )
)]
// Pedantic `missing_errors_doc` across nested config helpers: errors are always [`nestgate_types::error::NestGateError`].
#![expect(clippy::missing_errors_doc)]
// Legacy and generated config DTOs intentionally group many feature toggles in one struct.
#![expect(clippy::struct_excessive_bools)]
// Port and path structs use explicit `*_port` / `*_dir` suffixes for serde and builder clarity.
#![expect(clippy::struct_field_names)]
// Prefer explicit `match`/`if let` for env parsing in several modules; `map_or_else` is not always clearer.
#![expect(clippy::option_if_let_else)]
#[cfg(test)]
pub use nestgate_platform::env_process;

/// Canonical modernization helpers (migration, patterns, evolution).
pub mod canonical_modernization;
/// Capability-centric configuration surface.
pub mod capability_based_config;
/// Capability registry and discovery configuration types.
pub mod capability_config;
/// `NestGate` configuration tree, validation, and canonical primary config.
pub mod config;
/// Ports, addresses, timeouts, and shared compile-time defaults.
pub mod constants;
/// Default values, env-backed helpers, and URL builders.
pub mod defaults;
/// Defaults v2 bridge for layered configuration.
pub mod defaults_v2_config;
/// Runtime environment detection (`Environment`, operation mode).
pub mod environment;
/// Environment variable loading and `EnvironmentConfig`.
pub mod environment_config;
/// Sovereignty-related configuration types.
pub mod sovereignty_config;

pub use config::NestGateCanonicalConfig as CanonicalConfig;
pub use config::canonical_primary::NestGateCanonicalConfig;
pub use constants::*;
