// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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

//! RPC System Demo Example
//!
//! NOTE: This example demonstrates the RPC system concept.
//! The actual implementation may have evolved. This is kept as documentation.

use serde_json::json;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("🚀 NestGate RPC System Demo");
    println!("================================");
    println!();
    println!("This demo illustrates the RPC system architecture.");
    println!("The actual RPC implementation uses:");
    println!("  • UnifiedRpcRequest/Response for internal comms");
    println!("  • Capability-based routing via Universal Adapter");
    println!("  • Native async for zero-cost abstractions");
    println!();
    println!("Example RPC request structure:");
    let example_request = json!({
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "source": "nestgate",
        "target": "security",
        "method": "authenticate",
        "params": {"token": "example"},
        "timestamp": "2025-10-05T12:00:00Z",
        "streaming": false,
        "priority": "Normal"
    });
    println!("{}", serde_json::to_string_pretty(&example_request)?);
    println!();
    println!("✅ RPC system is capability-based and primal-agnostic");
    println!("✅ No hardcoded service endpoints");
    println!("✅ Runtime discovery via Universal Adapter");
    println!();
    println!("For real RPC usage, see:");
    println!("  • code/crates/nestgate-api/src/rest/rpc/");
    println!("  • code/crates/nestgate-core/src/universal_adapter/");

    Ok(())
}
