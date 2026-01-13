# 🔧 Unwrap Migration Guide
**Date**: January 13, 2026  
**Goal**: Replace `.unwrap()` with proper error handling

---

## 📊 **AUDIT SUMMARY**

### Statistics
- **Total unwraps**: 2,889
- **Test code**: 665 (23%) ✅ Acceptable
- **Examples**: 13 (0.4%) ✅ Acceptable
- **Production code**: 1,009 (35%) ⚠️ **Needs migration**
- **Other**: 1,202 (41%) - Benches, tools, etc.

### Production Code Breakdown
- **153 files** affected
- **Average**: 6.6 unwraps per file
- **Hotspots**: RPC modules, config parsing, socket handling

---

## 🎯 **MIGRATION PATTERNS**

### Pattern 1: Option::unwrap() → ?

```rust
// ❌ BEFORE: Can panic
let value = some_option.unwrap();

// ✅ AFTER: Propagate error
let value = some_option.ok_or_else(|| 
    NestGateError::configuration_error("field_name", "value is required")
)?;

// Or for internal errors:
let value = some_option.ok_or_else(|| 
    NestGateError::internal("expected value to be Some")
)?;
```

### Pattern 2: Result::unwrap() → ?

```rust
// ❌ BEFORE: Can panic
let config = parse_config().unwrap();

// ✅ AFTER: Direct propagation
let config = parse_config()?;

// Or with context:
let config = parse_config()
    .map_err(|e| NestGateError::configuration_error(
        "config", 
        format!("failed to parse config: {}", e)
    ))?;
```

### Pattern 3: Environment Variables

```rust
// ❌ BEFORE: Panics if not set
let port = env::var("PORT").unwrap();

// ✅ AFTER: Use default
let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

// Or with proper error:
let port = env::var("PORT")
    .map_err(|_| NestGateError::configuration_error(
        "PORT", 
        "PORT environment variable not set"
    ))?;
```

### Pattern 4: String Parsing

```rust
// ❌ BEFORE: Panics on invalid input
let num: u16 = value.parse().unwrap();

// ✅ AFTER: Proper error handling
let num: u16 = value.parse()
    .map_err(|_| NestGateError::invalid_input_with_field(
        "port",
        format!("invalid port number: {}", value)
    ))?;
```

### Pattern 5: Lock Poisoning (Mutex/RwLock)

```rust
// ❌ BEFORE: Panics on poison
let data = lock.read().unwrap();

// ✅ AFTER: Handle poison gracefully
let data = lock.read().map_err(|e| {
    NestGateError::internal(format!("lock poisoned: {}", e))
})?;
```

### Pattern 6: Index/Get Operations

```rust
// ❌ BEFORE: Panics if index out of bounds
let item = items[index];

// ✅ AFTER: Safe access
let item = items.get(index).ok_or_else(|| 
    NestGateError::internal(format!("index {} out of bounds", index))
)?;
```

---

## 📋 **MIGRATION PRIORITIES**

### Priority 1: Public API Functions (HIGH)
**Impact**: External callers  
**Count**: ~100 unwraps  
**Reason**: API functions must never panic

**Files**:
- `rpc/jsonrpc_server.rs`
- `rpc/tarpc_server.rs`
- `rpc/unix_socket_server.rs`
- Public trait implementations

### Priority 2: Configuration Parsing (HIGH)
**Impact**: Startup failures  
**Count**: ~150 unwraps  
**Reason**: Should return clear configuration errors

**Files**:
- `config/*.rs`
- `constants/*.rs`
- Environment variable parsing

### Priority 3: Error Handling Paths (MEDIUM)
**Impact**: Error reporting  
**Count**: ~200 unwraps  
**Reason**: Errors shouldn't cause panics

**Files**:
- `error/*.rs`
- Error conversion functions
- Logging and diagnostics

### Priority 4: Internal Functions (LOW)
**Impact**: Internal invariants  
**Count**: ~559 unwraps  
**Reason**: Lower priority, often valid invariants

**Approach**: Review case-by-case

---

## 🚫 **ACCEPTABLE UNWRAPS**

