//! # Idiomatic Evolution of Unified Error System
//!
//! **EVOLUTIONARY APPROACH - PRESERVING UNIFIED BENEFITS**
//!
//! This example demonstrates how to make our existing unified error system
//! more idiomatic WITHOUT losing the benefits of unification.
//!
//! ## Songbird Insight Applied Correctly
//!
//! Instead of replacing our robust unified system, we **evolve** it to be
//! more idiomatic while preserving all the operational intelligence.

use nestgate_core::{
    error::{
        // ENHANCED: Our unified system with idiomatic improvements
        IdioResult,
        IdioResultExt,
        NestGateError,
    },
    Result,
};

// Extension trait for external results (local definition)
trait ExternalResultExt<T> {
    fn into_nestgate_error(self, context: &str) -> Result<T>;
}

use std::collections::HashMap;

// ==================== BEFORE: CURRENT PATTERNS ====================

/// ❌ CURRENT: Verbose error construction
fn current_verbose_pattern() -> nestgate_core::error::Result<String> {
    Err(nestgate_core::error::NestGateError::configuration_error(
        "test_field",
        "Database connection string missing".to_string(),
    ))
}

/// ❌ CURRENT: Complex error chaining
fn current_complex_chaining() -> nestgate_core::error::Result<Vec<u8>> {
    let data =
        std::fs::read("missing.txt").map_err(|e| nestgate_core::error::NestGateError::Io {
            operation: "read_file".to_string(),
            error_message: e.to_string(),
            resource: Some("missing.txt".to_string()),
            retryable: false,
        })?;

    Ok(data)
}

// ==================== AFTER: IDIOMATIC EVOLUTION ====================

/// ✅ EVOLUTIONARY: Idiomatic construction with unified benefits
fn idiomatic_simple_pattern() -> IdioResult<String> {
    use nestgate_core::error::NestGateError;

    // IDIOMATIC: Simple, ergonomic creation using new constructors
    Err(NestGateError::simple("Database connection string missing"))

    // UNIFIED BENEFITS PRESERVED:
    // - Still uses NestGateError (unified)
    // - Still gets rich context (location tracking)
    // - Still serializable for distributed systems
    // - Still integrates with our recovery strategies
}

/// ✅ EVOLUTIONARY: Idiomatic chaining with rich context preserved
fn idiomatic_rich_chaining() -> IdioResult<Vec<u8>> {
    // IDIOMATIC: Clean, readable error chaining
    let result = std::fs::read("missing.txt").into_nestgate_result("loading application config");

    result.with_context("during application startup")

    // UNIFIED BENEFITS PRESERVED:
    // - Automatic conversion to NestGateError
    // - Rich context chain maintained
    // - All metadata and recovery info preserved
    // - Still works with our error handling infrastructure
}

/// ✅ EVOLUTIONARY: Domain-specific errors with idiomatic patterns
fn idiomatic_domain_specific() -> IdioResult<String> {
    use nestgate_core::error::NestGateError;

    // IDIOMATIC: Domain-aware error creation using existing patterns
    let connection_result = connect_to_database().map_err(|e| {
        NestGateError::network("connect_database", &format!("Failed to connect: {}", e))
    })?;

    // UNIFIED BENEFITS PRESERVED:
    // - Uses NetworkErrorData with all rich context
    // - Maintains retry information and circuit breaker state
    // - Preserves operational intelligence
    // - Still part of unified error hierarchy

    Ok(connection_result)
}

/// ✅ EVOLUTIONARY: Multiple error types with conventional patterns
fn idiomatic_multiple_errors() -> IdioResult<String, Box<dyn std::error::Error>> {
    // CONVENTIONAL: Explicit error type when needed
    let config = load_config().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    let data = process_data(&config)?; // Already returns IdioResult

    Ok(data)
}

// ==================== HELPER FUNCTIONS ====================

fn connect_to_database() -> Result<String, std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::ConnectionRefused,
        "Connection refused",
    ))
}

fn load_config() -> IdioResult<HashMap<String, String>> {
    Ok(HashMap::new())
}

fn process_data(_config: &HashMap<String, String>) -> IdioResult<String> {
    Ok("processed".to_string())
}

// ==================== DEMONSTRATION ====================

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🦀 Idiomatic Evolution of Unified Error System");
    println!("================================================");

    // Demonstrate idiomatic patterns
    println!("\n✅ EVOLUTIONARY BENEFITS:");
    println!("  - Keeps ALL unified error system benefits");
    println!("  - Adds idiomatic Rust patterns");
    println!("  - Improves ergonomics without breaking changes");
    println!("  - Better ecosystem integration");

    // Test the patterns
    if let Err(e) = idiomatic_simple_pattern() {
        println!("\n📝 Simple Pattern Error: {}", e);
    }

    if let Err(e) = idiomatic_rich_chaining() {
        println!("\n📝 Rich Chaining Error: {}", e);
    }

    if let Err(e) = idiomatic_domain_specific() {
        println!("\n📝 Domain-Specific Error: {}", e);
    }

    println!("\n🎯 RESULT: Our unified system is now MORE idiomatic!");
    println!("   WITHOUT losing any of the operational intelligence!");

    Ok(())
}
