//! Compile-Time Assertions for Constant Validation
//!
//! This module provides modern Rust patterns for verifying constant relationships
//! at compile time rather than runtime, eliminating redundant runtime checks and
//! clippy warnings while maintaining code quality guarantees.
//!
//! # Deep Debt Solution
//!
//! **Problem**: Runtime assertions on constants trigger clippy warnings:
//! ```rust,ignore
//! assert!(CONST_A < CONST_B);  // ❌ Warning: assert!(true)
//! ```
//!
//! **Solution**: Compile-time assertions provide zero-cost guarantees:
//! ```rust
//! # const CONST_A: u32 = 5;
//! # const CONST_B: u32 = 10;
//! const _: () = assert!(CONST_A < CONST_B);  // ✅ Verified at compile time
//! ```
//!
//! # Benefits
//!
//! - ✅ **Zero Runtime Cost**: Checked during compilation
//! - ✅ **Compile-Time Failures**: Errors caught before tests run
//! - ✅ **Self-Documenting**: Constraints visible in code
//! - ✅ **Clippy Compliant**: No warnings about constant assertions
//! - ✅ **Type Safe**: Leverages Rust's const evaluation
//!
//! # Usage
//!
//! ```rust
//! use nestgate_core::const_assert;
//!
//! // Define your constants
//! const TIMEOUT_CONNECT: u64 = 5_000;
//! const TIMEOUT_REQUEST: u64 = 30_000;
//!
//! // Verify relationships at compile time
//! const_assert!(TIMEOUT_CONNECT < TIMEOUT_REQUEST);
//! const_assert!(TIMEOUT_CONNECT > 0);
//! const_assert!(TIMEOUT_REQUEST <= 60_000);
//! ```
//!
//! # Architecture
//!
//! This follows modern Rust idioms:
//! - Compile-time evaluation (`const fn` and `const` contexts)
//! - Zero-cost abstractions (no runtime overhead)
//! - Type-level programming (constraints as types)
//! - Self-documenting code (constraints visible)

/// Compile-time assertion macro
///
/// Verifies a constant expression at compile time, producing a compilation error
/// if the assertion fails. This has zero runtime cost and satisfies clippy's
/// requirements for constant validation.
///
/// # Examples
///
/// ```rust
/// # use nestgate_core::const_assert;
/// const MIN_SIZE: usize = 64;
/// const MAX_SIZE: usize = 1024;
///
/// // Verify size constraints at compile time
/// const_assert!(MIN_SIZE < MAX_SIZE);
/// const_assert!(MIN_SIZE >= 64);
/// const_assert!(MAX_SIZE <= 1024);
/// ```
///
/// # Compile-Time Failure
///
/// ```compile_fail
/// # use nestgate_core::const_assert;
/// const A: u32 = 10;
/// const B: u32 = 5;
/// const_assert!(A < B);  // Compilation error: assertion failed
/// ```
///
/// # Technical Details
///
/// Uses const evaluation to verify the condition at compile time.
/// The expression `const _: () = assert!(condition);` creates an unnamed
/// constant that must evaluate successfully during compilation.
#[macro_export]
macro_rules! const_assert {
    ($condition:expr) => {
        const _: () = assert!($condition);
    };
    ($condition:expr, $message:literal) => {
        const _: () = assert!($condition, $message);
    };
}

/// Compile-time equality assertion
///
/// Verifies that two constant expressions are equal at compile time.
///
/// # Examples
///
/// ```rust
/// # use nestgate_core::const_assert_eq;
/// const EXPECTED: u32 = 42;
/// const ACTUAL: u32 = 42;
///
/// const_assert_eq!(EXPECTED, ACTUAL);
/// ```
#[macro_export]
macro_rules! const_assert_eq {
    ($left:expr, $right:expr) => {
        const _: () = assert!($left == $right);
    };
    ($left:expr, $right:expr, $message:literal) => {
        const _: () = assert!($left == $right, $message);
    };
}

/// Compile-time inequality assertion
///
/// Verifies that two constant expressions are not equal at compile time.
///
/// # Examples
///
/// ```rust
/// # use nestgate_core::const_assert_ne;
/// const PORT_A: u16 = 8080;
/// const PORT_B: u16 = 9090;
///
/// const_assert_ne!(PORT_A, PORT_B);
/// ```
#[macro_export]
macro_rules! const_assert_ne {
    ($left:expr, $right:expr) => {
        const _: () = assert!($left != $right);
    };
    ($left:expr, $right:expr, $message:literal) => {
        const _: () = assert!($left != $right, $message);
    };
}

// ==================== DOCUMENTATION ====================

/// # Migration Guide
///
/// ## From Runtime Assertions
///
/// ```rust,ignore
/// // ❌ OLD: Runtime assertion in test
/// #[test]
/// fn test_timeout_hierarchy() {
///     assert!(TIMEOUT_CONNECT < TIMEOUT_REQUEST);
///     assert!(TIMEOUT_REQUEST < TIMEOUT_LONG);
/// }
/// ```
///
/// ```rust,ignore
/// // ✅ NEW: Compile-time assertion in module
/// const_assert!(TIMEOUT_CONNECT < TIMEOUT_REQUEST);
/// const_assert!(TIMEOUT_REQUEST < TIMEOUT_LONG);
///
/// #[test]
/// fn test_timeout_values() {
///     // Test actual values, not relationships
///     assert_eq!(TIMEOUT_CONNECT, 5_000);
///     assert_eq!(TIMEOUT_REQUEST, 30_000);
/// }
/// ```
///
/// ## Benefits of Migration
///
/// 1. **Earlier Error Detection**: Failures at compile time, not test time
/// 2. **Zero Runtime Cost**: No test execution overhead
/// 3. **Self-Documenting**: Constraints visible in constant definitions
/// 4. **Clippy Compliant**: No warnings about constant conditions
/// 5. **Modern Rust**: Following idiomatic const programming patterns
///
/// ## When to Use
///
/// - ✅ Verifying constant relationships (sizes, limits, hierarchies)
/// - ✅ Ensuring compile-time configuration validity
/// - ✅ Documenting invariants that must hold
/// - ❌ Testing runtime behavior or computed values
/// - ❌ Checking user input or external data
///
/// ## Performance Impact
///
/// **Zero**: All checks happen at compile time. The generated code contains
/// no runtime assertions, branches, or overhead.
pub struct MigrationGuide;

// ==================== DEMONSTRATION ====================

#[cfg(test)]
mod tests {
    // Example constants for demonstration
    const SMALL: usize = 10;
    const MEDIUM: usize = 100;
    const LARGE: usize = 1000;

    // Compile-time verifications
    const_assert!(SMALL < MEDIUM);
    const_assert!(MEDIUM < LARGE);
    const_assert!(SMALL < LARGE);

    const_assert_eq!(SMALL * 10, MEDIUM);
    const_assert_ne!(SMALL, LARGE);

    #[test]
    fn test_const_assertions_work() {
        // These assertions were verified at compile time
        // This test just documents that the compilation succeeded
        assert_eq!(SMALL, 10);
        assert_eq!(MEDIUM, 100);
        assert_eq!(LARGE, 1000);
    }

    #[test]
    fn test_runtime_vs_compile_time() {
        // ❌ OLD WAY: Runtime assertion (clippy warning)
        // assert!(SMALL < MEDIUM);

        // ✅ NEW WAY: Already verified at compile time (see above)
        // No runtime cost, no clippy warning

        // Runtime tests should test actual behavior, not constants
        let runtime_small = 10usize;
        let runtime_medium = 100usize;
        assert!(runtime_small < runtime_medium);
    }
}