### Case 1: Test Code ✅
```rust
#[test]
fn test_something() {
    let value = get_value().unwrap();  // ✅ OK in tests
    assert_eq!(value, 42);
}
```

### Case 2: Known Invariants (Document!)
```rust
// ✅ OK if truly guaranteed
let value = option.unwrap(); // SAFETY: Always Some due to initialization above
```
**Requirement**: Must add comment explaining why safe

### Case 3: Static/Const Initialization
```rust
// ✅ OK for compile-time constants
static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    Config::from_env().unwrap()  // OK - fail-fast at startup
});
```

---

## 🔄 **MIGRATION WORKFLOW**

### Step 1: Identify Context
1. Is it test code? → Leave it
2. Is it a public API? → Priority 1
3. Is it configuration? → Priority 2
4. Is it an error path? → Priority 3
5. Is it internal logic? → Review

### Step 2: Choose Pattern
1. Option → Use `ok_or_else()` or `ok_or()`
2. Result → Use `?` directly
3. Env var → Use `unwrap_or_else()` or proper error
4. Parse → Add context with `map_err()`
5. Lock → Handle poison error
6. Index → Use `get()` instead

### Step 3: Add Error Context
```rust
// ❌ BAD: Generic error
.ok_or_else(|| NestGateError::internal("failed"))?

// ✅ GOOD: Specific context
.ok_or_else(|| NestGateError::configuration_error(
    "api_endpoint",
    "API endpoint not configured"
))?
```

### Step 4: Test
1. Compile and verify
2. Run affected tests
3. Check error messages are helpful

---

## 📝 **MIGRATION SCRIPT**

### Helper Function Pattern

```rust
/// Helper to convert Option to Result with context
#[inline]
fn require<T>(option: Option<T>, field: &str, context: &str) -> Result<T> {
    option.ok_or_else(|| NestGateError::configuration_error(field, context))
}

// Usage:
let value = require(
    some_option,
    "api_key", 
    "API key is required for authentication"
)?;
```

### Batch Migration Command

```bash
# Find all unwraps in production code (excluding tests)
rg '\.unwrap\(\)' --type rust code/crates/nestgate-core/src \
  | grep -v test \
  | wc -l
```

---

## 🎯 **SESSION GOALS**

### Phase 1: High Priority (This Session)
**Target**: 50-100 unwraps  
**Focus**: Public APIs + Configuration  
**Time**: 2-3 hours

**Files to tackle**:
1. `rpc/jsonrpc_server.rs` - Public RPC endpoints
2. `rpc/unix_socket_server.rs` - Socket handling
3. `rpc/socket_config.rs` - Configuration
4. `config/environment.rs` - Env var parsing

### Phase 2: Medium Priority (Next Session)
**Target**: 100-200 unwraps  
**Focus**: Error handling + Discovery  
**Time**: 3-4 hours

### Phase 3: Low Priority (Future)
**Target**: Remaining ~500 unwraps  
**Focus**: Internal logic (case-by-case review)  
**Time**: 4-6 hours

---

## 📊 **PROGRESS TRACKING**

### Metrics to Track
1. **Unwraps eliminated**: Count before/after
2. **Files modified**: Track progress
3. **Error handling improved**: Measure error context quality
4. **Tests passing**: Ensure no regressions

### Success Criteria
- ✅ Zero unwraps in public API functions
- ✅ Zero unwraps in configuration parsing
- ✅ All error paths return Result
- ✅ All error messages are helpful
- ✅ All tests still pass

---

## 🏆 **QUALITY STANDARDS**

### Error Messages Must Be:
1. **Specific**: Name the field/operation that failed
2. **Actionable**: Tell user what to fix
3. **Contextual**: Include relevant values
4. **Consistent**: Use standard error types

### Example: Good Error Messages
```rust
// ✅ EXCELLENT
NestGateError::configuration_error(
    "database_url",
    "DATABASE_URL environment variable not set. Please set it to your PostgreSQL connection string."
)

// ✅ GOOD
NestGateError::invalid_input_with_field(
    "port",
    format!("invalid port number '{}': must be between 1 and 65535", value)
)

// ❌ BAD
NestGateError::internal("failed")  // Too vague!
```

