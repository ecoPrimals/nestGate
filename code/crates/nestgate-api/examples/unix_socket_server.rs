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
#![allow(
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

//! **UNIX SOCKET SERVER EXAMPLE**
//!
//! Demonstrates TRUE PRIMAL transport with Unix sockets + JSON-RPC 2.0.
//!
//! ## Usage
//!
//! ```bash
//! # Terminal 1: Start server
//! NESTGATE_FAMILY_ID=example cargo run --example unix_socket_server
//!
//! # Terminal 2: Test with curl (requires socat)
//! echo '{"jsonrpc":"2.0","method":"health.ping","params":{},"id":1}' | socat - UNIX-CONNECT:/tmp/nestgate-example.sock
//! echo '{"jsonrpc":"2.0","method":"identity.get","params":{},"id":2}' | socat - UNIX-CONNECT:/tmp/nestgate-example.sock
//! ```

use nestgate_api::transport::{NestGateRpcHandler, TransportConfig, TransportServer};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create configuration from environment
    let config = TransportConfig::from_env()?;

    println!("🚀 Starting NestGate TRUE PRIMAL Server");
    println!("   Family: {}", config.family_id);
    println!("   Socket: {}", config.socket_path.display());
    println!();
    println!("📡 Listening for JSON-RPC 2.0 requests...");
    println!();
    println!("Try these methods:");
    println!("  - health.ping");
    println!("  - health.status");
    println!("  - identity.get");
    println!("  - identity.capabilities");
    println!("  - system.info");
    println!();

    // Create RPC handler
    let handler = NestGateRpcHandler::new();

    // Create and start server
    let server = TransportServer::new(config, handler)?;

    // Setup Ctrl+C handler
    let server_clone = server.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        println!("\n📡 Shutting down...");
        server_clone.shutdown();
    });

    // Start server
    server.start().await?;

    println!("✅ Server stopped");
    Ok(())
}
