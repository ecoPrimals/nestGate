# Contributing to NestGate

## Project Status

NestGate is in active development. Current metrics:

- **Build**: 24/24 workspace members, 0 errors
- **Tests**: 8,555 lib / 12,105 total passing, 0 failures
- **Coverage**: ~80% line (llvm-cov)
- **Clippy**: ZERO warnings — full workspace `cargo clippy --workspace --all-targets -- -D warnings` clean
- **Safety**: `#![forbid(unsafe_code)]` on all 22 crate roots except `nestgate-env-process-shim`
- **Serial tests**: Zero `#[serial]` outside chaos suite

See [STATUS.md](./STATUS.md) for full metrics.

---

## Architecture Principles

1. **UniBin**: Single `nestgate` binary per platform
2. **ecoBin**: Pure Rust, socket-only default, cross-compilation
3. **Primal Self-Knowledge**: No hardcoded primal names or ports — discover at runtime
4. **Capability-Based Discovery**: Find primals by capability, not by name
5. **JSON-RPC 2.0 + tarpc**: Dual IPC with semantic method naming
6. **Isomorphic IPC**: Auto-adapts Unix sockets or TCP based on platform
7. **Result over panic**: Zero `unwrap()/expect()` in production code
8. **File size limit**: Maximum 1000 lines per production file

### Unified Systems

| System | Location | Usage |
|--------|----------|-------|
| Error handling | `nestgate-types/src/error/` | `NestGateError` variants |
| Configuration | `nestgate-config/src/config/` | Environment-driven, XDG-compliant |
| Constants | `nestgate-config/src/constants/` | Unified constants |
| Traits | `nestgate-core/src/traits/` | Canonical service traits |
| Discovery | `nestgate-discovery/src/` | Capability-based IPC |

---

## Development Workflow

### Setup

```bash
git clone <repo-url>
cd nestgate

# Verify build
cargo check --workspace --all-features

# Run tests
cargo test --workspace

# Verify linting
cargo clippy --all-targets --all-features -- -D warnings
```

### Code Standards

**Required**:

```rust
// Use Result for all fallible operations — no panic/unwrap in production
pub async fn operation() -> Result<Data, NestGateError> {
    Ok(data)
}

// Environment-driven configuration
let port = std::env::var("NESTGATE_API_PORT")
    .unwrap_or_else(|_| "8080".to_string());

// Capability-based discovery (not hardcoded names)
let crypto = discovery.find("crypto").await?;
```

**Prohibited in production code**:

```rust
// NO: panic!, unwrap(), expect(), todo!(), unimplemented!()
let value = map.get("key").unwrap();         // Use .ok_or_else()?
panic!("something failed");                   // Use return Err(...)
todo!();                                      // Use return Err(NestGateError::not_implemented(...))
```

**Test code**: `unwrap()` and `expect()` are acceptable in tests for clarity.

### File Size

All production files must be under 1000 lines. If your file exceeds this:
1. Extract tests into a separate `_tests.rs` file
2. Split modules by responsibility
3. Use composition over monolithic implementations

---

## Testing Standards

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // ...
    }

    #[tokio::test]
    async fn test_async_operation() -> Result<(), Box<dyn std::error::Error>> {
        // ...
        Ok(())
    }
}
```

### Environment Variable Isolation

Tests that mutate environment variables MUST use `temp_env` + `serial_test`:

```rust
#[test]
#[serial_test::serial]
fn test_with_env_vars() {
    temp_env::with_var("NESTGATE_API_PORT", Some("9999"), || {
        // ... test logic ...
        // Environment is automatically restored when the closure returns
    });
}
```

### Coverage

```bash
cargo llvm-cov --workspace --summary-only --ignore-filename-regex 'tools/'
```

The `tools/` directory is excluded from coverage — it contains development tooling, not production code.

---

## Pull Request Checklist

- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo test --workspace` passes (0 failures)
- [ ] No new `unwrap()/expect()` in production code
- [ ] All files under 1000 lines
- [ ] Tests use env-var save/restore if mutating environment
- [ ] New functionality has tests

---

## Contribution Areas

### High Priority

1. **Coverage**: Push toward 90% — see STATUS.md for gap analysis
2. **IMPLEMENTATION STUBs**: Evolve boilerplate DefaultService patterns to real logic
3. **Semantic method naming**: Align internal methods with `{domain}.{operation}` format

### Other Areas

- Documentation improvements
- Performance optimization
- Platform-specific testing
- ZFS integration tests (requires ZFS-capable environment)

---

## Getting Help

- [STATUS.md](./STATUS.md) — Current measured metrics
- [CAPABILITY_MAPPINGS.md](./CAPABILITY_MAPPINGS.md) — Primal capability reference
- [specs/](./specs/) — Protocol specifications
- [docs/](./docs/) — Architecture, guides, session archives

---

**Last Updated**: March 31, 2026
