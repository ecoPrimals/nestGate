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
//! Modern `NestGate` Demo
//!
//! Demonstrates the modern Rust implementations including:
//! - HTTP Client with connection pooling
//! - Configuration validation with detailed reporting
//! - Performance monitoring with real-time metrics
//! - Error handling with rich context
//!
//! **Status**: ⚠️ OUTDATED - APIs have changed since this was written\
//! **Last Updated**: Before November 2025\
//! **Issues**: 22 compilation errors due to API changes\
//! **Needs**: Complete rewrite using current nestgate-core and nestgate-api APIs  
//!
//! **Update Status**: Deferred to examples refresh phase
//! **Priority**: Low (documentation/examples)
//! **Reference Modules**:
//! - `nestgate_core::traits` - Core trait definitions
//! - `nestgate_core::config::canonical_primary` - Unified configuration
//! - `nestgate_api::handlers` - API handler implementations
//!
//! This example is excluded from compilation pending API stabilization.
//! Current working examples can be found in integration tests and handler modules.

/// Main
fn main() {
    eprintln!("⚠️  This example is outdated and needs to be updated.");
    eprintln!("The APIs demonstrated here have undergone significant refactoring.");
    eprintln!();
    eprintln!("For current usage examples, see:");
    eprintln!("  - code/crates/nestgate-core/src/traits/");
    eprintln!("  - code/crates/nestgate-api/src/handlers/");
    eprintln!("  - tests/ directory for working examples");
    std::process::exit(1);
}

