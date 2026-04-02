# NestGate Advanced Unwrap Migrator v0.3.0

🚀 **Context-Aware Migration Tool for Safe Error Handling**

Sophisticated tool for migrating unsafe panic patterns (`unwrap()`, `expect()`, `panic!()`, etc.) to robust error handling using NestGate's unified error system.

## Features

- ✅ **Smart Pattern Detection**: Context-aware analysis of panic patterns
- 🔧 **Automatic Fixes**: Safe automated migration with confidence scoring
- 🧪 **Test Function Signature Fixer**: NEW! Automatically fixes test function signatures when using `SafeUnwrap`
- 📊 **Comprehensive Reporting**: Detailed HTML/Markdown/JSON reports
- 🎯 **Team-Friendly**: Interactive mode for collaborative migration
- 🏠 **NestGate-Specific**: Deeply integrated with NestGate error patterns

## Installation

```bash
cargo build --package unwrap-migrator --release
```

The binary will be available at: `target/release/unwrap-migrator`

## Quick Start

### Analyze Current State

```bash
# Analyze entire codebase
cargo run --package unwrap-migrator -- --analyze

# Analyze specific directory
cargo run --package unwrap-migrator -- code/crates/nestgate-core --analyze

# Include test files in analysis
cargo run --package unwrap-migrator -- --analyze --include-tests
```

### Fix Test Function Signatures (NEW!)

When you use `SafeUnwrap` with the `?` operator in test functions, they need to return `crate::Result<()>`:

```bash
# Fix all test function signatures
cargo run --package unwrap-migrator -- --fix-test-signatures

# Fix specific directory
cargo run --package unwrap-migrator -- code/crates/nestgate-core --fix-test-signatures --verbose
```

**What it fixes:**

```rust
// BEFORE:
#[test]
fn test_something() {
    let value = operation().safe_unwrap(ErrorCategory::Test, "context")?;
    assert_eq!(value, expected);
}

// AFTER:
#[test]
fn test_something() -> crate::Result<()> {
    let value = operation().safe_unwrap(ErrorCategory::Test, "context")?;
    assert_eq!(value, expected);
    Ok(())
}
```

**Also fixes:**

- ❌ `Result<(), NestGateError>` → ✅ `crate::Result<()>`
- ❌ Missing `Ok(())` at function end → ✅ Adds `Ok(())`
- ❌ Missing imports in test module → ✅ Adds `use crate::error::{ErrorCategory, SafeUnwrap};`

### Apply Automatic Fixes

```bash
# Apply safe fixes (standard mode)
cargo run --package unwrap-migrator -- --fix

# Apply safe fixes with advanced pattern detection
cargo run --package unwrap-migrator -- --fix --advanced

# Apply fixes with confidence threshold
cargo run --package unwrap-migrator -- --fix --confidence 90
```

### Generate Reports

```bash
# Generate markdown report
cargo run --package unwrap-migrator -- --report

# Generate HTML report
cargo run --package unwrap-migrator -- --report --format html --output report.html

# Generate JSON report for tooling
cargo run --package unwrap-migrator -- --report --format json --output report.json
```

## Command-Line Options

### Analysis Options

- `--analyze`, `-a`: Analyze patterns without making changes
- `--include-tests`: Include test files in analysis (normally excluded)
- `--verbose`, `-v`: Show detailed analysis output
- `--quiet`, `-q`: Minimal output, errors only

### Fix Options

- `--fix`, `-f`: Apply safe automatic fixes
- `--fix-test-signatures`: Fix test function signatures for SafeUnwrap usage *(NEW!)*
- `--interactive`, `-i`: Interactive mode - review each fix (coming soon)
- `--advanced`: Use advanced pattern detection and migration
- `--nestgate-mode`: Enable NestGate-specific patterns
- `--confidence LEVEL`: Minimum confidence level (50-100, default: 80)

### Filtering Options

- `--priority LEVEL`: Priority filter: `high`, `medium`, `low`, `all` (default: `medium`)
- `--exclude PATTERN`: Exclude files matching regex pattern (can be used multiple times)

### Output Options

- `--report`, `-r`: Generate detailed report
- `--format FORMAT`: Output format: `json`, `markdown`, `html` (default: `markdown`)
- `--output FILE`, `-o FILE`: Save report to file

## Usage Examples

### Example 1: Initial Assessment

