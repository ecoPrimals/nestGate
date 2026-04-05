// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **TEST TEMPLATES AND MACROS**
//
// Provides standard test patterns and macros for consistent test writing
// across the NestGate codebase.

/// Standard unit test pattern with setup and cleanup
///
/// # Example
/// ```ignore
/// unit_test!(test_my_function, {
///     let result = my_function(42);
///     assert_eq!(result, expected);
/// });
/// ```
#[macro_export]
macro_rules! unit_test {
    ($name:ident, $body:expr) => {
        #[test]
        fn $name() {
            // Setup test environment
            let _guard = $crate::common::test_environment::setup();

            // Execute test body
            $body

            // Cleanup handled by guard drop
        }
    };
}

/// Standard async unit test pattern
///
/// # Example
/// ```ignore
/// async_unit_test!(test_async_function, {
///     let result = async_function().await;
///     assert!(result.is_ok());
/// });
/// ```
#[macro_export]
macro_rules! async_unit_test {
    ($name:ident, $body:expr) => {
        #[tokio::test]
        async fn $name() {
            // Setup test environment
            let _guard = $crate::common::test_environment::setup();

            // Execute test body
            $body

            // Cleanup handled by guard drop
        }
    };
}

/// Standard integration test pattern with full environment setup
///
/// # Example
/// ```ignore
/// integration_test!(test_service_integration, {
///     let service = MyService::new(env.config()).await?;
///     let result = service.perform_action().await?;
///     assert!(result.is_valid());
///     Ok(())
/// });
/// ```
#[macro_export]
macro_rules! integration_test {
    ($name:ident, $body:expr) => {
        #[tokio::test]
        async fn $name() -> Result<(), Box<dyn std::error::Error>> {
            // Setup integration test environment
            let env = $crate::common::test_environment::integration_setup().await?;

            // Execute test body
            let result: Result<(), Box<dyn std::error::Error>> = async { $body }.await;

            // Cleanup
            env.teardown().await?;

            result
        }
    };
}

/// Standard E2E test pattern with scenario runner
///
/// # Example
/// ```ignore
/// e2e_test!(test_complete_workflow, {
///     runner.step("Create pool", |ctx| async {
///         ctx.create_pool("test-pool").await
///     }).await?;
///     Ok(())
/// });
/// ```
#[macro_export]
macro_rules! e2e_test {
    ($name:ident, $body:expr) => {
        #[tokio::test]
        #[ignore] // Run explicitly with --ignored
        async fn $name() -> Result<(), Box<dyn std::error::Error>> {
            let runner = $crate::e2e::framework::Runner::new().await?;

            // Execute test body
            let result: Result<(), Box<dyn std::error::Error>> = async { $body }.await;

            // Runner cleanup happens automatically
            result
        }
    };
}

/// Property-based test helper
///
/// # Example
/// ```ignore
/// property_test!(test_config_roundtrip, |port: u16| {
///     port > 1024 && port < 65535
/// });
/// ```
#[cfg(feature = "proptest")]
#[macro_export]
macro_rules! property_test {
    ($name:ident, $prop:expr) => {
        #[test]
        fn $name() {
            use proptest::prelude::*;
            proptest!($prop);
        }
    };
}

/// Parameterized test helper
///
/// # Example
/// ```ignore
/// #[rstest]
/// #[case(1024, true)]
/// #[case(65535, true)]
/// #[case(0, false)]
/// fn test_port_validation(#[case] port: u16, #[case] expected: bool) {
///     assert_eq!(is_valid_port(port), expected);
/// }
/// ```
pub use rstest::rstest;

/// Assert that a Result is Ok and return the value
#[macro_export]
macro_rules! assert_ok {
    ($result:expr) => {{
        match $result {
            Ok(val) => val,
            Err(e) => panic!("Expected Ok, got Err: {:?}", e),
        }
    }};
    ($result:expr, $msg:expr) => {{
        match $result {
            Ok(val) => val,
            Err(e) => panic!("{}: {:?}", $msg, e),
        }
    }};
}

/// Assert that a Result is Err
#[macro_export]
macro_rules! assert_err {
    ($result:expr) => {{
        match $result {
            Ok(val) => panic!("Expected Err, got Ok: {:?}", val),
            Err(e) => e,
        }
    }};
    ($result:expr, $msg:expr) => {{
        match $result {
            Ok(val) => panic!("{}: {:?}", $msg, val),
            Err(e) => e,
        }
    }};
}

/// Assert that a value matches a pattern
#[macro_export]
macro_rules! assert_matches {
    ($value:expr, $pattern:pat) => {
        match $value {
            $pattern => (),
            ref v => panic!(
                "Value {:?} does not match pattern {}",
                v,
                stringify!($pattern)
            ),
        }
    };
    ($value:expr, $pattern:pat, $msg:expr) => {
        match $value {
            $pattern => (),
            ref v => panic!("{}: {:?}", $msg, v),
        }
    };
}

/// Create a test configuration quickly
#[macro_export]
macro_rules! test_config {
    () => {
        $crate::common::test_config::create_test_config()
    };
    (production_like) => {
        $crate::common::test_config::create_production_like_config()
    };
}

/// Create a temporary test directory that auto-cleans
#[macro_export]
macro_rules! temp_dir {
    () => {{
        use tempfile::TempDir;
        TempDir::new().expect("Failed to create temp dir")
    }};
    ($prefix:expr) => {{
        use tempfile::Builder;
        Builder::new()
            .prefix($prefix)
            .tempdir()
            .expect("Failed to create temp dir")
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_ok_macro() {
        let result: Result<i32, &str> = Ok(42);
        assert_eq!(assert_ok!(result), 42);
    }

    #[test]
    #[should_panic(expected = "Expected Ok, got Err")]
    fn test_assert_ok_panics_on_err() {
        let result: Result<i32, &str> = Err("error");
        let _ = assert_ok!(result);
    }

    #[test]
    fn test_assert_err_macro() {
        let result: Result<i32, &str> = Err("error");
        assert_eq!(assert_err!(result), "error");
    }

    #[test]
    #[should_panic(expected = "Expected Err, got Ok")]
    fn test_assert_err_panics_on_ok() {
        let result: Result<i32, &str> = Ok(42);
        let _ = assert_err!(result);
    }
}
