///! # Environment Isolation for Tests
//!
//! Safe environment variable access for concurrent tests.
//!
//! ## Anti-Pattern: Global Environment Pollution
//!
//! ```rust,ignore
//! // ❌ BAD: Pollutes global environment
//! #[test]
//! fn test() {
//!     nestgate_core::env_process::set_var("VAR", "value");
//!     // test code
//!     nestgate_core::env_process::remove_var("VAR"); // May not run if panic!
//! }
//! ```
//!
//! ## Modern Pattern: Isolated Environment
//!
//! ```rust,ignore
//! // ✅ GOOD: Isolated, auto-restored
//! #[test]
//! fn test() {
//!     let env = IsolatedEnv::new();
//!     env.set("VAR", "value");
//!     // test code
//! } // Auto-restored on drop!
//! ```

pub use temp_env::{with_var, with_vars, async_with_var, async_with_vars};

/// Convenience wrapper for isolated environment
///
/// # Example
///
/// ```rust,ignore
/// #[test]
/// fn test_with_env() {
///     temp_env::with_var("NESTGATE_PORT", Some("8080"), || {
///         // Test code
///     }); // Auto-restored!
/// }
/// ```
pub struct IsolatedEnv;

impl IsolatedEnv {
    /// Run a test with isolated environment variables (sync)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// IsolatedEnv::run("MY_VAR", "value", || {
    ///     // Test code
    /// });
    /// ```
    pub fn run<F, R>(key: &str, value: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        with_var(key, Some(value), f)
    }

    /// Run an async test with isolated environment variables
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// IsolatedEnv::run_async("MY_VAR", "value", async {
    ///     // Async test code
    /// }).await;
    /// ```
    pub async fn run_async<F, Fut, R>(key: &str, value: &str, f: F) -> R
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        async_with_var(key, Some(value), f()).await
    }

    /// Run with multiple environment variables (sync)
    pub fn run_many<F, R>(vars: Vec<(&str, Option<&str>)>, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        with_vars(vars, f)
    }

    /// Run with multiple environment variables (async)
    pub async fn run_many_async<F, Fut, R>(
        vars: Vec<(&str, Option<&str>)>,
        f: F,
    ) -> R
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = R>,
    {
        async_with_vars(vars, f()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isolated_env_run() {
        IsolatedEnv::run("TEST_VAR", "test_value", || {
            assert_eq!(std::env::var("TEST_VAR").unwrap(), "test_value");
        });

        // Should be restored
        assert!(std::env::var("TEST_VAR").is_err());
    }

    #[tokio::test]
    async fn test_isolated_env_run_async() {
        IsolatedEnv::run_async("TEST_VAR_ASYNC", "async_value", async {
            assert_eq!(
                std::env::var("TEST_VAR_ASYNC").unwrap(),
                "async_value"
            );
        })
        .await;

        // Should be restored
        assert!(std::env::var("TEST_VAR_ASYNC").is_err());
    }

    #[test]
    fn test_isolated_env_many() {
        IsolatedEnv::run_many(
            vec![
                ("VAR1", Some("value1")),
                ("VAR2", Some("value2")),
            ],
            || {
                assert_eq!(std::env::var("VAR1").unwrap(), "value1");
                assert_eq!(std::env::var("VAR2").unwrap(), "value2");
            },
        );

        // Should be restored
        assert!(std::env::var("VAR1").is_err());
        assert!(std::env::var("VAR2").is_err());
    }
}