```bash
# Scan codebase and get risk assessment
cargo run --package unwrap-migrator -- --analyze --verbose

# Output:
# 📊 Analysis Results:
#    📁 Files scanned: 1452
#    🎯 Total patterns found: 342
#    ⚠️  Unwrap calls: 234
#    📝 Expect calls: 87
#    💥 Panic calls: 12
#    📋 todo!() macro occurrences: 9
#    🚫 Unimplemented calls: 0
# 🎯 Risk Assessment: 🟠 HIGH
```

### Example 2: Fix Test Signatures After SafeUnwrap Migration

```bash
# After migrating production code to SafeUnwrap, fix test signatures
cargo run --package unwrap-migrator -- code/crates/nestgate-core --fix-test-signatures --verbose

# Output:
# 🔍 Scanning for test functions needing signature fixes...
# 📁 Found 3 fixes needed in code/crates/nestgate-core/src/config/mod.rs
# ✅ Fixed test function test_config_serialization_deserialization
# ✅ Fixed test function test_config_can_be_sent_between_threads
# ✅ Added imports to test module
# 
# ╔═══════════════════════════════════════════════════════╗
# ║      Test Function Signature Fixer - Complete       ║
# ╚═══════════════════════════════════════════════════════╝
# ✅ Fixed 15 test functions across 4 files
```

### Example 3: Batch Migration with Confidence

```bash
# Apply fixes with high confidence threshold
cargo run --package unwrap-migrator -- code/crates/nestgate-core --fix --confidence 90 --advanced

# Then fix any test signature issues
cargo run --package unwrap-migrator -- code/crates/nestgate-core --fix-test-signatures

# Format the changes
cargo fmt --all

# Verify tests still pass
cargo test
```

### Example 4: Generate Migration Report

```bash
# Generate comprehensive HTML report
cargo run --package unwrap-migrator -- --report --format html --output unwrap-report.html --include-tests
```

## Migration Workflow

### Phase 1: Assessment

1. **Initial Scan**:
   ```bash
   cargo run --package unwrap-migrator -- --analyze --include-tests --verbose
   ```

2. **Generate Baseline Report**:
   ```bash
   cargo run --package unwrap-migrator -- --report --format markdown --output BASELINE.md
   ```

### Phase 2: Production Code Migration

1. **Fix High-Priority Patterns**:
   ```bash
   cargo run --package unwrap-migrator -- --fix --priority high --confidence 90
   ```

2. **Verify Compilation**:
   ```bash
   cargo check --workspace
   ```

3. **Fix Test Function Signatures**:
   ```bash
   cargo run --package unwrap-migrator -- --fix-test-signatures --verbose
   ```

4. **Run Tests**:
   ```bash
   cargo test --workspace
   ```

### Phase 3: Remaining Patterns

1. **Fix Medium-Priority Patterns**:
   ```bash
   cargo run --package unwrap-migrator -- --fix --priority medium --confidence 85
   ```

2. **Fix Test Signatures Again**:
   ```bash
   cargo run --package unwrap-migrator -- --fix-test-signatures
   ```

3. **Format & Verify**:
   ```bash
   cargo fmt --all
   cargo check --workspace
   cargo test --workspace
   ```

### Phase 4: Final Verification

1. **Generate Final Report**:
   ```bash
   cargo run --package unwrap-migrator -- --report --format html --output FINAL_REPORT.html
   ```

2. **Compare Progress**:
   ```bash
   diff BASELINE.md FINAL_REPORT.md
   ```

## Patterns Detected

### Safe for Auto-Migration

- `.unwrap()` in Result contexts with clear error handling
- `.expect("msg")` with proper messages in safe contexts
- Simple unwraps on operations that rarely fail

### Requires Review

- `.unwrap()` on complex operations
- `panic!()` in production code
- `.expect()` on critical paths
- Nested error handling

### Manual Migration Only

- `todo!()` and `unimplemented!()` in production
- Complex panic scenarios
- Multi-layered error contexts

## NestGate-Specific Patterns

The migrator understands NestGate's error system:

```rust
// Detects and migrates to SafeUnwrap
result.unwrap()
// Becomes:
result.safe_unwrap(ErrorCategory::Internal, "operation description")?

// Detects and fixes test signatures
#[test]
fn test_something() {
    let value = operation().safe_unwrap(...)?;
    // Missing return type!
}
// Becomes:
#[test]
fn test_something() -> crate::Result<()> {
    let value = operation().safe_unwrap(...)?;
    Ok(())
}
```