/*
// === OUTDATED CODE BELOW - KEPT FOR REFERENCE ===

use std::time::Duration;
use tokio::time::sleep;

// Import our modern modules
use nestgate_core::config::validation::{ConfigValidation, ConfigValidator, NetworkConfig};
use nestgate_core::error::Result;
use nestgate_core::network::client::{http_endpoint, https_endpoint, ClientConfig, HttpClient};
use nestgate_core::performance::{
    AlertThresholds, ConsoleAlertCallback, PerformanceMonitor, PerformanceTimer,
};

#[tokio::main]
fn main_old() -> impl std::future::Future<Output = Result<()>> + Send {
    println!("🚀 **MODERN NESTGATE DEMONSTRATION**");
    println!("====================================\n");

    // 1. Configuration Validation Demo
    println!("📋 **1. CONFIGURATION VALIDATION DEMO**");
    println!("---------------------------------------");

    demo_configuration_validation().await?;

    // 2. HTTP Client Demo
    println!("\n🌐 **2. HTTP CLIENT DEMO**");
    println!("-------------------------");

    demo_http_client().await?;

    // 3. Performance Monitoring Demo
    println!("\n📊 **3. PERFORMANCE MONITORING DEMO**");
    println!("------------------------------------");

    demo_performance_monitoring().await?;

    println!("\n✅ **DEMO COMPLETED SUCCESSFULLY!**");
    println!("All modern systems are working perfectly! 🎉");

    Ok(())
}

/// Demonstrate configuration validation with detailed reporting
fn demo_configuration_validation() -> impl std::future::Future<Output = Result<()>> + Send {
    // Create a valid configuration
    let valid_config = NetworkConfig::default();
    println!("✅ **Valid Configuration:**");
    let report = ConfigValidator::generate_report(&valid_config);
    println!("{}\n", report);

    // Create an invalid configuration
    let mut invalid_config = NetworkConfig::default();
    invalid_config.port = 0; // Invalid port
    invalid_config.bind_address = "invalid_ip".to_string(); // Invalid IP
    invalid_config.enable_tls = true; // TLS enabled but no cert paths

    println!("❌ **Invalid Configuration:**");
    let report = ConfigValidator::generate_report(&invalid_config);
    println!("{}\n", report);

    // Demonstrate strict validation
    match ConfigValidator::validate_strict(&invalid_config) {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("🔍 **Strict Validation Error**: {}\n", e),
    }

    Ok(())
}

/// Demonstrate HTTP client with connection pooling
fn demo_http_client() -> impl std::future::Future<Output = Result<()>> + Send {
    // Create HTTP client with custom configuration
    let config = ClientConfig {
        timeout: nestgate_core::network::client::TimeoutMs::new(5000),
        max_connections: 50,
        max_connections_per_host: 5,
        enable_compression: true,
        follow_redirects: true,
        max_redirects: 3,
        user_agent: "NestGate-Demo/1.0".to_string(),
    };

    let client = HttpClient::new(config);
    println!("🔧 **HTTP Client Created** with connection pooling");

    // Create endpoints
    let http_endpoint = http_endpoint("httpbin.org", 80)?;
    let https_endpoint = https_endpoint("httpbin.org", 443)?;

    println!("🌍 **HTTP Endpoint**: {}", http_endpoint.url());
    println!("🔒 **HTTPS Endpoint**: {}", https_endpoint.url());

    // Simulate multiple requests (would work with real endpoints)
    println!("📡 **Simulating HTTP requests...**");

    for i in 1..=3 {
        println!("   Request {}: Simulated GET /get", i);
        // In a real scenario: let response = client.get(&https_endpoint, "/get").await?;
        sleep(Duration::from_millis(100)).await; // Simulate network delay
    }

    // Get client statistics
    let stats = client.stats();
    println!("📈 **Client Statistics**:");
    println!("   • Total connections: {}", stats.total_connections);
    println!("   • Active requests: {}", stats.active_requests);
    println!("   • Total requests: {}", stats.total_requests);
    println!("   • Failed requests: {}", stats.failed_requests);

    Ok(())
}

/// Demonstrate performance monitoring with real-time metrics
fn demo_performance_monitoring() -> impl std::future::Future<Output = Result<()>> + Send {
    // Create performance monitor with custom thresholds
    let thresholds = AlertThresholds {
        min_success_rate: 90.0,
        max_response_time: Duration::from_millis(500),
        max_memory_bytes: 100_000_000, // 100MB
        max_error_rate: 10.0,
    };

    let monitor = PerformanceMonitor::new(thresholds);
    let collector = monitor.collector();

    // Add console alert callback
    monitor
        .add_alert_callback(Box::new(ConsoleAlertCallback))
        .await;

    println!("📊 **Performance Monitor Started**");

    // Simulate some operations with performance tracking
    println!("⚡ **Simulating Operations...**");

    // Simulate successful operations
    for i in 1..=5 {
        let timer = PerformanceTimer::start(collector.clone(), format!("operation_{}", i));

        // Simulate work
        sleep(Duration::from_millis(50 + i * 10)).await;
        timer.complete_success().await;

        collector.record_connection_opened();
        println!("   ✅ Operation {} completed successfully", i);
    }

    // Simulate some failures
    for i in 1..=2 {
        let timer = PerformanceTimer::start(collector.clone(), format!("failed_operation_{}", i));

        sleep(Duration::from_millis(30)).await;
        timer.complete_failure("timeout").await;

        println!("   ❌ Operation {} failed (timeout)", i);
    }

    // Update memory usage
    collector.update_memory_usage(75_000_000); // 75MB

    // Get performance snapshot
    let snapshot = collector.get_snapshot().await;
    println!("\n📈 **Performance Report:**");
    println!("{}", snapshot.generate_report());

    // Check for alerts
    println!("\n🚨 **Checking for Alerts...**");
    let alerts = monitor.check_alerts().await?;

    if alerts.is_empty() {
        println!("   ✅ No alerts triggered - system is healthy!");
    } else {
        println!("   ⚠️  {} alert(s) triggered:", alerts.len());
        for alert in alerts {
            println!("      • {:?}: {}", alert.alert_type, alert.message);
        }
    }

    // Demonstrate performance grading
    println!("\n🎯 **Performance Grade**: {:?}", snapshot.get_grade());
    println!(
        "🏥 **System Health**: {}",
        if snapshot.is_healthy() {
            "HEALTHY ✅"
        } else {
            "NEEDS ATTENTION ⚠️"
        }
    );

    Ok(())
}

/// Helper function to demonstrate error handling
fn demonstrate_error_handling() -> impl std::future::Future<Output = Result<()>> + Send {
    use nestgate_core::error::NestGateError;

    // Demonstrate different error types
    let validation_error =
        NestGateError::validation_error("demo_field", "This is a demonstration validation error");

    let network_error = NestGateError::network_error("Failed to connect to demonstration endpoint");

    let timeout_error = NestGateError::timeout_error("Demonstration operation timed out");

    println!("🔍 **Error Handling Examples:**");
    println!("   • Validation Error: {}", validation_error);
    println!("   • Network Error: {}", network_error);
    println!("   • Timeout Error: {}", timeout_error);

    Ok(())
}
*/
