# NestGate Testing Guide

**Last Updated**: April 6, 2026
**Tests**: ~11,820 passing, 0 failures, ~463 ignored
**Coverage**: ~80% line (workspace, all features)
**Goal**: Maintain high coverage with clean, maintainable tests

---

## Quick Stats

```
Tests passing:          ~11,820
Failures:               0
Ignored:                ~463 (mostly e2e/chaos; see tests/DISABLED_TESTS_REFERENCE.md)
Crates with tests:      Workspace-wide (23 members)
Clippy:                 cargo clippy --workspace --all-targets --all-features -- -D warnings PASS
```

---

## Running Tests

Primary command for the full suite (all crates, all features):

```bash
cargo test --workspace --all-features
```

Other useful invocations:

```bash
# Library tests only (often faster during development)
cargo test --workspace --all-features --lib

# Specific test name filter
cargo test --workspace --all-features test_name

# Run ignored tests (e2e, chaos, etc. — needs environment/setup)
cargo test --workspace --all-features -- --ignored

# With verbose output
cargo test --workspace --all-features -- --nocapture

# Single-threaded (for debugging)
cargo test --workspace --all-features -- --test-threads=1
```

### Coverage Reports

```bash
# Workspace coverage (requires cargo-llvm-cov)
cargo llvm-cov --workspace --all-features

# HTML report
cargo llvm-cov --workspace --all-features --html
```

---

## Test Organization

```
tests/
├── chaos/                   # Chaos / resilience scenarios (#[ignore])
├── common/                  # Shared utilities (config, mocks, test doubles)
├── integration/             # Multi-component / API integration
├── integration_test_suite/  # Comprehensive integration scenarios
├── performance/             # Performance and load-oriented tests
├── unit/                    # Focused unit-style tests
├── e2e_*.rs                 # End-to-end integration test files (top-level)
├── DISABLED_TESTS_REFERENCE.md  # Ignored test documentation
└── SLEEP_MIGRATION_GUIDE.md     # Patterns for eliminating sleep in tests
```

Crate-local tests remain under `code/crates/<crate>/tests/` and `#[cfg(test)]` modules inside each crate.

---

## Writing Tests

### Unit Test Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_with_valid_input_returns_expected() {
        let input = create_test_input();
        let result = my_function(input);
        assert!(result.is_ok());
        assert_eq!(result.expect("test"), expected_value);
    }

    #[test]
    fn function_returns_error_on_invalid_input() {
        let result = my_function(invalid_input());
        assert!(result.is_err());
    }
}
```

### Async Test Pattern

```rust
#[tokio::test]
async fn service_responds_to_health_check() {
    let config = test_config::create_test_config();
    let service = MyService::new(config).await.expect("test setup");
    let result = service.health_check().await;
    assert!(result.is_ok());
}
```

### Environment Isolation

```rust
use serial_test::serial;
use temp_env::with_vars;

#[test]
#[serial]
fn config_reads_from_environment() {
    with_vars(
        vec![("NESTGATE_API_PORT", Some("9999"))],
        || {
            let config = Config::from_env();
            assert_eq!(config.port, 9999);
        },
    );
}
```

---

## Testing Guidelines

**Do:**
- Write tests for new code
- Test edge cases and error conditions
- Use descriptive names: `test_config_builder_creates_valid_config`
- Keep tests focused (one concept per test)
- Use common helpers from `tests/common/`
- Use `temp_env` closures for environment isolation
- Test behavior, not implementation

**Don't:**
- Use `.unwrap()` in production code (tests can `expect("reason")`)
- Depend on test execution order
- Use hardcoded file paths (use `tempfile::tempdir()`)
- Leave flaky tests unfixed
- Duplicate test logic (extract to helpers)

---

## Coverage Targets

- **Core logic**: 90%+
- **API handlers**: 90%+
- **Storage operations**: 85%+
- **Utilities**: 80%+
- **Workspace minimum** (wateringHole standard): 80%

---

## Debugging Tests

```bash
# Single test with full output
cargo test --workspace --all-features test_name -- --nocapture

# With logging
RUST_LOG=debug cargo test --workspace --all-features test_name

# Single-threaded (avoids concurrency issues)
cargo test --workspace --all-features -- --test-threads=1
```

---

## Resources

**Internal:**
- [Disabled / ignored tests](DISABLED_TESTS_REFERENCE.md)
- [Sleep migration patterns](SLEEP_MIGRATION_GUIDE.md)

**External:**
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Tokio Testing Guide](https://tokio.rs/tokio/topics/testing)

---

**Status**: Workspace test suite green; ignored tests documented separately
**Coverage**: ~80% workspace (all features); re-measure after large changes