---

## 🔍 **BEFORE & AFTER EXAMPLES**

### Example 1: Configuration Parsing

```rust
// ❌ BEFORE
pub fn from_env() -> Self {
    Self {
        api_key: env::var("API_KEY").unwrap(),
        endpoint: env::var("ENDPOINT").unwrap(),
    }
}

// ✅ AFTER
pub fn from_env() -> Result<Self> {
    Ok(Self {
        api_key: env::var("API_KEY")
            .map_err(|_| NestGateError::configuration_error(
                "API_KEY",
                "API_KEY environment variable is required"
            ))?,
        endpoint: env::var("ENDPOINT")
            .map_err(|_| NestGateError::configuration_error(
                "ENDPOINT",
                "ENDPOINT environment variable is required"  
            ))?,
    })
}
```

### Example 2: Lock Access

```rust
// ❌ BEFORE
pub fn get_value(&self) -> String {
    self.data.read().unwrap().clone()
}

// ✅ AFTER
pub fn get_value(&self) -> Result<String> {
    self.data.read()
        .map_err(|e| NestGateError::internal(
            format!("data lock poisoned: {}", e)
        ))
        .map(|guard| guard.clone())
}
```

### Example 3: Optional Value

```rust
// ❌ BEFORE
pub fn process(&self, id: &str) -> String {
    self.items.get(id).unwrap().clone()
}

// ✅ AFTER
pub fn process(&self, id: &str) -> Result<String> {
    self.items.get(id)
        .ok_or_else(|| NestGateError::not_found(
            format!("item '{}' not found", id)
        ))
        .map(|item| item.clone())
}
```

---

## ⚠️  **COMMON PITFALLS**

### Pitfall 1: Losing Error Context
```rust
// ❌ BAD: Error context lost
.map_err(|_| NestGateError::internal("failed"))?

// ✅ GOOD: Preserve context
.map_err(|e| NestGateError::internal(
    format!("failed to parse config: {}", e)
))?
```

### Pitfall 2: Over-Generic Errors
```rust
// ❌ BAD
.ok_or_else(|| NestGateError::internal("error"))?

// ✅ GOOD
.ok_or_else(|| NestGateError::configuration_error(
    "field_name",
    "specific reason why this failed"
))?
```

### Pitfall 3: Unwrap in Error Handling
```rust
// ❌ BAD: Unwrap in error handling!
.map_err(|e| {
    log::error!("Error: {}", e);
    NestGateError::internal(e.to_string()).unwrap()  // ❌❌❌
})?

// ✅ GOOD
.map_err(|e| {
    log::error!("Error: {}", e);
    NestGateError::internal(e.to_string())
})?
```

---

## 🎓 **LEARNING RESOURCES**

### Rust Error Handling Best Practices
- [The Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Error Handling Survey](https://blog.burntsushi.net/rust-error-handling/)
- [anyhow vs thiserror](https://nick.groenen.me/posts/rust-error-handling/)

### NestGate Error Types
- `NestGateError::configuration_error(field, reason)` - Config issues
- `NestGateError::invalid_input_with_field(field, reason)` - Bad input
- `NestGateError::not_found(what)` - Resource not found
- `NestGateError::internal(reason)` - Internal errors
- `NestGateError::api(reason)` - API errors

---

## ✅ **COMPLETION CHECKLIST**

### Phase 1 (This Session)
- [ ] Migrate RPC module unwraps
- [ ] Migrate socket configuration unwraps
- [ ] Migrate environment parsing unwraps
- [ ] Add helper functions for common patterns
- [ ] Verify all tests pass
- [ ] Document patterns used

### Phase 2 (Next Session)
- [ ] Migrate error handling unwraps
- [ ] Migrate discovery module unwraps
- [ ] Add integration tests for error cases
- [ ] Review remaining unwraps

### Phase 3 (Future)
- [ ] Case-by-case review of internal unwraps
- [ ] Add SAFETY comments where unwrap is justified
- [ ] Final audit and documentation

---

**Guide Created**: January 13, 2026  
**Status**: Ready for migration  
**Next**: Start with RPC modules (Priority 1)