## Error Categories

When migrating to `SafeUnwrap`, the tool suggests appropriate categories:

- `Configuration`: Setup and config errors
- `Network`: Connectivity issues
- `Storage`: Filesystem/database errors
- `Security`: Auth and security errors
- `Validation`: Input validation errors
- `Internal`: Internal system errors
- `Adapter`: Universal adapter errors
- `Zfs`: ZFS-specific errors

## Test Function Signature Fixer Details

### What Gets Fixed

1. **Missing Return Type**:
   - Detects: `#[test]` or `#[tokio::test]` functions using `safe_unwrap` with `?`
   - Adds: `-> crate::Result<()>` to function signature

2. **Incorrect Return Type**:
   - Detects: `Result<(), NestGateError>` (2 generic arguments)
   - Fixes: Changes to `crate::Result<()>` (1 generic argument)

3. **Missing Ok(()) Return**:
   - Detects: Functions without explicit `Ok(())` at end
   - Adds: `Ok(())` with proper indentation

4. **Missing Imports**:
   - Detects: Test modules without error imports
   - Adds: `use crate::error::{ErrorCategory, SafeUnwrap};`

### Supported Test Attributes

- `#[test]`
- `#[tokio::test]`
- `#[serial_test::serial]`

### Safety

The fixer is conservative and only modifies:
- Functions explicitly marked as tests
- Functions that use `safe_unwrap` with `?` operator
- Test modules that are clearly structured with `#[cfg(test)]`

## Performance

- **Scanning**: ~1,450 files in < 2 seconds
- **Analysis**: Full codebase analysis in < 5 seconds
- **Migration**: Batch fixes applied in < 10 seconds
- **Test Signature Fixing**: 50+ functions in < 3 seconds

## Safety Guarantees

1. **Non-Destructive**: Original code is preserved until explicitly fixed
2. **Confidence Scoring**: Each fix is scored based on context analysis
3. **Selective Application**: Only patterns above confidence threshold are auto-fixed
4. **Test Integration**: Encourages running tests after each batch
5. **Git-Friendly**: Changes are easy to review and diff

## Development & Testing

```bash
# Run migrator tests
cargo test --package unwrap-migrator

# Check for linting issues
cargo clippy --package unwrap-migrator

# Format code
cargo fmt --package unwrap-migrator
```

## Architecture

```
unwrap-migrator/
├── src/
│   ├── main.rs                          # CLI and orchestration
│   ├── test_function_fixer.rs          # NEW! Test signature fixer
│   ├── advanced_panic_migrator.rs      # Advanced pattern detection
│   ├── enhanced_migrator.rs            # Standard migration
│   ├── nestgate_patterns.rs            # NestGate-specific patterns
│   ├── refined_nestgate_migrator.rs    # Refined migration logic
│   ├── scanner.rs                       # File scanning
│   ├── reporter.rs                      # Report generation
│   └── systematic_migrator.rs          # Systematic batch processing
├── Cargo.toml
└── README.md
```

## Contributing

1. Add new patterns to `nestgate_patterns.rs`
2. Extend detection logic in `advanced_panic_migrator.rs`
3. Add test cases in each module's `#[cfg(test)]` section
4. Update documentation

## Version History

### v0.3.0 (October 27, 2025)

- ✨ NEW: Test function signature fixer (`--fix-test-signatures`)
- 🔧 Automatically fixes `SafeUnwrap` test function patterns
- 📦 Adds missing imports to test modules
- 🎯 Fixes incorrect `Result<(), NestGateError>` to `crate::Result<()>`
- ✅ Adds `Ok(())` returns to test functions
- 📚 Comprehensive documentation and examples

### v0.2.0

- Advanced pattern detection
- NestGate-specific error categories
- Confidence-based fixing
- HTML/JSON/Markdown reports

### v0.1.0

- Initial release
- Basic unwrap/expect detection
- Simple migration patterns

## License

AGPL-3.0-only

## Support

For issues, questions, or contributions:
- GitHub: https://github.com/eastgate/nestgate
- Docs: See `/docs/UNWRAP_MIGRATION_PLAN_STRATEGIC.md`

---

**Built with ❤️ for the NestGate ecosystem**

