// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines,
        clippy::cognitive_complexity,
    )
)]
#![expect(
    deprecated,
    missing_docs,
    dead_code,
    unfulfilled_lint_expectations,
    unused_doc_comments,
    unused_imports,
    unused_variables,
    unused_comparisons,
    unused_must_use,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::too_long_first_doc_paragraph,
    clippy::inline_always,
    clippy::redundant_closure,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::single_char_pattern,
    clippy::implicit_hasher,
    clippy::float_cmp,
    clippy::uninlined_format_args,
    clippy::similar_names,
    clippy::items_after_statements,
    clippy::empty_line_after_doc_comments,
    clippy::unreadable_literal,
    clippy::manual_clamp,
    clippy::pub_underscore_fields,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::wildcard_in_or_patterns,
    clippy::type_complexity,
    clippy::field_reassign_with_default,
    clippy::module_inception,
    clippy::unnecessary_get_then_check,
    clippy::cmp_null,
    clippy::redundant_clone,
    clippy::absurd_extreme_comparisons,
    clippy::no_effect_underscore_binding,
    clippy::default_constructed_unit_structs,
    clippy::manual_string_new,
    clippy::assertions_on_constants,
    clippy::unnecessary_unwrap,
    clippy::needless_collect,
    clippy::drop_non_drop,
    clippy::zero_sized_map_values,
    clippy::match_single_binding,
    clippy::match_same_arms,
    clippy::overly_complex_bool_expr,
    clippy::needless_character_iteration,
    clippy::manual_range_contains,
    clippy::bool_assert_comparison,
    clippy::single_component_path_imports,
    clippy::used_underscore_binding
)]

//! **OUTDATED EXAMPLE - NEEDS UPDATE**
//!
//! This example demonstrates how to set up and run the `NestGate` API server
//! with full ZFS integration for development and testing purposes.
//!
//! **Status**: ⚠️ OUTDATED - APIs have changed since this was written
//! **Last Updated**: Before November 2025
//! **Needs**: Update to use current nestgate-api routing and handler APIs
//!
//! **Current API Usage**:
//! See `nestgate-api/src/routes/mod.rs` for `create_router()` and `AppState`
//!
//! **Update Status**: Deferred to examples refresh phase
//! **Priority**: Low (documentation/examples)
//!
//! This example is excluded from compilation pending API stabilization.
//! Current working examples can be found in integration tests.

/// Main
fn main() {
    eprintln!("⚠️  This example is outdated and needs to be updated.");
    eprintln!("See the comment at the top of this file for details.");
    eprintln!();
    eprintln!("For current API usage, see:");
    eprintln!("  - code/crates/nestgate-api/src/routes/mod.rs");
    eprintln!("  - code/crates/nestgate-api/src/handlers/");
    std::process::exit(1);
}

/*
// === OUTDATED CODE BELOW - KEPT FOR REFERENCE ===

use nestgate_api::{serve_with_zfs, Config};
use nestgate_core::config::defaults::{NetworkAddressDefaults, NetworkPortDefaults};
use nestgate_zfs::{config::ZfsConfig, ZfsManager};
use std::sync::Arc;
// Removed unused tracing import
use std::fmt;
use tracing::error;
use tracing::info;
use tracing::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "nestgate_api=debug,nestgate_zfs=debug,tower_http=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting NestGate API Development Server");

    // Create ZFS configuration
    let zfs_config = ZfsConfig::default();

    // Initialize ZFS manager
    info!("Initializing ZFS manager");
    let zfs_manager = match ZfsManager::new(zfs_config).await {
        Ok(manager) => {
            info!("ZFS manager initialized successfully");
            Arc::new(manager)
        }
        Err(e) => {
            error!("Failed to initialize ZFS manager: {}", e);
            warn!("Continuing without ZFS integration");
            return Err(e.into());
        }
    };

    // Create API configuration with centralized defaults
    let bind_addr = format!(
        "{}:{}",
        NetworkAddressDefaults::get_development_bind_address(),
        NetworkPortDefaults::get_dev_server_port()
    );

    let api_config = Config {
        bind_addr: std::env::var("NESTGATE_DEV_SERVER_BIND").unwrap_or_else(|_| bind_addr),
        enable_zfs_api: true,
        enable_sse: true,
        enable_websockets: true,
        max_request_size: 16 * 1024 * 1024, // 16MB
    };

    info!("API server configuration:");
    info!("  Bind endpoint: {}", api_config.bind_addr);
    info!("  ZFS API enabled: {}", api_config.enable_zfs_api);
    info!("  SSE enabled: {}", api_config.enable_sse);
    info!("  WebSockets enabled: {}", api_config.enable_websockets);
    info!(
        "  Max request size: {}MB",
        api_config.max_request_size / (1024 * 1024)
    );

    // Print available endpoints
    print_available_endpoints();

    // Start the server
    info!("Starting API server...");
    if let Err(e) = serve_with_zfs(api_config, zfs_manager).await {
        error!("API server failed: {}", e);
        return Err(e);
    }
}

/// Print Available Endpoints
fn print_available_endpoints() {
    let port = NetworkPortDefaults::get_dev_server_port().to_string();

    info!("Development server running successfully!");
    info!("Available endpoints:");
    info!("  curl http://localhost:{}/health", port);
    info!("  curl http://localhost:{}/api/v1/zfs/pools", port);
    info!(
        "  curl -X POST http://localhost:{}/api/v1/zfs/pools \\",
        port
    );
    info!("       -H 'Content-Type: application/json' \\");
    info!("       -d '{{\"name\":\"test-pool\",\"devices\":[\"/dev/loop0\"]}'");
    info!("Web interface (if available): http://localhost:{}/", port);
}

*/
